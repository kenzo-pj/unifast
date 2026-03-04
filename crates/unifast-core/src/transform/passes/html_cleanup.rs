use crate::ast::hast::nodes::*;

#[derive(Default)]
pub struct CleanupOptions {
    pub remove_empty_nodes: bool,
    pub minify_whitespace: bool,
}

pub fn cleanup(root: &mut HRoot, options: &CleanupOptions) {
    cleanup_children(&mut root.children, options);
}

fn cleanup_children(children: &mut Vec<HNode>, options: &CleanupOptions) {
    // Remove empty elements if enabled
    if options.remove_empty_nodes {
        children.retain(|node| !is_empty_element(node));
    }

    // Recurse into children
    for child in children.iter_mut() {
        if let Some(kids) = child.children_mut() {
            cleanup_children(kids, options);
        }
    }

    // Minify whitespace in text nodes if enabled
    if options.minify_whitespace {
        for child in children.iter_mut() {
            if let HNode::Text(t) = child {
                t.value = t.value.split_whitespace().collect::<Vec<_>>().join(" ");
            }
        }
    }
}

fn is_empty_element(node: &HNode) -> bool {
    match node {
        HNode::Element(e) => e.children.is_empty() && !is_void_tag(&e.tag),
        HNode::Text(t) => t.value.is_empty(),
        _ => false,
    }
}

fn is_void_tag(tag: &str) -> bool {
    matches!(
        tag,
        "br" | "hr"
            | "img"
            | "input"
            | "meta"
            | "link"
            | "area"
            | "base"
            | "col"
            | "embed"
            | "source"
            | "track"
            | "wbr"
    )
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
    fn remove_empty_paragraph() {
        let mut id_gen = NodeIdGen::new();
        let empty_p = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "Hello");
        let full_p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![empty_p, full_p]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: false,
            },
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(ref p) = root.children[0] {
            assert_eq!(p.tag, "p");
            assert_eq!(p.children.len(), 1);
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn keep_void_elements() {
        let mut id_gen = NodeIdGen::new();
        let hr = make_element(&mut id_gen, "hr", SmallMap::new(), vec![]);
        let br = make_element(&mut id_gen, "br", SmallMap::new(), vec![]);
        let mut attrs = SmallMap::new();
        attrs.insert("src".to_string(), "img.png".to_string());
        let img = make_element(&mut id_gen, "img", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![hr, br, img]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: false,
            },
        );

        // All void elements should be retained
        assert_eq!(root.children.len(), 3);
        if let HNode::Element(ref e) = root.children[0] {
            assert_eq!(e.tag, "hr");
        }
        if let HNode::Element(ref e) = root.children[1] {
            assert_eq!(e.tag, "br");
        }
        if let HNode::Element(ref e) = root.children[2] {
            assert_eq!(e.tag, "img");
        }
    }

    #[test]
    fn minify_whitespace() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "  Hello   world  ");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: false,
                minify_whitespace: true,
            },
        );

        if let HNode::Element(ref p_elem) = root.children[0] {
            if let HNode::Text(ref t) = p_elem.children[0] {
                assert_eq!(t.value, "Hello world");
            } else {
                panic!("expected text node");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn no_cleanup_by_default() {
        let mut id_gen = NodeIdGen::new();
        let empty_p = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "  spaces  ");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![empty_p, p]);

        cleanup(&mut root, &CleanupOptions::default());

        // Nothing changed
        assert_eq!(root.children.len(), 2);
        if let HNode::Element(ref p_elem) = root.children[1] {
            if let HNode::Text(ref t) = p_elem.children[0] {
                assert_eq!(t.value, "  spaces  ");
            } else {
                panic!("expected text node");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn nested_cleanup() {
        let mut id_gen = NodeIdGen::new();
        let empty_span = make_element(&mut id_gen, "span", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "Hello");
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![empty_span, text]);
        let mut root = make_root(&mut id_gen, vec![div]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: false,
            },
        );

        // The div is kept because it has children, but empty span inside is removed
        assert_eq!(root.children.len(), 1);
        if let HNode::Element(ref div_elem) = root.children[0] {
            assert_eq!(div_elem.tag, "div");
            assert_eq!(div_elem.children.len(), 1);
            if let HNode::Text(ref t) = div_elem.children[0] {
                assert_eq!(t.value, "Hello");
            } else {
                panic!("expected text node");
            }
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn remove_empty_text_nodes() {
        let mut id_gen = NodeIdGen::new();
        let empty_text = make_text(&mut id_gen, "");
        let normal_text = make_text(&mut id_gen, "Hello");
        let mut root = make_root(&mut id_gen, vec![empty_text, normal_text]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: false,
            },
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Text(ref t) = root.children[0] {
            assert_eq!(t.value, "Hello");
        } else {
            panic!("expected text node");
        }
    }

    #[test]
    fn cleanup_empty_root() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_root(&mut id_gen, vec![]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: true,
            },
        );

        assert!(root.children.is_empty());
    }

    #[test]
    fn both_options_combined() {
        let mut id_gen = NodeIdGen::new();
        let empty_p = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "  Hello   world  ");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![empty_p, p]);

        cleanup(
            &mut root,
            &CleanupOptions {
                remove_empty_nodes: true,
                minify_whitespace: true,
            },
        );

        assert_eq!(root.children.len(), 1);
        if let HNode::Element(ref p_elem) = root.children[0] {
            if let HNode::Text(ref t) = p_elem.children[0] {
                assert_eq!(t.value, "Hello world");
            } else {
                panic!("expected text node");
            }
        } else {
            panic!("expected p element");
        }
    }

    #[test]
    fn void_tag_detection() {
        assert!(is_void_tag("br"));
        assert!(is_void_tag("hr"));
        assert!(is_void_tag("img"));
        assert!(is_void_tag("input"));
        assert!(is_void_tag("meta"));
        assert!(is_void_tag("link"));
        assert!(is_void_tag("area"));
        assert!(is_void_tag("base"));
        assert!(is_void_tag("col"));
        assert!(is_void_tag("embed"));
        assert!(is_void_tag("source"));
        assert!(is_void_tag("track"));
        assert!(is_void_tag("wbr"));
        assert!(!is_void_tag("div"));
        assert!(!is_void_tag("p"));
        assert!(!is_void_tag("span"));
    }
}
