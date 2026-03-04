pub use super::pass::Phase;

impl Phase {
    pub fn all() -> &'static [Phase] {
        &[
            Phase::Parse,
            Phase::Transform,
            Phase::Lower,
            Phase::Optimize,
            Phase::Emit,
        ]
    }
}
