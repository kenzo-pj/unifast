use crate::ast::hast::nodes::*;

static BOOLEAN_ATTRS: &[&str] = &[
    "allowfullscreen",
    "async",
    "autofocus",
    "autoplay",
    "checked",
    "controls",
    "default",
    "defer",
    "disabled",
    "formnovalidate",
    "hidden",
    "inert",
    "ismap",
    "itemscope",
    "loop",
    "multiple",
    "muted",
    "nomodule",
    "novalidate",
    "open",
    "playsinline",
    "readonly",
    "required",
    "reversed",
    "selected",
];

fn is_block_element(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "details"
            | "dialog"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hgroup"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "ul"
    )
}

fn is_preformatted(tag: &str) -> bool {
    matches!(tag, "pre" | "code" | "script" | "style" | "textarea")
}

pub fn minify_hast(root: &mut HRoot) {
    minify_children(&mut root.children, false);
}

fn minify_children(children: &mut Vec<HNode>, in_preformatted: bool) {
    if !in_preformatted {
        children.retain(|node| !matches!(node, HNode::Comment(_)));

        let has_block = children
            .iter()
            .any(|node| matches!(node, HNode::Element(e) if is_block_element(&e.tag)));

        if has_block {
            children.retain(|node| {
                if let HNode::Text(t) = node {
                    !t.value.trim().is_empty()
                } else {
                    true
                }
            });
        }
    }

    for child in children.iter_mut() {
        match child {
            HNode::Element(elem) => {
                let child_preformatted = in_preformatted || is_preformatted(&elem.tag);

                let keys_to_remove: Vec<String> = elem
                    .attributes
                    .iter()
                    .filter(|(key, value)| {
                        (key.as_str() == "class" || key.as_str() == "style") && value.is_empty()
                    })
                    .map(|(key, _)| key.clone())
                    .collect();
                for key in keys_to_remove {
                    elem.attributes.remove(&key);
                }

                let boolean_updates: Vec<String> = elem
                    .attributes
                    .iter()
                    .filter(|(key, value)| {
                        BOOLEAN_ATTRS.contains(&key.as_str()) && value.as_str() == key.as_str()
                    })
                    .map(|(key, _)| key.clone())
                    .collect();
                for key in boolean_updates {
                    elem.attributes.remove(&key);
                    elem.attributes.insert(key, String::new());
                }

                minify_children(&mut elem.children, child_preformatted);
            }
            HNode::Text(t) if !in_preformatted => {
                let collapsed = collapse_whitespace(&t.value);
                t.value = collapsed;
            }
            _ => {}
        }
    }
}

fn collapse_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_was_space = false;
    for ch in s.chars() {
        if ch.is_ascii_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    fn make_element(
        id_gen: &mut NodeIdGen,
        tag: &str,
        attrs: SmallMap<String, String>,
        children: Vec<HNode>,
    ) -> HNode {
        HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            tag: tag.to_string(),
            attributes: attrs,
            children,
            self_closing: false,
        })
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    #[test]
    fn removes_whitespace_text_nodes() {
        let mut id_gen = NodeIdGen::new();
        let ws = make_text(&mut id_gen, "   \n  ");
        let text = make_text(&mut id_gen, "Hello");
        let p1 = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let text2 = make_text(&mut id_gen, "World");
        let p2 = make_element(&mut id_gen, "p", SmallMap::new(), vec![text2]);
        let mut root = make_root(&mut id_gen, vec![p1, ws, p2]);

        minify_hast(&mut root);

        assert_eq!(root.children.len(), 2);
        if let HNode::Element(ref e) = root.children[0] {
            assert_eq!(e.tag, "p");
        } else {
            panic!("expected element");
        }
        if let HNode::Element(ref e) = root.children[1] {
            assert_eq!(e.tag, "p");
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn preserves_pre_whitespace() {
        let mut id_gen = NodeIdGen::new();
        let code_text = make_text(&mut id_gen, "  fn main()  {\n    println!()\n  }  ");
        let code = make_element(&mut id_gen, "code", SmallMap::new(), vec![code_text]);
        let pre = make_element(&mut id_gen, "pre", SmallMap::new(), vec![code]);
        let mut root = make_root(&mut id_gen, vec![pre]);

        minify_hast(&mut root);

        if let HNode::Element(ref pre_elem) = root.children[0] {
            if let HNode::Element(ref code_elem) = pre_elem.children[0] {
                if let HNode::Text(ref t) = code_elem.children[0] {
                    assert_eq!(t.value, "  fn main()  {\n    println!()\n  }  ");
                } else {
                    panic!("expected text");
                }
            } else {
                panic!("expected code element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn shortens_boolean_attrs() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("disabled".to_string(), "disabled".to_string());
        attrs.insert("type".to_string(), "text".to_string());
        let input = make_element(&mut id_gen, "input", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![input]);

        minify_hast(&mut root);

        if let HNode::Element(ref e) = root.children[0] {
            assert_eq!(e.attributes.get("disabled"), Some(&String::new()));
            assert_eq!(e.attributes.get("type"), Some(&"text".to_string()));
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn removes_empty_class() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), String::new());
        attrs.insert("id".to_string(), "main".to_string());
        let text = make_text(&mut id_gen, "Hello");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![div]);

        minify_hast(&mut root);

        if let HNode::Element(ref e) = root.children[0] {
            assert!(e.attributes.get("class").is_none());
            assert_eq!(e.attributes.get("id"), Some(&"main".to_string()));
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn removes_empty_style() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("style".to_string(), String::new());
        let text = make_text(&mut id_gen, "Hello");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![div]);

        minify_hast(&mut root);

        if let HNode::Element(ref e) = root.children[0] {
            assert!(e.attributes.get("style").is_none());
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn removes_comments() {
        let mut id_gen = NodeIdGen::new();
        let comment = HNode::Comment(HComment {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: " a comment ".to_string(),
        });
        let text = make_text(&mut id_gen, "Hello");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![comment, p]);

        minify_hast(&mut root);

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(ref e) = root.children[0] {
            assert_eq!(e.tag, "p");
        } else {
            panic!("expected element");
        }
    }

    #[test]
    fn collapses_inline_whitespace() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello    world   foo");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        minify_hast(&mut root);

        if let HNode::Element(ref e) = root.children[0] {
            if let HNode::Text(ref t) = e.children[0] {
                assert_eq!(t.value, "Hello world foo");
            } else {
                panic!("expected text");
            }
        } else {
            panic!("expected element");
        }
    }
}
