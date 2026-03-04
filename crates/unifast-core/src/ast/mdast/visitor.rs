use super::nodes::*;

/// Immutable visitor trait for walking the Markdown AST.
///
/// Each `visit_*` method has a default implementation that visits children
/// (if any). Override specific methods to add custom logic; call
/// `self.visit_children(children)` at the end to continue the walk.
#[allow(unused_variables)]
pub trait MdVisitor {
    fn visit_node(&mut self, node: &MdNode) {
        match node {
            MdNode::Document(n) => self.visit_document(n),
            MdNode::Heading(n) => self.visit_heading(n),
            MdNode::Paragraph(n) => self.visit_paragraph(n),
            MdNode::Text(n) => self.visit_text(n),
            MdNode::Emphasis(n) => self.visit_emphasis(n),
            MdNode::Strong(n) => self.visit_strong(n),
            MdNode::InlineCode(n) => self.visit_inline_code(n),
            MdNode::Code(n) => self.visit_code(n),
            MdNode::Blockquote(n) => self.visit_blockquote(n),
            MdNode::List(n) => self.visit_list(n),
            MdNode::ListItem(n) => self.visit_list_item(n),
            MdNode::ThematicBreak(n) => self.visit_thematic_break(n),
            MdNode::Link(n) => self.visit_link(n),
            MdNode::Image(n) => self.visit_image(n),
            MdNode::Definition(n) => self.visit_definition(n),
            MdNode::Html(n) => self.visit_html(n),
            MdNode::Break(n) => self.visit_break(n),
            MdNode::Table(n) => self.visit_table(n),
            MdNode::TableRow(n) => self.visit_table_row(n),
            MdNode::TableCell(n) => self.visit_table_cell(n),
            MdNode::Delete(n) => self.visit_delete(n),
            MdNode::FootnoteDefinition(n) => self.visit_footnote_definition(n),
            MdNode::FootnoteReference(n) => self.visit_footnote_reference(n),
            MdNode::Yaml(n) => self.visit_yaml(n),
            MdNode::Toml(n) => self.visit_toml(n),
            MdNode::Json(n) => self.visit_json(n),
            MdNode::MdxJsxFlowElement(n) => self.visit_mdx_jsx_flow_element(n),
            MdNode::MdxJsxTextElement(n) => self.visit_mdx_jsx_text_element(n),
            MdNode::MdxjsEsm(n) => self.visit_mdxjs_esm(n),
            MdNode::MdxFlowExpression(n) => self.visit_mdx_flow_expression(n),
            MdNode::MdxTextExpression(n) => self.visit_mdx_text_expression(n),
        }
    }

    fn visit_children(&mut self, children: &[MdNode]) {
        for child in children {
            self.visit_node(child);
        }
    }

    // ── CommonMark ───────────────────────────────────────────────────

    fn visit_document(&mut self, node: &Document) {
        self.visit_children(&node.children);
    }

    fn visit_heading(&mut self, node: &Heading) {
        self.visit_children(&node.children);
    }

    fn visit_paragraph(&mut self, node: &Paragraph) {
        self.visit_children(&node.children);
    }

    fn visit_text(&mut self, node: &Text) {}

    fn visit_emphasis(&mut self, node: &Emphasis) {
        self.visit_children(&node.children);
    }

    fn visit_strong(&mut self, node: &Strong) {
        self.visit_children(&node.children);
    }

    fn visit_inline_code(&mut self, node: &InlineCode) {}

    fn visit_code(&mut self, node: &Code) {}

    fn visit_blockquote(&mut self, node: &Blockquote) {
        self.visit_children(&node.children);
    }

    fn visit_list(&mut self, node: &List) {
        self.visit_children(&node.children);
    }

    fn visit_list_item(&mut self, node: &ListItem) {
        self.visit_children(&node.children);
    }

    fn visit_thematic_break(&mut self, node: &ThematicBreak) {}

