use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
use super::sanitize;

pub struct SanitizePass;

impl Pass for SanitizePass {
    fn name(&self) -> &'static str {
        "sanitize"
    }

    fn phase(&self) -> Phase {
        Phase::Optimize
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let schema = if let Some(ref api_schema) = ctx.options.sanitize.schema {
            sanitize::from_api_schema(api_schema)
        } else {
            sanitize::default_safe_schema()
        };
        match ast {
            AstPayload::Hast(root) => {
                sanitize::sanitize(root, &schema, ctx.diagnostics);
                Ok(())
            }
            AstPayload::Both { hast, .. } => {
                sanitize::sanitize(hast, &schema, ctx.diagnostics);
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
        let pass = SanitizePass;
        assert_eq!(pass.name(), "sanitize");
        assert_eq!(pass.phase(), Phase::Optimize);
    }
}
