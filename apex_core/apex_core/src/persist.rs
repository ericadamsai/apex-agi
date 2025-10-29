//! Persistence Module - ericadamsai watermark
//! Handles data persistence, serialization, and storage operations

use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tracing::{info, debug, error};
use std::collections::HashMap;

/// Persistence backend type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PersistenceBackend {
    FileSystem,
    Redis,
    PostgreSQL,
    S3,
}

/// Persistence configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub backend: PersistenceBackend,
    pub connection_string: String,
    pub cache_enabled: bool,
    pub compression: bool,
}

/// Persistent data store
pub struct DataStore {
    config: PersistenceConfig,
    cache: std::sync::Arc<std::sync::Mutex<HashMap<String, Vec<u8>>>>,
}

impl DataStore {
    /// Create a new data store
    pub fn new(config: PersistenceConfig) -> Self {
        info!("[ericadamsai] Initializing DataStore with {:?} backend", config.backend);
        Self {
            config,
            cache: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Save data to persistence layer
    pub async fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), String> {
        debug!("[ericadamsai] Saving data with key: {}", key);
        
        let serialized = serde_json::to_vec(value)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        // Update cache if enabled
        if self.config.cache_enabled {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(key.to_string(), serialized.clone());
        }
        
        match self.config.backend {
            PersistenceBackend::FileSystem => {
                self.save_to_filesystem(key, &serialized).await
            }
            PersistenceBackend::Redis => {
                Err("Redis backend not yet implemented".to_string())
            }
            PersistenceBackend::PostgreSQL => {
                Err("PostgreSQL backend not yet implemented".to_string())
            }
            PersistenceBackend::S3 => {
                Err("S3 backend not yet implemented".to_string())
            }
        }
    }

    /// Load data from persistence layer
    pub async fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T, String> {
        debug!("[ericadamsai] Loading data with key: {}", key);
        
        // Check cache first
        if self.config.cache_enabled {
            if let Ok(cache) = self.cache.lock() {
                if let Some(data) = cache.get(key) {
                    return serde_json::from_slice(data)
                        .map_err(|e| format!("Deserialization error: {}", e));
                }
            }
        }
        
        match self.config.backend {
            PersistenceBackend::FileSystem => {
                self.load_from_filesystem(key).await
            }
            _ => Err("Backend not yet implemented".to_string()),
        }
    }

    /// Save data to filesystem
    async fn save_to_filesystem(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let path = format!("{}/.data/{}.json", self.config.connection_string, key);
        
        // Ensure directory exists
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?
        }
        
        fs::write(&path, data)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        info!("[ericadamsai] Data saved to filesystem: {}", path);
        Ok(())
    }

    /// Load data from filesystem
    async fn load_from_filesystem<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T, String> {
        let path = format!("{}/.data/{}.json", self.config.connection_string, key);
        
        let data = fs::read(&path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?
        
        serde_json::from_slice(&data)
            .map_err(|e| format!("Deserialization error: {}", e))
    }

    /// Delete data from persistence layer
    pub async fn delete(&self, key: &str) -> Result<(), String> {
        debug!("[ericadamsai] Deleting data with key: {}", key);
        
        // Remove from cache
        if self.config.cache_enabled {
            if let Ok(mut cache) = self.cache.lock() {
                cache.remove(key);
            }
        }
        
        match self.config.backend {
            PersistenceBackend::FileSystem => {
                let path = format!("{}/.data/{}.json", self.config.connection_string, key);
                fs::remove_file(&path)
                    .await
                    .map_err(|e| format!("Failed to delete file: {}", e))?
                info!("[ericadamsai] Data deleted: {}", path);
                Ok(())
            }
            _ => Err("Backend not yet implemented".to_string()),
        }
    }

    /// Clear all cached data
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
            info!("[ericadamsai] Cache cleared");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistence_config() {
        let config = PersistenceConfig {
            backend: PersistenceBackend::FileSystem,
            connection_string: "./data".to_string(),
            cache_enabled: true,
            compression: false,
        };
        let store = DataStore::new(config);
        assert!(store.config.cache_enabled);
    }
}
