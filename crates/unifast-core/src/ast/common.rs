use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn contains(&self, offset: u32) -> bool {
        offset >= self.start && offset < self.end
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn merge(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

pub struct NodeIdGen(u32);

impl NodeIdGen {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.0);
        self.0 += 1;
        id
    }
}

impl Default for NodeIdGen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_contains() {
        let span = Span::new(5, 10);
        assert!(span.contains(5));
        assert!(span.contains(7));
        assert!(span.contains(9));
        assert!(!span.contains(4));
        assert!(!span.contains(10));
    }

    #[test]
    fn span_merge() {
        let a = Span::new(5, 10);
        let b = Span::new(8, 15);
        let merged = a.merge(b);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 15);

        let c = Span::new(0, 3);
        let merged2 = a.merge(c);
        assert_eq!(merged2.start, 0);
        assert_eq!(merged2.end, 10);
    }

    #[test]
    fn span_len() {
        assert_eq!(Span::new(0, 10).len(), 10);
        assert_eq!(Span::new(5, 5).len(), 0);
        assert_eq!(Span::new(3, 7).len(), 4);
    }

    #[test]
    fn span_is_empty() {
        assert!(Span::new(0, 0).is_empty());
        assert!(Span::new(5, 5).is_empty());
        assert!(!Span::new(0, 1).is_empty());
    }

    #[test]
    fn span_empty() {
        let span = Span::empty();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 0);
        assert!(span.is_empty());
    }

    #[test]
    fn node_id_gen_sequential() {
        let mut id_gen = NodeIdGen::new();
        assert_eq!(id_gen.next_id(), NodeId(0));
        assert_eq!(id_gen.next_id(), NodeId(1));
        assert_eq!(id_gen.next_id(), NodeId(2));
    }

    #[test]
    fn node_id_gen_uniqueness() {
        let mut id_gen = NodeIdGen::new();
        let mut ids = std::collections::HashSet::new();
        for _ in 0..100 {
            assert!(ids.insert(id_gen.next_id()));
        }
        assert_eq!(ids.len(), 100);
    }

    #[test]
    fn node_id_gen_default() {
        let mut id_gen = NodeIdGen::default();
        assert_eq!(id_gen.next_id(), NodeId(0));
    }
}
