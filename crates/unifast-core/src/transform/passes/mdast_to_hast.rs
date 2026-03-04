use crate::api::options::RawHtmlPolicy;
use crate::ast::common::{NodeIdGen, Span};
use crate::ast::hast::builder::HBuilder;
use crate::ast::hast::nodes::*;
use crate::ast::mdast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;
use crate::util::small_map::SmallMap;

/// Lower a parsed MdAst `Document` into an HAst tree.
///
/// Every `MdNode` variant is mapped to its HTML equivalent.
/// MDX nodes and frontmatter nodes are stripped with diagnostics.
pub fn lower(
    doc: &Document,
    id_gen: &mut NodeIdGen,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
) -> HNode {
    let mut builder = HBuilder::new(id_gen);
    let children = lower_children(&doc.children, &mut builder, policy, diagnostics);
    builder.root(doc.span, children)
}

fn lower_children(
    children: &[MdNode],
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
) -> Vec<HNode> {
    children
        .iter()
        .filter_map(|node| lower_node(node, builder, policy, diagnostics))
        .collect()
}

fn lower_node(
    node: &MdNode,
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
) -> Option<HNode> {
    match node {
        MdNode::Document(_) => None, // shouldn't appear nested

        MdNode::Heading(h) => {
            let tag = format!("h{}", h.depth);
            let children = lower_children(&h.children, builder, policy, diagnostics);
            let mut attrs = SmallMap::new();
            if let Some(ref slug) = h.slug {
                attrs.insert("id".to_string(), slug.clone());
            }
            Some(builder.element(h.span, &tag, attrs, children, false))
        }

        MdNode::Paragraph(p) => {
            let children = lower_children(&p.children, builder, policy, diagnostics);
            Some(builder.elem(p.span, "p", children))
        }

        MdNode::Text(t) => Some(builder.text(t.span, &t.value)),

        MdNode::Emphasis(e) => {
            let children = lower_children(&e.children, builder, policy, diagnostics);
            Some(builder.elem(e.span, "em", children))
        }

        MdNode::Strong(s) => {
            let children = lower_children(&s.children, builder, policy, diagnostics);
            Some(builder.elem(s.span, "strong", children))
        }

        MdNode::InlineCode(c) => {
            let text = builder.text(c.span, &c.value);
            Some(builder.elem(c.span, "code", vec![text]))
        }

        MdNode::Code(c) => {
            // <pre><code class="language-xxx">value</code></pre>
            let mut code_attrs = SmallMap::new();
            if let Some(ref lang) = c.lang {
                code_attrs.insert("class".to_string(), format!("language-{}", lang));
            }
            let text = builder.text(c.span, &c.value);
            let code_elem = builder.element(c.span, "code", code_attrs, vec![text], false);
            Some(builder.elem(c.span, "pre", vec![code_elem]))
        }

        MdNode::Blockquote(bq) => {
            let children = lower_children(&bq.children, builder, policy, diagnostics);
            Some(builder.elem(bq.span, "blockquote", children))
        }

        MdNode::List(l) => {
            let tag = if l.ordered { "ol" } else { "ul" };
            let children = lower_children(&l.children, builder, policy, diagnostics);
            let mut attrs = SmallMap::new();
            if l.ordered
                && let Some(start) = l.start
                && start != 1
            {
                attrs.insert("start".to_string(), start.to_string());
            }
            Some(builder.element(l.span, tag, attrs, children, false))
        }

        MdNode::ListItem(li) => {
            let mut children = Vec::new();
            // Task list checkbox
            if let Some(checked) = li.checked {
                let mut checkbox_attrs = SmallMap::new();
                checkbox_attrs.insert("type".to_string(), "checkbox".to_string());
                checkbox_attrs.insert("disabled".to_string(), String::new());
                if checked {
                    checkbox_attrs.insert("checked".to_string(), String::new());
                }
                let checkbox = builder.element(li.span, "input", checkbox_attrs, vec![], true);
                children.push(checkbox);
            }
            children.extend(lower_children(&li.children, builder, policy, diagnostics));
            Some(builder.elem(li.span, "li", children))
        }

        MdNode::ThematicBreak(tb) => {
            Some(builder.element(tb.span, "hr", SmallMap::new(), vec![], true))
        }

        MdNode::Link(l) => {
            let mut attrs = SmallMap::new();
            attrs.insert("href".to_string(), l.url.clone());
            if let Some(ref title) = l.title {
                attrs.insert("title".to_string(), title.clone());
            }
            let children = lower_children(&l.children, builder, policy, diagnostics);
            Some(builder.element(l.span, "a", attrs, children, false))
        }

        MdNode::Image(img) => {
            let mut attrs = SmallMap::new();
            attrs.insert("alt".to_string(), img.alt.clone());
            attrs.insert("src".to_string(), img.url.clone());
            if let Some(ref title) = img.title {
                attrs.insert("title".to_string(), title.clone());
            }
            Some(builder.element(img.span, "img", attrs, vec![], true))
        }

        MdNode::Definition(_) => None, // definitions are metadata, not rendered

        MdNode::Html(h) => lower_raw_html(&h.value, h.span, builder, policy, diagnostics),

        MdNode::Break(br) => Some(builder.element(br.span, "br", SmallMap::new(), vec![], true)),

        // GFM
        MdNode::Table(t) => {
            let mut table_children = Vec::new();

            // First row is header (thead)
            if let Some(first) = t.children.first()
                && let Some(row) =
                    lower_table_row(first, builder, policy, diagnostics, true, &t.align)
            {
                let thead = builder.elem(first.span(), "thead", vec![row]);
                table_children.push(thead);
            }

            // Remaining rows are tbody
            if t.children.len() > 1 {
                let body_rows: Vec<HNode> = t.children[1..]
                    .iter()
                    .filter_map(|r| {
                        lower_table_row(r, builder, policy, diagnostics, false, &t.align)
                    })
                    .collect();
                if !body_rows.is_empty() {
                    let tbody_span = body_rows.first().map(|r| r.span()).unwrap_or(Span::empty());
                    let tbody = builder.elem(tbody_span, "tbody", body_rows);
                    table_children.push(tbody);
                }
            }

            Some(builder.elem(t.span, "table", table_children))
        }

        MdNode::TableRow(_) => None,  // handled by Table
        MdNode::TableCell(_) => None, // handled by TableRow

        MdNode::Delete(d) => {
            let children = lower_children(&d.children, builder, policy, diagnostics);
            Some(builder.elem(d.span, "del", children))
        }

        MdNode::FootnoteDefinition(fd) => {
            // Render as an <li> with id="fn-{identifier}"
            let children = lower_children(&fd.children, builder, policy, diagnostics);
            let mut attrs = SmallMap::new();
            attrs.insert("id".to_string(), format!("fn-{}", fd.identifier));
            Some(builder.element(fd.span, "li", attrs, children, false))
        }

        MdNode::FootnoteReference(fr) => {
            let mut link_attrs = SmallMap::new();
            link_attrs.insert("class".to_string(), "footnote-ref".to_string());
            link_attrs.insert("href".to_string(), format!("#fn-{}", fr.identifier));
            let text = builder.text(fr.span, &fr.identifier);
            let link = builder.element(fr.span, "a", link_attrs, vec![text], false);
            let mut sup_attrs = SmallMap::new();
            sup_attrs.insert("class".to_string(), "footnote-ref".to_string());
            Some(builder.element(fr.span, "sup", sup_attrs, vec![link], false))
        }

        // Frontmatter nodes shouldn't appear in rendered output
        MdNode::Yaml(_) | MdNode::Toml(_) | MdNode::Json(_) => None,

        // MDX nodes — cannot render to plain HTML
        MdNode::MdxJsxFlowElement(el) | MdNode::MdxJsxTextElement(el) => {
            diagnostics.warn(
                format!(
                    "JSX element '{}' cannot be rendered to HTML",
                    el.name.as_deref().unwrap_or("<fragment>")
                ),
                el.span,
            );
            None
        }

        MdNode::MdxjsEsm(esm) => {
            diagnostics.warn("ESM block cannot be rendered to HTML", esm.span);
            None
        }

        MdNode::MdxFlowExpression(expr) | MdNode::MdxTextExpression(expr) => {
            diagnostics.warn("MDX expression cannot be rendered to HTML", expr.span);
            None
        }
    }
}

