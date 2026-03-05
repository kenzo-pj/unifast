use super::normalize;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct NormalizePass;

impl Pass for NormalizePass {
    fn name(&self) -> &'static str {
        "normalize"
    }

    fn phase(&self) -> Phase {
        Phase::Transform
    }

    fn run(&self, _ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        match ast {
            AstPayload::Mdast(doc) => {
                normalize::normalize(doc);
                Ok(())
            }
            AstPayload::Both { mdast, .. } => {
                normalize::normalize(mdast);
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
        let pass = NormalizePass;
        assert_eq!(pass.name(), "normalize");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
