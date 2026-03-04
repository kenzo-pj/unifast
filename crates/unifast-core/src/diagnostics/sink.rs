use super::diagnostic::{DiagLevel, Diagnostic};
use crate::ast::common::Span;

pub struct DiagnosticSink {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSink {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn error(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic::error(message, span));
    }

    pub fn warn(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic::warning(message, span));
    }

    pub fn push(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagLevel::Error)
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

impl Default for DiagnosticSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sink_collects_diagnostics() {
        let mut sink = DiagnosticSink::new();
        assert!(sink.is_empty());

        sink.error("something went wrong", Span::new(0, 5));
        sink.warn("be careful", Span::new(10, 15));

        assert_eq!(sink.diagnostics().len(), 2);
        assert!(!sink.is_empty());
    }

    #[test]
    fn sink_has_errors() {
        let mut sink = DiagnosticSink::new();
        sink.warn("just a warning", Span::new(0, 1));
        assert!(!sink.has_errors());

        sink.error("an error", Span::new(0, 1));
        assert!(sink.has_errors());
    }

    #[test]
    fn sink_into_diagnostics() {
        let mut sink = DiagnosticSink::new();
        sink.error("err", Span::new(0, 1));
        let diags = sink.into_diagnostics();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].level, DiagLevel::Error);
    }

    #[test]
    fn sink_push_custom_diagnostic() {
        let mut sink = DiagnosticSink::new();
        let diag = Diagnostic::error("custom", Span::new(0, 5))
            .with_code("E001")
            .with_note("see docs");
        sink.push(diag);
        assert_eq!(sink.diagnostics().len(), 1);
        assert_eq!(sink.diagnostics()[0].code, Some("E001".to_string()));
    }
}
