use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use kv_store::{FileStorage, Storage};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

/// 应用程序共享状态
struct AppState {
    store: Mutex<FileStorage>,
}

/// 添加标签的请求体
#[derive(Deserialize)]
struct TagRequest {
    file: String,
    tag: String,
}

/// 通用响应结构
#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

/// 标签列表响应
#[derive(Serialize)]
struct TagsResponse {
    success: bool,
    tags: Vec<TagEntry>,
}

#[derive(Serialize)]
struct TagEntry {
    file: String,
    tags: Vec<String>,
}

/// 文件列表响应
#[derive(Serialize)]
struct FilesResponse {
    success: bool,
    files: Vec<String>,
}

/// 搜索响应
#[derive(Serialize)]
struct SearchResponse {
    success: bool,
    results: Vec<String>,
}

#[tokio::main]
async fn main() {
    // 确保 data 目录存在并初始化存储
    let store = FileStorage::new("data/store.json");

    let state = Arc::new(AppState {
        store: Mutex::new(store),
    });

    let app = Router::new()
        .route("/", get(welcome))
        .route("/files", get(list_files))
        .route("/search/:query", get(search_files))
        .route("/tags", post(add_tag))
        .route("/tags", get(list_tags))
        .layer(CorsLayer::permissive())
        .with_state(state);

    println!("服务器已启动，监听 0.0.0.0:3000");
    println!("可用端点：");
    println!("  GET  /              - 欢迎页面");
    println!("  GET  /files         - 列出所有已索引文件");
    println!("  GET  /search/:query - 搜索文件");
    println!("  POST /tags          - 为文件添加标签");
    println!("  GET  /tags          - 列出所有标签");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("无法绑定到 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("服务器启动失败");
}

/// GET / — 欢迎页面
async fn welcome() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
        "message": "欢迎使用 File Toolkit Web API！",
        "endpoints": {
            "GET /": "欢迎页面",
            "GET /files": "列出所有已索引文件",
            "GET /search/:query": "搜索文件",
            "POST /tags": "为文件添加标签（body: { \"file\": \"...\", \"tag\": \"...\" }）",
            "GET /tags": "列出所有标签"
        }
    }))
}

/// GET /files — 列出所有已索引文件
async fn list_files(State(state): State<Arc<AppState>>) -> Json<FilesResponse> {
    let store = state.store.lock().unwrap();
    let keys = store.keys();

    if keys.is_empty() {
        return Json(FilesResponse {
            success: true,
            files: vec![],
        });
    }

    Json(FilesResponse {
        success: true,
        files: keys,
    })
}

/// GET /search/:query — 搜索文件系统
async fn search_files(
    State(_state): State<Arc<AppState>>,
    Path(query): Path<String>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let results = search_file_system(&query);

    match results {
        Ok(files) => Ok(Json(SearchResponse {
            success: true,
            results: files,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "message": format!("搜索失败：{}", e)
            })),
        )),
    }
}

/// 使用 std::fs 递归搜索匹配的文件
fn search_file_system(pattern: &str) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let search_paths = vec!["."];

    for base_path in &search_paths {
        walk_dir(base_path, pattern, &mut results);
    }

    results.sort();
    results.dedup();
    Ok(results)
}

/// 递归遍历目录，查找文件名包含 pattern 的文件
fn walk_dir(dir: &str, pattern: &str, results: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy().to_string();
            if name_str.contains(pattern) {
                results.push(path.to_string_lossy().to_string());
            }
        }
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                walk_dir(path_str, pattern, results);
            }
        }
    }
}

/// POST /tags — 为文件添加标签
async fn add_tag(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TagRequest>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<serde_json::Value>)> {
    if req.file.trim().is_empty() || req.tag.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "文件名和标签不能为空"
            })),
        ));
    }

    let mut store = state.store.lock().unwrap();
    let key = format!("tag:{}", req.file);

    let existing_tags = store.get(&key).cloned().unwrap_or_default();
    let mut tags: Vec<String> = if existing_tags.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&existing_tags).unwrap_or_default()
    };

    if !tags.contains(&req.tag) {
        tags.push(req.tag.clone());
    }

    let tags_json = serde_json::to_string(&tags).unwrap();
    store.set(key, tags_json);

    Ok(Json(ApiResponse {
        success: true,
        message: format!("已为文件『{}』添加标签『{}』", req.file, req.tag),
    }))
}

/// GET /tags — 列出所有标签
async fn list_tags(State(state): State<Arc<AppState>>) -> Json<TagsResponse> {
    let store = state.store.lock().unwrap();
    let mut entries = Vec::new();

    for key in store.keys() {
        if let Some(tag_key) = key.strip_prefix("tag:") {
            if let Some(tags_json) = store.get(&key) {
                if let Ok(tags) = serde_json::from_str::<Vec<String>>(tags_json) {
                    entries.push(TagEntry {
                        file: tag_key.to_string(),
                        tags,
                    });
                }
            }
        }
    }

    Json(TagsResponse {
        success: true,
        tags: entries,
    })
}
