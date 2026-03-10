use crate::api::options::GithubAlertIconMode;
use crate::ast::mdast::nodes::{Document, MdNode, MdxJsxElement};
use crate::transform::passes::github_alert::{resolve_icon_import, resolve_icon_svg};
use crate::transform::passes::line_number::make_lines_self_contained;

pub type HighlightFn = dyn Fn(&str, &str) -> Option<String>;

#[must_use]
pub fn print_mdx_js(doc: &Document, highlight_code: Option<&HighlightFn>) -> MdxJsOutput {
    print_mdx_js_with_icons(doc, highlight_code, &GithubAlertIconMode::default())
}

#[must_use]
pub fn print_mdx_js_with_icons(
    doc: &Document,
    highlight_code: Option<&HighlightFn>,
    icon_mode: &GithubAlertIconMode,
) -> MdxJsOutput {
    print_mdx_js_with_options(doc, highlight_code, icon_mode, false)
}

#[must_use]
pub fn print_mdx_js_with_options(
    doc: &Document,
    highlight_code: Option<&HighlightFn>,
    icon_mode: &GithubAlertIconMode,
    line_numbers: bool,
) -> MdxJsOutput {
    print_mdx_js_full(doc, highlight_code, icon_mode, line_numbers, "/wiki/{slug}")
}

#[must_use]
pub fn print_mdx_js_full(
    doc: &Document,
    highlight_code: Option<&HighlightFn>,
    icon_mode: &GithubAlertIconMode,
    line_numbers: bool,
    wiki_link_template: &str,
) -> MdxJsOutput {
    let mut printer =
        MdxJsPrinter::new(highlight_code, icon_mode, line_numbers, wiki_link_template);
    printer.print_document(doc);
    MdxJsOutput {
        code: printer.output,
        source_mappings: printer.mappings,
    }
}

pub struct MdxJsOutput {
    pub code: String,
    pub source_mappings: Vec<SourceMapping>,
}

pub struct SourceMapping {
    pub generated_line: u32,
    pub generated_column: u32,
    pub original_offset: u32,
}

struct MdxJsPrinter<'a> {
    output: String,
    mappings: Vec<SourceMapping>,
    line: u32,
    column: u32,
    highlight_code: Option<&'a HighlightFn>,
    icon_mode: &'a GithubAlertIconMode,
    line_numbers: bool,
    wiki_link_template: &'a str,
    icon_imports: Vec<(String, String, String)>,
}

impl<'a> MdxJsPrinter<'a> {
    fn new(
        highlight_code: Option<&'a HighlightFn>,
        icon_mode: &'a GithubAlertIconMode,
        line_numbers: bool,
        wiki_link_template: &'a str,
    ) -> Self {
        Self {
            output: String::new(),
            mappings: Vec::new(),
            line: 1,
            column: 0,
            highlight_code,
            icon_mode,
            line_numbers,
            wiki_link_template,
            icon_imports: Vec::new(),
        }
    }

    fn emit(&mut self, s: &str) {
        for ch in s.chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        self.output.push_str(s);
    }

    fn emit_with_mapping(&mut self, s: &str, original_offset: u32) {
        self.mappings.push(SourceMapping {
            generated_line: self.line,
            generated_column: self.column,
            original_offset,
        });
        self.emit(s);
    }

    fn inject_jsx_key(&mut self, start_len: usize, key: usize) {
        let emitted = &self.output[start_len..];
        let trimmed = emitted.trim_start();
        if trimmed.starts_with("_jsx")
            && let Some(rel_pos) = emitted.rfind(')')
        {
            let abs_pos = start_len + rel_pos;
            let key_str = format!(", \"{key}\"");
            self.column += key_str.len() as u32;
            self.output.insert_str(abs_pos, &key_str);
        }
    }

    fn emit_keyed_children(&mut self, children: &[MdNode], indent: usize) {
        for (i, child) in children.iter().enumerate() {
            let start = self.output.len();
            self.print_mdx_node(child, indent);
            self.inject_jsx_key(start, i);
            if i < children.len() - 1 {
                self.emit(",\n");
            }
        }
    }

