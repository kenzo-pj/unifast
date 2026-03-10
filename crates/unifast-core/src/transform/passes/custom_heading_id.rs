use crate::ast::mdast::nodes::*;
use crate::util::small_map::SmallMap;

#[derive(Debug, Clone, Default)]
pub struct InlineAttributes {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: SmallMap<String, String>,
}

pub fn parse_inline_attributes(s: &str) -> Option<InlineAttributes> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let mut result = InlineAttributes::default();
    let mut has_content = false;

    for token in s.split_whitespace() {
        if let Some(id) = token.strip_prefix('#') {
            if !id.is_empty() {
                result.id = Some(id.to_string());
                has_content = true;
            }
        } else if let Some(class) = token.strip_prefix('.') {
            if !class.is_empty() {
                result.classes.push(class.to_string());
                has_content = true;
            }
        } else if let Some(eq_pos) = token.find('=') {
            let key = &token[..eq_pos];
            let value = &token[eq_pos + 1..];
            if !key.is_empty() {
                result.attrs.insert(key.to_string(), value.to_string());
                has_content = true;
            }
        }
    }

    if has_content { Some(result) } else { None }
}

fn find_trailing_brace_block(text: &str) -> Option<(usize, &str)> {
    let trimmed_end = text.trim_end();
    if !trimmed_end.ends_with('}') {
        return None;
    }

    let mut depth = 0i32;
    let mut brace_start = None;
    for (i, ch) in trimmed_end.char_indices().rev() {
        match ch {
            '}' => depth += 1,
            '{' => {
                depth -= 1;
                if depth == 0 {
                    brace_start = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }

    let start = brace_start?;
    let inner = &trimmed_end[start + 1..trimmed_end.len() - 1];
    Some((start, inner))
}

pub fn apply_custom_heading_ids(doc: &mut Document) {
    apply_recursive(&mut doc.children);
}

fn apply_recursive(children: &mut [MdNode]) {
    for child in children.iter_mut() {
        if let MdNode::Heading(h) = child {
            process_heading(h);
        }
        if let Some(kids) = child.children_mut() {
            apply_recursive(kids);
        }
    }
}

fn process_heading(h: &mut Heading) {
    let last_text = match h.children.last_mut() {
        Some(MdNode::Text(t)) => t,
        _ => return,
    };

    let (brace_start, inner) = match find_trailing_brace_block(&last_text.value) {
        Some(pair) => pair,
        None => return,
    };

    let attrs = match parse_inline_attributes(inner) {
        Some(a) => a,
        None => return,
    };

    let before = last_text.value[..brace_start].trim_end().to_string();
    if before.is_empty() {
        h.children.pop();
    } else {
        last_text.value = before;
    }

    if let Some(id) = attrs.id {
        h.slug = Some(id);
    }

    if !attrs.classes.is_empty() {
        h.extra_attrs
            .insert("class".to_string(), attrs.classes.join(" "));
    }

    for (k, v) in attrs.attrs {
        h.extra_attrs.insert(k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};

    #[test]
    fn parse_id_only() {
        let result = parse_inline_attributes("#custom-id").unwrap();
        assert_eq!(result.id, Some("custom-id".to_string()));
        assert!(result.classes.is_empty());
        assert!(result.attrs.is_empty());
    }

    #[test]
    fn parse_classes_only() {
        let result = parse_inline_attributes(".note .warning").unwrap();
        assert!(result.id.is_none());
        assert_eq!(result.classes, vec!["note", "warning"]);
        assert!(result.attrs.is_empty());
    }

    #[test]
    fn parse_combined() {
        let result = parse_inline_attributes("#my-id .cls data-x=1").unwrap();
        assert_eq!(result.id, Some("my-id".to_string()));
        assert_eq!(result.classes, vec!["cls"]);
        assert_eq!(result.attrs.get("data-x"), Some(&"1".to_string()));
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(parse_inline_attributes("").is_none());
        assert!(parse_inline_attributes("   ").is_none());
    }

    #[test]
    fn apply_sets_slug_from_id() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(3, 30),
            value: "My Heading {#custom-id}".to_string(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 30),
                depth: 2,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            })],
        };

        apply_custom_heading_ids(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.slug.as_deref(), Some("custom-id"));
            if let MdNode::Text(t) = &h.children[0] {
                assert_eq!(t.value, "My Heading");
            } else {
                panic!("expected text");
            }
        } else {
            panic!("expected heading");
        }
    }

    #[test]
    fn apply_sets_class_from_dot() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(3, 20),
            value: "Heading {.note}".to_string(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 20),
                depth: 2,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            })],
        };

        apply_custom_heading_ids(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.extra_attrs.get("class"), Some(&"note".to_string()));
        } else {
            panic!("expected heading");
        }
    }

    #[test]
    fn apply_sets_arbitrary_attrs() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(3, 40),
            value: "Heading {#id .cls data-level=2}".to_string(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 40),
            children: vec![MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 40),
                depth: 2,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            })],
        };

        apply_custom_heading_ids(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.slug.as_deref(), Some("id"));
            assert_eq!(h.extra_attrs.get("class"), Some(&"cls".to_string()));
            assert_eq!(h.extra_attrs.get("data-level"), Some(&"2".to_string()));
        } else {
            panic!("expected heading");
        }
    }

    #[test]
    fn no_braces_leaves_heading_unchanged() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(3, 15),
            value: "Plain Heading".to_string(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 15),
            children: vec![MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 15),
                depth: 1,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            })],
        };

        apply_custom_heading_ids(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert!(h.slug.is_none());
            assert!(h.extra_attrs.is_empty());
            if let MdNode::Text(t) = &h.children[0] {
                assert_eq!(t.value, "Plain Heading");
            }
        } else {
            panic!("expected heading");
        }
    }

    #[test]
    fn only_brace_text_removes_text_node() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(3, 15),
            value: "{#only-id}".to_string(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 15),
            children: vec![MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 15),
                depth: 1,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            })],
        };

        apply_custom_heading_ids(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.slug.as_deref(), Some("only-id"));
            assert!(h.children.is_empty());
        } else {
            panic!("expected heading");
        }
    }
}
