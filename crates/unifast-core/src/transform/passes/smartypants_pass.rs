use super::smartypants;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct SmartypantsPass;

impl Pass for SmartypantsPass {
    fn name(&self) -> &'static str {
        "smartypants"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.smartypants.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                smartypants::apply_smartypants(
                    doc,
                    ctx.options.smartypants.quotes,
                    ctx.options.smartypants.dashes,
                    ctx.options.smartypants.ellipses,
                );
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
        let pass = SmartypantsPass;
        assert_eq!(pass.name(), "smartypants");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
