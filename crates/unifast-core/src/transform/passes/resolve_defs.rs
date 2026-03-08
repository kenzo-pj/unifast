use crate::ast::mdast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;
use std::collections::HashMap;

pub fn resolve_definitions(
    doc: &mut Document,
    _definitions: &HashMap<String, (String, Option<String>)>,
    _diagnostics: &mut DiagnosticSink,
) {
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

        let defs = HashMap::new();
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

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

        let defs = HashMap::new();
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

        assert_eq!(doc.children.len(), 1);
        assert!(matches!(doc.children[0], MdNode::Paragraph(_)));
    }

    #[test]
    fn unused_definition_no_crash() {
        let mut id_gen = NodeIdGen::new();
        let def = make_definition(&mut id_gen, "unused", "https://unused.com");
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![def],
        };

        let mut defs = HashMap::new();
        defs.insert(
            "unused".to_string(),
            ("https://unused.com".to_string(), None),
        );
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

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

        let defs = HashMap::new();
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

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

        let defs = HashMap::new();
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

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

        let defs = HashMap::new();
        let mut diagnostics = DiagnosticSink::new();
        resolve_definitions(&mut doc, &defs, &mut diagnostics);

        assert_eq!(doc.children.len(), 1);
        if let MdNode::Blockquote(bq) = &doc.children[0] {
            assert!(bq.children.is_empty());
        } else {
            panic!("Expected blockquote");
        }
    }
}
