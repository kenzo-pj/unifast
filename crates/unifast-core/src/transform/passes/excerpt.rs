use crate::ast::mdast::nodes::{Document, MdNode};

#[must_use]
pub fn extract_excerpt(
    doc: &Document,
    separator: &str,
    fallback_paragraphs: Option<u32>,
    fallback_characters: Option<u32>,
) -> Option<String> {
    let marker_pos = doc
        .children
        .iter()
        .position(|node| matches!(node, MdNode::Html(h) if h.value.trim() == separator));

    if let Some(pos) = marker_pos {
        let text = collect_plain_text(&doc.children[..pos]);
        if text.is_empty() { None } else { Some(text) }
    } else if let Some(n) = fallback_paragraphs {
        let mut collected = Vec::new();
        let mut count = 0u32;
        for node in &doc.children {
            if matches!(node, MdNode::Paragraph(_)) {
                collected.push(node);
                count += 1;
                if count >= n {
                    break;
                }
            }
        }
        let text = collect_plain_text_from_refs(&collected);
        if text.is_empty() { None } else { Some(text) }
    } else if let Some(n) = fallback_characters {
        let all_text = collect_plain_text(&doc.children);
        if all_text.is_empty() {
            return None;
        }
        let truncated = truncate_on_word_boundary(&all_text, n as usize);
        if truncated.is_empty() {
            None
        } else {
            Some(truncated)
        }
    } else {
        None
    }
}

fn truncate_on_word_boundary(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let byte_end = text
        .char_indices()
        .nth(max_chars)
        .map_or(text.len(), |(i, _)| i);
    let truncated = &text[..byte_end];
    if let Some(last_space) = truncated.rfind(' ') {
        truncated[..last_space].trim().to_string()
    } else {
        truncated.trim().to_string()
    }
}

fn collect_plain_text(nodes: &[MdNode]) -> String {
    let mut text = String::new();
    for node in nodes {
        collect_text_recursive(node, &mut text);
    }
    text.trim().to_string()
}

fn collect_plain_text_from_refs(nodes: &[&MdNode]) -> String {
    let mut text = String::new();
    for node in nodes {
        collect_text_recursive(node, &mut text);
    }
    text.trim().to_string()
}

fn collect_text_recursive(node: &MdNode, text: &mut String) {
    match node {
        MdNode::Text(t) => {
            if !text.is_empty() && !text.ends_with(' ') && !t.value.starts_with(' ') {
                text.push(' ');
            }
            text.push_str(&t.value);
        }
        MdNode::Code(_) => {}
        MdNode::Html(_) => {}
        _ => {
            if let Some(kids) = node.children() {
                for kid in kids {
                    collect_text_recursive(kid, text);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeId, Span};
    use crate::ast::mdast::nodes::*;

    fn make_doc(children: Vec<MdNode>) -> Document {
        Document {
            id: NodeId(0),
            span: Span::empty(),
            children,
        }
    }

    fn text_node(value: &str) -> MdNode {
        MdNode::Text(Text {
            id: NodeId(0),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn paragraph(children: Vec<MdNode>) -> MdNode {
        MdNode::Paragraph(Paragraph {
            id: NodeId(0),
            span: Span::empty(),
            children,
        })
    }

    fn html_node(value: &str) -> MdNode {
        MdNode::Html(Html {
            id: NodeId(0),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    #[test]
    fn extract_with_marker() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("This is the intro.")]),
            html_node("<!-- more -->"),
            paragraph(vec![text_node("This is the rest.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result.as_deref(), Some("This is the intro."));
    }

    #[test]
    fn extract_with_marker_whitespace() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("Intro text.")]),
            html_node("  <!-- more -->  "),
            paragraph(vec![text_node("Rest.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result.as_deref(), Some("Intro text."));
    }

    #[test]
    fn fallback_first_paragraph() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("First paragraph.")]),
            paragraph(vec![text_node("Second paragraph.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", Some(1), None);
        assert_eq!(result.as_deref(), Some("First paragraph."));
    }

    #[test]
    fn fallback_multiple_paragraphs() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("First.")]),
            paragraph(vec![text_node("Second.")]),
            paragraph(vec![text_node("Third.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", Some(2), None);
        assert_eq!(result.as_deref(), Some("First. Second."));
    }

    #[test]
    fn fallback_characters() {
        let doc = make_doc(vec![paragraph(vec![text_node(
            "Hello world this is a long text.",
        )])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, Some(20));
        assert_eq!(result.as_deref(), Some("Hello world this is"));
    }

    #[test]
    fn fallback_paragraphs_takes_precedence() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("First paragraph.")]),
            paragraph(vec![text_node("Second paragraph.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", Some(1), Some(5));
        assert_eq!(result.as_deref(), Some("First paragraph."));
    }

    #[test]
    fn no_excerpt_no_marker_no_fallback() {
        let doc = make_doc(vec![paragraph(vec![text_node("Some text.")])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result, None);
    }

    #[test]
    fn marker_with_no_content_before() {
        let doc = make_doc(vec![
            html_node("<!-- more -->"),
            paragraph(vec![text_node("After marker.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result, None);
    }

    #[test]
    fn nested_inline_nodes() {
        let doc = make_doc(vec![
            paragraph(vec![
                text_node("Hello "),
                MdNode::Strong(Strong {
                    id: NodeId(0),
                    span: Span::empty(),
                    children: vec![text_node("bold")],
                }),
                text_node(" world"),
            ]),
            html_node("<!-- more -->"),
            paragraph(vec![text_node("Rest.")]),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result.as_deref(), Some("Hello bold world"));
    }

    #[test]
    fn skips_code_blocks() {
        let doc = make_doc(vec![
            paragraph(vec![text_node("Intro.")]),
            MdNode::Code(Code {
                id: NodeId(0),
                span: Span::empty(),
                value: "fn main() {}".to_string(),
                lang: None,
                meta: None,
            }),
            html_node("<!-- more -->"),
        ]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, None);
        assert_eq!(result.as_deref(), Some("Intro."));
    }

    #[test]
    fn empty_document() {
        let doc = make_doc(vec![]);
        let result = extract_excerpt(&doc, "<!-- more -->", Some(1), None);
        assert_eq!(result, None);
    }

    #[test]
    fn character_truncation_on_word_boundary() {
        let doc = make_doc(vec![paragraph(vec![text_node("abcde fghij klmno")])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, Some(12));
        assert_eq!(result.as_deref(), Some("abcde fghij"));
    }

    #[test]
    fn character_truncation_short_text() {
        let doc = make_doc(vec![paragraph(vec![text_node("Hi")])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, Some(100));
        assert_eq!(result.as_deref(), Some("Hi"));
    }

    #[test]
    fn character_truncation_multibyte_japanese() {
        let doc = make_doc(vec![paragraph(vec![text_node("こんにちは 世界です")])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, Some(6));
        assert_eq!(result.as_deref(), Some("こんにちは"));
    }

    #[test]
    fn character_truncation_emoji() {
        let doc = make_doc(vec![paragraph(vec![text_node("Hello 👋🌍 world")])]);
        let result = extract_excerpt(&doc, "<!-- more -->", None, Some(8));
        assert_eq!(result.as_deref(), Some("Hello"));

        let result2 = extract_excerpt(
            &make_doc(vec![paragraph(vec![text_node("👋🌍🎉 test")])]),
            "<!-- more -->",
            None,
            Some(3),
        );
        assert_eq!(result2.as_deref(), Some("👋🌍🎉"));
    }
}
