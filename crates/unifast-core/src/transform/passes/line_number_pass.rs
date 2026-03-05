use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
use super::line_number;

pub struct LineNumberPass;

impl Pass for LineNumberPass {
    fn name(&self) -> &'static str {
        "line_numbers"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        match ast {
            AstPayload::Hast(root) => {
                line_number::apply_line_numbers(root, ctx.id_gen);
                Ok(())
            }
            AstPayload::Both { hast, .. } => {
                line_number::apply_line_numbers(hast, ctx.id_gen);
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
        let pass = LineNumberPass;
        assert_eq!(pass.name(), "line_numbers");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
