use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// KV 存储引擎的抽象接口
pub trait Storage {
    fn get(&self, key: &str) -> Option<&String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
    fn keys(&self) -> Vec<String>;
}

/// 基于 HashMap 的内存存储
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemoryStorage {
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

/// 基于 JSON 文件的持久化存储
/// 内部使用 MemoryStorage，并通过 JSON 文件持久化
#[derive(Serialize, Deserialize, Clone)]
pub struct FileStorage {
    memory: MemoryStorage,
    path: String,
}

impl FileStorage {
    /// 从指定路径加载存储，若文件不存在则创建空的存储
    pub fn new(path: &str) -> Self {
        let memory = if Path::new(path).exists() {
            let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_else(|_| MemoryStorage::new())
        } else {
            // 确保父目录存在
            if let Some(parent) = Path::new(path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            MemoryStorage::new()
        };

        FileStorage {
            memory,
            path: path.to_string(),
        }
    }

    /// 将当前内存状态持久化到 JSON 文件
    fn persist(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.memory).map_err(|e| format!("序列化存储失败: {}", e))?;
        fs::write(&self.path, json).map_err(|e| format!("写入存储文件失败: {}", e))?;
        Ok(())
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<&String> {
        self.memory.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.memory.set(key, value);
        if let Err(e) = self.persist() {
            eprintln!("持久化存储失败: {}", e);
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        let removed = self.memory.delete(key);
        if removed {
            if let Err(e) = self.persist() {
                eprintln!("持久化存储失败: {}", e);
            }
        }
        removed
    }

    fn keys(&self) -> Vec<String> {
        self.memory.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage() {
        let mut store = MemoryStorage::new();
        assert!(store.get("hello").is_none());

        store.set("hello".to_string(), "world".to_string());
        assert_eq!(store.get("hello"), Some(&"world".to_string()));

        assert!(store.delete("hello"));
        assert!(store.get("hello").is_none());
    }

    #[test]
    fn test_memory_storage_keys() {
        let mut store = MemoryStorage::new();
        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());
        let mut keys = store.keys();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn test_file_storage_roundtrip() {
        let tmp = std::env::temp_dir().join("test_kv_store.json");
        let path = tmp.to_str().unwrap().to_string();

        // 写入
        {
            let mut store = FileStorage::new(&path);
            store.set("key1".to_string(), "val1".to_string());
            store.set("key2".to_string(), "val2".to_string());
        }

        // 重新加载并验证
        {
            let store = FileStorage::new(&path);
            assert_eq!(store.get("key1"), Some(&"val1".to_string()));
            assert_eq!(store.get("key2"), Some(&"val2".to_string()));
            assert!(store.get("nonexistent").is_none());
        }

        // 清理
        let _ = std::fs::remove_file(&path);
    }
}
