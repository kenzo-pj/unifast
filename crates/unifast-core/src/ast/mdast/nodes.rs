use crate::ast::common::{NodeId, Span};

/// Main enum covering all Markdown AST node types:
/// CommonMark, GFM extensions, frontmatter, and MDX.
#[derive(Debug, Clone)]
pub enum MdNode {
    // CommonMark
    Document(Document),
    Heading(Heading),
    Paragraph(Paragraph),
    Text(Text),
    Emphasis(Emphasis),
    Strong(Strong),
    InlineCode(InlineCode),
    Code(Code),
    Blockquote(Blockquote),
    List(List),
    ListItem(ListItem),
    ThematicBreak(ThematicBreak),
    Link(Link),
    Image(Image),
    Definition(Definition),
    Html(Html),
    Break(Break),
    // GFM
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    Delete(Delete),
    FootnoteDefinition(FootnoteDefinition),
    FootnoteReference(FootnoteReference),
    // Frontmatter
    Yaml(Yaml),
    Toml(Toml),
    Json(Json),
    // MDX
    MdxJsxFlowElement(MdxJsxElement),
    MdxJsxTextElement(MdxJsxElement),
    MdxjsEsm(MdxjsEsm),
    MdxFlowExpression(MdxExpression),
    MdxTextExpression(MdxExpression),
}

impl MdNode {
    /// Returns the source span for any node variant.
    pub fn span(&self) -> Span {
        match self {
            MdNode::Document(n) => n.span,
            MdNode::Heading(n) => n.span,
            MdNode::Paragraph(n) => n.span,
            MdNode::Text(n) => n.span,
            MdNode::Emphasis(n) => n.span,
            MdNode::Strong(n) => n.span,
            MdNode::InlineCode(n) => n.span,
            MdNode::Code(n) => n.span,
            MdNode::Blockquote(n) => n.span,
            MdNode::List(n) => n.span,
            MdNode::ListItem(n) => n.span,
            MdNode::ThematicBreak(n) => n.span,
            MdNode::Link(n) => n.span,
            MdNode::Image(n) => n.span,
            MdNode::Definition(n) => n.span,
            MdNode::Html(n) => n.span,
            MdNode::Break(n) => n.span,
            MdNode::Table(n) => n.span,
            MdNode::TableRow(n) => n.span,
            MdNode::TableCell(n) => n.span,
            MdNode::Delete(n) => n.span,
            MdNode::FootnoteDefinition(n) => n.span,
            MdNode::FootnoteReference(n) => n.span,
            MdNode::Yaml(n) => n.span,
            MdNode::Toml(n) => n.span,
            MdNode::Json(n) => n.span,
            MdNode::MdxJsxFlowElement(n) => n.span,
            MdNode::MdxJsxTextElement(n) => n.span,
            MdNode::MdxjsEsm(n) => n.span,
            MdNode::MdxFlowExpression(n) => n.span,
            MdNode::MdxTextExpression(n) => n.span,
        }
    }

    /// Returns the unique node ID for any node variant.
    pub fn id(&self) -> NodeId {
        match self {
            MdNode::Document(n) => n.id,
            MdNode::Heading(n) => n.id,
            MdNode::Paragraph(n) => n.id,
            MdNode::Text(n) => n.id,
            MdNode::Emphasis(n) => n.id,
            MdNode::Strong(n) => n.id,
            MdNode::InlineCode(n) => n.id,
            MdNode::Code(n) => n.id,
            MdNode::Blockquote(n) => n.id,
            MdNode::List(n) => n.id,
            MdNode::ListItem(n) => n.id,
            MdNode::ThematicBreak(n) => n.id,
            MdNode::Link(n) => n.id,
            MdNode::Image(n) => n.id,
            MdNode::Definition(n) => n.id,
            MdNode::Html(n) => n.id,
            MdNode::Break(n) => n.id,
            MdNode::Table(n) => n.id,
            MdNode::TableRow(n) => n.id,
            MdNode::TableCell(n) => n.id,
            MdNode::Delete(n) => n.id,
            MdNode::FootnoteDefinition(n) => n.id,
            MdNode::FootnoteReference(n) => n.id,
            MdNode::Yaml(n) => n.id,
            MdNode::Toml(n) => n.id,
            MdNode::Json(n) => n.id,
            MdNode::MdxJsxFlowElement(n) => n.id,
            MdNode::MdxJsxTextElement(n) => n.id,
            MdNode::MdxjsEsm(n) => n.id,
            MdNode::MdxFlowExpression(n) => n.id,
            MdNode::MdxTextExpression(n) => n.id,
        }
    }

