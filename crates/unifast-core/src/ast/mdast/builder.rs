use super::nodes::{
    AlignKind, Blockquote, Break, Code, Definition, Delete, Document, Emphasis, FootnoteDefinition,
    FootnoteReference, Heading, Html, Image, InlineCode, Json, Link, List, ListItem, MdNode,
    MdxExpression, MdxJsxAttribute, MdxJsxElement, MdxjsEsm, Paragraph, Strong, Table, TableCell,
    TableRow, Text, ThematicBreak, Toml, Yaml,
};
use crate::ast::common::{NodeIdGen, Span};

pub struct MdBuilder<'a> {
    id_gen: &'a mut NodeIdGen,
}

impl<'a> MdBuilder<'a> {
    pub const fn new(id_gen: &'a mut NodeIdGen) -> Self {
        Self { id_gen }
    }

    pub const fn document(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Document(Document {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub const fn heading(&mut self, span: Span, depth: u8, children: Vec<MdNode>) -> MdNode {
        MdNode::Heading(Heading {
            id: self.id_gen.next_id(),
            span,
            depth,
            children,
            slug: None,
        })
    }

    pub const fn paragraph(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Paragraph(Paragraph {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub fn text(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::Text(Text {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub const fn emphasis(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Emphasis(Emphasis {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub const fn strong(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Strong(Strong {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub fn inline_code(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::InlineCode(InlineCode {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn code(
        &mut self,
        span: Span,
        value: impl Into<String>,
        lang: Option<String>,
        meta: Option<String>,
    ) -> MdNode {
        MdNode::Code(Code {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
            lang,
            meta,
        })
    }

    pub const fn blockquote(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Blockquote(Blockquote {
            id: self.id_gen.next_id(),
            span,
            children,
            alert_type: None,
        })
    }

    pub const fn list(
        &mut self,
        span: Span,
        ordered: bool,
        start: Option<u32>,
        spread: bool,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::List(List {
            id: self.id_gen.next_id(),
            span,
            ordered,
            start,
            spread,
            children,
        })
    }

    pub const fn list_item(
        &mut self,
        span: Span,
        spread: bool,
        checked: Option<bool>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::ListItem(ListItem {
            id: self.id_gen.next_id(),
            span,
            spread,
            checked,
            children,
        })
    }

    pub const fn thematic_break(&mut self, span: Span) -> MdNode {
        MdNode::ThematicBreak(ThematicBreak {
            id: self.id_gen.next_id(),
            span,
        })
    }

    pub fn link(
        &mut self,
        span: Span,
        url: impl Into<String>,
        title: Option<String>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::Link(Link {
            id: self.id_gen.next_id(),
            span,
            url: url.into(),
            title,
            children,
        })
    }

    pub fn image(
        &mut self,
        span: Span,
        url: impl Into<String>,
        title: Option<String>,
        alt: impl Into<String>,
    ) -> MdNode {
        MdNode::Image(Image {
            id: self.id_gen.next_id(),
            span,
            url: url.into(),
            title,
            alt: alt.into(),
        })
    }

    pub fn definition(
        &mut self,
        span: Span,
        identifier: impl Into<String>,
        label: Option<String>,
        url: impl Into<String>,
        title: Option<String>,
    ) -> MdNode {
        MdNode::Definition(Definition {
            id: self.id_gen.next_id(),
            span,
            identifier: identifier.into(),
            label,
            url: url.into(),
            title,
        })
    }

    pub fn html(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::Html(Html {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub const fn hard_break(&mut self, span: Span) -> MdNode {
        MdNode::Break(Break {
            id: self.id_gen.next_id(),
            span,
        })
    }

    pub const fn table(
        &mut self,
        span: Span,
        align: Vec<AlignKind>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::Table(Table {
            id: self.id_gen.next_id(),
            span,
            align,
            children,
        })
    }

    pub const fn table_row(
        &mut self,
        span: Span,
        is_header: bool,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::TableRow(TableRow {
            id: self.id_gen.next_id(),
            span,
            is_header,
            children,
        })
    }

    pub const fn table_cell(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::TableCell(TableCell {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub const fn delete(&mut self, span: Span, children: Vec<MdNode>) -> MdNode {
        MdNode::Delete(Delete {
            id: self.id_gen.next_id(),
            span,
            children,
        })
    }

    pub fn footnote_definition(
        &mut self,
        span: Span,
        identifier: impl Into<String>,
        label: Option<String>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::FootnoteDefinition(FootnoteDefinition {
            id: self.id_gen.next_id(),
            span,
            identifier: identifier.into(),
            label,
            children,
        })
    }

    pub fn footnote_reference(
        &mut self,
        span: Span,
        identifier: impl Into<String>,
        label: Option<String>,
    ) -> MdNode {
        MdNode::FootnoteReference(FootnoteReference {
            id: self.id_gen.next_id(),
            span,
            identifier: identifier.into(),
            label,
        })
    }

    pub fn yaml(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::Yaml(Yaml {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn toml(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::Toml(Toml {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn json(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::Json(Json {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub const fn mdx_jsx_flow_element(
        &mut self,
        span: Span,
        name: Option<String>,
        attributes: Vec<MdxJsxAttribute>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::MdxJsxFlowElement(MdxJsxElement {
            id: self.id_gen.next_id(),
            span,
            name,
            attributes,
            children,
        })
    }

    pub const fn mdx_jsx_text_element(
        &mut self,
        span: Span,
        name: Option<String>,
        attributes: Vec<MdxJsxAttribute>,
        children: Vec<MdNode>,
    ) -> MdNode {
        MdNode::MdxJsxTextElement(MdxJsxElement {
            id: self.id_gen.next_id(),
            span,
            name,
            attributes,
            children,
        })
    }

    pub fn mdxjs_esm(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::MdxjsEsm(MdxjsEsm {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn mdx_flow_expression(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::MdxFlowExpression(MdxExpression {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }

    pub fn mdx_text_expression(&mut self, span: Span, value: impl Into<String>) -> MdNode {
        MdNode::MdxTextExpression(MdxExpression {
            id: self.id_gen.next_id(),
            span,
            value: value.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeId;

    #[test]
    fn build_simple_document() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let text = b.text(Span::new(2, 7), "Hello");
        let para = b.paragraph(Span::new(0, 8), vec![text]);
        let doc = b.document(Span::new(0, 8), vec![para]);

        assert_eq!(doc.id(), NodeId(2));
        assert_eq!(doc.span(), Span::new(0, 8));

        let children = doc.children().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id(), NodeId(1));

        let para_children = children[0].children().unwrap();
        assert_eq!(para_children.len(), 1);
        assert_eq!(para_children[0].id(), NodeId(0));
    }

    #[test]
    fn build_heading_with_depth() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let text = b.text(Span::new(3, 8), "Title");
        let heading = b.heading(Span::new(0, 9), 2, vec![text]);

        if let MdNode::Heading(h) = &heading {
            assert_eq!(h.depth, 2);
            assert_eq!(h.children.len(), 1);
            assert!(h.slug.is_none());
        } else {
            panic!("expected Heading");
        }
    }

    #[test]
    fn build_code_block() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let code = b.code(
            Span::new(0, 30),
            "fn main() {}",
            Some("rust".into()),
            Some("linenos".into()),
        );

        if let MdNode::Code(c) = &code {
            assert_eq!(c.value, "fn main() {}");
            assert_eq!(c.lang.as_deref(), Some("rust"));
            assert_eq!(c.meta.as_deref(), Some("linenos"));
        } else {
            panic!("expected Code");
        }
    }

    #[test]
    fn build_list() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let item1_text = b.text(Span::new(2, 7), "Item1");
        let item1_para = b.paragraph(Span::new(2, 7), vec![item1_text]);
        let item1 = b.list_item(Span::new(0, 8), false, None, vec![item1_para]);

        let item2_text = b.text(Span::new(10, 15), "Item2");
        let item2_para = b.paragraph(Span::new(10, 15), vec![item2_text]);
        let item2 = b.list_item(Span::new(8, 16), false, Some(true), vec![item2_para]);

        let list = b.list(Span::new(0, 16), false, None, false, vec![item1, item2]);

        if let MdNode::List(l) = &list {
            assert!(!l.ordered);
            assert!(l.start.is_none());
            assert_eq!(l.children.len(), 2);
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn build_link_and_image() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let link_text = b.text(Span::new(1, 6), "click");
        let link = b.link(
            Span::new(0, 20),
            "https://example.com",
            Some("Example".into()),
            vec![link_text],
        );

        if let MdNode::Link(l) = &link {
            assert_eq!(l.url, "https://example.com");
            assert_eq!(l.title.as_deref(), Some("Example"));
            assert_eq!(l.children.len(), 1);
        } else {
            panic!("expected Link");
        }

        let img = b.image(Span::new(0, 30), "img.png", None, "A photo");
        if let MdNode::Image(i) = &img {
            assert_eq!(i.url, "img.png");
            assert_eq!(i.alt, "A photo");
        } else {
            panic!("expected Image");
        }
    }

    #[test]
    fn build_mdx_jsx_element() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let child = b.text(Span::new(10, 20), "content");
        let el = b.mdx_jsx_flow_element(
            Span::new(0, 30),
            Some("MyComponent".into()),
            vec![MdxJsxAttribute {
                name: "color".into(),
                value: Some("red".into()),
            }],
            vec![child],
        );

        if let MdNode::MdxJsxFlowElement(e) = &el {
            assert_eq!(e.name.as_deref(), Some("MyComponent"));
            assert_eq!(e.attributes.len(), 1);
            assert_eq!(e.attributes[0].name, "color");
            assert_eq!(e.attributes[0].value.as_deref(), Some("red"));
            assert_eq!(e.children.len(), 1);
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn builder_assigns_sequential_ids() {
        let mut id_gen = NodeIdGen::new();
        let mut b = MdBuilder::new(&mut id_gen);

        let n0 = b.text(Span::empty(), "a");
        let n1 = b.text(Span::empty(), "b");
        let n2 = b.text(Span::empty(), "c");

        assert_eq!(n0.id(), NodeId(0));
        assert_eq!(n1.id(), NodeId(1));
        assert_eq!(n2.id(), NodeId(2));
    }
}
