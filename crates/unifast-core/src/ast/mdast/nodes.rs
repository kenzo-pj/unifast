use crate::ast::common::{NodeId, Span};

#[derive(Debug, Clone)]
pub enum MdNode {
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
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    Delete(Delete),
    FootnoteDefinition(FootnoteDefinition),
    FootnoteReference(FootnoteReference),
    Yaml(Yaml),
    Toml(Toml),
    Json(Json),
    MdxJsxFlowElement(MdxJsxElement),
    MdxJsxTextElement(MdxJsxElement),
    MdxjsEsm(MdxjsEsm),
    MdxFlowExpression(MdxExpression),
    MdxTextExpression(MdxExpression),
    Math(Math),
    InlineMath(InlineMath),
    ContainerDirective(ContainerDirective),
    LeafDirective(LeafDirective),
    TextDirective(TextDirective),
    WikiLink(WikiLink),
    DefinitionList(DefinitionList),
    DefinitionTerm(DefinitionTerm),
    DefinitionDescription(DefinitionDescription),
    RubyAnnotation(RubyAnnotation),
}

impl MdNode {
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Document(n) => n.span,
            Self::Heading(n) => n.span,
            Self::Paragraph(n) => n.span,
            Self::Text(n) => n.span,
            Self::Emphasis(n) => n.span,
            Self::Strong(n) => n.span,
            Self::InlineCode(n) => n.span,
            Self::Code(n) => n.span,
            Self::Blockquote(n) => n.span,
            Self::List(n) => n.span,
            Self::ListItem(n) => n.span,
            Self::ThematicBreak(n) => n.span,
            Self::Link(n) => n.span,
            Self::Image(n) => n.span,
            Self::Definition(n) => n.span,
            Self::Html(n) => n.span,
            Self::Break(n) => n.span,
            Self::Table(n) => n.span,
            Self::TableRow(n) => n.span,
            Self::TableCell(n) => n.span,
            Self::Delete(n) => n.span,
            Self::FootnoteDefinition(n) => n.span,
            Self::FootnoteReference(n) => n.span,
            Self::Yaml(n) => n.span,
            Self::Toml(n) => n.span,
            Self::Json(n) => n.span,
            Self::MdxJsxFlowElement(n) => n.span,
            Self::MdxJsxTextElement(n) => n.span,
            Self::MdxjsEsm(n) => n.span,
            Self::MdxFlowExpression(n) => n.span,
            Self::MdxTextExpression(n) => n.span,
            Self::Math(n) => n.span,
            Self::InlineMath(n) => n.span,
            Self::ContainerDirective(n) => n.span,
            Self::LeafDirective(n) => n.span,
            Self::TextDirective(n) => n.span,
            Self::WikiLink(n) => n.span,
            Self::DefinitionList(n) => n.span,
            Self::DefinitionTerm(n) => n.span,
            Self::DefinitionDescription(n) => n.span,
            Self::RubyAnnotation(n) => n.span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> NodeId {
        match self {
            Self::Document(n) => n.id,
            Self::Heading(n) => n.id,
            Self::Paragraph(n) => n.id,
            Self::Text(n) => n.id,
            Self::Emphasis(n) => n.id,
            Self::Strong(n) => n.id,
            Self::InlineCode(n) => n.id,
            Self::Code(n) => n.id,
            Self::Blockquote(n) => n.id,
            Self::List(n) => n.id,
            Self::ListItem(n) => n.id,
            Self::ThematicBreak(n) => n.id,
            Self::Link(n) => n.id,
            Self::Image(n) => n.id,
            Self::Definition(n) => n.id,
            Self::Html(n) => n.id,
            Self::Break(n) => n.id,
            Self::Table(n) => n.id,
            Self::TableRow(n) => n.id,
            Self::TableCell(n) => n.id,
            Self::Delete(n) => n.id,
            Self::FootnoteDefinition(n) => n.id,
            Self::FootnoteReference(n) => n.id,
            Self::Yaml(n) => n.id,
            Self::Toml(n) => n.id,
            Self::Json(n) => n.id,
            Self::MdxJsxFlowElement(n) => n.id,
            Self::MdxJsxTextElement(n) => n.id,
            Self::MdxjsEsm(n) => n.id,
            Self::MdxFlowExpression(n) => n.id,
            Self::MdxTextExpression(n) => n.id,
            Self::Math(n) => n.id,
            Self::InlineMath(n) => n.id,
            Self::ContainerDirective(n) => n.id,
            Self::LeafDirective(n) => n.id,
            Self::TextDirective(n) => n.id,
            Self::WikiLink(n) => n.id,
            Self::DefinitionList(n) => n.id,
            Self::DefinitionTerm(n) => n.id,
            Self::DefinitionDescription(n) => n.id,
            Self::RubyAnnotation(n) => n.id,
        }
    }

