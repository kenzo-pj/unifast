use super::slug;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct SlugPass;

impl Pass for SlugPass {
    fn name(&self) -> &'static str {
        "slug"
    }

    fn phase(&self) -> Phase {
        Phase::Transform
    }

    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        let mode = match ctx.options.slug.mode {
            crate::api::options::SlugMode::GitHub => slug::SlugMode::GitHub,
            crate::api::options::SlugMode::Unicode => slug::SlugMode::Unicode,
        };
        match ast {
            AstPayload::Mdast(doc) => {
                slug::apply_slugs(doc, mode);
                Ok(())
            }
            AstPayload::Both { mdast, .. } => {
                slug::apply_slugs(mdast, mode);
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
        let pass = SlugPass;
        assert_eq!(pass.name(), "slug");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
