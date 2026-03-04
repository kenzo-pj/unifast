use super::diagnostic::{DiagLevel, Diagnostic};
use crate::util::line_index::LineIndex;

pub fn render_compact(diag: &Diagnostic, line_index: &LineIndex) -> String {
    let pos = line_index.line_col(diag.span.start);
    let level = match diag.level {
        DiagLevel::Error => "error",
        DiagLevel::Warning => "warning",
    };
    format!("{}:{}:{}: {}", level, pos.line, pos.column, diag.message)
}

pub fn render_verbose(diag: &Diagnostic, source: &str, line_index: &LineIndex) -> String {
    let pos = line_index.line_col(diag.span.start);
    let level = match diag.level {
        DiagLevel::Error => "error",
        DiagLevel::Warning => "warning",
    };
    let mut out = format!(
        "{}{}: {}\n",
        level,
        diag.code
            .as_ref()
            .map(|c| format!("[{}]", c))
            .unwrap_or_default(),
        diag.message
    );
    out.push_str(&format!("  --> {}:{}\n", pos.line, pos.column));
    // Extract source line
    let line_start = source[..diag.span.start as usize]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = source[diag.span.start as usize..]
        .find('\n')
        .map(|i| i + diag.span.start as usize)
        .unwrap_or(source.len());
    let line_text = &source[line_start..line_end];
    out.push_str(&format!("  | {}\n", line_text));
    for note in &diag.notes {
        out.push_str(&format!("  = note: {}\n", note));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::Span;

    #[test]
    fn render_compact_error() {
        let source = "hello\nworld";
        let line_index = LineIndex::new(source);
        let diag = Diagnostic::error("unexpected token", Span::new(6, 11));
        let rendered = render_compact(&diag, &line_index);
        assert_eq!(rendered, "error:2:1: unexpected token");
    }

    #[test]
    fn render_compact_warning() {
        let source = "abc";
        let line_index = LineIndex::new(source);
        let diag = Diagnostic::warning("unused variable", Span::new(1, 2));
        let rendered = render_compact(&diag, &line_index);
        assert_eq!(rendered, "warning:1:2: unused variable");
    }

    #[test]
    fn render_verbose_with_source_excerpt() {
        let source = "let x = 42;\nlet y = ;";
        let line_index = LineIndex::new(source);
        let diag = Diagnostic::error("expected expression", Span::new(20, 21))
            .with_code("E001")
            .with_note("expressions must have a value");
        let rendered = render_verbose(&diag, source, &line_index);
        assert!(rendered.contains("error[E001]: expected expression"));
        assert!(rendered.contains("  --> 2:9"));
        assert!(rendered.contains("  | let y = ;"));
        assert!(rendered.contains("  = note: expressions must have a value"));
    }

    #[test]
    fn render_verbose_no_code_no_notes() {
        let source = "foo bar";
        let line_index = LineIndex::new(source);
        let diag = Diagnostic::warning("something", Span::new(0, 3));
        let rendered = render_verbose(&diag, source, &line_index);
        assert!(rendered.starts_with("warning: something\n"));
        assert!(!rendered.contains("note:"));
    }
}
