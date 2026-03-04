use super::{FrontmatterKind, FrontmatterResult, find_closing_delimiter, skip_newline};
use std::collections::HashMap;

/// Extract JSON frontmatter delimited by `;;;`.
///
/// The opening `;;;` must be the very first characters in the input, followed
/// immediately by a newline.  The closing `;;;` must appear on its own line.
/// The content between the delimiters must be a valid JSON object.
pub fn extract(input: &str) -> Option<FrontmatterResult> {
    if !input.starts_with(";;;") {
        return None;
    }
    let after_open = &input[3..];
    if !after_open.starts_with('\n') && !after_open.starts_with("\r\n") {
        return None;
    }
    let content_start = 3 + if after_open.starts_with("\r\n") { 2 } else { 1 };

    let rest = &input[content_start..];
    let close_pos = find_closing_delimiter(rest, ";;;")?;

    let raw = rest[..close_pos].to_string();
    let end_offset = content_start + close_pos + 3;
    let end_offset = skip_newline(input, end_offset);

    let data = parse_json_to_map(&raw)?;

    Some(FrontmatterResult {
        kind: FrontmatterKind::Json,
        data,
        raw,
        end_offset,
    })
}

fn parse_json_to_map(json_str: &str) -> Option<HashMap<String, serde_json::Value>> {
    if json_str.trim().is_empty() {
        return Some(HashMap::new());
    }
    let value: serde_json::Value = serde_json::from_str(json_str).ok()?;
    match value {
        serde_json::Value::Object(map) => Some(map.into_iter().collect()),
        _ => None, // frontmatter must be an object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_json() {
        let input = ";;;\n{\"title\": \"Hello\", \"author\": \"World\"}\n;;;\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
        assert_eq!(fm.data.get("author").unwrap(), "World");
    }

    #[test]
    fn json_with_types() {
        let input = ";;;\n{\"title\": \"Test\", \"count\": 42, \"tags\": [\"a\", \"b\"]}\n;;;\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["count"], 42);
        assert!(fm.data["tags"].is_array());
    }

    #[test]
    fn empty_json() {
        let input = ";;;\n{}\n;;;\n";
        let fm = extract(input).unwrap();
        assert!(fm.data.is_empty());
    }

    #[test]
    fn no_opening() {
        assert!(extract("# Hello\n").is_none());
    }

    #[test]
    fn no_closing() {
        assert!(extract(";;;\n{\"title\": \"Hello\"}\n").is_none());
    }

    #[test]
    fn json_nested_object() {
        let input = ";;;\n{\"meta\": {\"key\": \"value\", \"num\": 10}}\n;;;\n";
        let fm = extract(input).unwrap();
        let meta = fm.data.get("meta").unwrap();
        assert!(meta.is_object());
        assert_eq!(meta["key"], "value");
        assert_eq!(meta["num"], 10);
    }

    #[test]
    fn json_boolean() {
        let input = ";;;\n{\"draft\": true, \"published\": false}\n;;;\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["draft"], true);
        assert_eq!(fm.data["published"], false);
    }

    #[test]
    fn completely_empty_content() {
        let input = ";;;\n;;;\n";
        let fm = extract(input).unwrap();
        assert!(fm.data.is_empty());
    }

    #[test]
    fn json_multiline() {
        let input = ";;;\n{\n  \"title\": \"Hello\",\n  \"count\": 42\n}\n;;;\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
        assert_eq!(fm.data["count"], 42);
    }
}
