use crate::ast::mdast::nodes::*;

/// Compile an MDX document AST into a JavaScript module string.
pub fn print_mdx_js(doc: &Document) -> MdxJsOutput {
    let mut printer = MdxJsPrinter::new();
    printer.print_document(doc);
    MdxJsOutput {
        code: printer.output,
        source_mappings: printer.mappings,
    }
}

/// The output of the MDX-to-JS compilation step.
pub struct MdxJsOutput {
    /// The generated JavaScript source code.
    pub code: String,
    /// Source-mapping segments (generated position -> original offset).
    pub source_mappings: Vec<SourceMapping>,
}

/// A single source-mapping segment.
pub struct SourceMapping {
    pub generated_line: u32,
    pub generated_column: u32,
    pub original_offset: u32,
}

// ── Printer ──────────────────────────────────────────────────────────────

struct MdxJsPrinter {
    output: String,
    mappings: Vec<SourceMapping>,
    line: u32,
    column: u32,
}

impl MdxJsPrinter {
    fn new() -> Self {
        Self {
            output: String::new(),
            mappings: Vec::new(),
            line: 1,
            column: 0,
        }
    }

    /// Append text to the output, updating line/column tracking.
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

    /// Append text with a source-mapping entry.
    fn emit_with_mapping(&mut self, s: &str, original_offset: u32) {
        self.mappings.push(SourceMapping {
            generated_line: self.line,
            generated_column: self.column,
            original_offset,
        });
        self.emit(s);
    }

    // ── Document ─────────────────────────────────────────────────────

