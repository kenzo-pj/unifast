use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::*;
use crate::util::small_map::SmallMap;

/// Wrap each line of text inside `<pre><code>` blocks with
/// `<span class="line" data-line="N">…</span>`.
///
/// This runs on the HAST and is independent of any syntax highlighter.
/// If shiki has already wrapped lines in `<span class="line">`, this pass
/// adds the `data-line` attribute to those existing spans instead.
pub fn apply_line_numbers(root: &mut HRoot, id_gen: &mut NodeIdGen) {
    for child in &mut root.children {
        apply_to_node(child, id_gen);
    }
}

fn apply_to_node(node: &mut HNode, id_gen: &mut NodeIdGen) {
    match node {
        HNode::Element(elem) => {
            if elem.tag == "pre" {
                if let Some(code) = elem.children.iter_mut().find(|c| {
                    matches!(c, HNode::Element(e) if e.tag == "code")
                }) {
                    if let HNode::Element(code_elem) = code {
                        wrap_lines_in_code(code_elem, id_gen);
                    }
                }
            } else {
                for child in &mut elem.children {
                    apply_to_node(child, id_gen);
                }
            }
        }
        HNode::Root(r) => {
            for child in &mut r.children {
                apply_to_node(child, id_gen);
            }
        }
        _ => {}
    }
}

fn wrap_lines_in_code(code: &mut HElement, id_gen: &mut NodeIdGen) {
    // Check if lines are already wrapped in <span class="line"> (e.g. by shiki)
    let already_wrapped = code.children.iter().any(|c| {
        matches!(c, HNode::Element(e) if e.tag == "span" && has_class(&e.attributes, "line"))
    });

    if already_wrapped {
        // Just add data-line attributes to existing .line spans
        let mut line_num = 1u32;
        for child in &mut code.children {
            if let HNode::Element(e) = child {
                if e.tag == "span" && has_class(&e.attributes, "line") {
                    e.attributes.insert("data-line".to_string(), line_num.to_string());
                    line_num += 1;
                }
            }
        }
        return;
    }

    // Check if children contain HRaw nodes (e.g. from syntect highlighting).
    // Syntect replaces code children with a single HRaw containing highlighted HTML.
    // Syntect's output may have spans that cross line boundaries (e.g. a scope span
    // opened on line 1 and closed on line 3). We must close and re-open such spans
    // at each line boundary so each line is self-contained for CSS targeting.
    let has_raw = code.children.iter().any(|c| matches!(c, HNode::Raw(_)));
    if has_raw {
        let span = code.span;
        let old_children = std::mem::take(&mut code.children);

        // Concatenate all raw HTML content
        let mut raw_html = String::new();
        for child in &old_children {
            match child {
                HNode::Raw(r) => raw_html.push_str(&r.value),
                HNode::Text(t) => raw_html.push_str(&t.value),
                _ => {}
            }
        }

        let self_contained_lines = make_lines_self_contained(&raw_html);
        let mut new_children = Vec::with_capacity(self_contained_lines.len());

        for (i, line_html) in self_contained_lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            let wrapped = format!(
                "<span class=\"line\" data-line=\"{}\">{}</span>",
                line_num, line_html
            );

            new_children.push(HNode::Raw(HRaw {
                id: id_gen.next_id(),
                span,
                value: wrapped,
            }));

            // Add newline between lines
            if i < self_contained_lines.len() - 1 {
                new_children.push(HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: "\n".to_string(),
                }));
            }
        }

        code.children = new_children;
        return;
    }

    // Plain text content: split by newlines and wrap each in <span class="line" data-line="N">
    let span = code.span;
    let old_children = std::mem::take(&mut code.children);
    let full_text = collect_text(&old_children);
    let lines: Vec<&str> = full_text.split('\n').collect();

    let mut new_children = Vec::with_capacity(lines.len());
    for (i, line_text) in lines.iter().enumerate() {
        // Skip trailing empty line after final newline
        if i == lines.len() - 1 && line_text.is_empty() {
            break;
        }

        let line_num = (i + 1) as u32;
        let mut attrs = SmallMap::new();
        attrs.insert("class".to_string(), "line".to_string());
        attrs.insert("data-line".to_string(), line_num.to_string());

        let text_node = HNode::Text(HText {
            id: id_gen.next_id(),
            span,
            value: line_text.to_string(),
        });

        let line_span = HNode::Element(HElement {
            id: id_gen.next_id(),
            span,
            tag: "span".to_string(),
            attributes: attrs,
            children: vec![text_node],
            self_closing: false,
        });

        new_children.push(line_span);

        // Add newline text between lines (not after the last)
        if i < lines.len() - 1 {
            let next_is_trailing_empty =
                i == lines.len() - 2 && lines.last().map_or(false, |l| l.is_empty());
            if !next_is_trailing_empty {
                new_children.push(HNode::Text(HText {
                    id: id_gen.next_id(),
                    span,
                    value: "\n".to_string(),
                }));
            }
        }
    }

    code.children = new_children;
}

