#[must_use]
pub fn is_esm_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("import ") || trimmed.starts_with("export ")
}

#[must_use]
pub fn is_esm_continuation(line: &str, prev_lines: &[String]) -> bool {
    let prev = match prev_lines.last() {
        Some(s) => s.as_str(),
        None => return false,
    };
    let prev_trimmed = prev.trim();
    let line_trimmed = line.trim();

    if prev_trimmed.ends_with(',') || prev_trimmed.ends_with('{') {
        return true;
    }

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
}
