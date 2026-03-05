use std::hash::{DefaultHasher, Hash, Hasher};

#[must_use]
pub fn content_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

#[must_use]
pub fn options_hash(options_json: &str) -> u64 {
    content_hash(options_json)
}

#[must_use]
pub fn version_hash(version: &str) -> u64 {
    content_hash(version)
}

#[must_use]
pub fn cache_key(content: &str, options: &str, version: &str) -> String {
    format!(
        "{:016x}-{:016x}-{:016x}",
        content_hash(content),
        options_hash(options),
        version_hash(version)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_hashing() {
        let hash1 = content_hash("hello world");
        let hash2 = content_hash("hello world");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_inputs_different_hashes() {
        let hash1 = content_hash("hello");
        let hash2 = content_hash("world");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn cache_key_format() {
        let key = cache_key("content", "options", "1.0.0");
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 3);
        for part in &parts {
            assert_eq!(part.len(), 16);
            assert!(part.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn cache_key_deterministic() {
        let key1 = cache_key("content", "options", "1.0.0");
        let key2 = cache_key("content", "options", "1.0.0");
        assert_eq!(key1, key2);
    }

    #[test]
    fn cache_key_differs_with_different_input() {
        let key1 = cache_key("content_a", "options", "1.0.0");
        let key2 = cache_key("content_b", "options", "1.0.0");
        assert_ne!(key1, key2);
    }
}