    fn visit_link(&mut self, node: &Link) {
        self.visit_children(&node.children);
    }

    fn visit_image(&mut self, node: &Image) {}

    fn visit_definition(&mut self, node: &Definition) {}

    fn visit_html(&mut self, node: &Html) {}

    fn visit_break(&mut self, node: &Break) {}

    // ── GFM ──────────────────────────────────────────────────────────

    fn visit_table(&mut self, node: &Table) {
        self.visit_children(&node.children);
    }

    fn visit_table_row(&mut self, node: &TableRow) {
        self.visit_children(&node.children);
    }

    fn visit_table_cell(&mut self, node: &TableCell) {
        self.visit_children(&node.children);
    }

    fn visit_delete(&mut self, node: &Delete) {
        self.visit_children(&node.children);
    }

    fn visit_footnote_definition(&mut self, node: &FootnoteDefinition) {
        self.visit_children(&node.children);
    }

    fn visit_footnote_reference(&mut self, node: &FootnoteReference) {}

    // ── Frontmatter ──────────────────────────────────────────────────

    fn visit_yaml(&mut self, node: &Yaml) {}

    fn visit_toml(&mut self, node: &Toml) {}

    fn visit_json(&mut self, node: &Json) {}

    // ── MDX ──────────────────────────────────────────────────────────

    fn visit_mdx_jsx_flow_element(&mut self, node: &MdxJsxElement) {
        self.visit_children(&node.children);
    }

    fn visit_mdx_jsx_text_element(&mut self, node: &MdxJsxElement) {
        self.visit_children(&node.children);
    }

    fn visit_mdxjs_esm(&mut self, node: &MdxjsEsm) {}

    fn visit_mdx_flow_expression(&mut self, node: &MdxExpression) {}

    fn visit_mdx_text_expression(&mut self, node: &MdxExpression) {}
}

/// Mutable visitor trait for walking and modifying the Markdown AST in place.
#[allow(unused_variables)]
pub trait MdVisitorMut {
    fn visit_node_mut(&mut self, node: &mut MdNode) {
        match node {
            MdNode::Document(n) => self.visit_document_mut(n),
            MdNode::Heading(n) => self.visit_heading_mut(n),
            MdNode::Paragraph(n) => self.visit_paragraph_mut(n),
            MdNode::Text(n) => self.visit_text_mut(n),
            MdNode::Emphasis(n) => self.visit_emphasis_mut(n),
            MdNode::Strong(n) => self.visit_strong_mut(n),
            MdNode::InlineCode(n) => self.visit_inline_code_mut(n),
            MdNode::Code(n) => self.visit_code_mut(n),
            MdNode::Blockquote(n) => self.visit_blockquote_mut(n),
            MdNode::List(n) => self.visit_list_mut(n),
            MdNode::ListItem(n) => self.visit_list_item_mut(n),
            MdNode::ThematicBreak(n) => self.visit_thematic_break_mut(n),
            MdNode::Link(n) => self.visit_link_mut(n),
            MdNode::Image(n) => self.visit_image_mut(n),
            MdNode::Definition(n) => self.visit_definition_mut(n),
            MdNode::Html(n) => self.visit_html_mut(n),
            MdNode::Break(n) => self.visit_break_mut(n),
            MdNode::Table(n) => self.visit_table_mut(n),
            MdNode::TableRow(n) => self.visit_table_row_mut(n),
            MdNode::TableCell(n) => self.visit_table_cell_mut(n),
            MdNode::Delete(n) => self.visit_delete_mut(n),
            MdNode::FootnoteDefinition(n) => self.visit_footnote_definition_mut(n),
            MdNode::FootnoteReference(n) => self.visit_footnote_reference_mut(n),
            MdNode::Yaml(n) => self.visit_yaml_mut(n),
            MdNode::Toml(n) => self.visit_toml_mut(n),
            MdNode::Json(n) => self.visit_json_mut(n),
            MdNode::MdxJsxFlowElement(n) => self.visit_mdx_jsx_flow_element_mut(n),
            MdNode::MdxJsxTextElement(n) => self.visit_mdx_jsx_text_element_mut(n),
            MdNode::MdxjsEsm(n) => self.visit_mdxjs_esm_mut(n),
            MdNode::MdxFlowExpression(n) => self.visit_mdx_flow_expression_mut(n),
            MdNode::MdxTextExpression(n) => self.visit_mdx_text_expression_mut(n),
        }
    }

