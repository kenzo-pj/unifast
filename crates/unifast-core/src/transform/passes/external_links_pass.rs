use super::external_links;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct ExternalLinksPass;

impl Pass for ExternalLinksPass {
    fn name(&self) -> &'static str {
        "external_links"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.external_links.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Hast(root) | AstPayload::Both { hast: root, .. } => {
                external_links::apply_external_links(
                    root,
                    &ctx.options.external_links.rel,
                    ctx.options.external_links.target.as_deref(),
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
        let pass = ExternalLinksPass;
        assert_eq!(pass.name(), "external_links");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
