use super::{FrontmatterKind, FrontmatterResult, find_closing_delimiter, skip_newline};
use std::collections::HashMap;

/// Extract YAML frontmatter delimited by `---`.
///
/// The opening `---` must be the very first characters in the input, followed
/// immediately by a newline.  The closing `---` must appear on its own line.
pub fn extract(input: &str) -> Option<FrontmatterResult> {
    // Must start with "---" followed by a newline.
    if !input.starts_with("---") {
        return None;
    }
    let after_open = &input[3..];
    if !after_open.starts_with('\n') && !after_open.starts_with("\r\n") {
        return None;
    }
    let content_start = 3 + if after_open.starts_with("\r\n") { 2 } else { 1 };

    // Find closing ---
    let rest = &input[content_start..];
    let close_pos = find_closing_delimiter(rest, "---")?;

    let raw = rest[..close_pos].to_string();
    let end_offset = content_start + close_pos + 3; // skip closing ---
    let end_offset = skip_newline(input, end_offset);

    // Parse YAML
    let data = parse_yaml_to_map(&raw)?;

    Some(FrontmatterResult {
        kind: FrontmatterKind::Yaml,
        data,
        raw,
        end_offset,
    })
}

fn parse_yaml_to_map(yaml_str: &str) -> Option<HashMap<String, serde_json::Value>> {
    // Empty YAML parses to Null — treat as empty map.
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_str).ok()?;
    if value.is_null() {
        return Some(HashMap::new());
    }
    let json_value = yaml_to_json(value);
    match json_value {
        serde_json::Value::Object(map) => Some(map.into_iter().collect()),
        _ => None, // frontmatter must be a mapping
    }
}

fn yaml_to_json(yaml: serde_yaml::Value) -> serde_json::Value {
    match yaml {
        serde_yaml::Value::Null => serde_json::Value::Null,
        serde_yaml::Value::Bool(b) => serde_json::Value::Bool(b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(u) = n.as_u64() {
                serde_json::Value::Number(u.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::json!(f)
            } else {
                serde_json::Value::Null
            }
        }
        serde_yaml::Value::String(s) => serde_json::Value::String(s),
        serde_yaml::Value::Sequence(seq) => {
            serde_json::Value::Array(seq.into_iter().map(yaml_to_json).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        serde_yaml::Value::String(s) => s,
                        other => format!("{other:?}"),
                    };
                    (key, yaml_to_json(v))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        serde_yaml::Value::Tagged(tagged) => yaml_to_json(tagged.value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_yaml() {
        let input = "---\ntitle: Hello\nauthor: World\n---\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data.get("title").unwrap(), "Hello");
        assert_eq!(fm.data.get("author").unwrap(), "World");
        assert_eq!(fm.end_offset, input.len());
    }

    #[test]
    fn yaml_with_types() {
        let input = "---\ntitle: Test\ncount: 42\ntags:\n  - a\n  - b\n---\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["count"], 42);
        assert!(fm.data["tags"].is_array());
        let tags = fm.data["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0], "a");
        assert_eq!(tags[1], "b");
    }

    #[test]
    fn empty_yaml() {
        let input = "---\n---\n";
        let fm = extract(input).unwrap();
        assert!(fm.data.is_empty());
    }

    #[test]
    fn no_opening() {
        assert!(extract("# Hello\n").is_none());
    }

    #[test]
    fn no_closing() {
        assert!(extract("---\ntitle: Hello\n").is_none());
    }

    #[test]
    fn opening_without_newline() {
        // "---title" should not be treated as frontmatter
        assert!(extract("---title\n---\n").is_none());
    }

    #[test]
    fn yaml_nested_mapping() {
        let input = "---\nmeta:\n  key: value\n  num: 10\n---\n";
        let fm = extract(input).unwrap();
        let meta = fm.data.get("meta").unwrap();
        assert!(meta.is_object());
        assert_eq!(meta["key"], "value");
        assert_eq!(meta["num"], 10);
    }

    #[test]
    fn end_offset_content_after() {
        let input = "---\ntitle: Test\n---\n\n# Heading\n";
        let fm = extract(input).unwrap();
        // end_offset should point right after the closing --- and its newline
        let remaining = &input[fm.end_offset..];
        assert_eq!(remaining, "\n# Heading\n");
    }

    #[test]
    fn yaml_boolean_values() {
        let input = "---\ndraft: true\npublished: false\n---\n";
        let fm = extract(input).unwrap();
        assert_eq!(fm.data["draft"], true);
        assert_eq!(fm.data["published"], false);
    }
}