    fn visit_children_mut(&mut self, children: &mut Vec<MdNode>) {
        for child in children.iter_mut() {
            self.visit_node_mut(child);
        }
    }

    // ── CommonMark ───────────────────────────────────────────────────

    fn visit_document_mut(&mut self, node: &mut Document) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_heading_mut(&mut self, node: &mut Heading) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_paragraph_mut(&mut self, node: &mut Paragraph) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_text_mut(&mut self, node: &mut Text) {}

    fn visit_emphasis_mut(&mut self, node: &mut Emphasis) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_strong_mut(&mut self, node: &mut Strong) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_inline_code_mut(&mut self, node: &mut InlineCode) {}

    fn visit_code_mut(&mut self, node: &mut Code) {}

    fn visit_blockquote_mut(&mut self, node: &mut Blockquote) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_list_mut(&mut self, node: &mut List) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_list_item_mut(&mut self, node: &mut ListItem) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_thematic_break_mut(&mut self, node: &mut ThematicBreak) {}

    fn visit_link_mut(&mut self, node: &mut Link) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_image_mut(&mut self, node: &mut Image) {}

    fn visit_definition_mut(&mut self, node: &mut Definition) {}

    fn visit_html_mut(&mut self, node: &mut Html) {}

    fn visit_break_mut(&mut self, node: &mut Break) {}

    // ── GFM ──────────────────────────────────────────────────────────

    fn visit_table_mut(&mut self, node: &mut Table) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_table_row_mut(&mut self, node: &mut TableRow) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_table_cell_mut(&mut self, node: &mut TableCell) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_delete_mut(&mut self, node: &mut Delete) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_footnote_definition_mut(&mut self, node: &mut FootnoteDefinition) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_footnote_reference_mut(&mut self, node: &mut FootnoteReference) {}

    // ── Frontmatter ──────────────────────────────────────────────────

    fn visit_yaml_mut(&mut self, node: &mut Yaml) {}

    fn visit_toml_mut(&mut self, node: &mut Toml) {}

    fn visit_json_mut(&mut self, node: &mut Json) {}

    // ── MDX ──────────────────────────────────────────────────────────

    fn visit_mdx_jsx_flow_element_mut(&mut self, node: &mut MdxJsxElement) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_mdx_jsx_text_element_mut(&mut self, node: &mut MdxJsxElement) {
        self.visit_children_mut(&mut node.children);
    }

    fn visit_mdxjs_esm_mut(&mut self, node: &mut MdxjsEsm) {}

    fn visit_mdx_flow_expression_mut(&mut self, node: &mut MdxExpression) {}

