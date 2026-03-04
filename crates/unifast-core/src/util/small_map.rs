use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallMap<K: Ord, V>(BTreeMap<K, V>);

impl<K: Ord, V> SmallMap<K, V> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
}

impl<K: Ord, V> Default for SmallMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> IntoIterator for SmallMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_and_get() {
        let mut map = SmallMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), None);
    }

    #[test]
    fn stable_iteration_order() {
        let mut map = SmallMap::new();
        map.insert("c", 3);
        map.insert("a", 1);
        map.insert("b", 2);
        let keys: Vec<_> = map.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn len_and_is_empty() {
        let mut map: SmallMap<&str, i32> = SmallMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        map.insert("x", 10);
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn remove_and_contains_key() {
        let mut map = SmallMap::new();
        map.insert("key", 42);
        assert!(map.contains_key(&"key"));
        assert_eq!(map.remove(&"key"), Some(42));
        assert!(!map.contains_key(&"key"));
    }

    #[test]
    fn into_iter() {
        let mut map = SmallMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        let collected: Vec<_> = map.into_iter().collect();
        assert_eq!(collected, vec![(1, "a"), (2, "b")]);
    }
}
