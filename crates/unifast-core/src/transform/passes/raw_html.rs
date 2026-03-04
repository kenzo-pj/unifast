use crate::api::options::RawHtmlPolicy;
use crate::ast::hast::nodes::*;
use crate::diagnostics::sink::DiagnosticSink;

/// Post-lowering pass to handle raw HTML nodes in the HAst.
///
/// - `Disallow`: All raw nodes should have been converted to text during lowering.
/// - `AllowDangerous`: Raw nodes are kept as-is.
/// - `ParseAndSanitize`: Raw nodes will be sanitized by the sanitize pass later.
///
/// This pass is currently a no-op placeholder for future HTML parsing logic.
pub fn process_raw_html(
    _root: &mut HNode,
    _policy: RawHtmlPolicy,
    _diagnostics: &mut DiagnosticSink,
) {
    // Intentionally empty: raw HTML handling is done during lowering
    // and sanitization will be performed by a separate pass.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{NodeIdGen, Span};
    use crate::ast::hast::builder::HBuilder;

    #[test]
    fn process_raw_html_disallow_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);
        let text = b.text(Span::new(0, 5), "hello");
        let mut root = b.root(Span::new(0, 5), vec![text]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(&mut root, RawHtmlPolicy::Disallow, &mut diagnostics);

        assert!(diagnostics.is_empty());
        assert_eq!(root.children().unwrap().len(), 1);
    }

    #[test]
    fn process_raw_html_allow_dangerous_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);
        let raw = b.raw(Span::new(0, 10), "<div></div>");
        let mut root = b.root(Span::new(0, 10), vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(&mut root, RawHtmlPolicy::AllowDangerous, &mut diagnostics);

        assert!(diagnostics.is_empty());
        assert_eq!(root.children().unwrap().len(), 1);
    }

    #[test]
    fn process_raw_html_parse_and_sanitize_is_noop() {
        let mut id_gen = NodeIdGen::new();
        let mut b = HBuilder::new(&mut id_gen);
        let raw = b.raw(Span::new(0, 20), "<script>x</script>");
        let mut root = b.root(Span::new(0, 20), vec![raw]);
        let mut diagnostics = DiagnosticSink::new();

        process_raw_html(&mut root, RawHtmlPolicy::ParseAndSanitize, &mut diagnostics);

        assert!(diagnostics.is_empty());
        assert_eq!(root.children().unwrap().len(), 1);
    }
}
