#[must_use]
pub fn is_esm_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("import ") || trimmed.starts_with("export ")
}

fn count_nesting(text: &str, braces: &mut i32, brackets: &mut i32, parens: &mut i32) {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        let b = bytes[i];
        match b {
            b'"' | b'\'' | b'`' => {
                let quote = b;
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        i += 2; // skip escaped char
                        continue;
                    }
                    if bytes[i] == quote {
                        break;
                    }
                    i += 1;
                }
            }
            b'{' => *braces += 1,
            b'}' => *braces -= 1,
            b'[' => *brackets += 1,
            b']' => *brackets -= 1,
            b'(' => *parens += 1,
            b')' => *parens -= 1,
            _ => {}
        }
        i += 1;
    }
}

#[must_use]
pub fn is_esm_continuation(line: &str, prev_lines: &[String]) -> bool {
    if prev_lines.is_empty() {
        return false;
    }

    let mut braces: i32 = 0;
    let mut brackets: i32 = 0;
    let mut parens: i32 = 0;

    for prev in prev_lines {
        count_nesting(prev, &mut braces, &mut brackets, &mut parens);
    }

    if braces > 0 || brackets > 0 || parens > 0 {
        return true;
    }

    let line_trimmed = line.trim();
    if (line_trimmed.starts_with('}') || line_trimmed.contains("} from"))
        && prev_lines.iter().any(|l| l.contains('{'))
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_import() {
        assert!(is_esm_line("import { Button } from './Button'"));
        assert!(is_esm_line("import React from 'react'"));
    }

    #[test]
    fn detect_export() {
        assert!(is_esm_line("export const meta = {}"));
        assert!(is_esm_line("export default function App() {}"));
    }

    #[test]
    fn non_esm_lines() {
        assert!(!is_esm_line("# Heading"));
        assert!(!is_esm_line("Some paragraph text"));
        assert!(!is_esm_line("<Component />"));
        assert!(!is_esm_line(""));
    }

    #[test]
    fn multiline_import_continuation() {
        let prev = vec!["import {".to_string()];
        assert!(is_esm_continuation("  Button,", &prev));
    }

    #[test]
    fn multiline_import_closing_brace() {
        let prev = vec!["import {".to_string(), "  Button,".to_string()];
        assert!(is_esm_continuation("} from './ui'", &prev));
    }

    #[test]
    fn no_continuation_for_independent_line() {
        let prev = vec!["import React from 'react'".to_string()];
        assert!(!is_esm_continuation("# Hello", &prev));
    }

    #[test]
    fn multiline_export_with_array() {
        let prev = vec!["export const params = [".to_string()];
        assert!(is_esm_continuation("  { name: \"a\" },", &prev));

        let prev2 = vec![
            "export const params = [".to_string(),
            "  { name: \"a\" },".to_string(),
        ];
        assert!(is_esm_continuation("];", &prev2));
    }

    #[test]
    fn multiline_export_with_nested_objects_in_array() {
        let prev = vec![
            "export const params = [".to_string(),
            "  {".to_string(),
            "    name: \"options\",".to_string(),
            "    properties: [".to_string(),
            "      { name: \"a\", type: \"string\" },".to_string(),
        ];
        assert!(is_esm_continuation("    ],", &prev));
    }

    #[test]
    fn multiline_export_with_parens() {
        let prev = vec!["export const fn = (".to_string()];
        assert!(is_esm_continuation("  x,", &prev));

        let prev2 = vec!["export const fn = (".to_string(), "  x,".to_string()];
        assert!(is_esm_continuation(");", &prev2));
    }

    #[test]
    fn balanced_braces_no_continuation() {
        let prev = vec!["export const meta = { title: \"hi\" };".to_string()];
        assert!(!is_esm_continuation("# Heading", &prev));
    }

    #[test]
    fn brackets_inside_strings_are_ignored() {
        let prev = vec![
            "export const params = [".to_string(),
            "  {".to_string(),
            "    properties: [".to_string(),
            r#"      { name: "json", description: "Enable JSON frontmatter (opening {)" },"#
                .to_string(),
            "    ],".to_string(),
            "  },".to_string(),
            "];".to_string(),
        ];
        assert!(!is_esm_continuation("## Heading", &prev));
    }

    #[test]
    fn escaped_quotes_in_strings() {
        let prev = vec![r#"export const x = [{ desc: "a \"quoted\" {value}" }];"#.to_string()];
        assert!(!is_esm_continuation("# Next", &prev));
    }
}
