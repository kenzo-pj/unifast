use crate::api::result::ReadingTime;
use crate::ast::mdast::nodes::{Document, MdNode};

#[must_use]
pub fn calculate(doc: &Document, words_per_minute: u32, cjk_chars_per_minute: u32) -> ReadingTime {
    let mut latin_words = 0u32;
    let mut cjk_chars = 0u32;
    collect_text_stats(&doc.children, &mut latin_words, &mut cjk_chars);

    let total_words = latin_words + cjk_chars;
    let minutes = (f64::from(latin_words) / f64::from(words_per_minute))
        + (f64::from(cjk_chars) / f64::from(cjk_chars_per_minute));
    let minutes = (minutes * 2.0).ceil() / 2.0;

    ReadingTime {
        words: total_words,
        minutes: if minutes < 1.0 { 1.0 } else { minutes },
    }
}

fn collect_text_stats(children: &[MdNode], latin_words: &mut u32, cjk_chars: &mut u32) {
    for node in children {
        match node {
            MdNode::Text(t) => {
                for ch in t.value.chars() {
                    if is_cjk(ch) {
                        *cjk_chars += 1;
                    }
                }
                let non_cjk: String = t
                    .value
                    .chars()
                    .map(|c| if is_cjk(c) { ' ' } else { c })
                    .collect();
                *latin_words += non_cjk.split_whitespace().count() as u32;
            }
            MdNode::Code(_) => {}
            _ => {
                if let Some(kids) = node.children() {
                    collect_text_stats(kids, latin_words, cjk_chars);
                }
            }
        }
    }
}

const fn is_cjk(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |
        '\u{3040}'..='\u{309F}' |
        '\u{30A0}'..='\u{30FF}' |
        '\u{AC00}'..='\u{D7AF}' |
        '\u{3400}'..='\u{4DBF}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeId, Span};

    fn make_doc(children: Vec<MdNode>) -> Document {
        Document {
            id: NodeId(0),
            span: Span::empty(),
            children,
        }
    }

    fn text_node(value: &str) -> MdNode {
        MdNode::Text(crate::ast::mdast::nodes::Text {
            id: NodeId(0),
            span: Span::empty(),
            value: value.to_string(),
        })
    }

    fn paragraph(children: Vec<MdNode>) -> MdNode {
        MdNode::Paragraph(crate::ast::mdast::nodes::Paragraph {
            id: NodeId(0),
            span: Span::empty(),
            children,
        })
    }

    #[test]
    fn english_text() {
        let words: Vec<&str> = (0..200).map(|_| "word").collect();
        let text = words.join(" ");
        let doc = make_doc(vec![paragraph(vec![text_node(&text)])]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 200);
        assert!((rt.minutes - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn cjk_text() {
        let text: String = (0..500).map(|_| '\u{4E00}').collect();
        let doc = make_doc(vec![paragraph(vec![text_node(&text)])]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 500);
        assert!((rt.minutes - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn minimum_one_minute() {
        let doc = make_doc(vec![paragraph(vec![text_node("Hello")])]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 1);
        assert!((rt.minutes - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn skips_code_blocks() {
        let code = MdNode::Code(crate::ast::mdast::nodes::Code {
            id: NodeId(0),
            span: Span::empty(),
            lang: None,
            meta: None,
            value: "fn main() { println!(\"lots of words here to count\"); }".to_string(),
        });
        let doc = make_doc(vec![paragraph(vec![text_node("Hello")]), code]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 1);
    }

    #[test]
    fn mixed_cjk_and_latin() {
        let doc = make_doc(vec![paragraph(vec![text_node("Hello \u{4E16}\u{754C}")])]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 3); // 1 latin word + 2 CJK chars
    }

    #[test]
    fn rounds_up_to_nearest_half() {
        let words: Vec<&str> = (0..250).map(|_| "word").collect();
        let text = words.join(" ");
        let doc = make_doc(vec![paragraph(vec![text_node(&text)])]);
        let rt = calculate(&doc, 200, 500);
        assert_eq!(rt.words, 250);
        assert!((rt.minutes - 1.5).abs() < f64::EPSILON);
    }
}
