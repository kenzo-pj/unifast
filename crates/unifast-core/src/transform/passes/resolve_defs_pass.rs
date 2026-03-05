use super::resolve_defs;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
use std::collections::HashMap;

pub struct ResolveDefsPass;

impl Pass for ResolveDefsPass {
    fn name(&self) -> &'static str {
        "resolve_defs"
    }

    fn phase(&self) -> Phase {
        Phase::Transform
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let defs = HashMap::new();
        match ast {
            AstPayload::Mdast(doc) => {
                resolve_defs::resolve_definitions(doc, &defs, ctx.diagnostics);
                Ok(())
            }
            AstPayload::Both { mdast, .. } => {
                resolve_defs::resolve_definitions(mdast, &defs, ctx.diagnostics);
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
        let pass = ResolveDefsPass;
        assert_eq!(pass.name(), "resolve_defs");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
