use std::collections::BTreeMap;

use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallMap<K: Ord, V>(BTreeMap<K, V>);

impl<K: Ord, V> SmallMap<K, V> {
    #[must_use]
    pub const fn new() -> Self {
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

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
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

impl<K: Ord + Serialize, V: Serialize> Serialize for SmallMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
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

    #[test]
    fn serialize_to_json() {
        let mut map = SmallMap::new();
        map.insert("href".to_string(), "http://example.com".to_string());
        map.insert("class".to_string(), "link".to_string());
        let json = serde_json::to_string(&map).unwrap();
        assert_eq!(json, r#"{"class":"link","href":"http://example.com"}"#);
    }
}