    /// Returns a slice of children if the node has children, or `None` for leaf nodes.
    pub fn children(&self) -> Option<&[MdNode]> {
        match self {
            MdNode::Document(n) => Some(&n.children),
            MdNode::Heading(n) => Some(&n.children),
            MdNode::Paragraph(n) => Some(&n.children),
            MdNode::Emphasis(n) => Some(&n.children),
            MdNode::Strong(n) => Some(&n.children),
            MdNode::Blockquote(n) => Some(&n.children),
            MdNode::List(n) => Some(&n.children),
            MdNode::ListItem(n) => Some(&n.children),
            MdNode::Link(n) => Some(&n.children),
            MdNode::Table(n) => Some(&n.children),
            MdNode::TableRow(n) => Some(&n.children),
            MdNode::TableCell(n) => Some(&n.children),
            MdNode::Delete(n) => Some(&n.children),
            MdNode::FootnoteDefinition(n) => Some(&n.children),
            MdNode::MdxJsxFlowElement(n) => Some(&n.children),
            MdNode::MdxJsxTextElement(n) => Some(&n.children),
            MdNode::Text(_)
            | MdNode::InlineCode(_)
            | MdNode::Code(_)
            | MdNode::ThematicBreak(_)
            | MdNode::Image(_)
            | MdNode::Definition(_)
            | MdNode::Html(_)
            | MdNode::Break(_)
            | MdNode::FootnoteReference(_)
            | MdNode::Yaml(_)
            | MdNode::Toml(_)
            | MdNode::Json(_)
            | MdNode::MdxjsEsm(_)
            | MdNode::MdxFlowExpression(_)
            | MdNode::MdxTextExpression(_) => None,
        }
    }

    /// Returns a mutable reference to the children vec if the node has children.
    pub fn children_mut(&mut self) -> Option<&mut Vec<MdNode>> {
        match self {
            MdNode::Document(n) => Some(&mut n.children),
            MdNode::Heading(n) => Some(&mut n.children),
            MdNode::Paragraph(n) => Some(&mut n.children),
            MdNode::Emphasis(n) => Some(&mut n.children),
            MdNode::Strong(n) => Some(&mut n.children),
            MdNode::Blockquote(n) => Some(&mut n.children),
            MdNode::List(n) => Some(&mut n.children),
            MdNode::ListItem(n) => Some(&mut n.children),
            MdNode::Link(n) => Some(&mut n.children),
            MdNode::Table(n) => Some(&mut n.children),
            MdNode::TableRow(n) => Some(&mut n.children),
            MdNode::TableCell(n) => Some(&mut n.children),
            MdNode::Delete(n) => Some(&mut n.children),
            MdNode::FootnoteDefinition(n) => Some(&mut n.children),
            MdNode::MdxJsxFlowElement(n) => Some(&mut n.children),
            MdNode::MdxJsxTextElement(n) => Some(&mut n.children),
            MdNode::Text(_)
            | MdNode::InlineCode(_)
            | MdNode::Code(_)
            | MdNode::ThematicBreak(_)
            | MdNode::Image(_)
            | MdNode::Definition(_)
            | MdNode::Html(_)
            | MdNode::Break(_)
            | MdNode::FootnoteReference(_)
            | MdNode::Yaml(_)
            | MdNode::Toml(_)
            | MdNode::Json(_)
            | MdNode::MdxjsEsm(_)
            | MdNode::MdxFlowExpression(_)
            | MdNode::MdxTextExpression(_) => None,
        }
    }
}

