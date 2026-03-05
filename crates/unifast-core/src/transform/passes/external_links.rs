use crate::ast::hast::nodes::*;

pub fn apply_external_links(root: &mut HRoot, rel: &str, target: Option<&str>) {
    for child in &mut root.children {
        apply_to_node(child, rel, target);
    }
}

fn apply_to_node(node: &mut HNode, rel: &str, target: Option<&str>) {
    match node {
        HNode::Element(elem) => {
            if elem.tag == "a"
                && let Some(href) = elem.attributes.get(&"href".to_string())
                && (href.starts_with("http://") || href.starts_with("https://"))
            {
                elem.attributes.insert("rel".to_string(), rel.to_string());
                if let Some(t) = target {
                    elem.attributes.insert("target".to_string(), t.to_string());
                }
            }
            for child in &mut elem.children {
                apply_to_node(child, rel, target);
            }
        }
        HNode::Root(r) => {
            for child in &mut r.children {
                apply_to_node(child, rel, target);
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

    #[test]
    fn adds_rel_to_external_link() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "a".to_string(),
                attributes: attrs,
                children: vec![],
                self_closing: false,
            })],
        };
        apply_external_links(&mut root, "noopener noreferrer", Some("_blank"));
        if let HNode::Element(a) = &root.children[0] {
            assert_eq!(
                a.attributes.get(&"rel".to_string()),
                Some(&"noopener noreferrer".to_string())
            );
            assert_eq!(
                a.attributes.get(&"target".to_string()),
                Some(&"_blank".to_string())
            );
        }
    }

    #[test]
    fn skips_internal_link() {
        let mut id_gen = NodeIdGen::new();
        let mut attrs = SmallMap::new();
        attrs.insert("href".to_string(), "/about".to_string());
        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::empty(),
            children: vec![HNode::Element(HElement {
                id: id_gen.next_id(),
                span: Span::empty(),
                tag: "a".to_string(),
                attributes: attrs,
                children: vec![],
                self_closing: false,
            })],
        };
        apply_external_links(&mut root, "noopener", None);
        if let HNode::Element(a) = &root.children[0] {
            assert!(a.attributes.get(&"rel".to_string()).is_none());
        }
    }
}
