pub mod json;
pub mod toml;
pub mod yaml;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FrontmatterKind {
    Yaml,
    Toml,
    Json,
}

pub struct FrontmatterResult {
    pub kind: FrontmatterKind,
    pub data: HashMap<String, serde_json::Value>,
    pub raw: String,
    pub end_offset: usize,
}

#[must_use]
pub fn extract_frontmatter(input: &str) -> Option<FrontmatterResult> {
    if let Some(r) = yaml::extract(input) {
        return Some(r);
    }
    if let Some(r) = toml::extract(input) {
        return Some(r);
    }
    if let Some(r) = json::extract(input) {
        return Some(r);
    }
    None
}

fn find_closing_delimiter(text: &str, delimiter: &str) -> Option<usize> {
    let mut offset = 0;
    for line in text.lines() {
        if line.trim() == delimiter {
            return Some(offset);
        }
        offset += line.len();
        if offset < text.len() {
            if text.as_bytes()[offset] == b'\r' {
                offset += 1;
            }
            if offset < text.len() && text.as_bytes()[offset] == b'\n' {
                offset += 1;
            }
        }
    }
    None
}

fn skip_newline(input: &str, pos: usize) -> usize {
    if pos >= input.len() {
        return pos;
    }
    if input[pos..].starts_with("\r\n") {
        pos + 2
    } else if input[pos..].starts_with('\n') {
        pos + 1
    } else {
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_closing_delimiter_basic() {
        let text = "title: Hello\nauthor: World\n---\n";
        assert!(find_closing_delimiter(text, "---").is_some());
    }

    #[test]
    fn find_closing_delimiter_not_found() {
        let text = "title: Hello\nauthor: World\n";
        assert!(find_closing_delimiter(text, "---").is_none());
    }

    #[test]
    fn skip_newline_lf() {
        let input = "abc\ndef";
        assert_eq!(skip_newline(input, 3), 4);
    }

    #[test]
    fn skip_newline_crlf() {
        let input = "abc\r\ndef";
        assert_eq!(skip_newline(input, 3), 5);
    }

    #[test]
    fn skip_newline_none() {
        let input = "abcdef";
        assert_eq!(skip_newline(input, 3), 3);
    }

    #[test]
    fn skip_newline_at_end() {
        let input = "abc";
        assert_eq!(skip_newline(input, 3), 3);
    }

    #[test]
    fn extract_frontmatter_none() {
        assert!(extract_frontmatter("# Just content\n").is_none());
    }

    #[test]
    fn extract_frontmatter_yaml() {
        let input = "---\ntitle: Hello\n---\n\n# Content\n";
        let fm = extract_frontmatter(input).unwrap();
        assert!(matches!(fm.kind, FrontmatterKind::Yaml));
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
    }

    #[test]
    fn extract_frontmatter_toml() {
        let input = "+++\ntitle = \"Hello\"\n+++\n\n# Content\n";
        let fm = extract_frontmatter(input).unwrap();
        assert!(matches!(fm.kind, FrontmatterKind::Toml));
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
    }

    #[test]
    fn extract_frontmatter_json() {
        let input = ";;;\n{\"title\": \"Hello\"}\n;;;\n\n# Content\n";
        let fm = extract_frontmatter(input).unwrap();
        assert!(matches!(fm.kind, FrontmatterKind::Json));
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
    }
}
