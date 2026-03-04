use crate::ast::common::{NodeId, Span};

/// Minimal JavaScript AST node types for MDX support.
#[derive(Debug, Clone)]
pub enum JsNode {
    Program(JsProgram),
    ImportDeclaration(JsImportDecl),
    ExportDeclaration(JsExportDecl),
    JsxElement(JsJsxElement),
    JsxFragment(JsJsxFragment),
    Expression(JsExpression),
}

impl JsNode {
    /// Returns the source span for any node variant.
    pub fn span(&self) -> Span {
        match self {
            JsNode::Program(n) => n.span,
            JsNode::ImportDeclaration(n) => n.span,
            JsNode::ExportDeclaration(n) => n.span,
            JsNode::JsxElement(n) => n.span,
            JsNode::JsxFragment(n) => n.span,
            JsNode::Expression(n) => n.span,
        }
    }

    /// Returns the unique node ID for any node variant.
    pub fn id(&self) -> NodeId {
        match self {
            JsNode::Program(n) => n.id,
            JsNode::ImportDeclaration(n) => n.id,
            JsNode::ExportDeclaration(n) => n.id,
            JsNode::JsxElement(n) => n.id,
            JsNode::JsxFragment(n) => n.id,
            JsNode::Expression(n) => n.id,
        }
    }

    /// Returns a slice of children if the node has children, or `None` for leaf nodes.
    pub fn children(&self) -> Option<&[JsNode]> {
        match self {
            JsNode::Program(n) => Some(&n.body),
            JsNode::JsxElement(n) => Some(&n.children),
            JsNode::JsxFragment(n) => Some(&n.children),
            JsNode::ImportDeclaration(_) | JsNode::ExportDeclaration(_) | JsNode::Expression(_) => {
                None
            }
        }
    }

    /// Returns a mutable reference to the children vec if the node has children.
    pub fn children_mut(&mut self) -> Option<&mut Vec<JsNode>> {
        match self {
            JsNode::Program(n) => Some(&mut n.body),
            JsNode::JsxElement(n) => Some(&mut n.children),
            JsNode::JsxFragment(n) => Some(&mut n.children),
            JsNode::ImportDeclaration(_) | JsNode::ExportDeclaration(_) | JsNode::Expression(_) => {
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsProgram {
    pub id: NodeId,
    pub span: Span,
    pub body: Vec<JsNode>,
}

#[derive(Debug, Clone)]
pub struct JsImportDecl {
    pub id: NodeId,
    pub span: Span,
    pub source: String,
    pub specifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct JsExportDecl {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct JsJsxElement {
    pub id: NodeId,
    pub span: Span,
    pub tag: String,
    pub children: Vec<JsNode>,
}

#[derive(Debug, Clone)]
pub struct JsJsxFragment {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<JsNode>,
}

#[derive(Debug, Clone)]
pub struct JsExpression {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    #[test]
    fn basic_construction() {
        let mut id_gen = NodeIdGen::new();

        let import = JsNode::ImportDeclaration(JsImportDecl {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            source: "react".into(),
            specifiers: vec!["React".into(), "useState".into()],
        });

        assert_eq!(import.span(), Span::new(0, 30));
        assert_eq!(import.id(), NodeId(0));
        assert!(import.children().is_none());
    }

    #[test]
    fn program_with_body() {
        let mut id_gen = NodeIdGen::new();

        let import = JsNode::ImportDeclaration(JsImportDecl {
            id: id_gen.next_id(),
            span: Span::new(0, 25),
            source: "react".into(),
            specifiers: vec!["React".into()],
        });

        let export = JsNode::ExportDeclaration(JsExportDecl {
            id: id_gen.next_id(),
            span: Span::new(26, 60),
            value: "export default App".into(),
        });

        let program = JsNode::Program(JsProgram {
            id: id_gen.next_id(),
            span: Span::new(0, 60),
            body: vec![import, export],
        });

        assert_eq!(program.children().unwrap().len(), 2);
        assert_eq!(program.id(), NodeId(2));
    }

    #[test]
    fn jsx_element_with_children() {
        let mut id_gen = NodeIdGen::new();

        let expr = JsNode::Expression(JsExpression {
            id: id_gen.next_id(),
            span: Span::new(10, 20),
            value: "count + 1".into(),
        });

        let el = JsNode::JsxElement(JsJsxElement {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            tag: "div".into(),
            children: vec![expr],
        });

        assert_eq!(el.children().unwrap().len(), 1);

        if let JsNode::JsxElement(e) = &el {
            assert_eq!(e.tag, "div");
        } else {
            panic!("expected JsxElement");
        }
    }

    #[test]
    fn jsx_fragment() {
        let mut id_gen = NodeIdGen::new();

        let child = JsNode::Expression(JsExpression {
            id: id_gen.next_id(),
            span: Span::new(2, 10),
            value: "x".into(),
        });

        let frag = JsNode::JsxFragment(JsJsxFragment {
            id: id_gen.next_id(),
            span: Span::new(0, 12),
            children: vec![child],
        });

        assert_eq!(frag.children().unwrap().len(), 1);
    }

    #[test]
    fn span_and_id_accessors() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(5, 15);

        let nodes: Vec<JsNode> = vec![
            JsNode::Program(JsProgram {
                id: id_gen.next_id(),
                span,
                body: vec![],
            }),
            JsNode::ImportDeclaration(JsImportDecl {
                id: id_gen.next_id(),
                span,
                source: String::new(),
                specifiers: vec![],
            }),
            JsNode::ExportDeclaration(JsExportDecl {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            JsNode::JsxElement(JsJsxElement {
                id: id_gen.next_id(),
                span,
                tag: String::new(),
                children: vec![],
            }),
            JsNode::JsxFragment(JsJsxFragment {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            JsNode::Expression(JsExpression {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
        ];

        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(node.span(), span);
            assert_eq!(node.id(), NodeId(i as u32));
        }
    }

    #[test]
    fn children_mut_accessor() {
        let mut id_gen = NodeIdGen::new();
        let mut program = JsNode::Program(JsProgram {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            body: vec![],
        });

        program
            .children_mut()
            .unwrap()
            .push(JsNode::Expression(JsExpression {
                id: id_gen.next_id(),
                span: Span::new(0, 10),
                value: "1 + 1".into(),
            }));

        assert_eq!(program.children().unwrap().len(), 1);
    }
}
