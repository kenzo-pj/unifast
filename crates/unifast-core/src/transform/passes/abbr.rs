use crate::ast::common::{NodeIdGen, Span};
use crate::ast::mdast::nodes::{Abbr, Document, MdNode, Text};
use std::collections::HashMap;

pub fn apply_abbr(doc: &mut Document, id_gen: &mut NodeIdGen) {
    let definitions = collect_definitions(&doc.children);
    if definitions.is_empty() {
        return;
    }

    remove_definitions(&mut doc.children);
    replace_terms(&mut doc.children, &definitions, id_gen);
}

fn collect_definitions(children: &[MdNode]) -> HashMap<String, String> {
    let mut defs = HashMap::new();
    for node in children {
        if let MdNode::Paragraph(p) = node
            && p.children.len() == 1
            && let MdNode::Text(t) = &p.children[0]
            && let Some((term, def)) = parse_abbr_definition(&t.value)
        {
            defs.insert(term, def);
        }
    }
    defs
}

fn parse_abbr_definition(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    if !s.starts_with("*[") {
        return None;
    }
    let close = s.find("]:")?;
    let term = s[2..close].trim().to_string();
    let def = s[close + 2..].trim().to_string();
    if term.is_empty() || def.is_empty() {
        return None;
    }
    Some((term, def))
}

fn remove_definitions(children: &mut Vec<MdNode>) {
    children.retain(|node| {
        if let MdNode::Paragraph(p) = node
            && p.children.len() == 1
            && let MdNode::Text(t) = &p.children[0]
        {
            return parse_abbr_definition(&t.value).is_none();
        }
        true
    });
}

fn replace_terms(
    children: &mut Vec<MdNode>,
    defs: &HashMap<String, String>,
    id_gen: &mut NodeIdGen,
) {
    let mut i = 0;
    while i < children.len() {
        if let MdNode::Text(text) = &children[i] {
            let replacements = split_with_abbrs(&text.value, text.span, defs, id_gen);
            if replacements.len() > 1 {
                children.splice(i..=i, replacements.clone());
                i += replacements.len();
                continue;
            }
        }
        if let Some(kids) = children[i].children_mut() {
            replace_terms(kids, defs, id_gen);
        }
        i += 1;
    }
}

fn is_word_boundary(c: char) -> bool {
    !c.is_alphanumeric() && c != '_'
}

fn split_with_abbrs(
    text: &str,
    span: Span,
    defs: &HashMap<String, String>,
    id_gen: &mut NodeIdGen,
) -> Vec<MdNode> {
    let mut sorted_terms: Vec<&String> = defs.keys().collect();
    sorted_terms.sort_by_key(|t| std::cmp::Reverse(t.len()));

    let mut result: Vec<MdNode> = vec![MdNode::Text(Text {
        id: id_gen.next_id(),
        span,
        value: text.to_string(),
    })];

    for term in &sorted_terms {
        let definition = &defs[*term];
        let mut new_result = Vec::new();

        for node in result {
            if let MdNode::Text(t) = &node {
                let parts = split_text_by_term(&t.value, term);
                if parts.len() == 1 {
                    new_result.push(node);
                    continue;
                }
                for part in parts {
                    match part {
                        SplitPart::Text(s) => {
                            if !s.is_empty() {
                                new_result.push(MdNode::Text(Text {
                                    id: id_gen.next_id(),
                                    span,
                                    value: s,
                                }));
                            }
                        }
                        SplitPart::Abbr => {
                            new_result.push(MdNode::Abbr(Abbr {
                                id: id_gen.next_id(),
                                span,
                                term: (*term).clone(),
                                definition: definition.clone(),
                            }));
                        }
                    }
                }
            } else {
                new_result.push(node);
            }
        }

        result = new_result;
    }

    result
}

enum SplitPart {
    Text(String),
    Abbr,
}

