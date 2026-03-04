use super::printer::SourceMapping;

/// Generate a basic JSON source-map (v3) from collected source mappings.
///
/// The returned string is a self-contained JSON object suitable for writing to
/// a `.map` file or inlining as a data URI.
pub fn generate_sourcemap(file: &str, source_content: &str, mappings: &[SourceMapping]) -> String {
    let mappings_str = encode_mappings(mappings);
    let source_file = file.replace(".js", ".mdx");
    let escaped_content = escape_json_string(source_content);
    format!(
        r#"{{"version":3,"file":"{file}","sources":["{source_file}"],"sourcesContent":["{escaped_content}"],"mappings":"{mappings_str}"}}"#,
    )
}

/// Encode source mappings into a simplified VLQ-encoded mappings string.
///
/// Each mapping becomes a VLQ segment; lines are separated by `;` and segments
/// within a line by `,`.
fn encode_mappings(mappings: &[SourceMapping]) -> String {
    if mappings.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut current_line: u32 = 1;

    for mapping in mappings {
        // Add semicolons for any line gap.
        while current_line < mapping.generated_line {
            result.push(';');
            current_line += 1;
        }
        // Separate segments on the same line.
        if !result.is_empty() && !result.ends_with(';') {
            result.push(',');
        }
        result.push_str(&vlq_encode(i64::from(mapping.generated_column)));
    }

    result
}

/// Encode a single value using Base64 VLQ.
fn vlq_encode(value: i64) -> String {
    let mut result = String::new();
    let mut v = if value < 0 {
        ((-value) << 1) | 1
    } else {
        value << 1
    };
    loop {
        let mut digit = (v & 0x1F) as u8;
        v >>= 5;
        if v > 0 {
            digit |= 0x20;
        }
        result.push(VLQ_CHARS[digit as usize] as char);
        if v == 0 {
            break;
        }
    }
    result
}

const VLQ_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Escape a string so it can be embedded inside a JSON string literal.
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_mappings() {
        assert_eq!(encode_mappings(&[]), "");
    }

    #[test]
    fn single_mapping() {
        let mappings = vec![SourceMapping {
            generated_line: 1,
            generated_column: 0,
            original_offset: 0,
        }];
        let encoded = encode_mappings(&mappings);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn multiple_lines() {
        let mappings = vec![
            SourceMapping {
                generated_line: 1,
                generated_column: 0,
                original_offset: 0,
            },
            SourceMapping {
                generated_line: 3,
                generated_column: 4,
                original_offset: 10,
            },
        ];
        let encoded = encode_mappings(&mappings);
        // Should have at least one semicolon for the line gap.
        assert!(encoded.contains(';'));
    }

    #[test]
    fn vlq_zero() {
        assert_eq!(vlq_encode(0), "A");
    }

    #[test]
    fn vlq_positive() {
        // 1 => encoded as 1<<1 = 2, digit=2 => 'E' (but full VLQ check)
        let encoded = vlq_encode(1);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn vlq_negative() {
        let encoded = vlq_encode(-1);
        assert!(!encoded.is_empty());
        // Negative values set the sign bit
        assert_ne!(encoded, vlq_encode(1));
    }

    #[test]
    fn generate_sourcemap_basic() {
        let map = generate_sourcemap("output.js", "# Hello\n", &[]);
        assert!(map.contains("\"version\":3"));
        assert!(map.contains("output.js"));
        assert!(map.contains("output.mdx"));
    }

    #[test]
    fn generate_sourcemap_with_content() {
        let map = generate_sourcemap("out.js", "hello\nworld\n", &[]);
        assert!(map.contains("hello\\nworld\\n"));
    }

    #[test]
    fn escape_json_handles_special_chars() {
        let escaped = escape_json_string("a\"b\\c\nd\re\tf");
        assert_eq!(escaped, "a\\\"b\\\\c\\nd\\re\\tf");
    }
}
