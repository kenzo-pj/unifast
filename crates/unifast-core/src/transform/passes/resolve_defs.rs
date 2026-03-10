use crate::ast::mdast::nodes::*;

pub fn remove_definition_nodes(doc: &mut Document) {
    resolve_in_children(&mut doc.children);
}

fn resolve_in_children(children: &mut Vec<MdNode>) {
    for child in children.iter_mut() {
        if let Some(kids) = child.children_mut() {
            resolve_in_children(kids);
        }
    }
    children.retain(|n| !matches!(n, MdNode::Definition(_)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::util::small_map::SmallMap;

    fn make_definition(id_gen: &mut NodeIdGen, identifier: &str, url: &str) -> MdNode {
        MdNode::Definition(Definition {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            identifier: identifier.to_string(),
            label: Some(identifier.to_string()),
            url: url.to_string(),
            title: None,
        })
    }

    #[test]
    fn resolve_removes_definitions() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![text],
        });
        let def = make_definition(&mut id_gen, "example", "https://example.com");
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![para, def],
        };

        assert_eq!(doc.children.len(), 2);
        remove_definition_nodes(&mut doc);
        assert_eq!(doc.children.len(), 1);
        assert!(matches!(doc.children[0], MdNode::Paragraph(_)));
    }

    #[test]
    fn resolve_removes_multiple_definitions() {
        let mut id_gen = NodeIdGen::new();
        let def1 = make_definition(&mut id_gen, "foo", "https://foo.com");
        let def2 = make_definition(&mut id_gen, "bar", "https://bar.com");
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![text],
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            children: vec![def1, para, def2],
        };

        remove_definition_nodes(&mut doc);
        assert_eq!(doc.children.len(), 1);
        assert!(matches!(doc.children[0], MdNode::Paragraph(_)));
    }

    #[test]
    fn removes_single_definition() {
        let mut id_gen = NodeIdGen::new();
        let def = make_definition(&mut id_gen, "unused", "https://unused.com");
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![def],
        };

        remove_definition_nodes(&mut doc);
        assert!(doc.children.is_empty());
    }

    #[test]
    fn resolve_empty_document() {
        let mut id_gen = NodeIdGen::new();
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 0),
            children: vec![],
        };

        remove_definition_nodes(&mut doc);
        assert!(doc.children.is_empty());
    }

    #[test]
    fn resolve_preserves_non_definition_nodes() {
        let mut id_gen = NodeIdGen::new();
        let text1 = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".to_string(),
        });
        let para1 = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![text1],
        });
        let text2 = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "world".to_string(),
        });
        let para2 = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(6, 11),
            children: vec![text2],
        });
        let heading = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(12, 20),
            depth: 1,
            children: vec![],
            slug: None,
            extra_attrs: SmallMap::new(),
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![para1, para2, heading],
        };

        remove_definition_nodes(&mut doc);
        assert_eq!(doc.children.len(), 3);
    }

    #[test]
    fn resolve_removes_nested_definitions_in_blockquote() {
        let mut id_gen = NodeIdGen::new();
        let def = make_definition(&mut id_gen, "nested", "https://nested.com");
        let bq = MdNode::Blockquote(Blockquote {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![def],
            alert_type: None,
        });
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![bq],
        };

        remove_definition_nodes(&mut doc);
        assert_eq!(doc.children.len(), 1);
        if let MdNode::Blockquote(bq) = &doc.children[0] {
            assert!(bq.children.is_empty());
        } else {
            panic!("Expected blockquote");
        }
    }
}
