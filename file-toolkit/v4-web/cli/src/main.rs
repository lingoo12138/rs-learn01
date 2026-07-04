use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;

/// File Toolkit CLI — 与 Web API 交互的命令行工具
#[derive(Parser)]
#[command(name = "ftk-cli", version = "0.1.0", about = "File Toolkit 命令行客户端")]
struct Cli {
    /// API 服务器地址（也可通过 FTK_API_URL 环境变量设置）
    #[arg(long, default_value = "http://localhost:3000")]
    url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 搜索文件
    Search {
        /// 搜索模式
        pattern: String,
    },
    /// 列出所有已索引文件
    Files,
    /// 为文件添加标签
    Tag {
        /// 文件名
        file: String,
        /// 标签名
        tag: String,
    },
    /// 列出所有标签
    Tags,
}

#[derive(Deserialize)]
struct SearchResponse {
    success: bool,
    results: Vec<String>,
    #[allow(dead_code)]
    message: Option<String>,
}

#[derive(Deserialize)]
struct FilesResponse {
    #[allow(dead_code)]
    success: bool,
    files: Vec<String>,
}

#[derive(Deserialize)]
struct ApiResponse {
    #[allow(dead_code)]
    success: bool,
    message: String,
}

#[derive(Deserialize)]
struct TagsResponse {
    #[allow(dead_code)]
    success: bool,
    tags: Vec<TagEntry>,
}

#[derive(Deserialize)]
struct TagEntry {
    file: String,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::parse();

    // 如果设置了 FTK_API_URL 环境变量，则覆盖 url
    if let Ok(env_url) = std::env::var("FTK_API_URL") {
        if !env_url.is_empty() {
            cli.url = env_url;
        }
    }

    match cli.command {
        Commands::Search { pattern } => {
            let url = format!("{}/search/{}", cli.url, pattern);
            let resp = reqwest::get(&url).await?;

            if resp.status().is_success() {
                let data: SearchResponse = resp.json().await?;
                if data.success {
                    if data.results.is_empty() {
                        println!("没有找到匹配的文件。");
                    } else {
                        println!("找到 {} 个匹配的文件：", data.results.len());
                        for file in &data.results {
                            println!("  {}", file);
                        }
                    }
                } else if let Some(msg) = data.message {
                    eprintln!("错误：{}", msg);
                }
            } else {
                let body: HashMap<String, serde_json::Value> = resp.json().await?;
                eprintln!("错误：{}", body.get("message").and_then(|v| v.as_str()).unwrap_or("请求失败"));
            }
        }
        Commands::Files => {
            let url = format!("{}/files", cli.url);
            let resp = reqwest::get(&url).await?;

            if resp.status().is_success() {
                let data: FilesResponse = resp.json().await?;
                if data.files.is_empty() {
                    println!("暂无已索引的文件。");
                } else {
                    println!("已索引的文件（{} 个）：", data.files.len());
                    for file in &data.files {
                        println!("  {}", file);
                    }
                }
            } else {
                eprintln!("错误：无法获取文件列表");
            }
        }
        Commands::Tag { file, tag } => {
            let url = format!("{}/tags", cli.url);
            let body = serde_json::json!({ "file": file, "tag": tag });
            let client = reqwest::Client::new();
            let resp = client.post(&url).json(&body).send().await?;

            if resp.status().is_success() {
                let data: ApiResponse = resp.json().await?;
                println!("{}", data.message);
            } else {
                let body: HashMap<String, serde_json::Value> = resp.json().await?;
                eprintln!("错误：{}", body.get("message").and_then(|v| v.as_str()).unwrap_or("添加标签失败"));
            }
        }
        Commands::Tags => {
            let url = format!("{}/tags", cli.url);
            let resp = reqwest::get(&url).await?;

            if resp.status().is_success() {
                let data: TagsResponse = resp.json().await?;
                if data.tags.is_empty() {
                    println!("暂无标签。");
                } else {
                    println!("所有标签：");
                    for entry in &data.tags {
                        println!("  {} => [{}]", entry.file, entry.tags.join(", "));
                    }
                }
            } else {
                eprintln!("错误：无法获取标签列表");
            }
        }
    }

    Ok(())
}
