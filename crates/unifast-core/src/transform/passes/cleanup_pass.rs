use super::html_cleanup::{self, CleanupOptions};
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct CleanupPass;

impl Pass for CleanupPass {
    fn name(&self) -> &'static str {
        "html_cleanup"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, _ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        match ast {
            AstPayload::Hast(root) => {
                html_cleanup::cleanup(root, &CleanupOptions::default());
                Ok(())
            }
            AstPayload::Both { hast, .. } => {
                html_cleanup::cleanup(hast, &CleanupOptions::default());
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
        let pass = CleanupPass;
        assert_eq!(pass.name(), "html_cleanup");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