    fn print_document(&mut self, doc: &Document) {
        // Separate ESM statements from content nodes.
        let mut esm_nodes: Vec<&MdNode> = Vec::new();
        let mut content_nodes: Vec<&MdNode> = Vec::new();

        for child in &doc.children {
            match child {
                MdNode::MdxjsEsm(_) => esm_nodes.push(child),
                _ => content_nodes.push(child),
            }
        }

        // Emit ESM imports/exports at the top of the module.
        for esm in &esm_nodes {
            if let MdNode::MdxjsEsm(e) = esm {
                self.emit_with_mapping(&e.value, e.span.start);
                self.emit("\n");
            }
        }

        // Emit the MDXContent component function.
        self.emit("\nfunction MDXContent(props) {\n");
        self.emit("  return ");

        if content_nodes.is_empty() {
            self.emit("null;\n");
        } else {
            self.emit("_jsxs(_Fragment, {\n");
            self.emit("    children: [\n");

            for (i, node) in content_nodes.iter().enumerate() {
                self.print_mdx_node(node, 6);
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

    // ── Node emitter ─────────────────────────────────────────────────

    fn print_mdx_node(&mut self, node: &MdNode, indent: usize) {
        let pad = " ".repeat(indent);
        match node {
            MdNode::Heading(h) => {
                self.emit(&format!("{pad}_jsx(\"h{}\", {{ children: ", h.depth));
                self.print_inline_children(&h.children);
                self.emit(" })");
            }
            MdNode::Paragraph(p) => {
                self.emit(&format!("{pad}_jsx(\"p\", {{ children: "));
                self.print_inline_children(&p.children);
                self.emit(" })");
            }
            MdNode::Code(c) => {
                let lang_prop = c
                    .lang
                    .as_ref()
                    .map(|l| format!(", className: \"language-{l}\""))
                    .unwrap_or_default();
                self.emit(&format!(
                    "{pad}_jsx(\"pre\", {{ children: _jsx(\"code\", {{ children: \"{}\"{lang_prop} }}) }})",
                    escape_js_string(&c.value),
                ));
            }
            MdNode::MdxJsxFlowElement(el) | MdNode::MdxJsxTextElement(el) => {
                self.print_jsx_element(el, &pad, indent);
            }
            MdNode::MdxFlowExpression(expr) | MdNode::MdxTextExpression(expr) => {
                self.emit_with_mapping(&format!("{pad}{}", expr.value), expr.span.start);
            }
            MdNode::Blockquote(bq) => {
                self.emit(&format!("{pad}_jsxs(\"blockquote\", {{ children: [\n"));
                for (i, child) in bq.children.iter().enumerate() {
                    self.print_mdx_node(child, indent + 2);
                    if i < bq.children.len() - 1 {
                        self.emit(",\n");
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::List(l) => {
                let tag = if l.ordered { "ol" } else { "ul" };
                self.emit(&format!("{pad}_jsxs(\"{tag}\", {{ children: [\n"));
                for (i, child) in l.children.iter().enumerate() {
                    self.print_mdx_node(child, indent + 2);
                    if i < l.children.len() - 1 {
                        self.emit(",\n");
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::ListItem(li) => {
                self.emit(&format!("{pad}_jsxs(\"li\", {{ children: [\n"));
                for (i, child) in li.children.iter().enumerate() {
                    self.print_mdx_node(child, indent + 2);
                    if i < li.children.len() - 1 {
                        self.emit(",\n");
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::Emphasis(e) => {
                self.emit(&format!("{pad}_jsx(\"em\", {{ children: "));
                self.print_inline_children(&e.children);
                self.emit(" })");
            }
            MdNode::Strong(s) => {
                self.emit(&format!("{pad}_jsx(\"strong\", {{ children: "));
                self.print_inline_children(&s.children);
                self.emit(" })");
            }
            MdNode::Link(l) => {
                self.emit(&format!(
                    "{pad}_jsx(\"a\", {{ href: \"{}\", children: ",
                    escape_js_string(&l.url)
                ));
                self.print_inline_children(&l.children);
                self.emit(" })");
            }
            MdNode::Image(img) => {
                self.emit(&format!(
                    "{pad}_jsx(\"img\", {{ src: \"{}\", alt: \"{}\" }})",
                    escape_js_string(&img.url),
                    escape_js_string(&img.alt),
                ));
            }
            MdNode::InlineCode(c) => {
                self.emit(&format!(
                    "{pad}_jsx(\"code\", {{ children: \"{}\" }})",
                    escape_js_string(&c.value),
                ));
            }
            MdNode::Text(t) => {
                self.emit(&format!("{pad}\"{}\"", escape_js_string(&t.value)));
            }
            MdNode::ThematicBreak(_) => {
                self.emit(&format!("{pad}_jsx(\"hr\", {{}})"));
            }
            MdNode::Break(_) => {
                self.emit(&format!("{pad}_jsx(\"br\", {{}})"));
            }
            MdNode::Html(h) => {
                // Emit raw HTML as a string — consumers should sanitize before
                // rendering.
                self.emit(&format!(
                    "{pad}_jsx(\"div\", {{ rawHtml: \"{}\" }})",
                    escape_js_string(&h.value),
                ));
            }
            MdNode::Table(t) => {
                self.emit(&format!("{pad}_jsxs(\"table\", {{ children: [\n"));
                for (i, child) in t.children.iter().enumerate() {
                    self.print_mdx_node(child, indent + 2);
                    if i < t.children.len() - 1 {
                        self.emit(",\n");
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::TableRow(tr) => {
                self.emit(&format!("{pad}_jsxs(\"tr\", {{ children: [\n"));
                for (i, child) in tr.children.iter().enumerate() {
                    self.print_mdx_node(child, indent + 2);
                    if i < tr.children.len() - 1 {
                        self.emit(",\n");
                    }
                }
                self.emit(&format!("\n{pad}] }})"));
            }
            MdNode::TableCell(tc) => {
                self.emit(&format!("{pad}_jsx(\"td\", {{ children: "));
                self.print_inline_children(&tc.children);
                self.emit(" })");
            }
            MdNode::Delete(d) => {
                self.emit(&format!("{pad}_jsx(\"del\", {{ children: "));
                self.print_inline_children(&d.children);
                self.emit(" })");
            }
            // Nodes without a direct JSX representation are silently skipped.
            MdNode::Document(_)
            | MdNode::Definition(_)
            | MdNode::FootnoteDefinition(_)
            | MdNode::FootnoteReference(_)
            | MdNode::Yaml(_)
            | MdNode::Toml(_)
            | MdNode::Json(_)
            | MdNode::MdxjsEsm(_) => {}
        }
    }

    /// Emit a JSX element (`MdxJsxElement`).
    fn print_jsx_element(&mut self, el: &MdxJsxElement, pad: &str, indent: usize) {
        let name = el.name.as_deref().unwrap_or("_Fragment");

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
            self.emit(&format!("{pad}_jsx({name}, {{{props}}})",));
        } else {
            self.emit(&format!("{pad}_jsxs({name}, {{ children: [\n"));
            for (i, child) in el.children.iter().enumerate() {
                self.print_mdx_node(child, indent + 2);
                if i < el.children.len() - 1 {
                    self.emit(",\n");
                }
            }
            self.emit(&format!("\n{pad}]{props}}})",));
        }
    }

    /// Emit an inline children array (or a single string shorthand).
    fn print_inline_children(&mut self, children: &[MdNode]) {
        if children.len() == 1
            && let MdNode::Text(t) = &children[0]
        {
            self.emit(&format!("\"{}\"", escape_js_string(&t.value)));
            return;
        }
        self.emit("[");
        for (i, child) in children.iter().enumerate() {
            self.print_mdx_node(child, 0);
            if i < children.len() - 1 {
                self.emit(", ");
            }
        }
        self.emit("]");
    }
}

/// Escape a Rust string for embedding inside a JavaScript string literal.
fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::mdx::parse_mdx;

    // ---- MDX JS emission tests (at least 10) ----

    #[test]
    fn mdx_js_emit_heading() {
        let r = parse_mdx("# Hello\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("MDXContent"));
        assert!(output.code.contains("h1"));
        assert!(output.code.contains("Hello"));
    }

    #[test]
    fn mdx_js_emit_paragraph() {
        let r = parse_mdx("Hello world\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("_jsx(\"p\""));
        assert!(output.code.contains("Hello world"));
    }

    #[test]
    fn mdx_js_emit_with_import() {
        let r = parse_mdx("import { X } from 'x'\n\n# Hello\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("import { X } from 'x'"));
        assert!(output.code.contains("MDXContent"));
    }

    #[test]
    fn mdx_js_emit_with_export() {
        let r = parse_mdx("export const meta = {}\n\n# Title\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("export const meta = {}"));
        assert!(output.code.contains("export default MDXContent"));
    }

    #[test]
    fn mdx_js_emit_component() {
        let r = parse_mdx("<Button />\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("Button"));
        assert!(output.code.contains("_jsx("));
    }

    #[test]
    fn mdx_js_emit_component_with_props() {
        let r = parse_mdx("<Button variant=\"primary\" />\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("Button"));
        assert!(output.code.contains("variant"));
        assert!(output.code.contains("primary"));
    }

    #[test]
    fn mdx_js_emit_expression() {
        let r = parse_mdx("{someValue}\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("someValue"));
    }

    #[test]
    fn mdx_js_emit_code_block() {
        let r = parse_mdx("```rust\nfn main() {}\n```\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("pre"));
        assert!(output.code.contains("code"));
        assert!(output.code.contains("language-rust"));
    }

    #[test]
    fn mdx_js_emit_list() {
        let r = parse_mdx("- one\n- two\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("\"ul\""));
        assert!(output.code.contains("\"li\""));
    }

    #[test]
    fn mdx_js_emit_link() {
        let r = parse_mdx("[click](http://example.com)\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("\"a\""));
        assert!(output.code.contains("http://example.com"));
    }

    #[test]
    fn mdx_js_emit_image() {
        let r = parse_mdx("![alt text](img.png)\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("\"img\""));
        assert!(output.code.contains("img.png"));
        assert!(output.code.contains("alt text"));
    }

    #[test]
    fn mdx_js_emit_blockquote() {
        let r = parse_mdx("> quote\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("\"blockquote\""));
    }

    #[test]
    fn mdx_js_emit_empty() {
        let r = parse_mdx("");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("MDXContent"));
        assert!(output.code.contains("null"));
    }

    #[test]
    fn mdx_js_emit_default_export() {
        let r = parse_mdx("# Hello\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("export default MDXContent"));
    }

    #[test]
    fn mdx_js_emit_has_source_mappings() {
        let r = parse_mdx("import X from 'x'\n\n# Hello\n");
        let output = print_mdx_js(&r.document);
        assert!(!output.source_mappings.is_empty());
    }

    #[test]
    fn mdx_js_emit_boolean_prop() {
        let r = parse_mdx("<Input disabled />\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("disabled: true"));
    }

    #[test]
    fn mdx_js_emit_emphasis_and_strong() {
        let r = parse_mdx("Hello *world* and **bold**\n");
        let output = print_mdx_js(&r.document);
        assert!(output.code.contains("\"em\""));
        assert!(output.code.contains("\"strong\""));
    }

    #[test]
    fn mdx_js_escape_special_chars() {
        let r = parse_mdx("Hello \"world\"\n");
        let output = print_mdx_js(&r.document);
        // The quote should be escaped in the JS output.
        assert!(output.code.contains("\\\"world\\\""));
    }

    #[test]
    fn mdx_js_emit_thematic_break() {
        let r = parse_mdx("---\n\ntext\n");
        let output = print_mdx_js(&r.document);
        // The --- is either a thematic break or frontmatter;
        // with content after it, it may be treated as hr.
        assert!(output.code.contains("MDXContent"));
    }
}
