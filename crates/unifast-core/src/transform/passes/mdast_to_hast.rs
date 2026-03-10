use crate::api::options::{GithubAlertIconMode, RawHtmlPolicy};
use crate::ast::common::{NodeIdGen, Span};
use crate::ast::hast::builder::HBuilder;
use crate::ast::hast::nodes::*;
use crate::ast::mdast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;
use crate::transform::passes::github_alert::resolve_icon_svg;
use crate::util::small_map::SmallMap;

pub fn lower(
    doc: &Document,
    id_gen: &mut NodeIdGen,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
) -> HNode {
    lower_with_icons(
        doc,
        id_gen,
        policy,
        diagnostics,
        &GithubAlertIconMode::default(),
        false,
        "/wiki/{slug}",
    )
}

pub fn lower_with_icons(
    doc: &Document,
    id_gen: &mut NodeIdGen,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
    icon_mode: &GithubAlertIconMode,
    figure_enabled: bool,
    wiki_link_template: &str,
) -> HNode {
    let mut builder = HBuilder::new(id_gen);
    let children = lower_children(
        &doc.children,
        &mut builder,
        policy,
        diagnostics,
        icon_mode,
        figure_enabled,
        wiki_link_template,
    );
    builder.root(doc.span, children)
}

fn lower_children(
    children: &[MdNode],
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
    icon_mode: &GithubAlertIconMode,
    figure_enabled: bool,
    wiki_link_template: &str,
) -> Vec<HNode> {
    children
        .iter()
        .filter_map(|node| {
            lower_node(
                node,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            )
        })
        .collect()
}

