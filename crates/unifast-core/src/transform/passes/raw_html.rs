use crate::api::options::RawHtmlPolicy;
use crate::ast::hast::nodes::HNode;
use crate::diagnostics::sink::DiagnosticSink;

pub const fn process_raw_html(
    _root: &mut HNode,
    _policy: RawHtmlPolicy,
    _diagnostics: &mut DiagnosticSink,
) {
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