// ── CommonMark node structs ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Document {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub id: NodeId,
    pub span: Span,
    pub depth: u8, // 1-6
    pub children: Vec<MdNode>,
    pub slug: Option<String>, // filled by slug pass
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Emphasis {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct Strong {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct InlineCode {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
    pub lang: Option<String>,
    pub meta: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Blockquote {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub id: NodeId,
    pub span: Span,
    pub ordered: bool,
    pub start: Option<u32>,
    pub spread: bool,          // loose list
    pub children: Vec<MdNode>, // ListItem children
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: NodeId,
    pub span: Span,
    pub spread: bool,
    pub checked: Option<bool>, // Some(true/false) for task list items
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct ThematicBreak {
    pub id: NodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub id: NodeId,
    pub span: Span,
    pub url: String,
    pub title: Option<String>,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub id: NodeId,
    pub span: Span,
    pub url: String,
    pub title: Option<String>,
    pub alt: String,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub id: NodeId,
    pub span: Span,
    pub identifier: String,
    pub label: Option<String>,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Html {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Break {
    pub id: NodeId,
    pub span: Span,
}

// ── GFM nodes ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignKind {
    Left,
    Center,
    Right,
    None,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub id: NodeId,
    pub span: Span,
    pub align: Vec<AlignKind>,
    pub children: Vec<MdNode>, // TableRow
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub id: NodeId,
    pub span: Span,
    pub is_header: bool,
    pub children: Vec<MdNode>, // TableCell
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct FootnoteDefinition {
    pub id: NodeId,
    pub span: Span,
    pub identifier: String,
    pub label: Option<String>,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct FootnoteReference {
    pub id: NodeId,
    pub span: Span,
    pub identifier: String,
    pub label: Option<String>,
}

// ── Frontmatter nodes ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Yaml {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Toml {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Json {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

// ── MDX nodes ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MdxJsxAttribute {
    pub name: String,
    pub value: Option<String>, // None for boolean attrs
}

#[derive(Debug, Clone)]
pub struct MdxJsxElement {
    pub id: NodeId,
    pub span: Span,
    pub name: Option<String>, // None for fragments
    pub attributes: Vec<MdxJsxAttribute>,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct MdxjsEsm {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct MdxExpression {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    #[test]
    fn construct_simple_document() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(2, 13),
            value: "Hello world".into(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 14),
            children: vec![text],
        });
        let doc = MdNode::Document(Document {
            id: id_gen.next_id(),
            span: Span::new(0, 14),
            children: vec![para],
        });

        assert_eq!(doc.span(), Span::new(0, 14));
        assert_eq!(doc.id(), NodeId(2));
    }

    #[test]
    fn span_accessor_all_variants() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(10, 20);

        let nodes: Vec<MdNode> = vec![
            MdNode::Document(Document {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span,
                depth: 1,
                children: vec![],
                slug: None,
            }),
            MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::Text(Text {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::Emphasis(Emphasis {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::Strong(Strong {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::InlineCode(InlineCode {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::Code(Code {
                id: id_gen.next_id(),
                span,
                value: String::new(),
                lang: None,
                meta: None,
            }),
            MdNode::Blockquote(Blockquote {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::List(List {
                id: id_gen.next_id(),
                span,
                ordered: false,
                start: None,
                spread: false,
                children: vec![],
            }),
            MdNode::ListItem(ListItem {
                id: id_gen.next_id(),
                span,
                spread: false,
                checked: None,
                children: vec![],
            }),
            MdNode::ThematicBreak(ThematicBreak {
                id: id_gen.next_id(),
                span,
            }),
            MdNode::Link(Link {
                id: id_gen.next_id(),
                span,
                url: String::new(),
                title: None,
                children: vec![],
            }),
            MdNode::Image(Image {
                id: id_gen.next_id(),
                span,
                url: String::new(),
                title: None,
                alt: String::new(),
            }),
            MdNode::Definition(Definition {
                id: id_gen.next_id(),
                span,
                identifier: String::new(),
                label: None,
                url: String::new(),
                title: None,
            }),
            MdNode::Html(Html {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::Break(Break {
                id: id_gen.next_id(),
                span,
            }),
            MdNode::Table(Table {
                id: id_gen.next_id(),
                span,
                align: vec![],
                children: vec![],
            }),
            MdNode::TableRow(TableRow {
                id: id_gen.next_id(),
                span,
                is_header: false,
                children: vec![],
            }),
            MdNode::TableCell(TableCell {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::Delete(Delete {
                id: id_gen.next_id(),
                span,
                children: vec![],
            }),
            MdNode::FootnoteDefinition(FootnoteDefinition {
                id: id_gen.next_id(),
                span,
                identifier: String::new(),
                label: None,
                children: vec![],
            }),
            MdNode::FootnoteReference(FootnoteReference {
                id: id_gen.next_id(),
                span,
                identifier: String::new(),
                label: None,
            }),
            MdNode::Yaml(Yaml {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::Toml(Toml {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::Json(Json {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::MdxJsxFlowElement(MdxJsxElement {
                id: id_gen.next_id(),
                span,
                name: None,
                attributes: vec![],
                children: vec![],
            }),
            MdNode::MdxJsxTextElement(MdxJsxElement {
                id: id_gen.next_id(),
                span,
                name: None,
                attributes: vec![],
                children: vec![],
            }),
            MdNode::MdxjsEsm(MdxjsEsm {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::MdxFlowExpression(MdxExpression {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
            MdNode::MdxTextExpression(MdxExpression {
                id: id_gen.next_id(),
                span,
                value: String::new(),
            }),
        ];

        for node in &nodes {
            assert_eq!(node.span(), span);
        }
    }

    #[test]
    fn id_accessor() {
        let mut id_gen = NodeIdGen::new();
        let node = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".into(),
        });
        assert_eq!(node.id(), NodeId(0));
    }

    #[test]
    fn children_accessor_returns_some_for_parent_nodes() {
        let mut id_gen = NodeIdGen::new();
        let child = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".into(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![child],
        });
        assert_eq!(para.children().unwrap().len(), 1);
    }

    #[test]
    fn children_accessor_returns_none_for_leaf_nodes() {
        let mut id_gen = NodeIdGen::new();
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "leaf".into(),
        });
        assert!(text.children().is_none());

        let br = MdNode::Break(Break {
            id: id_gen.next_id(),
            span: Span::new(0, 2),
        });
        assert!(br.children().is_none());
    }

    #[test]
    fn children_mut_accessor() {
        let mut id_gen = NodeIdGen::new();
        let child = MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            value: "hello".into(),
        });
        let mut para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![child],
        });
        let children = para.children_mut().unwrap();
        children.push(MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(5, 10),
            value: " world".into(),
        }));
        assert_eq!(para.children().unwrap().len(), 2);
    }
}
