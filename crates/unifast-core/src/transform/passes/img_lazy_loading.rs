use crate::ast::hast::nodes::*;

pub fn apply_img_lazy_loading(root: &mut HRoot, skip_first: u32) {
    let mut count = 0u32;
    for child in &mut root.children {
        apply_to_node(child, &mut count, skip_first);
    }
}

fn apply_to_node(node: &mut HNode, count: &mut u32, skip_first: u32) {
    match node {
        HNode::Element(elem) => {
            if elem.tag == "img" {
                *count += 1;
                if *count > skip_first {
                    elem.attributes
                        .insert("loading".to_string(), "lazy".to_string());
                    elem.attributes
                        .insert("decoding".to_string(), "async".to_string());
                }
            }
            for child in &mut elem.children {
                apply_to_node(child, count, skip_first);
            }
        }
        HNode::Root(r) => {
            for child in &mut r.children {
                apply_to_node(child, count, skip_first);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    fn make_img(id_gen: &mut NodeIdGen, src: &str, alt: &str) -> HNode {
        let mut attrs = SmallMap::new();
        attrs.insert("src".to_string(), src.to_string());
        attrs.insert("alt".to_string(), alt.to_string());
        HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::empty(),
            tag: "img".to_string(),
            attributes: attrs,
            children: vec![],
            self_closing: true,
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
    fn adds_lazy_loading_to_img() {
        let mut id_gen = NodeIdGen::new();
        let img = make_img(&mut id_gen, "test.jpg", "alt text");
        let mut root = make_root(&mut id_gen, vec![img]);

        apply_img_lazy_loading(&mut root, 0);

        if let HNode::Element(elem) = &root.children[0] {
            assert_eq!(
                elem.attributes.get(&"loading".to_string()),
                Some(&"lazy".to_string())
            );
            assert_eq!(
                elem.attributes.get(&"decoding".to_string()),
                Some(&"async".to_string())
            );
        } else {
            panic!("expected img element");
        }
    }

    #[test]
    fn skips_first_n_images() {
        let mut id_gen = NodeIdGen::new();
        let img1 = make_img(&mut id_gen, "a.jpg", "first");
        let img2 = make_img(&mut id_gen, "b.jpg", "second");
        let img3 = make_img(&mut id_gen, "c.jpg", "third");
        let mut root = make_root(&mut id_gen, vec![img1, img2, img3]);

        apply_img_lazy_loading(&mut root, 1);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.get(&"loading".to_string()).is_none());
            assert!(elem.attributes.get(&"decoding".to_string()).is_none());
        }
        if let HNode::Element(elem) = &root.children[1] {
            assert_eq!(
                elem.attributes.get(&"loading".to_string()),
                Some(&"lazy".to_string())
            );
            assert_eq!(
                elem.attributes.get(&"decoding".to_string()),
                Some(&"async".to_string())
            );
        }
        if let HNode::Element(elem) = &root.children[2] {
            assert_eq!(
                elem.attributes.get(&"loading".to_string()),
                Some(&"lazy".to_string())
            );
        }
    }

    #[test]
    fn handles_nested_img() {
        let mut id_gen = NodeIdGen::new();
        let img = make_img(&mut id_gen, "nested.jpg", "nested");
        let div = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::empty(),
            tag: "div".to_string(),
            attributes: SmallMap::new(),
            children: vec![img],
            self_closing: false,
        });
        let mut root = make_root(&mut id_gen, vec![div]);

        apply_img_lazy_loading(&mut root, 0);

        if let HNode::Element(div_elem) = &root.children[0] {
            if let HNode::Element(img_elem) = &div_elem.children[0] {
                assert_eq!(
                    img_elem.attributes.get(&"loading".to_string()),
                    Some(&"lazy".to_string())
                );
                assert_eq!(
                    img_elem.attributes.get(&"decoding".to_string()),
                    Some(&"async".to_string())
                );
            } else {
                panic!("expected img element");
            }
        } else {
            panic!("expected div element");
        }
    }

    #[test]
    fn no_images_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let text = HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::empty(),
            value: "no images here".to_string(),
        });
        let mut root = make_root(&mut id_gen, vec![text]);

        apply_img_lazy_loading(&mut root, 0);

        assert_eq!(root.children.len(), 1);
        assert!(matches!(root.children[0], HNode::Text(_)));
    }

    #[test]
    fn skip_all_when_skip_first_exceeds_count() {
        let mut id_gen = NodeIdGen::new();
        let img = make_img(&mut id_gen, "only.jpg", "only");
        let mut root = make_root(&mut id_gen, vec![img]);

        apply_img_lazy_loading(&mut root, 10);

        if let HNode::Element(elem) = &root.children[0] {
            assert!(elem.attributes.get(&"loading".to_string()).is_none());
        }
    }
}
