pub fn escape_html_into(output: &mut String, text: &str) {
    output.reserve(text.len());
    for ch in text.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            _ => output.push(ch),
        }
    }
}

#[must_use]
pub fn escape_html(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    escape_html_into(&mut result, text);
    result
}

#[must_use]
pub fn escape_attribute(text: &str) -> String {
    escape_html(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_ampersand() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn escape_less_than() {
        assert_eq!(escape_html("a < b"), "a &lt; b");
    }

    #[test]
    fn escape_greater_than() {
        assert_eq!(escape_html("a > b"), "a &gt; b");
    }

    #[test]
    fn escape_double_quote() {
        assert_eq!(escape_html("a \"b\" c"), "a &quot;b&quot; c");
    }

    #[test]
    fn escape_single_quote() {
        assert_eq!(escape_html("a 'b' c"), "a &#x27;b&#x27; c");
    }

    #[test]
    fn escape_multiple_special_chars() {
        assert_eq!(
            escape_html("<a href=\"#\">"),
            "&lt;a href=&quot;#&quot;&gt;"
        );
    }

    #[test]
    fn escape_no_special_chars() {
        assert_eq!(escape_html("hello world"), "hello world");
    }

    #[test]
    fn escape_empty_string() {
        assert_eq!(escape_html(""), "");
    }

    #[test]
    fn escape_attribute_same_as_html() {
        let input = "value with <special> & \"chars\"";
        assert_eq!(escape_attribute(input), escape_html(input));
    }
}
