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
    let mut current_section: Option<(u8, Vec<HNode>)> = None;

    for child in children {
        if let Some(depth) = heading_depth(&child) {
            if let Some((prev_depth, section_children)) = current_section.take() {
                result.push(make_section(section_children, id_gen));
                if depth > prev_depth {}
            }
            current_section = Some((depth, vec![child]));
        } else if let Some(section) = &mut current_section {
            section.1.push(child);
        } else {
            result.push(child);
        }
    }

    if let Some((_, section_children)) = current_section.take() {
        result.push(make_section(section_children, id_gen));
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
