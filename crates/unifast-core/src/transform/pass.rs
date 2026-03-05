use crate::api::options::CompileOptions;
use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::HRoot;
use crate::ast::mdast::nodes::Document;
use crate::diagnostics::sink::DiagnosticSink;
use crate::transform::passes::toc::TocEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    Parse = 0,
    Transform = 1,
    Lower = 2,
    Optimize = 3,
    Emit = 4,
}

pub enum AstPayload {
    Mdast(Document),
    Hast(HRoot),
    Both { mdast: Document, hast: HRoot },
}

pub struct PassContext<'a> {
    pub source: &'a str,
    pub diagnostics: &'a mut DiagnosticSink,
    pub options: &'a CompileOptions,
    pub id_gen: &'a mut NodeIdGen,
    pub toc: Vec<TocEntry>,
}

pub type PassResult = Result<(), PassError>;

#[derive(Debug)]
pub struct PassError {
    pub message: String,
}

impl PassError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl std::fmt::Display for PassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PassError {}

pub trait Pass: Send + Sync {
    fn name(&self) -> &'static str;
    fn phase(&self) -> Phase;
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_ordering() {
        assert!(Phase::Parse < Phase::Transform);
        assert!(Phase::Transform < Phase::Lower);
        assert!(Phase::Lower < Phase::Optimize);
        assert!(Phase::Optimize < Phase::Emit);
    }

    #[test]
    fn pass_error_display() {
        let err = PassError::new("something broke");
        assert_eq!(err.message, "something broke");
        assert_eq!(format!("{err}"), "something broke");
    }

    #[test]
    fn pass_error_is_error_trait() {
        let err = PassError::new("test error");
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn pass_context_construction() {
        use crate::api::options::CompileOptions;
        use crate::ast::common::NodeIdGen;
        use crate::diagnostics::sink::DiagnosticSink;

        let source = "# Hello";
        let mut diag = DiagnosticSink::new();
        let opts = CompileOptions::default();
        let mut id_gen = NodeIdGen::new();
        let ctx = PassContext {
            source,
            diagnostics: &mut diag,
            options: &opts,
            id_gen: &mut id_gen,
            toc: Vec::new(),
        };
        assert_eq!(ctx.source, "# Hello");
    }
}
