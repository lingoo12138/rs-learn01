use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

const CACHE_FILE: &str = "index_cache.json";
const TAG_FILE: &str = "tags.json";

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

/// 标签存储结构
#[derive(Serialize, Deserialize, Default)]
struct TagStore {
    tags: HashMap<String, Vec<String>>,
}

impl TagStore {
    fn load(path: &str) -> Self {
        if !Path::new(path).exists() {
            return TagStore::default();
        }
        let json = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return TagStore::default(),
        };
        let tags: HashMap<String, Vec<String>> =
            serde_json::from_str(&json).unwrap_or_default();
        TagStore { tags }
    }

    fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.tags).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn add_tag(&mut self, file: String, tag: String) {
        self.tags.entry(file).or_default().push(tag);
    }

    fn list(&self) -> Vec<(&String, &Vec<String>)> {
        let mut entries: Vec<_> = self.tags.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }
}

/// 在单个文件中搜索关键词，返回 (文件路径, 结果行列表)
fn search_in_file(query: &str, file_path: &str) -> Vec<(usize, String)> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut results = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            results.push((i + 1, line.to_string()));
        }
    }
    results
}

fn print_usage() {
    eprintln!("用法:");
    eprintln!("  {} search <pattern> <file1> <file2> ...    并行搜索文件", env::args().next().unwrap_or_default());
    eprintln!("  {} tag <file> <tag>                         为文件添加标签", env::args().next().unwrap_or_default());
    eprintln!("  {} list                                     列出所有带标签的文件", env::args().next().unwrap_or_default());
    process::exit(1);
}

fn cmd_search(args: &[String]) {
    if args.len() < 3 {
        eprintln!("用法: {} search <pattern> <file1> <file2> ...", args[0]);
        process::exit(1);
    }

    let query = &args[1];
    let files = &args[2..];

    let mut storage = MemoryStorage::default();
    if let Err(e) = storage.load(CACHE_FILE) {
        eprintln!("警告: 加载缓存失败: {}", e);
    }

    // 检查是否所有文件结果都已缓存
    let all_cached: bool = files.iter().all(|f| {
        let key = format!("{}:{}", query, f);
        storage.get(&key).is_some()
    });

    if all_cached {
        // 全部缓存：从缓存读取并按文件输出
        for file in files {
            let key = format!("{}:{}", query, file);
            if let Some(cached) = storage.get(&key) {
                println!("{}:", file);
                println!("{}", cached);
                println!("(来自缓存)");
                println!();
            }
        }
        return;
    }

    // 找出未缓存的文件，用 rayon 并行搜索
    let uncached: Vec<&String> = files
        .iter()
        .filter(|f| storage.get(&format!("{}:{}", query, f)).is_none())
        .collect();

    let new_results: Vec<Vec<(usize, String)>> = uncached
        .par_iter()
        .map(|file| {
            search_in_file(query, file)
        })
        .collect();

    // 将新结果写入缓存（单次持久化，无并发竞态）
    for (file, results) in uncached.iter().zip(new_results.iter()) {
        if !results.is_empty() {
            let cached_str = results
                .iter()
                .map(|(line, text)| format!("{}: {}", line, text))
                .collect::<Vec<_>>()
                .join("\n");
            let key = format!("{}:{}", query, file);
            storage.set(key, cached_str);
        }
    }
    if let Err(e) = storage.save(CACHE_FILE) {
        eprintln!("保存缓存失败: {}", e);
    }

    // 输出所有结果（缓存 + 新搜索）
    let mut has_any = false;
    for file in files {
        let key = format!("{}:{}", query, file);
        if let Some(cached) = storage.get(&key) {
            has_any = true;
            println!("{}:", file);
            println!("{}", cached);
            println!();
        }
    }

    if !has_any {
        eprintln!("所有文件中均未找到匹配 '{}' 的行", query);
        process::exit(1);
    }
}

fn cmd_tag(args: &[String]) {
    if args.len() < 3 {
        eprintln!("用法: {} tag <file> <tag>", args[0]);
        process::exit(1);
    }

    let file = &args[1];
    let tag = &args[2];

    let mut store = TagStore::load(TAG_FILE);
    store.add_tag(file.to_string(), tag.to_string());

    match store.save(TAG_FILE) {
        Ok(()) => println!("已为 '{}' 添加标签 '{}'", file, tag),
        Err(e) => {
            eprintln!("保存标签失败: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_list(_args: &[String]) {
    let store = TagStore::load(TAG_FILE);
    let entries = store.list();

    if entries.is_empty() {
        println!("暂无标签数据");
        return;
    }

    for (file, tags) in entries {
        println!("{}: {}", file, tags.join(", "));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    let command = &args[1];

    match command.as_str() {
        "search" => cmd_search(&args[1..]),
        "tag" => cmd_tag(&args[1..]),
        "list" => cmd_list(&args[1..]),
        _ => print_usage(),
    }
}