    fn print_document(&mut self, doc: &Document) {
        let mut esm_nodes: Vec<&MdNode> = Vec::new();
        let mut content_nodes: Vec<&MdNode> = Vec::new();

        for child in &doc.children {
            match child {
                MdNode::MdxjsEsm(_) => esm_nodes.push(child),
                _ => content_nodes.push(child),
            }
        }

        self.collect_alert_icon_imports(&content_nodes);

        for esm in &esm_nodes {
            if let MdNode::MdxjsEsm(e) = esm {
                self.emit_with_mapping(&e.value, e.span.start);
                self.emit("\n");
            }
        }

        self.emit_icon_imports();

        self.emit("\nfunction MDXContent({ components: _components = {}, ...props }) {\n");
        self.emit("  const _c = (t) => _components[t] || t;\n");
        self.emit("  return ");

        if content_nodes.is_empty() {
            self.emit("null;\n");
        } else {
            self.emit("_jsxs(_Fragment, {\n");
            self.emit("    children: [\n");

            for (i, node) in content_nodes.iter().enumerate() {
                let start = self.output.len();
                self.print_mdx_node(node, 6);
                self.inject_jsx_key(start, i);
                if i < content_nodes.len() - 1 {
                    self.emit(",\n");
                }
            }

            self.emit("\n    ]\n");
            self.emit("  });\n");
        }

        self.emit("}\n\n");
        self.emit("export default MDXContent;\n");
    }

