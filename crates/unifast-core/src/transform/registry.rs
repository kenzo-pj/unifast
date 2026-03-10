use super::pass::{AstPayload, FnPass, FnPtrPass, Pass, PassContext, PassFnPtr, PassResult, Phase};

const PHASE_COUNT: usize = 5;

pub(crate) enum PassSlot {
    FnPtr(FnPtrPass),
    Boxed(Box<dyn Pass>),
}

impl PassSlot {
    fn phase_index(&self) -> usize {
        match self {
            Self::FnPtr(p) => p.phase as usize,
            Self::Boxed(p) => p.phase() as usize,
        }
    }
}

impl Pass for PassSlot {
    fn name(&self) -> &'static str {
        match self {
            Self::FnPtr(p) => p.name,
            Self::Boxed(p) => p.name(),
        }
    }
    fn phase(&self) -> Phase {
        match self {
            Self::FnPtr(p) => p.phase,
            Self::Boxed(p) => p.phase(),
        }
    }
    fn run(&self, ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
        match self {
            Self::FnPtr(p) => (p.run_fn)(ctx, ast),
            Self::Boxed(p) => p.run(ctx, ast),
        }
    }
}

pub struct PassRegistry {
    phases: [Vec<PassSlot>; PHASE_COUNT],
}

impl PassRegistry {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phases: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn register(&mut self, pass: Box<dyn Pass>) {
        let idx = pass.phase() as usize;
        self.phases[idx].push(PassSlot::Boxed(pass));
    }

    pub fn register_fn_ptr(&mut self, name: &'static str, phase: Phase, run: PassFnPtr) {
        let slot = PassSlot::FnPtr(FnPtrPass {
            name,
            phase,
            run_fn: run,
        });
        self.phases[slot.phase_index()].push(slot);
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
        self.phases[phase as usize].push(PassSlot::Boxed(Box::new(FnPass {
            name,
            phase,
            run_fn: Box::new(run),
        })));
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &PassSlot> {
        self.phases.iter().flat_map(|bucket| bucket.iter())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.phases.iter().map(Vec::len).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.phases.iter().all(Vec::is_empty)
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
    fn registry_ordered_by_phase() {
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

        let names: Vec<&str> = reg.iter().map(|p| p.name()).collect();
        assert_eq!(names.len(), 5);
        assert_eq!(names[0], "parse_pass");
        assert_eq!(names[1], "transform_pass");
        assert_eq!(names[2], "lower_pass");
        assert_eq!(names[3], "optimize_pass");
        assert_eq!(names[4], "emit_pass");
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

        let phases: Vec<Phase> = reg.iter().map(|p| p.phase()).collect();
        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0], Phase::Parse);
        assert_eq!(phases[1], Phase::Transform);
        assert_eq!(phases[2], Phase::Transform);
    }

    #[test]
    fn registry_default() {
        let reg = PassRegistry::default();
        assert!(reg.is_empty());
    }

    #[test]
    fn registry_fn_ptr_inline() {
        let mut reg = PassRegistry::new();
        reg.register_fn_ptr("inline_pass", Phase::Transform, |_ctx, _ast| Ok(()));
        assert_eq!(reg.len(), 1);
        let names: Vec<&str> = reg.iter().map(|p| p.name()).collect();
        assert_eq!(names[0], "inline_pass");
    }

    #[test]
    fn registry_insertion_order_within_phase() {
        let mut reg = PassRegistry::new();
        reg.register_fn_ptr("a", Phase::Optimize, |_ctx, _ast| Ok(()));
        reg.register_fn_ptr("b", Phase::Optimize, |_ctx, _ast| Ok(()));
        reg.register_fn_ptr("c", Phase::Optimize, |_ctx, _ast| Ok(()));

        let names: Vec<&str> = reg.iter().map(|p| p.name()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }
}
