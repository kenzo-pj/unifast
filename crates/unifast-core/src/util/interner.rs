pub use lasso::Spur as Symbol;

pub struct Interner {
    rodeo: lasso::Rodeo,
}

impl Interner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rodeo: lasso::Rodeo::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        self.rodeo.get_or_intern(s)
    }

    #[must_use]
    pub fn resolve(&self, sym: Symbol) -> &str {
        self.rodeo.resolve(&sym)
    }
}

impl Default for Interner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut interner = Interner::new();
        let sym = interner.intern("hello");
        assert_eq!(interner.resolve(sym), "hello");
    }

    #[test]
    fn dedup_same_symbol() {
        let mut interner = Interner::new();
        let sym1 = interner.intern("world");
        let sym2 = interner.intern("world");
        assert_eq!(sym1, sym2);
    }

    #[test]
    fn multiple_strings() {
        let mut interner = Interner::new();
        let a = interner.intern("alpha");
        let b = interner.intern("beta");
        let c = interner.intern("gamma");
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
        assert_eq!(interner.resolve(a), "alpha");
        assert_eq!(interner.resolve(b), "beta");
        assert_eq!(interner.resolve(c), "gamma");
    }
}