fn split_text_by_term(text: &str, term: &str) -> Vec<SplitPart> {
    let mut parts = Vec::new();
    let mut remaining = text;
    let mut found_any = false;

    while let Some(pos) = remaining.find(term) {
        let before_ok = pos == 0
            || remaining[..pos]
                .chars()
                .next_back()
                .is_none_or(is_word_boundary);
        let after_pos = pos + term.len();
        let after_ok = after_pos >= remaining.len()
            || remaining[after_pos..]
                .chars()
                .next()
                .is_none_or(is_word_boundary);

        if before_ok && after_ok {
            found_any = true;
            if pos > 0 {
                parts.push(SplitPart::Text(remaining[..pos].to_string()));
            }
            parts.push(SplitPart::Abbr);
            remaining = &remaining[after_pos..];
        } else {
            parts.push(SplitPart::Text(remaining[..after_pos].to_string()));
            remaining = &remaining[after_pos..];
        }
    }

    if !found_any {
        return vec![SplitPart::Text(text.to_string())];
    }

    if !remaining.is_empty() {
        parts.push(SplitPart::Text(remaining.to_string()));
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::NodeIdGen;
    use crate::ast::mdast::nodes::Paragraph;

    #[test]
    fn parse_definition() {
        let (term, def) = parse_abbr_definition("*[HTML]: Hyper Text Markup Language").unwrap();
        assert_eq!(term, "HTML");
        assert_eq!(def, "Hyper Text Markup Language");
    }

    #[test]
    fn parse_definition_with_whitespace() {
        let (term, def) = parse_abbr_definition("  *[CSS]: Cascading Style Sheets  ").unwrap();
        assert_eq!(term, "CSS");
        assert_eq!(def, "Cascading Style Sheets");
    }

    #[test]
    fn parse_definition_empty_term_returns_none() {
        assert!(parse_abbr_definition("*[]: Something").is_none());
    }

    #[test]
    fn parse_definition_empty_def_returns_none() {
        assert!(parse_abbr_definition("*[HTML]:").is_none());
    }

    #[test]
    fn parse_definition_no_prefix_returns_none() {
        assert!(parse_abbr_definition("HTML: Hyper Text Markup Language").is_none());
    }

    #[test]
    fn replaces_terms() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 50);
        let defs = {
            let mut m = HashMap::new();
            m.insert("HTML".to_string(), "Hyper Text Markup Language".to_string());
            m
        };
        let result = split_with_abbrs("HTML is great", span, &defs, &mut id_gen);
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], MdNode::Abbr(a) if a.term == "HTML"));
        assert!(matches!(&result[1], MdNode::Text(t) if t.value == " is great"));
    }

    #[test]
    fn does_not_replace_partial_word() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 50);
        let defs = {
            let mut m = HashMap::new();
            m.insert("HTML".to_string(), "Hyper Text Markup Language".to_string());
            m
        };
        let result = split_with_abbrs("HTMLX is not a thing", span, &defs, &mut id_gen);
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], MdNode::Text(_)));
    }

    #[test]
    fn apply_abbr_full() {
        let mut id_gen = NodeIdGen::new();
        let mut doc = Document {
            id: id_gen.next_id(),
            span: Span::new(0, 100),
            children: vec![
                MdNode::Paragraph(Paragraph {
                    id: id_gen.next_id(),
                    span: Span::new(0, 36),
                    children: vec![MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span: Span::new(0, 36),
                        value: "*[HTML]: Hyper Text Markup Language".to_string(),
                    })],
                }),
                MdNode::Paragraph(Paragraph {
                    id: id_gen.next_id(),
                    span: Span::new(37, 55),
                    children: vec![MdNode::Text(Text {
                        id: id_gen.next_id(),
                        span: Span::new(37, 55),
                        value: "HTML is great.".to_string(),
                    })],
                }),
            ],
        };

        apply_abbr(&mut doc, &mut id_gen);

        assert_eq!(
            doc.children.len(),
            1,
            "definition paragraph should be removed"
        );
        if let MdNode::Paragraph(p) = &doc.children[0] {
            assert!(p.children.len() >= 2, "should have abbr + text nodes");
            assert!(
                matches!(&p.children[0], MdNode::Abbr(a) if a.term == "HTML" && a.definition == "Hyper Text Markup Language"),
                "first child should be Abbr"
            );
        } else {
            panic!("expected Paragraph");
        }
    }

    #[test]
    fn multiple_occurrences() {
        let mut id_gen = NodeIdGen::new();
        let span = Span::new(0, 50);
        let defs = {
            let mut m = HashMap::new();
            m.insert("JS".to_string(), "JavaScript".to_string());
            m
        };
        let result = split_with_abbrs("JS and JS are cool", span, &defs, &mut id_gen);
        let abbr_count = result
            .iter()
            .filter(|n| matches!(n, MdNode::Abbr(_)))
            .count();
        assert_eq!(abbr_count, 2);
    }
}
