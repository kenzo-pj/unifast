use crate::ast::common::{NodeIdGen, Span};
use crate::ast::hast::nodes::*;
use crate::util::small_map::SmallMap;

pub fn apply_sectionize(root: &mut HRoot, id_gen: &mut NodeIdGen) {
    let old_children = std::mem::take(&mut root.children);
    root.children = wrap_in_sections(old_children, id_gen);
}

fn heading_depth(node: &HNode) -> Option<u8> {
    if let HNode::Element(e) = node {
        match e.tag.as_str() {
            "h1" => Some(1),
            "h2" => Some(2),
            "h3" => Some(3),
            "h4" => Some(4),
            "h5" => Some(5),
            "h6" => Some(6),
            _ => None,
        }
    } else {
        None
    }
}

fn wrap_in_sections(children: Vec<HNode>, id_gen: &mut NodeIdGen) -> Vec<HNode> {
    let mut result: Vec<HNode> = Vec::new();
    let mut iter = children.into_iter().peekable();

    while let Some(child) = iter.next() {
        if let Some(depth) = heading_depth(&child) {
            let mut section_body = vec![child];
            while let Some(next) = iter.peek() {
                if let Some(next_depth) = heading_depth(next)
                    && next_depth <= depth
                {
                    break;
                }
                section_body.push(iter.next().unwrap());
            }
            let heading = section_body.remove(0);
            let nested = wrap_in_sections(section_body, id_gen);
            let mut final_children = vec![heading];
            final_children.extend(nested);
            result.push(make_section(final_children, id_gen));
        } else {
            result.push(child);
        }
    }

    result
}

fn make_section(children: Vec<HNode>, id_gen: &mut NodeIdGen) -> HNode {
    let span = children
        .first()
        .map_or(Span::empty(), crate::ast::hast::nodes::HNode::span);
    HNode::Element(HElement {
        id: id_gen.next_id(),
        span,
        tag: "section".to_string(),
        attributes: SmallMap::new(),
        children,
        self_closing: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    #[test]
    fn content_before_heading_is_not_wrapped() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Intro".to_string(),
                    })],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "After heading".to_string(),
                    })],
                    self_closing: false,
                }),
            ],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert_eq!(root.children.len(), 2);
        if let HNode::Element(p) = &root.children[0] {
            assert_eq!(p.tag, "p");
        } else {
            panic!("expected p element");
        }
        if let HNode::Element(section) = &root.children[1] {
            assert_eq!(section.tag, "section");
            assert_eq!(section.children.len(), 2);
        } else {
            panic!("expected section element");
        }
    }

    #[test]
    fn multiple_same_level_headings_create_separate_sections() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "First".to_string(),
                    })],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Second".to_string(),
                    })],
                    self_closing: false,
                }),
            ],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert_eq!(root.children.len(), 2);
        for child in &root.children {
            if let HNode::Element(section) = child {
                assert_eq!(section.tag, "section");
                assert_eq!(section.children.len(), 2);
            } else {
                panic!("expected section");
            }
        }
    }

    #[test]
    fn empty_root_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert!(root.children.is_empty());
    }

    #[test]
    fn nests_deeper_headings_inside_parent_section() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h1".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Intro".to_string(),
                    })],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Sub content".to_string(),
                    })],
                    self_closing: false,
                }),
            ],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert_eq!(root.children.len(), 1);
        if let HNode::Element(outer) = &root.children[0] {
            assert_eq!(outer.tag, "section");
            assert_eq!(outer.children.len(), 3); // h1, p, section(h2, p)
            if let HNode::Element(inner) = &outer.children[2] {
                assert_eq!(inner.tag, "section");
                assert_eq!(inner.children.len(), 2); // h2, p
            } else {
                panic!("expected nested section");
            }
        } else {
            panic!("expected section");
        }
    }

    #[test]
    fn deeper_heading_followed_by_shallower_creates_siblings() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h1".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "A".to_string(),
                    })],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "B".to_string(),
                    })],
                    self_closing: false,
                }),
            ],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert_eq!(root.children.len(), 1);
        if let HNode::Element(outer) = &root.children[0] {
            assert_eq!(outer.tag, "section");
            assert_eq!(outer.children.len(), 3);
            assert!(matches!(&outer.children[0], HNode::Element(e) if e.tag == "h1"));
            assert!(matches!(&outer.children[1], HNode::Element(e) if e.tag == "section"));
            assert!(matches!(&outer.children[2], HNode::Element(e) if e.tag == "section"));
        } else {
            panic!("expected section");
        }
    }

    #[test]
    fn wraps_heading_and_content_in_section() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "intro".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h2".to_string(),
                    attributes: attrs,
                    children: vec![],
                    self_closing: false,
                }),
                HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "p".to_string(),
                    attributes: SmallMap::new(),
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Content".to_string(),
                    })],
                    self_closing: false,
                }),
            ],
        };
        apply_sectionize(&mut root, &mut id_gen);
        assert_eq!(root.children.len(), 1);
        if let HNode::Element(section) = &root.children[0] {
            assert_eq!(section.tag, "section");
            assert_eq!(section.children.len(), 2);
        } else {
            panic!("expected section element");
        }
    }
}
