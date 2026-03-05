use super::toc::generate_toc;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct TocPass;

impl Pass for TocPass {
    fn name(&self) -> &'static str {
        "toc"
    }

    fn phase(&self) -> Phase {
        Phase::Transform
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.toc.enabled {
            return Ok(());
        }
        let max_depth = ctx.options.toc.max_depth;
        match ast {
            AstPayload::Mdast(doc) => {
                ctx.toc = generate_toc(doc, max_depth);
                Ok(())
            }
            AstPayload::Both { mdast, .. } => {
                ctx.toc = generate_toc(mdast, max_depth);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let pass = TocPass;
        assert_eq!(pass.name(), "toc");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