fn lower_node(
    node: &MdNode,
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
    icon_mode: &GithubAlertIconMode,
    figure_enabled: bool,
    wiki_link_template: &str,
) -> Option<HNode> {
    match node {
        MdNode::Document(_) => None,

        MdNode::Heading(h) => {
            let tag = format!("h{}", h.depth);
            let children = lower_children(
                &h.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            let mut attrs = SmallMap::new();
            if let Some(ref slug) = h.slug {
                attrs.insert("id".to_string(), slug.clone());
            }
            for (k, v) in h.extra_attrs.iter() {
                attrs.insert(k.clone(), v.clone());
            }
            Some(builder.element(h.span, &tag, attrs, children, false))
        }

        MdNode::Paragraph(p) => {
            let children = lower_children(
                &p.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(p.span, "p", children))
        }

        MdNode::Text(t) => {
            let value = t.value.replace('\n', " ");
            Some(builder.text(t.span, &value))
        }

        MdNode::Emphasis(e) => {
            let children = lower_children(
                &e.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(e.span, "em", children))
        }

        MdNode::Strong(s) => {
            let children = lower_children(
                &s.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(s.span, "strong", children))
        }

        MdNode::InlineCode(c) => {
            let text = builder.text(c.span, &c.value);
            Some(builder.elem(c.span, "code", vec![text]))
        }

        MdNode::Code(c) => {
            let mut code_attrs = SmallMap::new();
            if let Some(ref lang) = c.lang {
                code_attrs.insert("class".to_string(), format!("language-{lang}"));
            }
            let text = builder.text(c.span, &c.value);
            let code_elem = builder.element(c.span, "code", code_attrs, vec![text], false);
            let mut pre_attrs = SmallMap::new();
            if let Some(ref meta) = c.meta {
                pre_attrs.insert("data-meta".to_string(), meta.clone());
            }
            if let Some(ref lang) = c.lang {
                pre_attrs.insert("data-lang".to_string(), lang.clone());
            }
            Some(builder.element(c.span, "pre", pre_attrs, vec![code_elem], false))
        }

        MdNode::Blockquote(bq) => {
            let children = lower_children(
                &bq.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            if let Some(ref alert_kind) = bq.alert_type {
                lower_alert(bq.span, alert_kind, children, builder, icon_mode)
            } else {
                Some(builder.elem(bq.span, "blockquote", children))
            }
        }

        MdNode::List(l) => {
            let tag = if l.ordered { "ol" } else { "ul" };
            let children = lower_children(
                &l.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
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
            children.extend(lower_children(
                &li.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            ));
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
            let children = lower_children(
                &l.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.element(l.span, "a", attrs, children, false))
        }

        MdNode::Image(img) => {
            let mut attrs = SmallMap::new();
            attrs.insert("src".to_string(), img.url.clone());
            attrs.insert("alt".to_string(), img.alt.clone());
            if let Some(ref title) = img.title {
                attrs.insert("title".to_string(), title.clone());
            }
            let img_elem = builder.element(img.span, "img", attrs, vec![], true);

            if figure_enabled {
                let mut children = vec![img_elem];
                if !img.alt.is_empty() {
                    let caption_text = builder.text(img.span, &img.alt);
                    let figcaption = builder.elem(img.span, "figcaption", vec![caption_text]);
                    children.push(figcaption);
                }
                Some(builder.elem(img.span, "figure", children))
            } else {
                Some(img_elem)
            }
        }

        MdNode::Definition(_) => None,

        MdNode::Html(h) => lower_raw_html(&h.value, h.span, builder, policy, diagnostics),

        MdNode::Break(br) => Some(builder.element(br.span, "br", SmallMap::new(), vec![], true)),

        MdNode::Table(t) => {
            let mut table_children = Vec::new();

            if let Some(first) = t.children.first()
                && let Some(row) = lower_table_row(
                    first,
                    builder,
                    policy,
                    diagnostics,
                    true,
                    &t.align,
                    icon_mode,
                    figure_enabled,
                    wiki_link_template,
                )
            {
                let thead = builder.elem(first.span(), "thead", vec![row]);
                table_children.push(thead);
            }

            if t.children.len() > 1 {
                let body_rows: Vec<HNode> = t.children[1..]
                    .iter()
                    .filter_map(|r| {
                        lower_table_row(
                            r,
                            builder,
                            policy,
                            diagnostics,
                            false,
                            &t.align,
                            icon_mode,
                            figure_enabled,
                            wiki_link_template,
                        )
                    })
                    .collect();
                if !body_rows.is_empty() {
                    let tbody_span = body_rows
                        .first()
                        .map_or(Span::empty(), crate::ast::hast::nodes::HNode::span);
                    let tbody = builder.elem(tbody_span, "tbody", body_rows);
                    table_children.push(tbody);
                }
            }

            Some(builder.elem(t.span, "table", table_children))
        }

        MdNode::TableRow(_) => None,
        MdNode::TableCell(_) => None,

        MdNode::Delete(d) => {
            let children = lower_children(
                &d.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(d.span, "del", children))
        }

        MdNode::FootnoteDefinition(fd) => {
            let children = lower_children(
                &fd.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
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

        MdNode::Yaml(_) | MdNode::Toml(_) | MdNode::Json(_) => None,

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

        MdNode::Math(m) => {
            let mut attrs = SmallMap::new();
            attrs.insert("class".to_string(), "math math-display".to_string());
            let code_text = builder.text(m.span, &m.value);
            let code = builder.elem(m.span, "code", vec![code_text]);
            Some(builder.element(m.span, "pre", attrs, vec![code], false))
        }

        MdNode::InlineMath(m) => {
            let mut attrs = SmallMap::new();
            attrs.insert("class".to_string(), "math math-inline".to_string());
            let text = builder.text(m.span, &m.value);
            Some(builder.element(m.span, "code", attrs, vec![text], false))
        }

        MdNode::ContainerDirective(d) => {
            let mut attrs = SmallMap::new();
            attrs.insert(
                "class".to_string(),
                format!("directive directive-{}", d.name),
            );
            attrs.insert("data-directive".to_string(), d.name.clone());
            for (k, v) in &d.attributes {
                if k != "title" {
                    attrs.insert(format!("data-{k}"), v.clone());
                }
            }
            let mut children = Vec::new();

            if let Some(title) = d
                .attributes
                .iter()
                .find_map(|(k, v)| if k == "title" { Some(v.as_str()) } else { None })
            {
                let mut title_attrs = SmallMap::new();
                title_attrs.insert("class".to_string(), "directive-title".to_string());
                let title_text = builder.text(d.span, title);
                children.push(builder.element(d.span, "p", title_attrs, vec![title_text], false));
            }

            children.extend(lower_children(
                &d.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            ));
            Some(builder.element(d.span, "div", attrs, children, false))
        }

        MdNode::LeafDirective(d) => {
            let mut attrs = SmallMap::new();
            attrs.insert(
                "class".to_string(),
                format!("directive directive-{}", d.name),
            );
            attrs.insert("data-directive".to_string(), d.name.clone());
            for (k, v) in &d.attributes {
                attrs.insert(format!("data-{k}"), v.clone());
            }
            let text = builder.text(d.span, &d.value);
            Some(builder.element(d.span, "div", attrs, vec![text], false))
        }

        MdNode::TextDirective(d) => {
            let mut attrs = SmallMap::new();
            attrs.insert(
                "class".to_string(),
                format!("directive directive-{}", d.name),
            );
            attrs.insert("data-directive".to_string(), d.name.clone());
            for (k, v) in &d.attributes {
                attrs.insert(format!("data-{k}"), v.clone());
            }
            let text = builder.text(d.span, &d.value);
            Some(builder.element(d.span, "span", attrs, vec![text], false))
        }

        MdNode::WikiLink(w) => {
            let slug = w.target.to_lowercase().replace(' ', "-");
            let mut attrs = SmallMap::new();
            attrs.insert(
                "href".to_string(),
                #[allow(clippy::literal_string_with_formatting_args)]
                wiki_link_template.replace("{slug}", &slug),
            );
            attrs.insert("class".to_string(), "wiki-link".to_string());
            let children = lower_children(
                &w.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.element(w.span, "a", attrs, children, false))
        }

        MdNode::DefinitionList(dl) => {
            let children = lower_children(
                &dl.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(dl.span, "dl", children))
        }

        MdNode::DefinitionTerm(dt) => {
            let children = lower_children(
                &dt.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(dt.span, "dt", children))
        }

        MdNode::DefinitionDescription(dd) => {
            let children = lower_children(
                &dd.children,
                builder,
                policy,
                diagnostics,
                icon_mode,
                figure_enabled,
                wiki_link_template,
            );
            Some(builder.elem(dd.span, "dd", children))
        }

        MdNode::RubyAnnotation(r) => {
            let base = builder.text(r.span, &r.base);
            let rp_open_text = builder.text(r.span, "(");
            let rp_open = builder.elem(r.span, "rp", vec![rp_open_text]);
            let rt_text = builder.text(r.span, &r.annotation);
            let rt = builder.elem(r.span, "rt", vec![rt_text]);
            let rp_close_text = builder.text(r.span, ")");
            let rp_close = builder.elem(r.span, "rp", vec![rp_close_text]);
            Some(builder.elem(r.span, "ruby", vec![base, rp_open, rt, rp_close]))
        }

        MdNode::Abbr(a) => {
            let mut attrs = SmallMap::new();
            attrs.insert("title".to_string(), a.definition.clone());
            let text = builder.text(a.span, &a.term);
            Some(builder.element(a.span, "abbr", attrs, vec![text], false))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_table_row(
    node: &MdNode,
    builder: &mut HBuilder,
    policy: RawHtmlPolicy,
    diagnostics: &mut DiagnosticSink,
    is_header: bool,
    align: &[AlignKind],
    icon_mode: &GithubAlertIconMode,
    figure_enabled: bool,
    wiki_link_template: &str,
) -> Option<HNode> {
    if let MdNode::TableRow(row) = node {
        let cell_tag = if is_header { "th" } else { "td" };
        let cells: Vec<HNode> = row
            .children
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| {
                if let MdNode::TableCell(tc) = cell {
                    let children = lower_children(
                        &tc.children,
                        builder,
                        policy,
                        diagnostics,
                        icon_mode,
                        figure_enabled,
                        wiki_link_template,
                    );
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

fn lower_alert(
    span: Span,
    alert_kind: &str,
    content_children: Vec<HNode>,
    builder: &mut HBuilder,
    icon_mode: &GithubAlertIconMode,
) -> Option<HNode> {
    let title = match alert_kind {
        "note" => "Note",
        "tip" => "Tip",
        "important" => "Important",
        "warning" => "Warning",
        "caution" => "Caution",
        other => other,
    };

    let mut outer_attrs = SmallMap::new();
    outer_attrs.insert("class".to_string(), format!("alert alert-{alert_kind}"));
    outer_attrs.insert("role".to_string(), "alert".to_string());

    let mut title_children: Vec<HNode> = Vec::new();

    match icon_mode {
        GithubAlertIconMode::Octicon => {
            if let Some(svg_html) = resolve_icon_svg(alert_kind, icon_mode) {
                build_icon_element(span, &svg_html, builder, &mut title_children);
            }
        }
        GithubAlertIconMode::Custom(map) => {
            if let Some(svg_html) = resolve_icon_svg(alert_kind, icon_mode) {
                if map.get(alert_kind).and_then(|d| d.svg.as_ref()).is_some() {
                    title_children.push(builder.raw(span, &svg_html));
                } else {
                    build_icon_element(span, &svg_html, builder, &mut title_children);
                }
            }
        }
        GithubAlertIconMode::None => {}
    }

    title_children.push(builder.text(span, title));

    let mut title_attrs = SmallMap::new();
    title_attrs.insert("class".to_string(), "alert-title".to_string());
    let title_elem = builder.element(span, "p", title_attrs, title_children, false);

    let mut all_children = vec![title_elem];
    all_children.extend(content_children);

    Some(builder.element(span, "div", outer_attrs, all_children, false))
}

fn build_icon_element(span: Span, svg_html: &str, builder: &mut HBuilder, target: &mut Vec<HNode>) {
    let path_d = svg_html.find("d=\"").and_then(|start| {
        let rest = &svg_html[start + 3..];
        rest.find('"').map(|end| &rest[..end])
    });

    let mut svg_attrs = SmallMap::new();
    svg_attrs.insert("class".to_string(), "alert-icon".to_string());
    svg_attrs.insert("viewBox".to_string(), "0 0 16 16".to_string());
    svg_attrs.insert("width".to_string(), "16".to_string());
    svg_attrs.insert("height".to_string(), "16".to_string());
    svg_attrs.insert("aria-hidden".to_string(), "true".to_string());

    let mut children = Vec::new();
    if let Some(d) = path_d {
        let mut path_attrs = SmallMap::new();
        path_attrs.insert("d".to_string(), d.to_string());
        children.push(builder.element(span, "path", path_attrs, vec![], true));
    }

    target.push(builder.element(span, "svg", svg_attrs, children, false));
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
            Some(builder.text(span, value))
        }
        RawHtmlPolicy::AllowDangerous => Some(builder.raw(span, value)),
        RawHtmlPolicy::ParseAndSanitize => Some(builder.raw(span, value)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;

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

    fn root_children(root: &HNode) -> &[HNode] {
        root.children().expect("expected Root with children")
    }

    fn expect_element(node: &HNode) -> &HElement {
        if let HNode::Element(e) = node {
            e
        } else {
            panic!("expected HNode::Element, got {node:?}");
        }
    }

    fn expect_text(node: &HNode) -> &HText {
        if let HNode::Text(t) = node {
            t
        } else {
            panic!("expected HNode::Text, got {node:?}");
        }
    }

    fn expect_raw(node: &HNode) -> &HRaw {
        if let HNode::Raw(r) = node {
            r
        } else {
            panic!("expected HNode::Raw, got {node:?}");
        }
    }

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
            extra_attrs: SmallMap::new(),
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
                extra_attrs: SmallMap::new(),
            });
            let doc = make_doc(&mut id_gen, vec![heading]);
            let root = lower_doc(&doc);

            let el = expect_element(&root_children(&root)[0]);
            assert_eq!(el.tag, format!("h{depth}"));
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
            extra_attrs: SmallMap::new(),
        });
        let doc = make_doc(&mut id_gen, vec![heading]);
        let root = lower_doc(&doc);

        let el = expect_element(&root_children(&root)[0]);
        assert_eq!(el.tag, "h2");
        assert_eq!(el.attributes.get("id"), Some(&"hello-world".to_string()));
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
            code_el.attributes.get("class"),
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
            alert_type: None,
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
        assert_eq!(ol.attributes.get("start"), Some(&"5".to_string()));
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
        assert!(ol.attributes.get("start").is_none());
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
        let checkbox = expect_element(&li_el.children[0]);
        assert_eq!(checkbox.tag, "input");
        assert_eq!(
            checkbox.attributes.get("type"),
            Some(&"checkbox".to_string())
        );
        assert!(checkbox.attributes.contains_key("checked"));
        assert!(checkbox.attributes.contains_key("disabled"));
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
        assert!(!checkbox.attributes.contains_key("checked"));
        assert!(checkbox.attributes.contains_key("disabled"));
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
            a.attributes.get("href"),
            Some(&"https://example.com".to_string())
        );
        assert!(a.attributes.get("title").is_none());
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
        assert_eq!(a.attributes.get("title"), Some(&"Example".to_string()));
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
        assert_eq!(img_el.attributes.get("src"), Some(&"photo.jpg".to_string()));
        assert_eq!(img_el.attributes.get("alt"), Some(&"A photo".to_string()));
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
            img_el.attributes.get("title"),
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
        assert_eq!(table_el.children.len(), 2);

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
        assert_eq!(th0.attributes.get("align"), Some(&"left".to_string()));

        let th1 = expect_element(&tr.children[1]);
        assert_eq!(th1.attributes.get("align"), Some(&"center".to_string()));

        let th2 = expect_element(&tr.children[2]);
        assert_eq!(th2.attributes.get("align"), Some(&"right".to_string()));
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
            sup.attributes.get("class"),
            Some(&"footnote-ref".to_string())
        );
        let link = expect_element(&sup.children[0]);
        assert_eq!(link.tag, "a");
        assert_eq!(link.attributes.get("href"), Some(&"#fn-1".to_string()));
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
        assert_eq!(li.attributes.get("id"), Some(&"fn-1".to_string()));
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

        let children = root_children(&root);
        assert_eq!(children.len(), 1);
        let t = expect_text(&children[0]);
        assert_eq!(t.value, "<div>hello</div>");
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
            name: None,
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
            extra_attrs: SmallMap::new(),
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
