use super::{CacheKey, CacheStore};

/// Stub disk cache -- filesystem backing will be implemented in a later milestone.
pub struct DiskCache;

impl DiskCache {
    pub fn new(_dir: Option<&str>) -> Self {
        Self
    }
}

impl CacheStore for DiskCache {
    fn get(&self, _key: &CacheKey) -> Option<Vec<u8>> {
        None
    }

    fn put(&mut self, _key: CacheKey, _value: Vec<u8>) {
        // No-op stub
    }

    fn clear(&mut self) {
        // No-op stub
    }
}
