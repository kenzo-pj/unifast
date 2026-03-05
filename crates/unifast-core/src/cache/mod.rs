pub mod disk;
pub mod memory;

use crate::util::hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub content_hash: String,
    pub options_hash: String,
    pub version_hash: String,
}

impl CacheKey {
    #[must_use]
    pub fn new(content: &str, options: &str, version: &str) -> Self {
        Self {
            content_hash: format!("{:016x}", hash::content_hash(content)),
            options_hash: format!("{:016x}", hash::options_hash(options)),
            version_hash: format!("{:016x}", hash::version_hash(version)),
        }
    }

    #[must_use]
    pub fn key_string(&self) -> String {
        format!(
            "{}-{}-{}",
            self.content_hash, self.options_hash, self.version_hash
        )
    }
}

pub trait CacheStore: Send + Sync {
    fn get(&self, key: &CacheKey) -> Option<Vec<u8>>;
    fn put(&mut self, key: CacheKey, value: Vec<u8>);
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_construction() {
        let key = CacheKey::new("hello", "opts", "1.0");
        assert_eq!(key.content_hash.len(), 16);
        assert_eq!(key.options_hash.len(), 16);
        assert_eq!(key.version_hash.len(), 16);
        assert!(key.content_hash.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key.options_hash.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key.version_hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn cache_key_string_format() {
        let key = CacheKey::new("content", "options", "1.0.0");
        let key_str = key.key_string();
        let parts: Vec<&str> = key_str.split('-').collect();
        assert_eq!(parts.len(), 3);
        for part in &parts {
            assert_eq!(part.len(), 16);
            assert!(part.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn cache_key_deterministic() {
        let key1 = CacheKey::new("test", "opts", "v1");
        let key2 = CacheKey::new("test", "opts", "v1");
        assert_eq!(key1, key2);
        assert_eq!(key1.key_string(), key2.key_string());
    }

    #[test]
    fn cache_key_differs_for_different_inputs() {
        let key1 = CacheKey::new("content_a", "opts", "v1");
        let key2 = CacheKey::new("content_b", "opts", "v1");
        assert_ne!(key1, key2);
        assert_ne!(key1.key_string(), key2.key_string());
    }
}