fn lower_table_row(
    node: &MdNode,
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
    is_header: bool,
    align: &[AlignKind],
) -> Option<HNode> {
    if let MdNode::TableRow(row) = node {
        let cell_tag = if is_header { "th" } else { "td" };
        let cells: Vec<HNode> = row
            .children
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| {
                if let MdNode::TableCell(tc) = cell {
                    let children = lower_children(&tc.children, builder, policy, diagnostics);
                    let mut attrs = SmallMap::new();
                    if let Some(a) = align.get(i) {
                        match a {
                            AlignKind::Left => {
                                attrs.insert("align".to_string(), "left".to_string());
                            }
                            AlignKind::Center => {
                                attrs.insert("align".to_string(), "center".to_string());
                            }
                            AlignKind::Right => {
                                attrs.insert("align".to_string(), "right".to_string());
                            }
                            AlignKind::None => {}
                        }
                    }
                    Some(builder.element(tc.span, cell_tag, attrs, children, false))
                } else {
                    None
                }
            })
            .collect();
        Some(builder.elem(row.span, "tr", cells))
    } else {
        None
    }
}

fn lower_raw_html(
    value: &str,
    span: Span,
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
) -> Option<HNode> {
    match policy {
        RawHtmlPolicy::Disallow => {
            diagnostics.warn(
                format!(
                    "Raw HTML disallowed: {}",
                    value.chars().take(50).collect::<String>()
                ),
                span,
            );
            // Escape and emit as text
            Some(builder.text(span, value))
        }
        RawHtmlPolicy::AllowDangerous => Some(builder.raw(span, value)),
        RawHtmlPolicy::ParseAndSanitize => {
            // For now, treat as AllowDangerous — sanitization happens in a later pass
            Some(builder.raw(span, value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

    // ── Helpers ──────────────────────────────────────────────────────

    fn make_doc(id_gen: &mut NodeIdGen, children: Vec<MdNode>) -> Document {
        Document {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children,
        }
    }

    fn make_text(id_gen: &mut NodeIdGen, value: &str) -> MdNode {
        MdNode::Text(Text {
            id: id_gen.next_id(),
            span: Span::new(0, value.len() as u32),
            value: value.to_string(),
        })
    }

    fn lower_doc(doc: &Document) -> HNode {
        lower_doc_with_policy(doc, RawHtmlPolicy::Disallow)
    }

    fn lower_doc_with_policy(doc: &Document, policy: RawHtmlPolicy) -> HNode {
        let mut id_gen = NodeIdGen::new();
        let mut diagnostics = DiagnosticSink::new();
        lower(doc, &mut id_gen, policy, &mut diagnostics)
    }

    fn lower_doc_with_diagnostics(
        doc: &Document,
        policy: RawHtmlPolicy,
    ) -> (HNode, DiagnosticSink) {
        let mut id_gen = NodeIdGen::new();
        let mut diagnostics = DiagnosticSink::new();
        let root = lower(doc, &mut id_gen, policy, &mut diagnostics);
        (root, diagnostics)
    }

    /// Unwrap the root's children vec from an HNode::Root.
    fn root_children(root: &HNode) -> &[HNode] {
        root.children().expect("expected Root with children")
    }

    /// Unwrap an HNode::Element, panicking with a message if it's the wrong variant.
    fn expect_element(node: &HNode) -> &HElement {
        if let HNode::Element(e) = node {
            e
        } else {
            panic!("expected HNode::Element, got {:?}", node);
        }
    }

    fn expect_text(node: &HNode) -> &HText {
        if let HNode::Text(t) = node {
            t
        } else {
            panic!("expected HNode::Text, got {:?}", node);
        }
    }

    fn expect_raw(node: &HNode) -> &HRaw {
        if let HNode::Raw(r) = node {
            r
        } else {
            panic!("expected HNode::Raw, got {:?}", node);
        }
    }

    // ── Tests ────────────────────────────────────────────────────────

    #[test]
    fn lower_empty_document() {
        let mut id_gen = NodeIdGen::new();
        let doc = make_doc(&mut id_gen, vec![]);
        let root = lower_doc(&doc);
        assert!(root_children(&root).is_empty());
    }

    #[test]
    fn lower_heading_h1() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello");
        let heading = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            depth: 1,
            children: vec![text],
            slug: None,
        });
        let doc = make_doc(&mut id_gen, vec![heading]);
        let root = lower_doc(&doc);

        let children = root_children(&root);
        assert_eq!(children.len(), 1);
        let h1 = expect_element(&children[0]);
        assert_eq!(h1.tag, "h1");
        assert_eq!(expect_text(&h1.children[0]).value, "Hello");
    }

    #[test]
    fn lower_heading_h2_through_h6() {
        for depth in 2..=6u8 {
            let mut id_gen = NodeIdGen::new();
            let text = make_text(&mut id_gen, "Title");
            let heading = MdNode::Heading(Heading {
                id: id_gen.next_id(),
                span: Span::new(0, 10),
                depth,
                children: vec![text],
                slug: None,
            });
            let doc = make_doc(&mut id_gen, vec![heading]);
            let root = lower_doc(&doc);

            let el = expect_element(&root_children(&root)[0]);
            assert_eq!(el.tag, format!("h{}", depth));
        }
    }

    #[test]
    fn lower_heading_with_slug() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Hello World");
        let heading = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, 14),
            depth: 2,
            children: vec![text],
            slug: Some("hello-world".to_string()),
        });
        let doc = make_doc(&mut id_gen, vec![heading]);
        let root = lower_doc(&doc);

        let el = expect_element(&root_children(&root)[0]);
        assert_eq!(el.tag, "h2");
        assert_eq!(
            el.attributes.get(&"id".to_string()),
            Some(&"hello-world".to_string())
        );
    }

    #[test]
    fn lower_paragraph() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Some text");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![text],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let el = expect_element(&root_children(&root)[0]);
        assert_eq!(el.tag, "p");
        assert_eq!(expect_text(&el.children[0]).value, "Some text");
    }

    #[test]
    fn lower_text_node() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "plain text");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![text],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let t = expect_text(&p.children[0]);
        assert_eq!(t.value, "plain text");
    }

    #[test]
    fn lower_emphasis_to_em() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "italic");
        let em = MdNode::Emphasis(Emphasis {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![text],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![em],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let em_el = expect_element(&p.children[0]);
        assert_eq!(em_el.tag, "em");
        assert_eq!(expect_text(&em_el.children[0]).value, "italic");
    }

    #[test]
    fn lower_strong_to_strong() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "bold");
        let strong = MdNode::Strong(Strong {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![text],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![strong],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let strong_el = expect_element(&p.children[0]);
        assert_eq!(strong_el.tag, "strong");
        assert_eq!(expect_text(&strong_el.children[0]).value, "bold");
    }

    #[test]
    fn lower_inline_code() {
        let mut id_gen = NodeIdGen::new();
        let ic = MdNode::InlineCode(InlineCode {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            value: "foo()".to_string(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![ic],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let code_el = expect_element(&p.children[0]);
        assert_eq!(code_el.tag, "code");
        assert_eq!(expect_text(&code_el.children[0]).value, "foo()");
    }

    #[test]
    fn lower_code_fence_with_lang() {
        let mut id_gen = NodeIdGen::new();
        let code = MdNode::Code(Code {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            value: "fn main() {}".to_string(),
            lang: Some("rust".to_string()),
            meta: None,
        });
        let doc = make_doc(&mut id_gen, vec![code]);
        let root = lower_doc(&doc);

        let pre = expect_element(&root_children(&root)[0]);
        assert_eq!(pre.tag, "pre");
        let code_el = expect_element(&pre.children[0]);
        assert_eq!(code_el.tag, "code");
        assert_eq!(
            code_el.attributes.get(&"class".to_string()),
            Some(&"language-rust".to_string())
        );
        assert_eq!(expect_text(&code_el.children[0]).value, "fn main() {}");
    }

    #[test]
    fn lower_code_fence_no_lang() {
        let mut id_gen = NodeIdGen::new();
        let code = MdNode::Code(Code {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "hello".to_string(),
            lang: None,
            meta: None,
        });
        let doc = make_doc(&mut id_gen, vec![code]);
        let root = lower_doc(&doc);

        let pre = expect_element(&root_children(&root)[0]);
        let code_el = expect_element(&pre.children[0]);
        assert_eq!(code_el.tag, "code");
        assert!(code_el.attributes.is_empty());
    }

    #[test]
    fn lower_blockquote() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "quoted");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(2, 9),
            children: vec![text],
        });
        let bq = MdNode::Blockquote(Blockquote {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![para],
        });
        let doc = make_doc(&mut id_gen, vec![bq]);
        let root = lower_doc(&doc);

        let bq_el = expect_element(&root_children(&root)[0]);
        assert_eq!(bq_el.tag, "blockquote");
        let p = expect_element(&bq_el.children[0]);
        assert_eq!(p.tag, "p");
    }

    #[test]
    fn lower_unordered_list() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "item");
        let li = MdNode::ListItem(ListItem {
            id: id_gen.next_id(),
            span: Span::new(2, 6),
            spread: false,
            checked: None,
            children: vec![text],
        });
        let list = MdNode::List(List {
            id: id_gen.next_id(),
            span: Span::new(0, 7),
            ordered: false,
            start: None,
            spread: false,
            children: vec![li],
        });
        let doc = make_doc(&mut id_gen, vec![list]);
        let root = lower_doc(&doc);

        let ul = expect_element(&root_children(&root)[0]);
        assert_eq!(ul.tag, "ul");
        assert!(ul.attributes.is_empty());
        let li_el = expect_element(&ul.children[0]);
        assert_eq!(li_el.tag, "li");
    }

    #[test]
    fn lower_ordered_list_with_start() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "item");
        let li = MdNode::ListItem(ListItem {
            id: id_gen.next_id(),
            span: Span::new(3, 7),
            spread: false,
            checked: None,
            children: vec![text],
        });
        let list = MdNode::List(List {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            ordered: true,
            start: Some(5),
            spread: false,
            children: vec![li],
        });
        let doc = make_doc(&mut id_gen, vec![list]);
        let root = lower_doc(&doc);

        let ol = expect_element(&root_children(&root)[0]);
        assert_eq!(ol.tag, "ol");
        assert_eq!(
            ol.attributes.get(&"start".to_string()),
            Some(&"5".to_string())
        );
    }

    #[test]
    fn lower_ordered_list_start_1_no_attr() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "item");
        let li = MdNode::ListItem(ListItem {
            id: id_gen.next_id(),
            span: Span::new(3, 7),
            spread: false,
            checked: None,
            children: vec![text],
        });
        let list = MdNode::List(List {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            ordered: true,
            start: Some(1),
            spread: false,
            children: vec![li],
        });
        let doc = make_doc(&mut id_gen, vec![list]);
        let root = lower_doc(&doc);

        let ol = expect_element(&root_children(&root)[0]);
        assert_eq!(ol.tag, "ol");
        // start=1 should not produce a "start" attribute
        assert!(ol.attributes.get(&"start".to_string()).is_none());
    }

    #[test]
    fn lower_task_list_item_checked() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "done");
        let li = MdNode::ListItem(ListItem {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            spread: false,
            checked: Some(true),
            children: vec![text],
        });
        let list = MdNode::List(List {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            ordered: false,
            start: None,
            spread: false,
            children: vec![li],
        });
        let doc = make_doc(&mut id_gen, vec![list]);
        let root = lower_doc(&doc);

        let ul = expect_element(&root_children(&root)[0]);
        let li_el = expect_element(&ul.children[0]);
        assert_eq!(li_el.tag, "li");
        // First child should be the checkbox input
        let checkbox = expect_element(&li_el.children[0]);
        assert_eq!(checkbox.tag, "input");
        assert_eq!(
            checkbox.attributes.get(&"type".to_string()),
            Some(&"checkbox".to_string())
        );
        assert!(checkbox.attributes.contains_key(&"checked".to_string()));
        assert!(checkbox.attributes.contains_key(&"disabled".to_string()));
        assert!(checkbox.self_closing);
    }

    #[test]
    fn lower_task_list_item_unchecked() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "todo");
        let li = MdNode::ListItem(ListItem {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            spread: false,
            checked: Some(false),
            children: vec![text],
        });
        let list = MdNode::List(List {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            ordered: false,
            start: None,
            spread: false,
            children: vec![li],
        });
        let doc = make_doc(&mut id_gen, vec![list]);
        let root = lower_doc(&doc);

        let ul = expect_element(&root_children(&root)[0]);
        let li_el = expect_element(&ul.children[0]);
        let checkbox = expect_element(&li_el.children[0]);
        assert_eq!(checkbox.tag, "input");
        assert!(!checkbox.attributes.contains_key(&"checked".to_string()));
        assert!(checkbox.attributes.contains_key(&"disabled".to_string()));
    }

    #[test]
    fn lower_thematic_break_to_hr() {
        let mut id_gen = NodeIdGen::new();
        let tb = MdNode::ThematicBreak(ThematicBreak {
            id: id_gen.next_id(),
            span: Span::new(0, 3),
        });
        let doc = make_doc(&mut id_gen, vec![tb]);
        let root = lower_doc(&doc);

        let hr = expect_element(&root_children(&root)[0]);
        assert_eq!(hr.tag, "hr");
        assert!(hr.self_closing);
        assert!(hr.children.is_empty());
    }

    #[test]
    fn lower_link_to_a() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "click");
        let link = MdNode::Link(Link {
            id: id_gen.next_id(),
            span: Span::new(0, 25),
            url: "https://example.com".to_string(),
            title: None,
            children: vec![text],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 25),
            children: vec![link],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let a = expect_element(&p.children[0]);
        assert_eq!(a.tag, "a");
        assert_eq!(
            a.attributes.get(&"href".to_string()),
            Some(&"https://example.com".to_string())
        );
        assert!(a.attributes.get(&"title".to_string()).is_none());
        assert_eq!(expect_text(&a.children[0]).value, "click");
    }

    #[test]
    fn lower_link_with_title() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "click");
        let link = MdNode::Link(Link {
            id: id_gen.next_id(),
            span: Span::new(0, 40),
            url: "https://example.com".to_string(),
            title: Some("Example".to_string()),
            children: vec![text],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 40),
            children: vec![link],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let a = expect_element(&p.children[0]);
        assert_eq!(a.tag, "a");
        assert_eq!(
            a.attributes.get(&"title".to_string()),
            Some(&"Example".to_string())
        );
    }

    #[test]
    fn lower_image_to_img() {
        let mut id_gen = NodeIdGen::new();
        let img = MdNode::Image(Image {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            url: "photo.jpg".to_string(),
            alt: "A photo".to_string(),
            title: None,
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            children: vec![img],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let img_el = expect_element(&p.children[0]);
        assert_eq!(img_el.tag, "img");
        assert!(img_el.self_closing);
        assert_eq!(
            img_el.attributes.get(&"src".to_string()),
            Some(&"photo.jpg".to_string())
        );
        assert_eq!(
            img_el.attributes.get(&"alt".to_string()),
            Some(&"A photo".to_string())
        );
    }

    #[test]
    fn lower_image_with_title() {
        let mut id_gen = NodeIdGen::new();
        let img = MdNode::Image(Image {
            id: id_gen.next_id(),
            span: Span::new(0, 40),
            url: "photo.jpg".to_string(),
            alt: "A photo".to_string(),
            title: Some("My Photo".to_string()),
        });
        let doc = make_doc(&mut id_gen, vec![img]);
        let root = lower_doc(&doc);

        let img_el = expect_element(&root_children(&root)[0]);
        assert_eq!(
            img_el.attributes.get(&"title".to_string()),
            Some(&"My Photo".to_string())
        );
    }

    #[test]
    fn lower_break_to_br() {
        let mut id_gen = NodeIdGen::new();
        let br = MdNode::Break(Break {
            id: id_gen.next_id(),
            span: Span::new(0, 2),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![br],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let br_el = expect_element(&p.children[0]);
        assert_eq!(br_el.tag, "br");
        assert!(br_el.self_closing);
    }

    #[test]
    fn lower_strikethrough_to_del() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "deleted");
        let del = MdNode::Delete(Delete {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            children: vec![text],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            children: vec![del],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let del_el = expect_element(&p.children[0]);
        assert_eq!(del_el.tag, "del");
        assert_eq!(expect_text(&del_el.children[0]).value, "deleted");
    }

    #[test]
    fn lower_table_basic() {
        let mut id_gen = NodeIdGen::new();
        // Header row
        let h_text = make_text(&mut id_gen, "Name");
        let h_cell = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            children: vec![h_text],
        });
        let header_row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            is_header: true,
            children: vec![h_cell],
        });
        // Body row
        let b_text = make_text(&mut id_gen, "Alice");
        let b_cell = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(10, 17),
            children: vec![b_text],
        });
        let body_row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(10, 19),
            is_header: false,
            children: vec![b_cell],
        });
        let table = MdNode::Table(Table {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            align: vec![AlignKind::None],
            children: vec![header_row, body_row],
        });
        let doc = make_doc(&mut id_gen, vec![table]);
        let root = lower_doc(&doc);

        let table_el = expect_element(&root_children(&root)[0]);
        assert_eq!(table_el.tag, "table");
        assert_eq!(table_el.children.len(), 2); // thead + tbody

        let thead = expect_element(&table_el.children[0]);
        assert_eq!(thead.tag, "thead");
        let tr_head = expect_element(&thead.children[0]);
        assert_eq!(tr_head.tag, "tr");
        let th = expect_element(&tr_head.children[0]);
        assert_eq!(th.tag, "th");

        let tbody = expect_element(&table_el.children[1]);
        assert_eq!(tbody.tag, "tbody");
        let tr_body = expect_element(&tbody.children[0]);
        assert_eq!(tr_body.tag, "tr");
        let td = expect_element(&tr_body.children[0]);
        assert_eq!(td.tag, "td");
    }

    #[test]
    fn lower_table_alignment() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "Left");
        let c1 = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            children: vec![t1],
        });
        let t2 = make_text(&mut id_gen, "Center");
        let c2 = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(7, 15),
            children: vec![t2],
        });
        let t3 = make_text(&mut id_gen, "Right");
        let c3 = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(16, 23),
            children: vec![t3],
        });
        let header_row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(0, 24),
            is_header: true,
            children: vec![c1, c2, c3],
        });
        let table = MdNode::Table(Table {
            id: id_gen.next_id(),
            span: Span::new(0, 25),
            align: vec![AlignKind::Left, AlignKind::Center, AlignKind::Right],
            children: vec![header_row],
        });
        let doc = make_doc(&mut id_gen, vec![table]);
        let root = lower_doc(&doc);

        let table_el = expect_element(&root_children(&root)[0]);
        let thead = expect_element(&table_el.children[0]);
        let tr = expect_element(&thead.children[0]);

        let th0 = expect_element(&tr.children[0]);
        assert_eq!(
            th0.attributes.get(&"align".to_string()),
            Some(&"left".to_string())
        );

        let th1 = expect_element(&tr.children[1]);
        assert_eq!(
            th1.attributes.get(&"align".to_string()),
            Some(&"center".to_string())
        );

        let th2 = expect_element(&tr.children[2]);
        assert_eq!(
            th2.attributes.get(&"align".to_string()),
            Some(&"right".to_string())
        );
    }

    #[test]
    fn lower_footnote_reference() {
        let mut id_gen = NodeIdGen::new();
        let fr = MdNode::FootnoteReference(FootnoteReference {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            identifier: "1".to_string(),
            label: None,
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![fr],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let sup = expect_element(&p.children[0]);
        assert_eq!(sup.tag, "sup");
        assert_eq!(
            sup.attributes.get(&"class".to_string()),
            Some(&"footnote-ref".to_string())
        );
        let link = expect_element(&sup.children[0]);
        assert_eq!(link.tag, "a");
        assert_eq!(
            link.attributes.get(&"href".to_string()),
            Some(&"#fn-1".to_string())
        );
    }

    #[test]
    fn lower_footnote_definition() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "Footnote text");
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(5, 19),
            children: vec![text],
        });
        let fd = MdNode::FootnoteDefinition(FootnoteDefinition {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            identifier: "1".to_string(),
            label: None,
            children: vec![para],
        });
        let doc = make_doc(&mut id_gen, vec![fd]);
        let root = lower_doc(&doc);

        let li = expect_element(&root_children(&root)[0]);
        assert_eq!(li.tag, "li");
        assert_eq!(
            li.attributes.get(&"id".to_string()),
            Some(&"fn-1".to_string())
        );
    }

    #[test]
    fn lower_definition_is_stripped() {
        let mut id_gen = NodeIdGen::new();
        let def = MdNode::Definition(Definition {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            identifier: "example".to_string(),
            label: None,
            url: "https://example.com".to_string(),
            title: None,
        });
        let doc = make_doc(&mut id_gen, vec![def]);
        let root = lower_doc(&doc);

        assert!(root_children(&root).is_empty());
    }

    #[test]
    fn lower_frontmatter_nodes_are_stripped() {
        let mut id_gen = NodeIdGen::new();
        let yaml = MdNode::Yaml(Yaml {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "title: test".to_string(),
        });
        let toml = MdNode::Toml(Toml {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "title = 'test'".to_string(),
        });
        let json = MdNode::Json(Json {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            value: "{\"title\": \"test\"}".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![yaml, toml, json]);
        let root = lower_doc(&doc);

        assert!(root_children(&root).is_empty());
    }

    #[test]
    fn lower_nested_document_is_stripped() {
        let mut id_gen = NodeIdGen::new();
        let nested_doc = MdNode::Document(Document {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![],
        });
        let doc = make_doc(&mut id_gen, vec![nested_doc]);
        let root = lower_doc(&doc);

        assert!(root_children(&root).is_empty());
    }

    #[test]
    fn raw_html_disallow_policy() {
        let mut id_gen = NodeIdGen::new();
        let html = MdNode::Html(Html {
            id: id_gen.next_id(),
            span: Span::new(0, 18),
            value: "<div>hello</div>".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![html]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        // Raw HTML should become text
        let children = root_children(&root);
        assert_eq!(children.len(), 1);
        let t = expect_text(&children[0]);
        assert_eq!(t.value, "<div>hello</div>");

        // A warning should be emitted
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn raw_html_allow_dangerous() {
        let mut id_gen = NodeIdGen::new();
        let html = MdNode::Html(Html {
            id: id_gen.next_id(),
            span: Span::new(0, 18),
            value: "<div>hello</div>".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![html]);
        let root = lower_doc_with_policy(&doc, RawHtmlPolicy::AllowDangerous);

        let children = root_children(&root);
        assert_eq!(children.len(), 1);
        let raw = expect_raw(&children[0]);
        assert_eq!(raw.value, "<div>hello</div>");
    }

    #[test]
    fn raw_html_parse_and_sanitize() {
        let mut id_gen = NodeIdGen::new();
        let html = MdNode::Html(Html {
            id: id_gen.next_id(),
            span: Span::new(0, 18),
            value: "<script>alert(1)</script>".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![html]);
        let root = lower_doc_with_policy(&doc, RawHtmlPolicy::ParseAndSanitize);

        // For now, ParseAndSanitize keeps as Raw (sanitization in later pass)
        let children = root_children(&root);
        assert_eq!(children.len(), 1);
        let raw = expect_raw(&children[0]);
        assert_eq!(raw.value, "<script>alert(1)</script>");
    }

    #[test]
    fn lower_mdx_jsx_flow_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let jsx = MdNode::MdxJsxFlowElement(MdxJsxElement {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            name: Some("MyComponent".to_string()),
            attributes: vec![],
            children: vec![],
        });
        let doc = make_doc(&mut id_gen, vec![jsx]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(root_children(&root).is_empty());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_mdx_jsx_text_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let jsx = MdNode::MdxJsxTextElement(MdxJsxElement {
            id: id_gen.next_id(),
            span: Span::new(0, 20),
            name: Some("Inline".to_string()),
            attributes: vec![],
            children: vec![],
        });
        let doc = make_doc(&mut id_gen, vec![jsx]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(root_children(&root).is_empty());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_mdx_jsx_fragment_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let jsx = MdNode::MdxJsxFlowElement(MdxJsxElement {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            name: None, // fragment
            attributes: vec![],
            children: vec![],
        });
        let doc = make_doc(&mut id_gen, vec![jsx]);
        let (_, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_mdx_esm_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let esm = MdNode::MdxjsEsm(MdxjsEsm {
            id: id_gen.next_id(),
            span: Span::new(0, 30),
            value: "import x from 'y'".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![esm]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(root_children(&root).is_empty());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_mdx_flow_expression_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let expr = MdNode::MdxFlowExpression(MdxExpression {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "1 + 2".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![expr]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(root_children(&root).is_empty());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_mdx_text_expression_emits_warning() {
        let mut id_gen = NodeIdGen::new();
        let expr = MdNode::MdxTextExpression(MdxExpression {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            value: "x".to_string(),
        });
        let doc = make_doc(&mut id_gen, vec![expr]);
        let (root, diagnostics) = lower_doc_with_diagnostics(&doc, RawHtmlPolicy::Disallow);

        assert!(root_children(&root).is_empty());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn lower_nested_emphasis_in_strong() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "both");
        let em = MdNode::Emphasis(Emphasis {
            id: id_gen.next_id(),
            span: Span::new(2, 8),
            children: vec![text],
        });
        let strong = MdNode::Strong(Strong {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![em],
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            children: vec![strong],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        let strong_el = expect_element(&p.children[0]);
        assert_eq!(strong_el.tag, "strong");
        let em_el = expect_element(&strong_el.children[0]);
        assert_eq!(em_el.tag, "em");
        assert_eq!(expect_text(&em_el.children[0]).value, "both");
    }

    #[test]
    fn lower_multiple_paragraphs() {
        let mut id_gen = NodeIdGen::new();
        let t1 = make_text(&mut id_gen, "First");
        let p1 = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(0, 6),
            children: vec![t1],
        });
        let t2 = make_text(&mut id_gen, "Second");
        let p2 = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span: Span::new(7, 14),
            children: vec![t2],
        });
        let doc = make_doc(&mut id_gen, vec![p1, p2]);
        let root = lower_doc(&doc);

        let children = root_children(&root);
        assert_eq!(children.len(), 2);
        assert_eq!(expect_element(&children[0]).tag, "p");
        assert_eq!(expect_element(&children[1]).tag, "p");
    }

    #[test]
    fn lower_table_row_standalone_is_none() {
        // TableRow and TableCell as direct children should be stripped
        let mut id_gen = NodeIdGen::new();
        let row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            is_header: false,
            children: vec![],
        });
        let cell = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(0, 5),
            children: vec![],
        });
        let doc = make_doc(&mut id_gen, vec![row, cell]);
        let root = lower_doc(&doc);

        assert!(root_children(&root).is_empty());
    }

    #[test]
    fn lower_heading_no_slug_no_id_attr() {
        let mut id_gen = NodeIdGen::new();
        let text = make_text(&mut id_gen, "No Slug");
        let heading = MdNode::Heading(Heading {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            depth: 3,
            children: vec![text],
            slug: None,
        });
        let doc = make_doc(&mut id_gen, vec![heading]);
        let root = lower_doc(&doc);

        let h3 = expect_element(&root_children(&root)[0]);
        assert_eq!(h3.tag, "h3");
        assert!(h3.attributes.is_empty());
    }

    #[test]
    fn lower_table_header_only_no_tbody() {
        let mut id_gen = NodeIdGen::new();
        let t = make_text(&mut id_gen, "Header");
        let cell = MdNode::TableCell(TableCell {
            id: id_gen.next_id(),
            span: Span::new(0, 8),
            children: vec![t],
        });
        let header_row = MdNode::TableRow(TableRow {
            id: id_gen.next_id(),
            span: Span::new(0, 10),
            is_header: true,
            children: vec![cell],
        });
        let table = MdNode::Table(Table {
            id: id_gen.next_id(),
            span: Span::new(0, 11),
            align: vec![AlignKind::None],
            children: vec![header_row],
        });
        let doc = make_doc(&mut id_gen, vec![table]);
        let root = lower_doc(&doc);

        let table_el = expect_element(&root_children(&root)[0]);
        assert_eq!(table_el.tag, "table");
        // Only thead, no tbody
        assert_eq!(table_el.children.len(), 1);
        let thead = expect_element(&table_el.children[0]);
        assert_eq!(thead.tag, "thead");
    }

    #[test]
    fn lower_preserves_span_information() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(5, 15);
        let text = MdNode::Text(Text {
            id: id_gen.next_id(),
            span,
            value: "hello".to_string(),
        });
        let para = MdNode::Paragraph(Paragraph {
            id: id_gen.next_id(),
            span,
            children: vec![text],
        });
        let doc = make_doc(&mut id_gen, vec![para]);
        let root = lower_doc(&doc);

        let p = expect_element(&root_children(&root)[0]);
        assert_eq!(p.span, span);
        let t = expect_text(&p.children[0]);
        assert_eq!(t.span, span);
    }
}