    fn visit_mdx_text_expression_mut(&mut self, node: &mut MdxExpression) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};

    /// A simple visitor that counts total nodes visited.
    struct NodeCounter {
        count: usize,
    }

    impl NodeCounter {
        fn new() -> Self {
            Self { count: 0 }
        }
    }

    impl MdVisitor for NodeCounter {
        fn visit_node(&mut self, node: &MdNode) {
            self.count += 1;
            // Continue walking children via the default dispatch
            if let Some(children) = node.children() {
                self.visit_children(children);
            }
        }
    }

    #[test]
    fn counting_visitor() {
        let mut id_gen = NodeIdGen::new();
        let doc = MdNode::Document(Document {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            children: vec![
                MdNode::Heading(Heading {
                    id: id_gen.next_id(),
                    span: Span::new(0, 10),
                    depth: 1,
                    children: vec![MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span: Span::new(2, 10),
                        value: "Title".into(),
                    })],
                    slug: None,
                }),
                MdNode::Paragraph(Paragraph {
                    id: id_gen.next_id(),
                    span: Span::new(11, 50),
                    children: vec![
                        MdNode::Text(Text {
                            id: id_gen.next_id(),
                            span: Span::new(11, 30),
                            value: "Some ".into(),
                        }),
                        MdNode::Strong(Strong {
                            id: id_gen.next_id(),
                            span: Span::new(30, 45),
                            children: vec![MdNode::Text(Text {
                                id: id_gen.next_id(),
                                span: Span::new(32, 43),
                                value: "bold".into(),
                            })],
                        }),
                        MdNode::Text(Text {
                            id: id_gen.next_id(),
                            span: Span::new(45, 50),
                            value: " text".into(),
                        }),
                    ],
                }),
            ],
        });

        let mut counter = NodeCounter::new();
        counter.visit_node(&doc);
        // doc(1) + heading(1) + text(1) + paragraph(1) + text(1) + strong(1) + text(1) + text(1) = 8
        assert_eq!(counter.count, 8);
    }

    /// A simple visitor that collects all heading depths.
    struct HeadingCollector {
        depths: Vec<u8>,
    }

    impl HeadingCollector {
        fn new() -> Self {
            Self { depths: vec![] }
        }
    }

    impl MdVisitor for HeadingCollector {
        fn visit_heading(&mut self, node: &Heading) {
            self.depths.push(node.depth);
            self.visit_children(&node.children);
        }
    }

    #[test]
    fn heading_collector_visitor() {
        let mut id_gen = NodeIdGen::new();
        let doc = MdNode::Document(Document {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![
                MdNode::Heading(Heading {
                    id: id_gen.next_id(),
                    span: Span::new(0, 10),
                    depth: 1,
                    children: vec![],
                    slug: None,
                }),
                MdNode::Heading(Heading {
                    id: id_gen.next_id(),
                    span: Span::new(10, 20),
                    depth: 2,
                    children: vec![],
                    slug: None,
                }),
                MdNode::Paragraph(Paragraph {
                    id: id_gen.next_id(),
                    span: Span::new(20, 40),
                    children: vec![],
                }),
                MdNode::Heading(Heading {
                    id: id_gen.next_id(),
                    span: Span::new(40, 50),
                    depth: 3,
                    children: vec![],
                    slug: None,
                }),
            ],
        });

        let mut collector = HeadingCollector::new();
        collector.visit_node(&doc);
        assert_eq!(collector.depths, vec![1, 2, 3]);
    }

    /// A mutable visitor that uppercases all text nodes.
    struct TextUppercaser;

    impl MdVisitorMut for TextUppercaser {
        fn visit_text_mut(&mut self, node: &mut Text) {
            node.value = node.value.to_uppercase();
        }
    }

    #[test]
    fn mutable_visitor() {
        let mut id_gen = NodeIdGen::new();
        let mut doc = MdNode::Document(Document {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            children: vec![MdNode::Paragraph(Paragraph {
                id: id_gen.next_id(),
                span: Span::new(0, 20),
                children: vec![MdNode::Text(Text {
                    id: id_gen.next_id(),
                    span: Span::new(0, 20),
                    value: "hello world".into(),
                })],
            })],
        });

        let mut visitor = TextUppercaser;
        visitor.visit_node_mut(&mut doc);

        // Verify the text was uppercased
        if let MdNode::Document(doc) = &doc {
            if let MdNode::Paragraph(para) = &doc.children[0] {
                if let MdNode::Text(text) = &para.children[0] {
                    assert_eq!(text.value, "HELLO WORLD");
                } else {
                    panic!("expected Text node");
                }
            } else {
                panic!("expected Paragraph node");
            }
        } else {
            panic!("expected Document node");
        }
    }
}
