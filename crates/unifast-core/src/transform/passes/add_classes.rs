use std::collections::HashMap;

use crate::ast::common::NodeId;
use crate::ast::hast::arena::HastArena;
use crate::ast::hast::nodes::*;
use crate::ast::hast::selector::{matches_selector, parse_css_selector};

pub fn apply_add_classes(root: &mut HRoot, rules: &[(String, String)]) {
    if rules.is_empty() {
        return;
    }

    let parsed_rules: Vec<_> = rules
        .iter()
        .filter_map(|(sel, cls)| {
            parse_css_selector(sel)
                .ok()
                .map(|selector| (selector, cls.as_str()))
        })
        .collect();

    if parsed_rules.is_empty() {
        return;
    }

    let arena = HastArena::from_hroot(root);

    let mut class_map: HashMap<NodeId, Vec<&str>> = HashMap::new();

    for elem_id in arena.elements() {
        let elem_ref = arena.node_ref(elem_id);
        for (selector, classes) in &parsed_rules {
            if matches_selector(selector, elem_ref.clone()) {
                class_map
                    .entry(arena.node(elem_id).original_id)
                    .or_default()
                    .push(classes);
            }
        }
    }

    if !class_map.is_empty() {
        apply_classes_to_tree(&mut root.children, &class_map);
    }
}

fn apply_classes_to_tree(children: &mut [HNode], class_map: &HashMap<NodeId, Vec<&str>>) {
    for child in children.iter_mut() {
        match child {
            HNode::Element(elem) => {
                if let Some(classes_to_add) = class_map.get(&elem.id) {
                    for cls in classes_to_add {
                        add_class(elem, cls);
                    }
                }
                apply_classes_to_tree(&mut elem.children, class_map);
            }
            HNode::Root(r) => {
                apply_classes_to_tree(&mut r.children, class_map);
            }
            _ => {}
        }
    }
}

