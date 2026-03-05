use super::{CacheKey, CacheStore};

pub struct DiskCache;

impl DiskCache {
    #[must_use]
    pub const fn new(_dir: Option<&str>) -> Self {
        Self
    }
}

impl CacheStore for DiskCache {
    fn get(&self, _key: &CacheKey) -> Option<Vec<u8>> {
        None
    }

    fn put(&mut self, _key: CacheKey, _value: Vec<u8>) {}

    fn clear(&mut self) {}
}
