use super::{FrontmatterKind, FrontmatterResult, find_closing_delimiter, skip_newline};
use std::collections::HashMap;

#[must_use]
pub fn extract(input: &str) -> Option<FrontmatterResult> {
    if !input.starts_with("+++") {
        return None;
    }
    let after_open = &input[3..];
    if !after_open.starts_with('\n') && !after_open.starts_with("\r\n") {
        return None;
    }
    let content_start = 3 + if after_open.starts_with("\r\n") { 2 } else { 1 };

    let rest = &input[content_start..];
    let close_pos = find_closing_delimiter(rest, "+++")?;

    let raw = rest[..close_pos].to_string();
    let end_offset = content_start + close_pos + 3;
    let end_offset = skip_newline(input, end_offset);

    let data = parse_toml_to_map(&raw)?;

    Some(FrontmatterResult {
        kind: FrontmatterKind::Toml,
        data,
        raw,
        end_offset,
    })
}

fn parse_toml_to_map(toml_str: &str) -> Option<HashMap<String, serde_json::Value>> {
    if toml_str.trim().is_empty() {
        return Some(HashMap::new());
    }
    let table: toml::Table = toml::from_str(toml_str).ok()?;
    let json_value = toml_to_json(toml::Value::Table(table));
    match json_value {
        serde_json::Value::Object(map) => Some(map.into_iter().collect()),
        _ => None,
    }
}

fn toml_to_json(toml_val: toml::Value) -> serde_json::Value {
    match toml_val {
        toml::Value::String(s) => serde_json::Value::String(s),
        toml::Value::Integer(i) => serde_json::Value::Number(i.into()),
        toml::Value::Float(f) => serde_json::json!(f),
        toml::Value::Boolean(b) => serde_json::Value::Bool(b),
        toml::Value::Datetime(dt) => serde_json::Value::String(dt.to_string()),
        toml::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(toml_to_json).collect())
        }
        toml::Value::Table(table) => {
            let obj: serde_json::Map<String, serde_json::Value> = table
                .into_iter()
                .map(|(k, v)| (k, toml_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_toml() {
        let input = "+++\ntitle = \"Hello\"\nauthor = \"World\"\n+++\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
        assert_eq!(fm.data.get("author").unwrap(), "World");
    }

    #[test]
    fn toml_with_types() {
        let input = "+++\ntitle = \"Test\"\ncount = 42\ntags = [\"a\", \"b\"]\n+++\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["count"], 42);
        assert!(fm.data["tags"].is_array());
    }

    #[test]
    fn empty_toml() {
        let input = "+++\n+++\n";
        let fm = extract(input).unwrap();
        assert!(fm.data.is_empty());
    }

    #[test]
    fn no_opening() {
        assert!(extract("# Hello\n").is_none());
    }

    #[test]
    fn no_closing() {
        assert!(extract("+++\ntitle = \"Hello\"\n").is_none());
    }

    #[test]
    fn toml_nested_table() {
        let input = "+++\n[meta]\nkey = \"value\"\nnum = 10\n+++\n";
        let fm = extract(input).unwrap();
        let meta = fm.data.get("meta").unwrap();
        assert!(meta.is_object());
        assert_eq!(meta["key"], "value");
        assert_eq!(meta["num"], 10);
    }

    #[test]
    fn toml_boolean() {
        let input = "+++\ndraft = true\npublished = false\n+++\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["draft"], true);
        assert_eq!(fm.data["published"], false);
    }
}
