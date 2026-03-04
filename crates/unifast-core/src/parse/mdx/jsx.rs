use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{MdNode, MdxJsxAttribute, MdxJsxElement};

/// Result of parsing a JSX flow element, including how many source lines and
/// bytes were consumed so the caller can advance correctly.
pub struct JsxFlowResult {
    pub node: MdNode,
    pub lines_consumed: usize,
    pub bytes_consumed: usize,
}

/// Check whether `line` begins a JSX flow element.
///
/// A line is considered JSX if it starts with `<` followed by either an
/// uppercase letter (component), `>` (fragment open), or `/` (closing tag or
/// fragment close).
pub fn is_jsx_start(line: &str) -> bool {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix('<') {
        let after = rest.trim_start();
        after.starts_with(|c: char| c.is_uppercase())
            || after.starts_with('>')
            || after.starts_with('/')
    } else {
        false
    }
}

/// Try to parse a JSX flow element from a set of lines.
///
/// `line` is the first line, `remaining_lines` is a slice starting from that
/// same line (i.e. `remaining_lines[0] == line`). `offset` is the byte offset
/// of `line` in the original source.
pub fn try_parse_jsx_flow(
    line: &str,
    remaining_lines: &[&str],
    offset: usize,
    id_gen: &mut NodeIdGen,
) -> Option<JsxFlowResult> {
    let trimmed = line.trim();
    if !trimmed.starts_with('<') {
        return None;
    }

    let after = &trimmed[1..];
    let after_trimmed = after.trim_start();

    let is_closing = after_trimmed.starts_with('/');
    let is_component = if is_closing {
        // Closing tag like </Component>
        let after_slash = after_trimmed[1..].trim_start();
        after_slash.starts_with(|c: char| c.is_uppercase())
    } else {
        after_trimmed.starts_with(|c: char| c.is_uppercase())
    };
    let is_fragment = after_trimmed.starts_with('>') || after_trimmed.starts_with("/>");

    if !is_component && !is_fragment {
        return None;
    }

    // Closing tags on their own should not be parsed as standalone elements
    // (they will be consumed by the parent opening tag logic).
    if is_closing {
        return None;
    }

    // Parse tag name.
    let name = if is_fragment {
        None
    } else {
        let name_start = after.find(|c: char| c.is_alphabetic()).unwrap_or(0);
        let name_end = after[name_start..]
            .find(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-')
            .map_or(after.len(), |i| i + name_start);
        Some(after[name_start..name_end].to_string())
    };

    // Parse attributes from the opening tag line.
    let attrs = parse_jsx_attributes(trimmed);

    // Self-closing tag: <Component /> or <Component attr="val" />
    if trimmed.ends_with("/>") {
        return Some(JsxFlowResult {
            node: MdNode::MdxJsxFlowElement(MdxJsxElement {
                id: id_gen.next_id(),
                span: Span::new(offset as u32, (offset + line.len()) as u32),
                name,
                attributes: attrs,
                children: vec![],
            }),
            lines_consumed: 1,
            bytes_consumed: line.len() + 1,
        });
    }

    // Fragment self-close: </>
    if trimmed == "</>" {
        return None; // stray closing fragment — skip
    }

    // Multi-line JSX: find the matching closing tag.
    let closing_tag = match name {
        Some(ref n) => format!("</{n}>"),
        None => "</>".to_string(),
    };

    let mut total_lines = 1;
    let mut total_bytes = line.len() + 1; // +1 for the newline
    let mut content_lines: Vec<String> = Vec::new();

    for l in remaining_lines.iter().skip(1) {
        total_lines += 1;
        total_bytes += l.len() + 1;
        if l.trim() == closing_tag || l.trim().starts_with(&closing_tag) {
            break;
        }
        content_lines.push((*l).to_string());
    }

    // Parse inner content as children — for now we store the inner text
    // content as a Text node if there is any. A full recursive parse would
    // invoke the MDX parser again on the inner content.
    let children = if content_lines.is_empty() {
        vec![]
    } else {
        let inner = content_lines.join("\n");
        let inner_trimmed = inner.trim();
        if inner_trimmed.is_empty() {
            vec![]
        } else {
            // Use markdown parser for inner content.
            use crate::diagnostics::sink::DiagnosticSink;
            use crate::parse::markdown::parser;

            let mut diagnostics = DiagnosticSink::new();
            let inner_with_newline = format!("{inner_trimmed}\n");
            let doc = parser::parse(&inner_with_newline, id_gen, &mut diagnostics);
            doc.children
        }
    };

    Some(JsxFlowResult {
        node: MdNode::MdxJsxFlowElement(MdxJsxElement {
            id: id_gen.next_id(),
            span: Span::new(
                offset as u32,
                (offset + total_bytes.saturating_sub(1)) as u32,
            ),
            name,
            attributes: attrs,
            children,
        }),
        lines_consumed: total_lines,
        bytes_consumed: total_bytes,
    })
}

/// Parse JSX attributes from a single tag line.
///
/// Supports: `name="value"`, `name='value'`, `name={expr}`, and boolean `name`.
fn parse_jsx_attributes(tag_line: &str) -> Vec<MdxJsxAttribute> {
    let mut attrs = Vec::new();

    // Find the region between the tag name and the closing `>` or `/>`.
    let content = extract_attribute_region(tag_line);
    if content.is_empty() {
        return attrs;
    }

    let mut chars = content.char_indices().peekable();
    while let Some(&(_, ch)) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '/' || ch == '>' {
            break;
        }
        if ch.is_alphabetic() || ch == '_' {
            // Attribute name
            let mut name = String::new();
            while let Some(&(_, c)) = chars.peek() {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            // Check for `=`
            if let Some(&(_, '=')) = chars.peek() {
                chars.next();
                if let Some(&(_, quote)) = chars.peek() {
                    if quote == '"' || quote == '\'' {
                        chars.next();
                        let mut value = String::new();
                        for (_, c) in chars.by_ref() {
                            if c == quote {
                                break;
                            }
                            value.push(c);
                        }
                        attrs.push(MdxJsxAttribute {
                            name,
                            value: Some(value),
                        });
                    } else if quote == '{' {
                        // Expression value
                        chars.next();
                        let mut depth = 1;
                        let mut value = String::new();
                        for (_, c) in chars.by_ref() {
                            if c == '{' {
                                depth += 1;
                            }
                            if c == '}' {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            value.push(c);
                        }
                        attrs.push(MdxJsxAttribute {
                            name,
                            value: Some(format!("{{{value}}}")),
                        });
                    }
                }
            } else {
                // Boolean attribute (no value)
                attrs.push(MdxJsxAttribute { name, value: None });
            }
        } else {
            chars.next();
        }
    }
    attrs
}

/// Extract the substring between the tag name and the closing `>` / `/>`.
fn extract_attribute_region(tag_line: &str) -> &str {
    let trimmed = tag_line.trim();
    // Skip the `<` and tag name.
    let after_open = match trimmed.strip_prefix('<') {
        Some(rest) => rest.trim_start(),
        None => return "",
    };

    // Skip past the tag name (letters, digits, dots, dashes, underscores).
    let name_end = after_open
        .find(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-')
        .unwrap_or(after_open.len());

    let attr_start = &after_open[name_end..];

    // Trim the closing `/>` or `>` from the end.
    if let Some(s) = attr_start.strip_suffix("/>") {
        s
    } else if let Some(s) = attr_start.strip_suffix('>') {
        s
    } else {
        attr_start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_jsx_start_component() {
        assert!(is_jsx_start("<Button />"));
        assert!(is_jsx_start("<MyComponent>"));
        assert!(is_jsx_start("  <Card>"));
    }

    #[test]
    fn is_jsx_start_fragment() {
        assert!(is_jsx_start("<>"));
        assert!(is_jsx_start("</>"));
    }

    #[test]
    fn is_jsx_start_not_html() {
        assert!(!is_jsx_start("<div>"));
        assert!(!is_jsx_start("<p>hello</p>"));
        assert!(!is_jsx_start("<span>"));
    }

    #[test]
    fn is_jsx_start_not_jsx() {
        assert!(!is_jsx_start("hello"));
        assert!(!is_jsx_start("# heading"));
    }

    #[test]
    fn self_closing_component() {
        let mut id_gen = NodeIdGen::new();
        let lines = ["<Button />"];
        let result = try_parse_jsx_flow("<Button />", &lines, 0, &mut id_gen);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.lines_consumed, 1);
        if let MdNode::MdxJsxFlowElement(el) = &r.node {
            assert_eq!(el.name.as_deref(), Some("Button"));
            assert!(el.children.is_empty());
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn self_closing_with_attributes() {
        let mut id_gen = NodeIdGen::new();
        let line = r#"<Button variant="primary" disabled />"#;
        let lines = [line];
        let result = try_parse_jsx_flow(line, &lines, 0, &mut id_gen);
        assert!(result.is_some());
        let r = result.unwrap();
        if let MdNode::MdxJsxFlowElement(el) = &r.node {
            assert_eq!(el.name.as_deref(), Some("Button"));
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "variant" && a.value.as_deref() == Some("primary"))
            );
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "disabled" && a.value.is_none())
            );
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn component_with_closing_tag() {
        let mut id_gen = NodeIdGen::new();
        let lines = ["<Card>", "  hello", "</Card>"];
        let result = try_parse_jsx_flow("<Card>", &lines, 0, &mut id_gen);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.lines_consumed, 3);
        if let MdNode::MdxJsxFlowElement(el) = &r.node {
            assert_eq!(el.name.as_deref(), Some("Card"));
        } else {
            panic!("expected MdxJsxFlowElement");
        }
    }

    #[test]
    fn closing_tag_returns_none() {
        let mut id_gen = NodeIdGen::new();
        let lines = ["</Card>"];
        let result = try_parse_jsx_flow("</Card>", &lines, 0, &mut id_gen);
        assert!(result.is_none());
    }

    #[test]
    fn html_tag_returns_none() {
        let mut id_gen = NodeIdGen::new();
        let lines = ["<div>"];
        let result = try_parse_jsx_flow("<div>", &lines, 0, &mut id_gen);
        assert!(result.is_none());
    }

    #[test]
    fn attribute_with_expression_value() {
        let mut id_gen = NodeIdGen::new();
        let line = "<Button onClick={handleClick} />";
        let lines = [line];
        let result = try_parse_jsx_flow(line, &lines, 0, &mut id_gen);
        assert!(result.is_some());
        if let Some(JsxFlowResult {
            node: MdNode::MdxJsxFlowElement(el),
            ..
        }) = result
        {
            assert!(
                el.attributes
                    .iter()
                    .any(|a| a.name == "onClick" && a.value.as_deref() == Some("{handleClick}"))
            );
        }
    }

    #[test]
    fn parse_dotted_component_name() {
        let mut id_gen = NodeIdGen::new();
        let line = "<Form.Input />";
        let lines = [line];
        let result = try_parse_jsx_flow(line, &lines, 0, &mut id_gen);
        assert!(result.is_some());
        if let Some(JsxFlowResult {
            node: MdNode::MdxJsxFlowElement(el),
            ..
        }) = result
        {
            assert_eq!(el.name.as_deref(), Some("Form.Input"));
        }
    }

    #[test]
    fn empty_fragment_returns_none_for_close() {
        let mut id_gen = NodeIdGen::new();
        let lines = ["</>"];
        let result = try_parse_jsx_flow("</>", &lines, 0, &mut id_gen);
        assert!(result.is_none());
    }
}
