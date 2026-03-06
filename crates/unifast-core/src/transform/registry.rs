use super::pass::{FnPass, Pass, PassResult, Phase};

pub struct PassRegistry {
    passes: Vec<Box<dyn Pass>>,
}

impl PassRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self { passes: vec![] }
    }

    pub fn register(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    pub fn register_fn(
        &mut self,
        name: &'static str,
        phase: Phase,
        run: impl Fn(&mut super::pass::PassContext, &mut super::pass::AstPayload) -> PassResult
        + Send
        + Sync
        + 'static,
    ) {
        self.passes.push(Box::new(FnPass {
            name,
            phase,
            run_fn: Box::new(run),
        }));
    }

    #[must_use]
    pub fn ordered_passes(&self) -> Vec<&dyn Pass> {
        let mut refs: Vec<&dyn Pass> = self
            .passes
            .iter()
            .map(std::convert::AsRef::as_ref)
            .collect();
        refs.sort_by_key(|p| p.phase());
        refs
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }
}

impl Default for PassRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::pass::{AstPayload, PassContext, PassResult, Phase};

    struct MockPass {
        name: &'static str,
        phase: Phase,
    }

    impl Pass for MockPass {
        fn name(&self) -> &'static str {
            self.name
        }
        fn phase(&self) -> Phase {
            self.phase
        }
        fn run(&self, _ctx: &mut PassContext, _ast: &mut AstPayload) -> PassResult {
            Ok(())
        }
    }

    #[test]
    fn registry_new_is_empty() {
        let reg = PassRegistry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn registry_register_and_len() {
        let mut reg = PassRegistry::new();
        reg.register(Box::new(MockPass {
            name: "pass1",
            phase: Phase::Parse,
        }));
        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());

        reg.register(Box::new(MockPass {
            name: "pass2",
            phase: Phase::Emit,
        }));
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn registry_ordered_passes_by_phase() {
        let mut reg = PassRegistry::new();
        reg.register(Box::new(MockPass {
            name: "emit_pass",
            phase: Phase::Emit,
        }));
        reg.register(Box::new(MockPass {
            name: "parse_pass",
            phase: Phase::Parse,
        }));
        reg.register(Box::new(MockPass {
            name: "lower_pass",
            phase: Phase::Lower,
        }));
        reg.register(Box::new(MockPass {
            name: "transform_pass",
            phase: Phase::Transform,
        }));
        reg.register(Box::new(MockPass {
            name: "optimize_pass",
            phase: Phase::Optimize,
        }));

        let ordered = reg.ordered_passes();
        assert_eq!(ordered.len(), 5);
        assert_eq!(ordered[0].name(), "parse_pass");
        assert_eq!(ordered[1].name(), "transform_pass");
        assert_eq!(ordered[2].name(), "lower_pass");
        assert_eq!(ordered[3].name(), "optimize_pass");
        assert_eq!(ordered[4].name(), "emit_pass");
    }

    #[test]
    fn registry_multiple_passes_same_phase() {
        let mut reg = PassRegistry::new();
        reg.register(Box::new(MockPass {
            name: "t1",
            phase: Phase::Transform,
        }));
        reg.register(Box::new(MockPass {
            name: "t2",
            phase: Phase::Transform,
        }));
        reg.register(Box::new(MockPass {
            name: "p1",
            phase: Phase::Parse,
        }));

        let ordered = reg.ordered_passes();
        assert_eq!(ordered.len(), 3);
        assert_eq!(ordered[0].phase(), Phase::Parse);
        assert_eq!(ordered[1].phase(), Phase::Transform);
        assert_eq!(ordered[2].phase(), Phase::Transform);
    }

    #[test]
    fn registry_default() {
        let reg = PassRegistry::default();
        assert!(reg.is_empty());
    }
}
