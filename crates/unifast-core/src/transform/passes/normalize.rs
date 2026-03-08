use crate::ast::mdast::nodes::*;

pub fn normalize(doc: &mut Document) {
    normalize_children(&mut doc.children);
}

fn normalize_children(children: &mut Vec<MdNode>) {
    for child in children.iter_mut() {
        if let MdNode::Heading(h) = child {
            h.depth = h.depth.clamp(1, 6);
        }
        if let Some(kids) = child.children_mut() {
            normalize_children(kids);
        }
    }

    merge_adjacent_text(children);
}

fn merge_adjacent_text(children: &mut Vec<MdNode>) {
    let mut i = 0;
    while i + 1 < children.len() {
        let is_both_text = matches!(
            (&children[i], &children[i + 1]),
            (MdNode::Text(_), MdNode::Text(_))
        );
        if is_both_text {
            let next = children.remove(i + 1);
            if let (MdNode::Text(current), MdNode::Text(next_text)) = (&mut children[i], next) {
                current.value.push_str(&next_text.value);
                current.span = current.span.merge(next_text.span);
            }
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    fn make_text(id_gen: &mut NodeIdGen, value: &str, start: u32, end: u32) -> MdNode {
        MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(start, end),
            value: value.to_string(),
        })
    }

    fn make_heading(
        id_gen: &mut NodeIdGen,
        depth: u8,
        children: Vec<MdNode>,
        start: u32,
        end: u32,
    ) -> MdNode {
        MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(start, end),
            depth,
            children,
            slug: None,
            extra_attrs: SmallMap::new(),
        })
    }

    fn make_doc(id_gen: &mut NodeIdGen, children: Vec<MdNode>) -> Document {
        Document {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    #[test]
    fn normalize_clamps_heading_depth_too_high() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Title", 2, 7);
        let heading = make_heading(&mut id_gen, 7, vec![text], 0, 8);
        let mut doc = make_doc(&mut id_gen, vec![heading]);

        normalize(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.depth, 6);
        } else {
            panic!("Expected heading");
        }
    }

    #[test]
    fn normalize_clamps_heading_depth_too_low() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Title", 2, 7);
        let heading = make_heading(&mut id_gen, 0, vec![text], 0, 8);
        let mut doc = make_doc(&mut id_gen, vec![heading]);

        normalize(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.depth, 1);
        } else {
            panic!("Expected heading");
        }
    }

    #[test]
    fn normalize_valid_heading_depth_unchanged() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Title", 2, 7);
        let heading = make_heading(&mut id_gen, 3, vec![text], 0, 8);
        let mut doc = make_doc(&mut id_gen, vec![heading]);

        normalize(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.depth, 3);
        } else {
            panic!("Expected heading");
        }
    }

    #[test]
    fn normalize_merges_adjacent_text() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "hello", 0, 5);
        let t2 = make_text(&mut id_gen, " world", 5, 11);
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            children: vec![t1, t2],
        });
        let mut doc = make_doc(&mut id_gen, vec![para]);

        normalize(&mut doc);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            assert_eq!(p.children.len(), 1);
            if let MdNode::Text(t) = &p.children[0] {
                assert_eq!(t.value, "hello world");
                assert_eq!(t.span, Span::new(0, 11));
            } else {
                panic!("Expected text");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn normalize_merges_three_adjacent_text_nodes() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "a", 0, 1);
        let t2 = make_text(&mut id_gen, "b", 1, 2);
        let t3 = make_text(&mut id_gen, "c", 2, 3);
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 3),
            children: vec![t1, t2, t3],
        });
        let mut doc = make_doc(&mut id_gen, vec![para]);

        normalize(&mut doc);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            assert_eq!(p.children.len(), 1);
            if let MdNode::Text(t) = &p.children[0] {
                assert_eq!(t.value, "abc");
            } else {
                panic!("Expected text");
            }
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn normalize_no_merge_with_non_text_between() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "a", 0, 1);
        let br = MdNode::Break(Break {
            id: id_gen.next_id(),
            span: Span::new(1, 2),
        });
        let t2 = make_text(&mut id_gen, "b", 2, 3);
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 3),
            children: vec![t1, br, t2],
        });
        let mut doc = make_doc(&mut id_gen, vec![para]);

        normalize(&mut doc);

        if let MdNode::Paragraph(p) = &doc.children[0] {
            assert_eq!(p.children.len(), 3);
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn normalize_recurses_into_blockquote() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Nested", 5, 11);
        let heading = make_heading(&mut id_gen, 10, vec![text], 3, 12);
        let bq = MdNode::Blockquote(Blockquote {
            id: id_gen.next_id(),
            span: Span::new(0, 15),
            children: vec![heading],
            alert_type: None,
        });
        let mut doc = make_doc(&mut id_gen, vec![bq]);

        normalize(&mut doc);

        if let MdNode::Blockquote(bq) = &doc.children[0] {
            if let MdNode::Heading(h) = &bq.children[0] {
                assert_eq!(h.depth, 6);
            } else {
                panic!("Expected heading");
            }
        } else {
            panic!("Expected blockquote");
        }
    }

    #[test]
    fn normalize_empty_document() {
        let mut id_gen = NodeIdGen::new();
        let mut doc = make_doc(&mut id_gen, vec![]);
        normalize(&mut doc);
        assert!(doc.children.is_empty());
    }

    #[test]
    fn normalize_heading_with_merged_text_children() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "Hello", 2, 7);
        let t2 = make_text(&mut id_gen, " World", 7, 13);
        let heading = make_heading(&mut id_gen, 8, vec![t1, t2], 0, 14);
        let mut doc = make_doc(&mut id_gen, vec![heading]);

        normalize(&mut doc);

        if let MdNode::Heading(h) = &doc.children[0] {
            assert_eq!(h.depth, 6);
            assert_eq!(h.children.len(), 1);
            if let MdNode::Text(t) = &h.children[0] {
                assert_eq!(t.value, "Hello World");
            } else {
                panic!("Expected text");
            }
        } else {
            panic!("Expected heading");
        }
    }
}
