use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use serde::{Deserialize, Serialize};

const CACHE_FILE: &str = "index_cache.json";

/// 索引存储 trait —— 定义 KV 缓存的通用接口
trait IndexStorage {
    fn get(&self, key: &str) -> Option<&String>;
    fn set(&mut self, key: String, value: String);
    fn save(&self, path: &str) -> Result<(), String>;
    fn load(&mut self, path: &str) -> Result<(), String>;
}

/// 基于 HashMap 的内存存储，支持 serde 序列化/反序列化
#[derive(Serialize, Deserialize, Default)]
struct MemoryStorage {
    cache: HashMap<String, String>,
}

impl IndexStorage for MemoryStorage {
    fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
    }

    fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.cache).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn load(&mut self, path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            return Ok(());
        }
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let cache: HashMap<String, String> =
            serde_json::from_str(&json).map_err(|e| e.to_string())?;
        self.cache = cache;
        Ok(())
    }
}

/// 在文件中搜索关键词，返回格式化的结果字符串
fn search_in_file(query: &str, file_path: &str) -> Result<String, String> {
    let content = fs::read_to_string(file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    let mut results = String::new();
    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            results.push_str(&format!("{}: {}\n", i + 1, line));
        }
    }
    if results.is_empty() {
        return Err(format!("未找到匹配 '{}' 的行", query));
    }
    Ok(results.trim().to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <搜索词> <文件路径>", args[0]);
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    let cache_key = format!("{}:{}", query, file_path);

    let mut storage = MemoryStorage::default();

    // 加载缓存（文件不存在时静默跳过）
    if let Err(e) = storage.load(CACHE_FILE) {
        eprintln!("警告: 加载缓存失败: {}", e);
    }

    // 缓存命中
    if let Some(cached) = storage.get(&cache_key) {
        println!("{}", cached);
        println!("(来自缓存)");
        return;
    }

    // 缓存未命中：执行搜索
    match search_in_file(query, file_path) {
        Ok(result) => {
            println!("{}", result);
            storage.set(cache_key, result.clone());
            if let Err(e) = storage.save(CACHE_FILE) {
                eprintln!("警告: 保存缓存失败: {}", e);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
