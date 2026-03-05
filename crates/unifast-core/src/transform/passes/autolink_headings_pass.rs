use super::autolink_headings;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct AutolinkHeadingsPass;

impl Pass for AutolinkHeadingsPass {
    fn name(&self) -> &'static str {
        "autolink_headings"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.autolink_headings.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Hast(root) | AstPayload::Both { hast: root, .. } => {
                autolink_headings::apply_autolink_headings(
                    root,
                    ctx.options.autolink_headings.behavior,
                    ctx.id_gen,
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
        let pass = AutolinkHeadingsPass;
        assert_eq!(pass.name(), "autolink_headings");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
