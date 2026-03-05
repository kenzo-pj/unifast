pub use super::pass::Phase;

impl Phase {
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Parse,
            Self::Transform,
            Self::Lower,
            Self::Optimize,
            Self::Emit,
        ]
    }
}
