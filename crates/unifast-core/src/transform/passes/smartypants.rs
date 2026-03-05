use crate::ast::mdast::nodes::{Document, MdNode};

pub fn apply_smartypants(doc: &mut Document, quotes: bool, dashes: bool, ellipses: bool) {
    apply_to_children(&mut doc.children, quotes, dashes, ellipses);
}

fn apply_to_children(children: &mut [MdNode], quotes: bool, dashes: bool, ellipses: bool) {
    for child in children.iter_mut() {
        if let MdNode::Text(text) = child {
            text.value = transform_text(&text.value, quotes, dashes, ellipses);
        }
        if matches!(child, MdNode::Code(_) | MdNode::InlineCode(_)) {
            continue;
        }
        if let Some(kids) = child.children_mut() {
            apply_to_children(kids, quotes, dashes, ellipses);
        }
    }
}

fn transform_text(input: &str, quotes: bool, dashes: bool, ellipses: bool) -> String {
    let mut result = input.to_string();

    if ellipses {
        result = result.replace("...", "\u{2026}");
    }

    if dashes {
        result = result.replace("---", "\u{2014}");
        result = result.replace("--", "\u{2013}");
    }

    if quotes {
        result = replace_quotes(&result);
    }

    result
}

fn replace_quotes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut in_double = false;
    let mut in_single = false;

    while i < chars.len() {
        match chars[i] {
            '"' => {
                if in_double {
                    result.push('\u{201D}');
                    in_double = false;
                } else {
                    result.push('\u{201C}');
                    in_double = true;
                }
            }
            '\'' => {
                let prev_alpha = i > 0 && chars[i - 1].is_alphanumeric();
                let next_alpha = i + 1 < chars.len() && chars[i + 1].is_alphanumeric();
                if prev_alpha && next_alpha {
                    result.push('\u{2019}');
                } else if in_single {
                    result.push('\u{2019}');
                    in_single = false;
                } else {
                    result.push('\u{2018}');
                    in_single = true;
                }
            }
            c => result.push(c),
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_ellipsis() {
        assert_eq!(
            transform_text("wait...", false, false, true),
            "wait\u{2026}"
        );
    }

    #[test]
    fn replaces_dashes() {
        assert_eq!(transform_text("a--b", false, true, false), "a\u{2013}b");
        assert_eq!(transform_text("a---b", false, true, false), "a\u{2014}b");
    }

    #[test]
    fn replaces_quotes() {
        let result = transform_text("\"hello\" world", true, false, false);
        assert!(result.contains('\u{201C}'));
        assert!(result.contains('\u{201D}'));
    }

    #[test]
    fn handles_apostrophe() {
        let result = transform_text("it's", true, false, false);
        assert!(result.contains('\u{2019}'));
    }
}
