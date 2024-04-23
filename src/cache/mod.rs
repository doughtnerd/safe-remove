use std::{collections::HashMap, path::PathBuf};

pub trait Cache {
    fn has(&self, key: &str) -> bool;
    fn load(&self, key: &str) -> Option<Vec<u8>>;
    fn store(&mut self, key: &str, value: Vec<u8>);
}

pub struct MemoryCache {
    data: HashMap<String, Vec<u8>>,
}

impl MemoryCache {
    pub fn new() -> MemoryCache {
        MemoryCache {
            data: HashMap::new(),
        }
    }
}

impl Cache for MemoryCache {
    fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn load(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }

    fn store(&mut self, key: &str, value: Vec<u8>) {
        self.data.insert(key.to_string(), value);
    }
}

pub struct DiskCache {
    cache_location: PathBuf,
}

impl DiskCache {
    pub fn new(cache_location: PathBuf) -> Result<DiskCache, &'static str> {
        if cache_location.is_file() {
            return Err("Cache location must be a directory");
        }

        if !cache_location.exists() {
            if let Err(_) = std::fs::create_dir_all(&cache_location) {
                return Err("Error creating cache directory");
            }
        }

        let cache = DiskCache { cache_location };

        Ok(cache)
    }
}

impl Cache for DiskCache {
    fn has(&self, key: &str) -> bool {
        let file_path = format!(
            "{}/{}",
            self.cache_location.to_str().unwrap_or_default(),
            key
        );
        std::fs::metadata(&file_path).is_ok()
    }

    fn load(&self, key: &str) -> Option<Vec<u8>> {
        let file_path = format!(
            "{}/{}",
            self.cache_location.to_str().unwrap_or_default(),
            key
        );
        match std::fs::read(&file_path) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    fn store(&mut self, key: &str, value: Vec<u8>) {
        let file_path = format!(
            "{}/{}",
            self.cache_location.to_str().unwrap_or_default(),
            key
        );

        if let Err(e) = std::fs::write(&file_path, &value) {
            eprintln!("Error writing to file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_cache() {
        let mut cache = MemoryCache::new();

        cache.store("key", vec![1, 2, 3]);
        assert_eq!(cache.load("key"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_disk_cache() {
        let mut cache = DiskCache::new(PathBuf::from("/tmp/cache")).unwrap();
        cache.store("key", vec![1, 2, 3]);
        std::fs::remove_dir_all("/tmp/cache").unwrap();

        assert_eq!(cache.load("key"), Some(vec![1, 2, 3]));
    }
}