    fn print_mdx_node(&mut self, node: &MdNode, indent: usize) {
        let pad = " ".repeat(indent);
        match node {
            MdNode::Heading(h) => {
                let id_prop = h
                    .slug
                    .as_ref()
                    .map(|s| format!(", id: \"{}\"", escape_js_string(s)))
                    .unwrap_or_default();
                self.emit(&format!("{pad}_jsx(_c(\"h{}\"), {{ children: ", h.depth));
                self.print_inline_children(&h.children);
                self.emit(&format!("{id_prop} }})"));
            }
            MdNode::Paragraph(p) => {
                self.emit(&format!("{pad}_jsx(_c(\"p\"), {{ children: "));
                self.print_inline_children(&p.children);
                self.emit(" })");
            }
            MdNode::Code(c) => {
                self.emit_code_block(c, &pad);
            }
            MdNode::MdxJsxFlowElement(el) | MdNode::MdxJsxTextElement(el) => {
                self.print_jsx_element(el, &pad, indent);
            }
            MdNode::MdxFlowExpression(expr) | MdNode::MdxTextExpression(expr) => {
                self.emit_with_mapping(&format!("{pad}{}", expr.value), expr.span.start);
            }
            MdNode::Blockquote(bq) => {
                if let Some(ref alert_kind) = bq.alert_type {
                    self.emit_alert_jsx(alert_kind, &bq.children, &pad, indent);
                } else {
                    self.emit(&format!("{pad}_jsxs(_c(\"blockquote\"), {{ children: [\n"));
                    self.emit_keyed_children(&bq.children, indent + 2);
                    self.emit(&format!("\n{pad}] }})"));
                }
            }
            MdNode::List(l) => {
                let tag = if l.ordered { "ol" } else { "ul" };
                self.emit(&format!("{pad}_jsxs(_c(\"{tag}\"), {{ children: [\n"));
                self.emit_keyed_children(&l.children, indent + 2);
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::ListItem(li) => {
                self.emit(&format!("{pad}_jsxs(_c(\"li\"), {{ children: [\n"));
                self.emit_keyed_children(&li.children, indent + 2);
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::Emphasis(e) => {
                self.emit(&format!("{pad}_jsx(_c(\"em\"), {{ children: "));
                self.print_inline_children(&e.children);
                self.emit(" })");
            }
            MdNode::Strong(s) => {
                self.emit(&format!("{pad}_jsx(_c(\"strong\"), {{ children: "));
                self.print_inline_children(&s.children);
                self.emit(" })");
            }
            MdNode::Link(l) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"a\"), {{ href: \"{}\", children: ",
                    escape_js_string(&l.url)
                ));
                self.print_inline_children(&l.children);
                self.emit(" })");
            }
            MdNode::Image(img) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"img\"), {{ src: \"{}\", alt: \"{}\" }})",
                    escape_js_string(&img.url),
                    escape_js_string(&img.alt),
                ));
            }
            MdNode::InlineCode(c) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"code\"), {{ children: \"{}\" }})",
                    escape_js_string(&c.value),
                ));
            }
            MdNode::Text(t) => {
                self.emit(&format!("{pad}\"{}\"", escape_js_string(&t.value)));
            }
            MdNode::ThematicBreak(_) => {
                self.emit(&format!("{pad}_jsx(_c(\"hr\"), {{}})"));
            }
            MdNode::Break(_) => {
                self.emit(&format!("{pad}_jsx(_c(\"br\"), {{}})"));
            }
            MdNode::Html(h) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"div\"), {{ rawHtml: \"{}\" }})",
                    escape_js_string(&h.value),
                ));
            }
            MdNode::Table(t) => {
                self.emit(&format!("{pad}_jsxs(_c(\"table\"), {{ children: [\n"));
                if let Some(head_row) = t.children.first() {
                    self.emit(&format!("{pad}  _jsx(_c(\"thead\"), {{ children: \n"));
                    self.print_table_row(head_row, indent + 4, true);
                    self.emit(&format!("\n{pad}  }}, \"0\")"));
                    if t.children.len() > 1 {
                        self.emit(",\n");
                        self.emit(&format!("{pad}  _jsxs(_c(\"tbody\"), {{ children: [\n"));
                        for (i, child) in t.children[1..].iter().enumerate() {
                            let start = self.output.len();
                            self.print_table_row(child, indent + 4, false);
                            self.inject_jsx_key(start, i);
                            if i < t.children.len() - 2 {
                                self.emit(",\n");
                            }
                        }
                        self.emit(&format!("\n{pad}  ] }}, \"1\")"));
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::TableRow(_) => {
                self.print_table_row(node, indent, false);
            }
            MdNode::TableCell(tc) => {
                self.emit(&format!("{pad}_jsx(_c(\"td\"), {{ children: "));
                self.print_inline_children(&tc.children);
                self.emit(" })");
            }
            MdNode::Delete(d) => {
                self.emit(&format!("{pad}_jsx(_c(\"del\"), {{ children: "));
                self.print_inline_children(&d.children);
                self.emit(" })");
            }
            MdNode::Math(m) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"pre\"), {{ className: \"math math-display\", children: _jsx(_c(\"code\"), {{ children: \"{}\" }}) }})",
                    escape_js_string(&m.value),
                ));
            }
            MdNode::InlineMath(m) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"code\"), {{ className: \"math math-inline\", children: \"{}\" }})",
                    escape_js_string(&m.value),
                ));
            }
            MdNode::ContainerDirective(d) => {
                self.emit(&format!(
                    "{pad}_jsxs(_c(\"div\"), {{ className: \"directive directive-{}\", \"data-directive\": \"{}\", children: [\n",
                    escape_js_string(&d.name), escape_js_string(&d.name),
                ));
                self.emit_keyed_children(&d.children, indent + 2);
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::WikiLink(w) => {
                let slug = w.target.to_lowercase().replace(' ', "-");
                #[allow(clippy::literal_string_with_formatting_args)]
                let href = self.wiki_link_template.replace("{slug}", &slug);
                self.emit(&format!(
                    "{pad}_jsx(_c(\"a\"), {{ href: \"{}\", className: \"wiki-link\", children: ",
                    escape_js_string(&href),
                ));
                self.print_inline_children(&w.children);
                self.emit(" })");
            }
            MdNode::DefinitionList(dl) => {
                self.emit(&format!("{pad}_jsxs(_c(\"dl\"), {{ children: [\n"));
                self.emit_keyed_children(&dl.children, indent + 2);
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::DefinitionTerm(dt) => {
                self.emit(&format!("{pad}_jsx(_c(\"dt\"), {{ children: "));
                self.print_inline_children(&dt.children);
                self.emit(" })");
            }
            MdNode::DefinitionDescription(dd) => {
                self.emit(&format!("{pad}_jsx(_c(\"dd\"), {{ children: "));
                self.print_inline_children(&dd.children);
                self.emit(" })");
            }
            MdNode::RubyAnnotation(r) => {
                self.emit(&format!(
                    "{pad}_jsxs(_c(\"ruby\"), {{ children: [\"{}\", _jsx(_c(\"rp\"), {{ children: \"(\" }}), _jsx(_c(\"rt\"), {{ children: \"{}\" }}), _jsx(_c(\"rp\"), {{ children: \")\" }})] }})",
                    escape_js_string(&r.base),
                    escape_js_string(&r.annotation),
                ));
            }
            MdNode::Abbr(a) => {
                self.emit(&format!(
                    "{pad}_jsx(_c(\"abbr\"), {{ title: \"{}\", children: \"{}\" }})",
                    escape_js_string(&a.definition),
                    escape_js_string(&a.term),
                ));
            }
            MdNode::Document(_)
            | MdNode::Definition(_)
            | MdNode::FootnoteDefinition(_)
            | MdNode::FootnoteReference(_)
            | MdNode::Yaml(_)
            | MdNode::Toml(_)
            | MdNode::Json(_)
            | MdNode::MdxjsEsm(_)
            | MdNode::LeafDirective(_)
            | MdNode::TextDirective(_) => {}
        }
    }

    fn print_jsx_element(&mut self, el: &MdxJsxElement, pad: &str, indent: usize) {
        let name = el.name.as_deref().unwrap_or("_Fragment");
        let tag_ref = format_element_ref(name);

        let mut props = String::new();
        for attr in &el.attributes {
            if let Some(ref val) = attr.value {
                if val.starts_with('{') && val.ends_with('}') {
                    let inner = &val[1..val.len() - 1];
                    props.push_str(&format!(", {}: {inner}", attr.name));
                } else {
                    props.push_str(&format!(", {}: \"{}\"", attr.name, escape_js_string(val)));
                }
            } else {
                props.push_str(&format!(", {}: true", attr.name));
            }
        }

        if el.children.is_empty() {
            let clean_props = props.strip_prefix(", ").unwrap_or(&props);
            self.emit(&format!("{pad}_jsx({tag_ref}, {{ {clean_props} }})"));
        } else {
            self.emit(&format!("{pad}_jsxs({tag_ref}, {{ children: [\n"));
            self.emit_keyed_children(&el.children, indent + 2);
            self.emit(&format!("\n{pad}]{props}}})"));
        }
    }

    fn print_table_row(&mut self, node: &MdNode, indent: usize, is_header: bool) {
        let pad = " ".repeat(indent * 2);
        if let MdNode::TableRow(tr) = node {
            self.emit(&format!("{pad}_jsxs(_c(\"tr\"), {{ children: [\n"));
            let tag = if is_header { "th" } else { "td" };
            for (i, child) in tr.children.iter().enumerate() {
                let start = self.output.len();
                if let MdNode::TableCell(tc) = child {
                    let cpad = " ".repeat((indent + 1) * 2);
                    self.emit(&format!("{cpad}_jsx(_c(\"{tag}\"), {{ children: "));
                    self.print_inline_children(&tc.children);
                    self.emit(" })");
                } else {
                    self.print_mdx_node(child, indent + 1);
                }
                self.inject_jsx_key(start, i);
                if i < tr.children.len() - 1 {
                    self.emit(",\n");
                }
            }
            self.emit(&format!("\n{pad}] }})"));
        }
    }

    fn print_inline_children(&mut self, children: &[MdNode]) {
        if children.len() == 1
            && let MdNode::Text(t) = &children[0]
        {
            self.emit(&format!("\"{}\"", escape_js_string(&t.value)));
            return;
        }
        self.emit("[");
        for (i, child) in children.iter().enumerate() {
            let start = self.output.len();
            self.print_mdx_node(child, 0);
            self.inject_jsx_key(start, i);
            if i < children.len() - 1 {
                self.emit(", ");
            }
        }
        self.emit("]");
    }

    fn collect_alert_icon_imports(&mut self, nodes: &[&MdNode]) {
        for node in nodes {
            self.scan_node_for_alerts(node);
        }
    }

    fn scan_node_for_alerts(&mut self, node: &MdNode) {
        if let MdNode::Blockquote(bq) = node
            && let Some(ref alert_kind) = bq.alert_type
            && let Some(def) = resolve_icon_import(alert_kind, self.icon_mode)
        {
            let alias = format!("_AlertIcon_{alert_kind}");
            let import_name = def.import_name.unwrap_or_default();
            let source = def.import_source.unwrap_or_default();
            if !self.icon_imports.iter().any(|(a, _, _)| a == &alias) {
                self.icon_imports.push((alias, import_name, source));
            }
        }
        if let Some(children) = node.children() {
            for child in children {
                self.scan_node_for_alerts(child);
            }
        }
    }

    fn emit_icon_imports(&mut self) {
        let imports = self.icon_imports.clone();
        let mut by_source: std::collections::BTreeMap<String, Vec<(String, String)>> =
            std::collections::BTreeMap::new();
        for (alias, name, source) in imports {
            by_source.entry(source).or_default().push((name, alias));
        }
        for (source, items) in &by_source {
            let specifiers: Vec<String> = items
                .iter()
                .map(|(name, alias)| {
                    if name == alias {
                        name.clone()
                    } else {
                        format!("{name} as {alias}")
                    }
                })
                .collect();
            self.emit(&format!(
                "import {{ {} }} from '{}';\n",
                specifiers.join(", "),
                source,
            ));
        }
    }

    fn emit_alert_jsx(&mut self, alert_kind: &str, children: &[MdNode], pad: &str, indent: usize) {
        let title = match alert_kind {
            "note" => "Note",
            "tip" => "Tip",
            "important" => "Important",
            "warning" => "Warning",
            "caution" => "Caution",
            other => other,
        };

        self.emit(&format!(
            "{pad}_jsxs(_c(\"div\"), {{ className: \"alert alert-{alert_kind}\", role: \"alert\", children: [\n"
        ));

        let inner_pad = " ".repeat(indent + 2);

        self.emit(&format!(
            "{inner_pad}_jsxs(_c(\"p\"), {{ className: \"alert-title\", children: ["
        ));

        let has_npm_import = resolve_icon_import(alert_kind, self.icon_mode).is_some();
        if has_npm_import {
            let alias = format!("_AlertIcon_{alert_kind}");
            self.emit(&format!("_jsx({alias}, {{}}), "));
        } else if let Some(svg_html) = resolve_icon_svg(alert_kind, self.icon_mode) {
            self.emit(&format!(
                "_jsx(\"span\", {{ className: \"alert-icon\", dangerouslySetInnerHTML: {{ __html: \"{}\" }} }}), ",
                escape_js_string(&svg_html),
            ));
        }

        self.emit(&format!("\"{title}\"] }}, \"0\"),\n"));

        self.emit_keyed_children(children, indent + 2);

        self.emit(&format!("\n{pad}] }})"));
    }

    fn emit_code_block(&mut self, c: &crate::ast::mdast::nodes::Code, pad: &str) {
        let highlighted = c
            .lang
            .as_ref()
            .and_then(|lang| self.highlight_code.and_then(|f| f(&c.value, lang)));

        if let Some(html) = highlighted {
            let lang = c.lang.as_deref().unwrap_or("");
            let code_html = if self.line_numbers {
                wrap_highlighted_with_line_numbers(&html)
            } else {
                html
            };
            let inner_html = format!(
                "<code class=\"language-{} highlighted\">{}</code>",
                escape_html_attr(lang),
                code_html
            );
            self.emit(&format!(
                concat!(
                    "{pad}_jsx(_c(\"pre\"), {{ __rawCode: \"{raw}\",",
                    " dangerouslySetInnerHTML: {{ __html: \"{html}\" }} }})",
                ),
                pad = pad,
                raw = escape_js_string(&c.value),
                html = escape_js_string(&inner_html),
            ));
        } else if self.line_numbers {
            let lang = c.lang.as_deref().unwrap_or("");
            let lang_class = if !lang.is_empty() {
                format!("language-{} ", escape_html_attr(lang))
            } else {
                String::new()
            };
            let code_html = wrap_plain_text_with_line_numbers(&c.value);
            let inner_html = format!("<code class=\"{lang_class}\">{code_html}</code>");
            self.emit(&format!(
                concat!(
                    "{pad}_jsx(_c(\"pre\"), {{ __rawCode: \"{raw}\",",
                    " dangerouslySetInnerHTML: {{ __html: \"{html}\" }} }})",
                ),
                pad = pad,
                raw = escape_js_string(&c.value),
                html = escape_js_string(&inner_html),
            ));
        } else {
            let lang_prop = c
                .lang
                .as_ref()
                .map(|l| format!(", className: \"language-{l}\""))
                .unwrap_or_default();
            self.emit(&format!(
                "{pad}_jsx(_c(\"pre\"), {{ children: _jsx(_c(\"code\"), {{ children: \"{}\"{lang_prop} }}) }})",
                escape_js_string(&c.value),
            ));
        }
    }
}

