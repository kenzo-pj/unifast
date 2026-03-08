use crate::api::options::AutolinkHeadingsBehavior;
use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::*;
use crate::util::small_map::SmallMap;

pub fn apply_autolink_headings(
    root: &mut HRoot,
    behavior: AutolinkHeadingsBehavior,
    id_gen: &mut NodeIdGen,
) {
    for child in &mut root.children {
        apply_to_node(child, behavior, id_gen);
    }
}

fn apply_to_node(node: &mut HNode, behavior: AutolinkHeadingsBehavior, id_gen: &mut NodeIdGen) {
    match node {
        HNode::Element(elem) => {
            let is_heading = matches!(elem.tag.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6");
            if is_heading {
                if let Some(id) = elem.attributes.get(&"id".to_string()).cloned() {
                    let href = format!("#{id}");
                    let mut link_attrs = SmallMap::new();
                    link_attrs.insert("href".to_string(), href);
                    link_attrs.insert("class".to_string(), "anchor".to_string());
                    link_attrs.insert("aria-hidden".to_string(), "true".to_string());

                    match behavior {
                        AutolinkHeadingsBehavior::Prepend => {
                            let link = HNode::Element(HElement {
                                id: id_gen.next_id(),
                                span: elem.span,
                                tag: "a".to_string(),
                                attributes: link_attrs,
                                children: vec![],
                                self_closing: false,
                            });
                            elem.children.insert(0, link);
                        }
                        AutolinkHeadingsBehavior::Append => {
                            let link = HNode::Element(HElement {
                                id: id_gen.next_id(),
                                span: elem.span,
                                tag: "a".to_string(),
                                attributes: link_attrs,
                                children: vec![],
                                self_closing: false,
                            });
                            elem.children.push(link);
                        }
                        AutolinkHeadingsBehavior::Wrap => {
                            let old_children = std::mem::take(&mut elem.children);
                            let link = HNode::Element(HElement {
                                id: id_gen.next_id(),
                                span: elem.span,
                                tag: "a".to_string(),
                                attributes: link_attrs,
                                children: old_children,
                                self_closing: false,
                            });
                            elem.children = vec![link];
                        }
                    }
                }
            } else {
                for child in &mut elem.children {
                    apply_to_node(child, behavior, id_gen);
                }
            }
        }
        HNode::Root(r) => {
            for child in &mut r.children {
                apply_to_node(child, behavior, id_gen);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};

    #[test]
    fn prepends_anchor_to_heading() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "hello".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "h2".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    value: "Hello".to_string(),
                })],
                self_closing: false,
            })],
        };
        apply_autolink_headings(&mut root, AutolinkHeadingsBehavior::Prepend, &mut id_gen);
        if let HNode::Element(h) = &root.children[0] {
            assert_eq!(h.children.len(), 2);
            if let HNode::Element(a) = &h.children[0] {
                assert_eq!(a.tag, "a");
                assert_eq!(
                    a.attributes.get(&"href".to_string()),
                    Some(&"#hello".to_string())
                );
            } else {
                panic!("expected anchor element");
            }
        }
    }

    #[test]
    fn appends_anchor_to_heading() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "foo".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "h3".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    value: "Foo".to_string(),
                })],
                self_closing: false,
            })],
        };
        apply_autolink_headings(&mut root, AutolinkHeadingsBehavior::Append, &mut id_gen);
        if let HNode::Element(h) = &root.children[0] {
            assert_eq!(h.children.len(), 2);
            assert!(matches!(&h.children[0], HNode::Text(_)));
            if let HNode::Element(a) = &h.children[1] {
                assert_eq!(a.tag, "a");
                assert_eq!(
                    a.attributes.get(&"href".to_string()),
                    Some(&"#foo".to_string())
                );
            } else {
                panic!("expected anchor");
            }
        }
    }

    #[test]
    fn wraps_heading_content_in_anchor() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "bar".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "h1".to_string(),
                attributes: attrs,
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    value: "Bar".to_string(),
                })],
                self_closing: false,
            })],
        };
        apply_autolink_headings(&mut root, AutolinkHeadingsBehavior::Wrap, &mut id_gen);
        if let HNode::Element(h) = &root.children[0] {
            assert_eq!(h.children.len(), 1);
            if let HNode::Element(a) = &h.children[0] {
                assert_eq!(a.tag, "a");
                assert_eq!(
                    a.attributes.get(&"href".to_string()),
                    Some(&"#bar".to_string())
                );
                assert_eq!(a.children.len(), 1);
                assert!(matches!(&a.children[0], HNode::Text(t) if t.value == "Bar"));
            } else {
                panic!("expected anchor");
            }
        }
    }

    #[test]
    fn processes_nested_heading() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "deep".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "div".to_string(),
                attributes: SmallMap::new(),
                children: vec![HNode::Element(HElement {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    tag: "h4".to_string(),
                    attributes: attrs,
                    children: vec![HNode::Text(HText {
                        id: id_gen.next_id(),
                        span: Span::empty(),
                        value: "Deep".to_string(),
                    })],
                    self_closing: false,
                })],
                self_closing: false,
            })],
        };
        apply_autolink_headings(&mut root, AutolinkHeadingsBehavior::Prepend, &mut id_gen);
        if let HNode::Element(div) = &root.children[0]
            && let HNode::Element(h) = &div.children[0]
        {
            assert_eq!(h.children.len(), 2);
            if let HNode::Element(a) = &h.children[0] {
                assert_eq!(a.tag, "a");
            } else {
                panic!("expected anchor");
            }
        }
    }

    #[test]
    fn skips_heading_without_id() {
        let mut id_gen = NodeIdGen::new();
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "h1".to_string(),
                attributes: SmallMap::new(),
                children: vec![HNode::Text(HText {
                    id: id_gen.next_id(),
                    span: Span::empty(),
                    value: "No ID".to_string(),
                })],
                self_closing: false,
            })],
        };
        apply_autolink_headings(&mut root, AutolinkHeadingsBehavior::Prepend, &mut id_gen);
        if let HNode::Element(h) = &root.children[0] {
            assert_eq!(h.children.len(), 1);
        }
    }
}