fn add_class(elem: &mut HElement, classes: &str) {
    if let Some(existing) = elem.attributes.get("class") {
        let mut merged = String::with_capacity(existing.len() + 1 + classes.len());
        merged.push_str(existing);
        merged.push(' ');
        merged.push_str(classes);
        elem.attributes.insert("class".into(), merged);
    } else {
        elem.attributes.insert("class".into(), classes.to_string());
    }
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
            span: Span::empty(),
            tag: tag.to_string(),
            attributes: attrs,
            children,
            self_closing: false,
        })
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> HNode {
        HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn make_root(id_gen: &mut NodeIdGen, children: Vec<HNode>) -> HRoot {
        HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children,
        }
    }

    #[test]
    fn match_tag() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![h1]);

        apply_add_classes(
            &mut root,
            &[("h1".to_string(), "text-3xl font-bold".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(
                elem.attributes.get("class"),
                Some(&"text-3xl font-bold".to_string())
            );
        } else {
            panic!("expected h1 element");
        }
    }

    #[test]
    fn match_class() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "alert info".to_string());
        let text = make_text(&mut id_gen, "Warning");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(
            &mut root,
            &[(".alert".to_string(), "border-red".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert!(
                class.contains("border-red"),
                "expected border-red in class: {class}"
            );
            assert!(
                class.contains("alert"),
                "expected original alert class: {class}"
            );
            assert!(
                class.contains("info"),
                "expected original info class: {class}"
            );
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_id() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("id".to_string(), "main".to_string());
        let text = make_text(&mut id_gen, "Content");
        let div = make_element(&mut id_gen, "div", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(&mut root, &[("#main".to_string(), "container".to_string())]);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.get("class"), Some(&"container".to_string()));
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_attribute_presence() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        let text = make_text(&mut id_gen, "Link");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);

        apply_add_classes(&mut root, &[("[href]".to_string(), "has-link".to_string())]);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.get("class"), Some(&"has-link".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn match_attribute_prefix() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        let text = make_text(&mut id_gen, "Link");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);

        apply_add_classes(
            &mut root,
            &[("[href^=\"https\"]".to_string(), "external".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.get("class"), Some(&"external".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn match_attribute_suffix() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "document.pdf".to_string());
        let text = make_text(&mut id_gen, "PDF");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);

        apply_add_classes(
            &mut root,
            &[("[href$=\".pdf\"]".to_string(), "pdf-link".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.get("class"), Some(&"pdf-link".to_string()));
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn match_attribute_contains() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com/path".to_string());
        let text = make_text(&mut id_gen, "Link");
        let a = make_element(&mut id_gen, "a", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![a]);

        apply_add_classes(
            &mut root,
            &[(
                "[href*=\"example\"]".to_string(),
                "example-link".to_string(),
            )],
        );

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(
                elem.attributes.get("class"),
                Some(&"example-link".to_string())
            );
        } else {
            panic!("expected a element");
        }
    }

    #[test]
    fn match_child_combinator() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "code here");
        let code = make_element(&mut id_gen, "code", SmallMap::new(), vec![text]);
        let pre = make_element(&mut id_gen, "pre", SmallMap::new(), vec![code]);
        let mut root = make_root(&mut id_gen, vec![pre]);

        apply_add_classes(
            &mut root,
            &[("pre > code".to_string(), "highlighted".to_string())],
        );

        if let HNode::Element(pre_elem) = &root.children[0] {
            if let HNode::Element(code_elem) = &pre_elem.children[0] {
                assert_eq!(
                    code_elem.attributes.get("class"),
                    Some(&"highlighted".to_string())
                );
            } else {
                panic!("expected code element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn child_combinator_no_match_on_grandchild() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "code here");
        let code = make_element(&mut id_gen, "code", SmallMap::new(), vec![text]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![code]);
        let pre = make_element(&mut id_gen, "pre", SmallMap::new(), vec![div]);
        let mut root = make_root(&mut id_gen, vec![pre]);

        apply_add_classes(
            &mut root,
            &[("pre > code".to_string(), "highlighted".to_string())],
        );

        if let HNode::Element(pre_elem) = &root.children[0] {
            if let HNode::Element(div_elem) = &pre_elem.children[0] {
                if let HNode::Element(code_elem) = &div_elem.children[0] {
                    assert!(
                        code_elem.attributes.get("class").is_none(),
                        "child combinator should not match grandchild"
                    );
                } else {
                    panic!("expected code element");
                }
            } else {
                panic!("expected div element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn match_descendant_combinator() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "code here");
        let code = make_element(&mut id_gen, "code", SmallMap::new(), vec![text]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![code]);
        let pre = make_element(&mut id_gen, "pre", SmallMap::new(), vec![div]);
        let mut root = make_root(&mut id_gen, vec![pre]);

        apply_add_classes(
            &mut root,
            &[("pre code".to_string(), "highlighted".to_string())],
        );

        if let HNode::Element(pre_elem) = &root.children[0] {
            if let HNode::Element(div_elem) = &pre_elem.children[0] {
                if let HNode::Element(code_elem) = &div_elem.children[0] {
                    assert_eq!(
                        code_elem.attributes.get("class"),
                        Some(&"highlighted".to_string())
                    );
                } else {
                    panic!("expected code element");
                }
            } else {
                panic!("expected div element");
            }
        } else {
            panic!("expected pre element");
        }
    }

    #[test]
    fn merges_with_existing_classes() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "existing".to_string());
        let text = make_text(&mut id_gen, "Hello");
        let h1 = make_element(&mut id_gen, "h1", attrs, vec![text]);
        let mut root = make_root(&mut id_gen, vec![h1]);

        apply_add_classes(&mut root, &[("h1".to_string(), "added".to_string())]);

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert_eq!(class, "existing added");
        } else {
            panic!("expected h1 element");
        }
    }

    #[test]
    fn multiple_rules_apply() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![h1]);

        apply_add_classes(
            &mut root,
            &[
                ("h1".to_string(), "text-3xl".to_string()),
                ("h1".to_string(), "font-bold".to_string()),
            ],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert!(class.contains("text-3xl"), "missing text-3xl: {class}");
            assert!(class.contains("font-bold"), "missing font-bold: {class}");
        } else {
            panic!("expected h1 element");
        }
    }

    #[test]
    fn comma_selector_matches_any() {
        let mut id_gen = NodeIdGen::new();
        let text1 = make_text(&mut id_gen, "Title");
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![text1]);
        let text2 = make_text(&mut id_gen, "Subtitle");
        let h2 = make_element(&mut id_gen, "h2", SmallMap::new(), vec![text2]);
        let text3 = make_text(&mut id_gen, "Paragraph");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text3]);
        let mut root = make_root(&mut id_gen, vec![h1, h2, p]);

        apply_add_classes(&mut root, &[("h1, h2".to_string(), "heading".to_string())]);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(elem.attributes.get("class"), Some(&"heading".to_string()));
        }
        if let HNode::Element(elem) = &root.children[1] {
            assert_eq!(elem.attributes.get("class"), Some(&"heading".to_string()));
        }
        if let HNode::Element(elem) = &root.children[2] {
            assert!(
                elem.attributes.get("class").is_none(),
                "p should not match h1, h2 selector"
            );
        }
    }

    #[test]
    fn no_match_when_selector_does_not_match() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        apply_add_classes(&mut root, &[("h1".to_string(), "text-3xl".to_string())]);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.get("class").is_none());
        }
    }

    #[test]
    fn empty_rules_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![p]);

        apply_add_classes(&mut root, &[]);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.get("class").is_none());
        }
    }

    #[test]
    fn deeply_nested_descendant_match() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "deep");
        let span = make_element(&mut id_gen, "span", SmallMap::new(), vec![text]);
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![span]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![p]);
        let article = make_element(&mut id_gen, "article", SmallMap::new(), vec![div]);
        let mut root = make_root(&mut id_gen, vec![article]);

        apply_add_classes(
            &mut root,
            &[("article span".to_string(), "deep-span".to_string())],
        );

        if let HNode::Element(art) = &root.children[0] {
            if let HNode::Element(div_e) = &art.children[0] {
                if let HNode::Element(p_e) = &div_e.children[0] {
                    if let HNode::Element(span_e) = &p_e.children[0] {
                        assert_eq!(
                            span_e.attributes.get("class"),
                            Some(&"deep-span".to_string())
                        );
                    } else {
                        panic!("expected span");
                    }
                } else {
                    panic!("expected p");
                }
            } else {
                panic!("expected div");
            }
        } else {
            panic!("expected article");
        }
    }

    #[test]
    fn match_compound_selector() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "alert".to_string());
        attrs.insert("id".to_string(), "main".to_string());
        attrs.insert("data-type".to_string(), "warning".to_string());
        let div = make_element(&mut id_gen, "div", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(
            &mut root,
            &[(
                "div.alert#main[data-type=\"warning\"]".to_string(),
                "matched".to_string(),
            )],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert!(class.contains("matched"), "expected matched in: {class}");
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_multiple_classes() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "foo bar baz".to_string());
        let div = make_element(&mut id_gen, "div", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(
            &mut root,
            &[(".foo.bar.baz".to_string(), "matched".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert!(class.contains("matched"), "expected matched in: {class}");
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_first_child() {
        let mut id_gen = NodeIdGen::new();
        let a = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let b = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let c = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let ul = make_element(&mut id_gen, "ul", SmallMap::new(), vec![a, b, c]);
        let mut root = make_root(&mut id_gen, vec![ul]);

        apply_add_classes(
            &mut root,
            &[("li:first-child".to_string(), "first".to_string())],
        );

        if let HNode::Element(ul_e) = &root.children[0] {
            if let HNode::Element(first) = &ul_e.children[0] {
                assert_eq!(first.attributes.get("class"), Some(&"first".to_string()));
            }
            if let HNode::Element(second) = &ul_e.children[1] {
                assert!(second.attributes.get("class").is_none());
            }
            if let HNode::Element(third) = &ul_e.children[2] {
                assert!(third.attributes.get("class").is_none());
            }
        } else {
            panic!("expected ul element");
        }
    }

    #[test]
    fn match_last_child() {
        let mut id_gen = NodeIdGen::new();
        let a = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let b = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let c = make_element(&mut id_gen, "li", SmallMap::new(), vec![]);
        let ul = make_element(&mut id_gen, "ul", SmallMap::new(), vec![a, b, c]);
        let mut root = make_root(&mut id_gen, vec![ul]);

        apply_add_classes(
            &mut root,
            &[("li:last-child".to_string(), "last".to_string())],
        );

        if let HNode::Element(ul_e) = &root.children[0] {
            if let HNode::Element(first) = &ul_e.children[0] {
                assert!(first.attributes.get("class").is_none());
            }
            if let HNode::Element(last) = &ul_e.children[2] {
                assert_eq!(last.attributes.get("class"), Some(&"last".to_string()));
            }
        } else {
            panic!("expected ul element");
        }
    }

    #[test]
    fn match_nth_child() {
        let mut id_gen = NodeIdGen::new();
        let items: Vec<HNode> = (0..6)
            .map(|_| make_element(&mut id_gen, "li", SmallMap::new(), vec![]))
            .collect();
        let ul = make_element(&mut id_gen, "ul", SmallMap::new(), items);
        let mut root = make_root(&mut id_gen, vec![ul]);

        apply_add_classes(
            &mut root,
            &[("li:nth-child(2n+1)".to_string(), "odd".to_string())],
        );

        if let HNode::Element(ul_e) = &root.children[0] {
            for (i, child) in ul_e.children.iter().enumerate() {
                if let HNode::Element(li) = child {
                    if i % 2 == 0 {
                        assert_eq!(
                            li.attributes.get("class"),
                            Some(&"odd".to_string()),
                            "expected odd at index {i}"
                        );
                    } else {
                        assert!(
                            li.attributes.get("class").is_none(),
                            "expected no class at index {i}"
                        );
                    }
                }
            }
        } else {
            panic!("expected ul element");
        }
    }

    #[test]
    fn match_not_selector() {
        let mut id_gen = NodeIdGen::new();
        let mut foo_attrs = SmallMap::new();
        foo_attrs.insert("class".to_string(), "foo".to_string());
        let a = make_element(&mut id_gen, "div", foo_attrs, vec![]);
        let b = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let mut root = make_root(&mut id_gen, vec![a, b]);

        apply_add_classes(
            &mut root,
            &[("div:not(.foo)".to_string(), "not-foo".to_string())],
        );

        if let HNode::Element(first) = &root.children[0] {
            let class = first.attributes.get("class").unwrap();
            assert!(
                !class.contains("not-foo"),
                "div.foo should not match :not(.foo)"
            );
        }
        if let HNode::Element(second) = &root.children[1] {
            assert_eq!(second.attributes.get("class"), Some(&"not-foo".to_string()));
        }
    }

    #[test]
    fn match_adjacent_sibling() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![]);
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let mut root = make_root(&mut id_gen, vec![h1, p, div]);

        apply_add_classes(&mut root, &[("h1 + p".to_string(), "after-h1".to_string())]);

        if let HNode::Element(p_elem) = &root.children[1] {
            assert_eq!(
                p_elem.attributes.get("class"),
                Some(&"after-h1".to_string())
            );
        }
        if let HNode::Element(div_elem) = &root.children[2] {
            assert!(
                div_elem.attributes.get("class").is_none(),
                "div is not adjacent to h1"
            );
        }
    }

    #[test]
    fn match_general_sibling() {
        let mut id_gen = NodeIdGen::new();
        let h1 = make_element(&mut id_gen, "h1", SmallMap::new(), vec![]);
        let p1 = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let p2 = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let mut root = make_root(&mut id_gen, vec![h1, p1, div, p2]);

        apply_add_classes(
            &mut root,
            &[("h1 ~ p".to_string(), "sibling-of-h1".to_string())],
        );

        if let HNode::Element(p1_elem) = &root.children[1] {
            assert_eq!(
                p1_elem.attributes.get("class"),
                Some(&"sibling-of-h1".to_string())
            );
        }
        if let HNode::Element(p2_elem) = &root.children[3] {
            assert_eq!(
                p2_elem.attributes.get("class"),
                Some(&"sibling-of-h1".to_string())
            );
        }
        if let HNode::Element(div_elem) = &root.children[2] {
            assert!(
                div_elem.attributes.get("class").is_none(),
                "div should not match h1 ~ p"
            );
        }
    }

    #[test]
    fn match_universal_selector() {
        let mut id_gen = NodeIdGen::new();
        let p = make_element(&mut id_gen, "p", SmallMap::new(), vec![]);
        let div = make_element(&mut id_gen, "div", SmallMap::new(), vec![p]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(
            &mut root,
            &[("div > *".to_string(), "child-of-div".to_string())],
        );

        if let HNode::Element(div_e) = &root.children[0] {
            assert!(
                div_e.attributes.get("class").is_none(),
                "div itself should not match div > *"
            );
            if let HNode::Element(p_e) = &div_e.children[0] {
                assert_eq!(
                    p_e.attributes.get("class"),
                    Some(&"child-of-div".to_string())
                );
            }
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_empty_selector() {
        let mut id_gen = NodeIdGen::new();
        let empty_div = make_element(&mut id_gen, "div", SmallMap::new(), vec![]);
        let text = make_text(&mut id_gen, "content");
        let nonempty_div = make_element(&mut id_gen, "div", SmallMap::new(), vec![text]);
        let mut root = make_root(&mut id_gen, vec![empty_div, nonempty_div]);

        apply_add_classes(
            &mut root,
            &[("div:empty".to_string(), "is-empty".to_string())],
        );

        if let HNode::Element(empty) = &root.children[0] {
            assert_eq!(empty.attributes.get("class"), Some(&"is-empty".to_string()));
        }
        if let HNode::Element(nonempty) = &root.children[1] {
            assert!(
                nonempty.attributes.get("class").is_none(),
                "non-empty div should not match :empty"
            );
        }
    }

    #[test]
    fn match_attribute_word_match() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "foo bar baz".to_string());
        let div = make_element(&mut id_gen, "div", attrs, vec![]);
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_add_classes(
            &mut root,
            &[("[class~=\"bar\"]".to_string(), "has-bar".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class").unwrap();
            assert!(class.contains("has-bar"), "expected has-bar in: {class}");
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn match_attribute_lang_prefix() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("lang".to_string(), "en-US".to_string());
        let div = make_element(&mut id_gen, "div", attrs, vec![]);

        let mut attrs2 = SmallMap::new();
        attrs2.insert("lang".to_string(), "fr".to_string());
        let div2 = make_element(&mut id_gen, "div", attrs2, vec![]);

        let mut root = make_root(&mut id_gen, vec![div, div2]);

        apply_add_classes(
            &mut root,
            &[("[lang|=\"en\"]".to_string(), "english".to_string())],
        );

        if let HNode::Element(elem) = &root.children[0] {
            let class = elem.attributes.get("class");
            assert_eq!(class, Some(&"english".to_string()));
        }
        if let HNode::Element(elem) = &root.children[1] {
            assert!(
                elem.attributes.get("class").is_none(),
                "fr should not match [lang|=en]"
            );
        }
    }
}