    #[must_use]
    pub fn children(&self) -> Option<&[Self]> {
        match self {
            Self::Document(n) => Some(&n.children),
            Self::Heading(n) => Some(&n.children),
            Self::Paragraph(n) => Some(&n.children),
            Self::Emphasis(n) => Some(&n.children),
            Self::Strong(n) => Some(&n.children),
            Self::Blockquote(n) => Some(&n.children),
            Self::List(n) => Some(&n.children),
            Self::ListItem(n) => Some(&n.children),
            Self::Link(n) => Some(&n.children),
            Self::Table(n) => Some(&n.children),
            Self::TableRow(n) => Some(&n.children),
            Self::TableCell(n) => Some(&n.children),
            Self::Delete(n) => Some(&n.children),
            Self::FootnoteDefinition(n) => Some(&n.children),
            Self::MdxJsxFlowElement(n) => Some(&n.children),
            Self::MdxJsxTextElement(n) => Some(&n.children),
            Self::ContainerDirective(n) => Some(&n.children),
            Self::DefinitionList(n) => Some(&n.children),
            Self::DefinitionTerm(n) => Some(&n.children),
            Self::DefinitionDescription(n) => Some(&n.children),
            Self::WikiLink(n) => Some(&n.children),
            Self::Text(_)
            | Self::InlineCode(_)
            | Self::Code(_)
            | Self::ThematicBreak(_)
            | Self::Image(_)
            | Self::Definition(_)
            | Self::Html(_)
            | Self::Break(_)
            | Self::FootnoteReference(_)
            | Self::Yaml(_)
            | Self::Toml(_)
            | Self::Json(_)
            | Self::MdxjsEsm(_)
            | Self::MdxFlowExpression(_)
            | Self::MdxTextExpression(_)
            | Self::Math(_)
            | Self::InlineMath(_)
            | Self::LeafDirective(_)
            | Self::TextDirective(_)
            | Self::RubyAnnotation(_) => None,
        }
    }

    pub const fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::Document(n) => Some(&mut n.children),
            Self::Heading(n) => Some(&mut n.children),
            Self::Paragraph(n) => Some(&mut n.children),
            Self::Emphasis(n) => Some(&mut n.children),
            Self::Strong(n) => Some(&mut n.children),
            Self::Blockquote(n) => Some(&mut n.children),
            Self::List(n) => Some(&mut n.children),
            Self::ListItem(n) => Some(&mut n.children),
            Self::Link(n) => Some(&mut n.children),
            Self::Table(n) => Some(&mut n.children),
            Self::TableRow(n) => Some(&mut n.children),
            Self::TableCell(n) => Some(&mut n.children),
            Self::Delete(n) => Some(&mut n.children),
            Self::FootnoteDefinition(n) => Some(&mut n.children),
            Self::MdxJsxFlowElement(n) => Some(&mut n.children),
            Self::MdxJsxTextElement(n) => Some(&mut n.children),
            Self::ContainerDirective(n) => Some(&mut n.children),
            Self::DefinitionList(n) => Some(&mut n.children),
            Self::DefinitionTerm(n) => Some(&mut n.children),
            Self::DefinitionDescription(n) => Some(&mut n.children),
            Self::WikiLink(n) => Some(&mut n.children),
            Self::Text(_)
            | Self::InlineCode(_)
            | Self::Code(_)
            | Self::ThematicBreak(_)
            | Self::Image(_)
            | Self::Definition(_)
            | Self::Html(_)
            | Self::Break(_)
            | Self::FootnoteReference(_)
            | Self::Yaml(_)
            | Self::Toml(_)
            | Self::Json(_)
            | Self::MdxjsEsm(_)
            | Self::MdxFlowExpression(_)
            | Self::MdxTextExpression(_)
            | Self::Math(_)
            | Self::InlineMath(_)
            | Self::LeafDirective(_)
            | Self::TextDirective(_)
            | Self::RubyAnnotation(_) => None,
        }
    }
}

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
    pub depth: u8,
    pub children: Vec<MdNode>,
    pub slug: Option<String>,
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
    pub alert_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub id: NodeId,
    pub span: Span,
    pub ordered: bool,
    pub start: Option<u32>,
    pub spread: bool,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: NodeId,
    pub span: Span,
    pub spread: bool,
    pub checked: Option<bool>,
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
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub id: NodeId,
    pub span: Span,
    pub is_header: bool,
    pub children: Vec<MdNode>,
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

#[derive(Debug, Clone)]
pub struct MdxJsxAttribute {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MdxJsxElement {
    pub id: NodeId,
    pub span: Span,
    pub name: Option<String>,
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

#[derive(Debug, Clone)]
pub struct Math {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
    pub meta: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InlineMath {
    pub id: NodeId,
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ContainerDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct LeafDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct TextDirective {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub id: NodeId,
    pub span: Span,
    pub target: String,
    pub alias: Option<String>,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct DefinitionList {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct DefinitionTerm {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct DefinitionDescription {
    pub id: NodeId,
    pub span: Span,
    pub children: Vec<MdNode>,
}

#[derive(Debug, Clone)]
pub struct RubyAnnotation {
    pub id: NodeId,
    pub span: Span,
    pub base: String,
    pub annotation: String,
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
                alert_type: None,
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
