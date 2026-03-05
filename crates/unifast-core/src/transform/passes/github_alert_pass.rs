use super::github_alert;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct GithubAlertPass;

impl Pass for GithubAlertPass {
    fn name(&self) -> &'static str {
        "github_alert"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.github_alert.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                github_alert::apply_github_alerts(doc, ctx.id_gen);
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
        let pass = GithubAlertPass;
        assert_eq!(pass.name(), "github_alert");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
