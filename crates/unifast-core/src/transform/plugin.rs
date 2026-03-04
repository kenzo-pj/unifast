use super::registry::PassRegistry;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn apply(&self, registry: &mut PassRegistry);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};

    struct MockPass;

    impl Pass for MockPass {
        fn name(&self) -> &'static str {
            "mock_pass"
        }
        fn phase(&self) -> Phase {
            Phase::Transform
        }
        fn run(&self, _ctx: &mut PassContext, _ast: &mut AstPayload) -> PassResult {
            Ok(())
        }
    }

    struct MockPlugin;

    impl Plugin for MockPlugin {
        fn name(&self) -> &'static str {
            "mock_plugin"
        }
        fn apply(&self, registry: &mut PassRegistry) {
            registry.register(Box::new(MockPass));
        }
    }

    #[test]
    fn plugin_registers_pass() {
        let plugin = MockPlugin;
        assert_eq!(plugin.name(), "mock_plugin");

        let mut registry = PassRegistry::new();
        assert!(registry.is_empty());

        plugin.apply(&mut registry);
        assert_eq!(registry.len(), 1);

        let passes = registry.ordered_passes();
        assert_eq!(passes[0].name(), "mock_pass");
        assert_eq!(passes[0].phase(), Phase::Transform);
    }

    #[test]
    fn plugin_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockPlugin>();
    }
}
