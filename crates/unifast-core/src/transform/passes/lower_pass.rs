use crate::ast::common::Span;
use crate::ast::hast::nodes::{HNode, HRoot};
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
use super::mdast_to_hast;

pub struct LowerPass;

impl Pass for LowerPass {
    fn name(&self) -> &'static str {
        "mdast_to_hast"
    }

    fn phase(&self) -> Phase {
        Phase::Lower
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let doc = match ast {
            AstPayload::Mdast(doc) => doc,
            AstPayload::Both { mdast, .. } => mdast,
            AstPayload::Hast(_) => return Ok(()),
        };
        let hast_node = mdast_to_hast::lower(doc, ctx.id_gen, ctx.options.raw_html, ctx.diagnostics);
        let hast_root = match hast_node {
            HNode::Root(root) => root,
            other => HRoot {
                id: ctx.id_gen.next_id(),
                span: Span::empty(),
                children: vec![other],
            },
        };
        *ast = AstPayload::Hast(hast_root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let pass = LowerPass;
        assert_eq!(pass.name(), "mdast_to_hast");
        assert_eq!(pass.phase(), Phase::Lower);
    }
}
