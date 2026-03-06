use crate::ast::mdast::nodes::MdNode;

pub const fn is_cjk(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |
        '\u{3400}'..='\u{4DBF}' |
        '\u{3000}'..='\u{303F}' |
        '\u{3040}'..='\u{309F}' |
        '\u{30A0}'..='\u{30FF}' |
        '\u{FF00}'..='\u{FFEF}' |
        '\u{AC00}'..='\u{D7AF}' |
        '\u{1100}'..='\u{11FF}'
    )
}

pub fn apply_cjk(children: &mut [MdNode]) {
    for child in children.iter_mut() {
        if let MdNode::Text(text) = child {
            text.value = remove_cjk_line_join_spaces(&text.value);
        }
        if let Some(kids) = child.children_mut() {
            apply_cjk(kids);
        }
    }
}

pub fn remove_cjk_line_join_spaces(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\n' {
            let prev_cjk = i > 0 && is_cjk(chars[i - 1]);
            let next_cjk = i + 1 < chars.len() && is_cjk(chars[i + 1]);
            if prev_cjk && next_cjk {
                i += 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_newline_between_cjk() {
        assert_eq!(
            remove_cjk_line_join_spaces("\u{65E5}\u{672C}\u{8A9E}\n\u{30C6}\u{30B9}\u{30C8}"),
            "\u{65E5}\u{672C}\u{8A9E}\u{30C6}\u{30B9}\u{30C8}"
        );
    }

    #[test]
    fn keeps_newline_between_latin() {
        assert_eq!(remove_cjk_line_join_spaces("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn is_cjk_test() {
        assert!(is_cjk('\u{65E5}'));
        assert!(is_cjk('\u{3042}'));
        assert!(is_cjk('\u{30A2}'));
        assert!(!is_cjk('a'));
        assert!(!is_cjk('1'));
    }
}
