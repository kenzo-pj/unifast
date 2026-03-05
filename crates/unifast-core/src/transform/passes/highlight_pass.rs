use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
use super::highlight::{self, SyntectHighlighter};

pub struct HighlightPass;

impl Pass for HighlightPass {
    fn name(&self) -> &'static str {
        "highlight"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let engine = SyntectHighlighter::new();
        match ast {
            AstPayload::Hast(root) => {
                highlight::apply_highlight(root, &engine, ctx.id_gen);
                Ok(())
            }
            AstPayload::Both { hast, .. } => {
                highlight::apply_highlight(hast, &engine, ctx.id_gen);
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
        let pass = HighlightPass;
        assert_eq!(pass.name(), "highlight");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
