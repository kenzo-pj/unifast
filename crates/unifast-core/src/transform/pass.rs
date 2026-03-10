use crate::api::options::CompileOptions;
use crate::api::result::ReadingTime;
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
}

impl AstPayload {
    pub const fn mdast_mut(&mut self) -> Option<&mut Document> {
        match self {
            Self::Mdast(doc) => Some(doc),
            Self::Hast(_) => None,
        }
    }

    pub const fn hast_mut(&mut self) -> Option<&mut HRoot> {
        match self {
            Self::Hast(root) => Some(root),
            Self::Mdast(_) => None,
        }
    }
}

pub struct PassContext<'a> {
    pub source: &'a str,
    pub diagnostics: &'a mut DiagnosticSink,
    pub options: &'a CompileOptions,
    pub id_gen: &'a mut NodeIdGen,
    pub toc: Vec<TocEntry>,
    pub reading_time: Option<ReadingTime>,
    pub excerpt: Option<String>,
}

pub type PassResult = Result<(), PassError>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct PassError(pub String);

impl PassError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

pub trait Pass: Send + Sync {
    fn name(&self) -> &'static str;
    fn phase(&self) -> Phase;
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult;
}

pub type PassFnPtr = fn(&mut PassContext, &mut AstPayload) -> PassResult;

pub(crate) struct FnPtrPass {
    pub name: &'static str,
    pub phase: Phase,
    pub run_fn: PassFnPtr,
}

impl Pass for FnPtrPass {
    fn name(&self) -> &'static str {
        self.name
    }
    fn phase(&self) -> Phase {
        self.phase
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        (self.run_fn)(ctx, ast)
    }
}

type PassFn = Box<dyn Fn(&mut PassContext, &mut AstPayload) -> PassResult + Send + Sync>;

pub(crate) struct FnPass {
    pub name: &'static str,
    pub phase: Phase,
    pub run_fn: PassFn,
}

impl Pass for FnPass {
    fn name(&self) -> &'static str {
        self.name
    }
    fn phase(&self) -> Phase {
        self.phase
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        (self.run_fn)(ctx, ast)
    }
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
        assert_eq!(err.0, "something broke");
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
            reading_time: None,
            excerpt: None,
        };
        assert_eq!(ctx.source, "# Hello");
    }

    #[test]
    fn ast_payload_mdast_mut() {
        use crate::ast::common::{NodeId, Span};
        let mut payload = AstPayload::Mdast(Document {
            id: NodeId(0),
            span: Span::empty(),
            children: vec![],
        });
        assert!(payload.mdast_mut().is_some());
        assert!(payload.hast_mut().is_none());
    }

    #[test]
    fn ast_payload_hast_mut() {
        use crate::ast::common::{NodeId, Span};
        let mut payload = AstPayload::Hast(HRoot {
            id: NodeId(0),
            span: Span::empty(),
            children: vec![],
        });
        assert!(payload.hast_mut().is_some());
        assert!(payload.mdast_mut().is_none());
    }

    #[test]
    fn fn_pass_works() {
        let pass = FnPass {
            name: "test",
            phase: Phase::Transform,
            run_fn: Box::new(|_ctx, _ast| Ok(())),
        };
        assert_eq!(pass.name(), "test");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