fn format_element_ref(name: &str) -> String {
    if name.starts_with(|c: char| c.is_ascii_lowercase()) {
        format!("_c(\"{name}\")")
    } else {
        name.to_string()
    }
}

fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn escape_html_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_content(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn wrap_highlighted_with_line_numbers(html: &str) -> String {
    let lines = make_lines_self_contained(html);
    let mut result = String::new();
    for (i, line_html) in lines.iter().enumerate() {
        let line_num = i + 1;
        result.push_str(&format!(
            "<span class=\"line\" data-line=\"{line_num}\">{line_html}</span>"
        ));
        if i < lines.len() - 1 {
            result.push('\n');
        }
    }
    result
}

fn wrap_plain_text_with_line_numbers(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 && line.is_empty() {
            break;
        }
        let line_num = i + 1;
        result.push_str(&format!(
            "<span class=\"line\" data-line=\"{line_num}\">{}</span>",
            escape_html_content(line)
        ));
        if i < lines.len() - 1 {
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::options::{FrontmatterOptions, GfmOptions};

    fn parse_mdx(input: &str) -> crate::parse::mdx::MdxParseResult {
        crate::parse::mdx::parse_mdx(input, &GfmOptions::default(), &FrontmatterOptions::all())
    }

    #[test]
    fn mdx_js_emit_heading() {
        let r = parse_mdx("# Hello\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("MDXContent"));
        assert!(output.code.contains("h1"));
        assert!(output.code.contains("Hello"));
    }

    #[test]
    fn mdx_js_emit_paragraph() {
        let r = parse_mdx("Hello world\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("_jsx(_c(\"p\")"));
        assert!(output.code.contains("Hello world"));
    }

    #[test]
    fn mdx_js_emit_with_import() {
        let r = parse_mdx("import { X } from 'x'\n\n# Hello\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("import { X } from 'x'"));
        assert!(output.code.contains("MDXContent"));
    }

    #[test]
    fn mdx_js_emit_with_export() {
        let r = parse_mdx("export const meta = {}\n\n# Title\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("export const meta = {}"));
        assert!(output.code.contains("export default MDXContent"));
    }

    #[test]
    fn mdx_js_emit_component() {
        let r = parse_mdx("<Button />\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("Button"));
        assert!(output.code.contains("_jsx("));
    }

    #[test]
    fn mdx_js_emit_component_with_props() {
        let r = parse_mdx("<Button variant=\"primary\" />\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("Button"));
        assert!(output.code.contains("variant"));
        assert!(output.code.contains("primary"));
    }

    #[test]
    fn mdx_js_emit_expression() {
        let r = parse_mdx("{someValue}\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("someValue"));
    }

    #[test]
    fn mdx_js_emit_code_block() {
        let r = parse_mdx("```rust\nfn main() {}\n```\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("pre"));
        assert!(output.code.contains("code"));
        assert!(output.code.contains("language-rust"));
    }

    #[test]
    fn mdx_js_emit_list() {
        let r = parse_mdx("- one\n- two\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\"ul\""));
        assert!(output.code.contains("\"li\""));
    }

    #[test]
    fn mdx_js_emit_link() {
        let r = parse_mdx("[click](http://example.com)\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\"a\""));
        assert!(output.code.contains("http://example.com"));
    }

    #[test]
    fn mdx_js_emit_image() {
        let r = parse_mdx("![alt text](img.png)\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\"img\""));
        assert!(output.code.contains("img.png"));
        assert!(output.code.contains("alt text"));
    }

    #[test]
    fn mdx_js_emit_blockquote() {
        let r = parse_mdx("> quote\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\"blockquote\""));
    }

    #[test]
    fn mdx_js_emit_empty() {
        let r = parse_mdx("");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("MDXContent"));
        assert!(output.code.contains("null"));
    }

    #[test]
    fn mdx_js_emit_default_export() {
        let r = parse_mdx("# Hello\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("export default MDXContent"));
    }

    #[test]
    fn mdx_js_emit_has_source_mappings() {
        let r = parse_mdx("import X from 'x'\n\n# Hello\n");
        let output = print_mdx_js(&r.document, None);
        assert!(!output.source_mappings.is_empty());
    }

    #[test]
    fn mdx_js_emit_boolean_prop() {
        let r = parse_mdx("<Input disabled />\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("disabled: true"));
    }

    #[test]
    fn mdx_js_emit_emphasis_and_strong() {
        let r = parse_mdx("Hello *world* and **bold**\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\"em\""));
        assert!(output.code.contains("\"strong\""));
    }

    #[test]
    fn mdx_js_escape_special_chars() {
        let r = parse_mdx("Hello \"world\"\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("\\\"world\\\""));
    }

    #[test]
    fn mdx_js_emit_thematic_break() {
        let r = parse_mdx("---\n\ntext\n");
        let output = print_mdx_js(&r.document, None);
        assert!(output.code.contains("MDXContent"));
    }

    #[test]
    fn mdx_js_emit_components_prop_support() {
        let r = parse_mdx("# Hello\n");
        let output = print_mdx_js(&r.document, None);
        assert!(
            output.code.contains("{ components: _components = {}"),
            "should destructure components prop"
        );
        assert!(
            output
                .code
                .contains("const _c = (t) => _components[t] || t;"),
            "should define _c helper"
        );
        assert!(
            output.code.contains("_jsx(_c(\"h1\")"),
            "should wrap lowercase tags with _c"
        );
    }

    #[test]
    fn mdx_js_emit_component_not_wrapped() {
        let r = parse_mdx("<Alert>warn</Alert>\n\ntext\n");
        let output = print_mdx_js(&r.document, None);
        assert!(
            output.code.contains("_jsxs(Alert,"),
            "capitalized components should NOT be wrapped with _c"
        );
        assert!(
            output.code.contains("_jsx(_c(\"p\")"),
            "lowercase tags should be wrapped with _c"
        );
    }

    #[test]
    fn mdx_js_emit_code_block_with_highlight() {
        let highlight = |code: &str, _lang: &str| -> Option<String> {
            Some(format!(
                "<span class=\"hl\">{}</span>",
                code.replace('<', "&lt;")
            ))
        };
        let r = parse_mdx("```js\nconst x = 1;\n```\n");
        let output = print_mdx_js(&r.document, Some(&highlight));
        assert!(
            output.code.contains("__rawCode:"),
            "should include __rawCode prop"
        );
        assert!(
            output.code.contains("dangerouslySetInnerHTML"),
            "should include dangerouslySetInnerHTML"
        );
        assert!(
            output.code.contains("const x = 1;"),
            "should include raw code"
        );
        assert!(
            output.code.contains("highlighted"),
            "should include highlighted class"
        );
    }

    #[test]
    fn mdx_js_emit_code_block_without_highlight() {
        let r = parse_mdx("```js\nconst x = 1;\n```\n");
        let output = print_mdx_js(&r.document, None);
        assert!(
            output.code.contains("language-js"),
            "should include language class when not highlighted"
        );
        assert!(
            !output.code.contains("dangerouslySetInnerHTML"),
            "should not use dangerouslySetInnerHTML without highlighting"
        );
    }

    #[test]
    fn mdx_js_emit_children_have_keys() {
        let r = parse_mdx("# Title\n\nParagraph\n\n- item\n");
        let output = print_mdx_js(&r.document, None);
        assert!(
            output.code.contains(", \"0\")"),
            "first child should have key \"0\": {}",
            output.code
        );
        assert!(
            output.code.contains(", \"1\")"),
            "second child should have key \"1\": {}",
            output.code
        );
    }
}