/// Split raw HTML by newlines and ensure each line is self-contained.
///
/// Syntect's output may have `<span>` elements that span multiple lines.
/// For example: `<span class="sy-source sy-rust">line1\nline2\n</span>`
/// This function closes open spans at each line boundary and re-opens
/// them at the start of the next line, so each resulting line can be
/// independently wrapped without breaking DOM nesting.
fn make_lines_self_contained(html: &str) -> Vec<String> {
    let raw_lines: Vec<&str> = html.split('\n').collect();
    let mut result = Vec::new();
    // Stack of class attribute values for currently open spans that carry across lines
    let mut carry_spans: Vec<String> = Vec::new();

    for (line_idx, raw_line) in raw_lines.iter().enumerate() {
        // Skip trailing empty line after final newline
        if line_idx == raw_lines.len() - 1 && raw_line.is_empty() {
            break;
        }

        let mut line_html = String::new();

        // Re-open spans carried from previous line
        for span_class in &carry_spans {
            line_html.push_str("<span class=\"");
            line_html.push_str(span_class);
            line_html.push_str("\">");
        }

        // Parse the line, tracking span opens/closes
        let mut pos = 0;
        let bytes = raw_line.as_bytes();
        while pos < bytes.len() {
            if bytes[pos] == b'<' {
                // Find end of tag
                let tag_end = raw_line[pos..].find('>').map(|p| pos + p + 1).unwrap_or(raw_line.len());
                let tag = &raw_line[pos..tag_end];

                if tag.starts_with("<span ") {
                    // Extract class value from <span class="...">
                    if let Some(cls_start) = tag.find("class=\"") {
                        let val_start = cls_start + 7;
                        if let Some(val_len) = tag[val_start..].find('"') {
                            let class_val = &tag[val_start..val_start + val_len];
                            carry_spans.push(class_val.to_string());
                        }
                    }
                    line_html.push_str(tag);
                } else if tag == "</span>" {
                    carry_spans.pop();
                    line_html.push_str(tag);
                } else {
                    line_html.push_str(tag);
                }
                pos = tag_end;
            } else {
                line_html.push(bytes[pos] as char);
                pos += 1;
            }
        }

        // Close all spans that are still open (they'll be re-opened on the next line)
        for _ in 0..carry_spans.len() {
            line_html.push_str("</span>");
        }

        result.push(line_html);
    }

    result
}

fn has_class(attrs: &SmallMap<String, String>, class: &str) -> bool {
    attrs
        .get(&"class".to_string())
        .map(|v| v.split_whitespace().any(|c| c == class))
        .unwrap_or(false)
}

fn collect_text(nodes: &[HNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        collect_text_inner(node, &mut out);
    }
    out
}

fn collect_text_inner(node: &HNode, out: &mut String) {
    match node {
        HNode::Text(t) => out.push_str(&t.value),
        HNode::Element(e) => {
            for child in &e.children {
                collect_text_inner(child, out);
            }
        }
        HNode::Root(r) => {
            for child in &r.children {
                collect_text_inner(child, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::emit::html::stringify;

    fn make_code_block(id_gen: &mut NodeIdGen, code_text: &str, lang: Option<&str>) -> HRoot {
        let text = HNode::Text(HText {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            value: code_text.to_string(),
        });

        let mut code_attrs = SmallMap::new();
        if let Some(l) = lang {
            code_attrs.insert("class".to_string(), format!("language-{}", l));
        }

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            tag: "code".to_string(),
            attributes: code_attrs,
            children: vec![text],
            self_closing: false,
        });

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            tag: "pre".to_string(),
            attributes: SmallMap::new(),
            children: vec![code_elem],
            self_closing: false,
        });

        HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![pre],
        }
    }

    #[test]
    fn wraps_single_line() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_code_block(&mut id_gen, "hello", None);
        apply_line_numbers(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(html.contains("data-line=\"1\""));
        assert!(html.contains("class=\"line\""));
        assert!(html.contains("hello"));
    }

    #[test]
    fn wraps_multiple_lines() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_code_block(&mut id_gen, "line1\nline2\nline3\n", None);
        apply_line_numbers(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(html.contains("data-line=\"1\""));
        assert!(html.contains("data-line=\"2\""));
        assert!(html.contains("data-line=\"3\""));
        assert!(!html.contains("data-line=\"4\""));
    }

    #[test]
    fn adds_data_line_to_existing_shiki_spans() {
        let mut id_gen = NodeIdGen::new();

        // Simulate shiki output: <code><span class="line">...</span>\n<span class="line">...</span></code>
        let line1 = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 50),
            tag: "span".to_string(),
            attributes: {
                let mut a = SmallMap::new();
                a.insert("class".to_string(), "line".to_string());
                a
            },
            children: vec![HNode::Text(HText {
                id: id_gen.next_id(),
                span: Span::new(0, 10),
                value: "first".to_string(),
            })],
            self_closing: false,
        });
        let line2 = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(50, 100),
            tag: "span".to_string(),
            attributes: {
                let mut a = SmallMap::new();
                a.insert("class".to_string(), "line".to_string());
                a
            },
            children: vec![HNode::Text(HText {
                id: id_gen.next_id(),
                span: Span::new(50, 60),
                value: "second".to_string(),
            })],
            self_closing: false,
        });

        let code_elem = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            tag: "code".to_string(),
            attributes: SmallMap::new(),
            children: vec![line1, line2],
            self_closing: false,
        });

        let pre = HNode::Element(HElement {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            tag: "pre".to_string(),
            attributes: SmallMap::new(),
            children: vec![code_elem],
            self_closing: false,
        });

        let mut root = HRoot {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![pre],
        };

        apply_line_numbers(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(html.contains("data-line=\"1\""));
        assert!(html.contains("data-line=\"2\""));
        // Original class preserved
        assert!(html.contains("class=\"line\""));
    }

    #[test]
    fn preserves_language_class() {
        let mut id_gen = NodeIdGen::new();
        let mut root = make_code_block(&mut id_gen, "fn main() {}\n", Some("rust"));
        apply_line_numbers(&mut root, &mut id_gen);

        let html = stringify::stringify(&root);
        assert!(html.contains("language-rust"));
        assert!(html.contains("data-line=\"1\""));
    }
}
