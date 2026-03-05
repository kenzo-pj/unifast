use super::emoji;
use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

pub struct EmojiPass;

impl Pass for EmojiPass {
    fn name(&self) -> &'static str {
        "emoji"
    }
    fn phase(&self) -> Phase {
        Phase::Transform
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        if !ctx.options.emoji.enabled {
            return Ok(());
        }
        match ast {
            AstPayload::Mdast(doc) | AstPayload::Both { mdast: doc, .. } => {
                emoji::apply_emoji(doc);
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
        let pass = EmojiPass;
        assert_eq!(pass.name(), "emoji");
        assert_eq!(pass.phase(), Phase::Transform);
    }
}
