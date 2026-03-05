use super::highlight::{self, HighlightEngine, SyntectHighlighter, TreeSitterHighlighter};
use crate::api::options::HighlightEngine as EngineOption;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct HighlightPass;

impl Pass for HighlightPass {
    fn name(&self) -> &'static str {
        "highlight"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let engine: Box<dyn HighlightEngine> = match ctx.options.highlight.engine {
            EngineOption::Syntect => Box::new(SyntectHighlighter::new()),
            EngineOption::TreeSitter => Box::new(TreeSitterHighlighter),
            EngineOption::None => return Ok(()),
        };
        match ast {
            AstPayload::Hast(root) => {
                highlight::apply_highlight(root, engine.as_ref(), ctx.id_gen);
                Ok(())
            }
            AstPayload::Both { hast, .. } => {
                highlight::apply_highlight(hast, engine.as_ref(), ctx.id_gen);
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
