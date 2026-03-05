use crate::ast::common::{NodeId, Span};

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
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Program(n) => n.span,
            Self::ImportDeclaration(n) => n.span,
            Self::ExportDeclaration(n) => n.span,
            Self::JsxElement(n) => n.span,
            Self::JsxFragment(n) => n.span,
            Self::Expression(n) => n.span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> NodeId {
        match self {
            Self::Program(n) => n.id,
            Self::ImportDeclaration(n) => n.id,
            Self::ExportDeclaration(n) => n.id,
            Self::JsxElement(n) => n.id,
            Self::JsxFragment(n) => n.id,
            Self::Expression(n) => n.id,
        }
    }

    #[must_use]
    pub fn children(&self) -> Option<&[Self]> {
        match self {
            Self::Program(n) => Some(&n.body),
            Self::JsxElement(n) => Some(&n.children),
            Self::JsxFragment(n) => Some(&n.children),
            Self::ImportDeclaration(_) | Self::ExportDeclaration(_) | Self::Expression(_) => None,
        }
    }

    pub const fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::Program(n) => Some(&mut n.body),
            Self::JsxElement(n) => Some(&mut n.children),
            Self::JsxFragment(n) => Some(&mut n.children),
            Self::ImportDeclaration(_) | Self::ExportDeclaration(_) | Self::Expression(_) => None,
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
