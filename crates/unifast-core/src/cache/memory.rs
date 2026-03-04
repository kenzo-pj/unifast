use std::collections::HashMap;

use super::{CacheKey, CacheStore};

pub struct MemoryCache {
    store: HashMap<String, Vec<u8>>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheStore for MemoryCache {
    fn get(&self, key: &CacheKey) -> Option<Vec<u8>> {
        self.store.get(&key.key_string()).cloned()
    }

    fn put(&mut self, key: CacheKey, value: Vec<u8>) {
        self.store.insert(key.key_string(), value);
    }

    fn clear(&mut self) {
        self.store.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_cache_get_miss() {
        let cache = MemoryCache::new();
        let key = CacheKey::new("test", "opts", "v1");
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn memory_cache_put_and_get() {
        let mut cache = MemoryCache::new();
        let key = CacheKey::new("test", "opts", "v1");
        let value = b"cached output".to_vec();

        cache.put(key.clone(), value.clone());
        let retrieved = cache.get(&key);
        assert_eq!(retrieved, Some(value));
    }

    #[test]
    fn memory_cache_overwrite() {
        let mut cache = MemoryCache::new();
        let key = CacheKey::new("test", "opts", "v1");

        cache.put(key.clone(), b"first".to_vec());
        cache.put(key.clone(), b"second".to_vec());

        assert_eq!(cache.get(&key), Some(b"second".to_vec()));
    }

    #[test]
    fn memory_cache_clear() {
        let mut cache = MemoryCache::new();
        let key1 = CacheKey::new("a", "opts", "v1");
        let key2 = CacheKey::new("b", "opts", "v1");

        cache.put(key1.clone(), b"val1".to_vec());
        cache.put(key2.clone(), b"val2".to_vec());

        assert!(cache.get(&key1).is_some());
        assert!(cache.get(&key2).is_some());

        cache.clear();

        assert!(cache.get(&key1).is_none());
        assert!(cache.get(&key2).is_none());
    }

    #[test]
    fn memory_cache_default() {
        let cache = MemoryCache::default();
        let key = CacheKey::new("x", "y", "z");
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn memory_cache_different_keys_independent() {
        let mut cache = MemoryCache::new();
        let key1 = CacheKey::new("content1", "opts", "v1");
        let key2 = CacheKey::new("content2", "opts", "v1");

        cache.put(key1.clone(), b"val1".to_vec());
        cache.put(key2.clone(), b"val2".to_vec());

        assert_eq!(cache.get(&key1), Some(b"val1".to_vec()));
        assert_eq!(cache.get(&key2), Some(b"val2".to_vec()));
    }
}
