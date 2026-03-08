use crate::ast::mdast::nodes::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlugMode {
    GitHub,
    Unicode,
}

pub struct SlugGenerator {
    mode: SlugMode,
    seen: HashMap<String, u32>,
}

impl SlugGenerator {
    #[must_use]
    pub fn new(mode: SlugMode) -> Self {
        Self {
            mode,
            seen: HashMap::new(),
        }
    }

    pub fn generate(&mut self, text: &str) -> String {
        let base = match self.mode {
            SlugMode::GitHub => github_slugify(text),
            SlugMode::Unicode => unicode_slugify(text),
        };
        self.make_unique(base)
    }

    fn make_unique(&mut self, base: String) -> String {
        let count = self.seen.entry(base.clone()).or_insert(0);
        let slug = if *count == 0 {
            base
        } else {
            format!("{base}-{count}")
        };
        *count += 1;
        slug
    }
}

fn github_slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn unicode_slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

#[must_use]
pub fn extract_text(nodes: &[MdNode]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            MdNode::Text(t) => text.push_str(&t.value),
            MdNode::InlineCode(c) => text.push_str(&c.value),
            MdNode::Emphasis(e) => text.push_str(&extract_text(&e.children)),
            MdNode::Strong(s) => text.push_str(&extract_text(&s.children)),
            MdNode::Delete(d) => text.push_str(&extract_text(&d.children)),
            MdNode::Link(l) => text.push_str(&extract_text(&l.children)),
            _ => {}
        }
    }
    text
}

pub fn apply_slugs(doc: &mut Document, mode: SlugMode) {
    let mut slug_gen = SlugGenerator::new(mode);
    apply_slugs_recursive(&mut doc.children, &mut slug_gen);
}

fn apply_slugs_recursive(children: &mut [MdNode], slug_gen: &mut SlugGenerator) {
    for child in children.iter_mut() {
        if let MdNode::Heading(h) = child
            && h.slug.is_none()
        {
            let text = extract_text(&h.children);
            h.slug = Some(slug_gen.generate(&text));
        }
        if let Some(kids) = child.children_mut() {
            apply_slugs_recursive(kids, slug_gen);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    #[test]
    fn slug_github_basic() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("Hello World"), "hello-world");
    }

    #[test]
    fn slug_github_special_chars() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("Hello, World!"), "hello-world");
    }

    #[test]
    fn slug_github_multiple_spaces() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("Hello   World"), "hello-world");
    }

    #[test]
    fn slug_github_leading_trailing_spaces() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("  Hello World  "), "hello-world");
    }

    #[test]
    fn slug_dedup() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("Hello"), "hello");
        assert_eq!(slug_gen.generate("Hello"), "hello-1");
        assert_eq!(slug_gen.generate("Hello"), "hello-2");
    }

    #[test]
    fn slug_unicode_keeps_letters() {
        let mut slug_gen = SlugGenerator::new(SlugMode::Unicode);
        assert_eq!(slug_gen.generate("cafe"), "cafe");
    }

    #[test]
    fn slug_unicode_basic() {
        let mut slug_gen = SlugGenerator::new(SlugMode::Unicode);
        assert_eq!(slug_gen.generate("Hello World"), "hello-world");
    }

    #[test]
    fn slug_empty_string() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate(""), "");
    }

    #[test]
    fn slug_only_special_chars() {
        let mut slug_gen = SlugGenerator::new(SlugMode::GitHub);
        assert_eq!(slug_gen.generate("!@#$%"), "");
    }

    #[test]
    fn extract_text_from_plain_text() {
        let mut id_gen = NodeIdGen::new();
        let nodes = vec![MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        })];
        assert_eq!(extract_text(&nodes), "hello");
    }

    #[test]
    fn extract_text_from_emphasis() {
        let mut id_gen = NodeIdGen::new();
        let inner = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(1, 6),
            value: "world".to_string(),
        });
        let nodes = vec![MdNode::Emphasis(Emphasis {
            id: id_gen.next_id(),
            span: Span::new(0, 7),
            children: vec![inner],
        })];
        assert_eq!(extract_text(&nodes), "world");
    }

    #[test]
    fn extract_text_from_inline_code() {
        let mut id_gen = NodeIdGen::new();
        let nodes = vec![MdNode::InlineCode(InlineCode {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            value: "code".to_string(),
        })];
        assert_eq!(extract_text(&nodes), "code");
    }

    #[test]
    fn extract_text_mixed() {
        let mut id_gen = NodeIdGen::new();
        let text1 = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            value: "Hello ".to_string(),
        });
        let inner = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(7, 12),
            value: "World".to_string(),
        });
        let strong = MdNode::Strong(Strong {
            id: id_gen.next_id(),
            span: Span::new(6, 13),
            children: vec![inner],
        });
        let nodes = vec![text1, strong];
        assert_eq!(extract_text(&nodes), "Hello World");
    }

    #[test]
    fn apply_slugs_to_document() {
        let mut id_gen = NodeIdGen::new();
        let text1 = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(2, 7),
            value: "Hello".to_string(),
        });
        let h1 = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            depth: 1,
            children: vec![text1],
            slug: None,
            extra_attrs: SmallMap::new(),
        });
        let text2 = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(12, 17),
            value: "World".to_string(),
        });
        let h2 = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(9, 18),
            depth: 2,
            children: vec![text2],
            slug: None,
            extra_attrs: SmallMap::new(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 18),
            children: vec![h1, h2],
        };

        apply_slugs(&mut doc, SlugMode::GitHub);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.slug.as_deref(), Some("hello"));
        } else {
            panic!("Expected heading");
        }
        if let MdNode::Heading(h) = &doc.children[1] {
            assert_eq!(h.slug.as_deref(), Some("world"));
        } else {
            panic!("Expected heading");
        }
    }

    #[test]
    fn apply_slugs_dedup_in_document() {
        let mut id_gen = NodeIdGen::new();
        let mut headings = Vec::new();
        for _ in 0..3 {
            let text = MdNode::Text(Text {
                id: id_gen.next_id(),
                span: Span::new(0, 5),
                value: "Title".to_string(),
            });
            headings.push(MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 8),
                depth: 1,
                children: vec![text],
                slug: None,
                extra_attrs: SmallMap::new(),
            }));
        }
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: headings,
        };

        apply_slugs(&mut doc, SlugMode::GitHub);

        let slugs: Vec<_> = doc
            .children
            .iter()
            .map(|c| {
                if let MdNode::Heading(h) = c {
                    h.slug.clone().unwrap()
                } else {
                    panic!("Expected heading");
                }
            })
            .collect();
        assert_eq!(slugs, vec!["title", "title-1", "title-2"]);
    }
}
