use super::sectionize;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct SectionizePass;

impl Pass for SectionizePass {
    fn name(&self) -> &'static str {
        "sectionize"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.sectionize.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Hast(root) | AstPayload::Both { hast: root, .. } => {
                sectionize::apply_sectionize(root, ctx.id_gen);
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
        let pass = SectionizePass;
        assert_eq!(pass.name(), "sectionize");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
