# Rust 入门实战项目：File Toolkit 多阶段学习设计

## 概述

一个面向 Rust 新手的多阶段入门项目，采用"章节语法练习 → 多版本实战项目"的混合式教学方式。实战项目以 **File Toolkit**（文件工具箱）为核心，从简单的文件搜索工具逐步演化为包含 Web API 和 KV 存储引擎的完整系统。

## 项目结构

```
rs-learn01/
├── Cargo.toml                      # workspace 根
├── exercises/                      # 章节式语法练习 crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                 # 入口：显示章节列表，提示运行测试
│   │   ├── chapter01_variables/    # mod + test
│   │   │   ├── mod.rs
│   │   │   └── ch01_variables.html
│   │   ├── chapter02_ownership/
│   │   │   ├── mod.rs
│   │   │   └── ch02_ownership.html
│   │   ├── chapter03_borrow/
│   │   ├── chapter04_string/
│   │   ├── chapter05_struct_enum/
│   │   ├── chapter06_lifetime/
│   │   ├── chapter07_generics_trait/
│   │   ├── chapter08_error/
│   │   ├── chapter09_closure_iter/
│   │   ├── chapter10_smartptr/
│   │   └── chapter11_concurrency/
│   └── tests/                     # 可选集成测试
├── file-toolkit/                   # 实战项目 workspace 成员
│   ├── Cargo.toml
│   ├── v1-basic/                  # 基础版：文件搜索
│   ├── v2-indexed/                # 进阶版：KV 索引缓存
│   ├── v3-parallel/               # 高级版：并行搜索 + 标签
│   └── v4-web/                    # 终极版：Web API + KV 存储引擎
└── docs/
    └── superpowers/
        └── specs/
            └── 2026-07-05-rust-learning-design.md
```

## 阶段划分

| 阶段 | 章节/版本 | 内容 | 交互方式 |
|------|-----------|------|---------|
| **基础语法** | 01-11 章 | 变量→所有权→借用→字符串→结构体→生命周期→泛型→错误→闭包→智能指针→并发 | 每章 HTML 教学 + 填空练习 + cargo test 验证 |
| **入门实战** | V1 Basic | 文件搜索 CLI，约50行 | 实战 + README |
| **进阶实战** | V2 Indexed | KV 索引缓存、序列化、trait 设计 | 实战 + README |
| **高级实战** | V3 Parallel | 并行搜索、标签管理 | 实战 + README |
| **终极实战** | V4 Web | 三层架构：KV 存储引擎 → Web API → CLI 客户端 | 全栈实战 |

## 教学文件格式

每章一个独立 HTML 文件（浏览器直接打开，无需阅读器），风格统一，包含：
- 本节学习目标
- 核心概念讲解（中文，配代码片段）
- 对比记忆（与常见语言对比）
- 常见坑点
- 练习题说明（指出每个题练习哪个知识点）
- 关联 Rust Book 章节（可选阅读）

HTML 采用内联 CSS/JS，无外部依赖，可直接在浏览器打开。

## 练习机制

- 每章 Rust 模块包含若干 `todo!()` 占位的函数
- 下方通过 `#[cfg(test)] mod tests` 内联测试
- 用户填充代码后 `cargo test` 验证
- 编译通过 + 测试通过 = 正确
- 示例：
```rust
// 填空：实现一个函数，返回两个数的和
pub fn add(a: i32, b: i32) -> i32 {
    todo!()  // 用户填入 a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}
```

## 章节详细大纲

### 阶段一：基础语法（第1-11章）

#### 第01章：变量与类型
- let、mut、const
- 标量类型：i32/u32/f64/bool/char
- 复合类型：元组、数组
- 类型推导与类型标注
- 练习量：8题（变量声明、类型推断、元组解构、数组访问）

#### 第02章：所有权
- 所有权三原则
- 移动语义
- Clone 与 Copy trait
- 函数传参与返回值
- 练习量：8题

#### 第03章：借用与引用
- 不可变引用 &T
- 可变引用 &mut T
- 借用规则：一个可变/多个不可变
- 悬垂引用（编译期阻止）
- 练习量：8题

#### 第04章：切片与字符串
- &str 与 String
- 字符串切片
- 字符串拼接（+、format!）
- 常用方法
- 练习量：8题

#### 第05章：结构体与枚举
- struct 定义与实例化
- impl 方法
- Option<T>、Result<T, E>
- match 与 if let
- 练习量：10题

#### 第06章：生命周期
- 生命周期标注语法
- 函数中的生命周期
- 结构体中的生命周期
- 省略规则
- 'static
- 练习量：6题

#### 第07章：泛型与 trait
- 泛型函数与结构体
- trait 定义与实现
- trait 默认实现
- trait 作为参数（impl trait / where）
- 练习量：10题

#### 第08章：错误处理
- panic! 与不可恢复错误
- Result<T, E> 与 ? 运算符
- 自定义错误类型（Display + Debug + Error）
- 练习量：8题

#### 第09章：闭包与迭代器
- 闭包语法 |params| expr
- Fn/FnMut/FnOnce
- Iterator trait
- map/filter/fold/collect 等适配器
- 练习量：8题

#### 第10章：智能指针
- Box<T> 堆分配
- Rc<T> 引用计数
- RefCell<T> 内部可变性
- 练习量：6题

#### 第11章：并发与异步基础
- std::thread 线程创建
- 消息传递 mpsc
- 共享状态 Arc<Mutex<T>>
- async/await 入门
- 练习量：6题

### 阶段二：入门实战

#### V1 Basic — 文件搜索

功能：从命令行接收搜索词和文件路径，输出匹配行。

覆盖知识点：
- 环境变量与 CLI 参数（std::env::args）
- 文件读取（std::fs::read_to_string）
- 字符串搜索（lines + contains）
- Result 处理与 ? 运算符
- 简单的错误输出

代码量：~50行

### 阶段三：进阶实战

#### V2 Indexed — 引入 KV 索引

功能：在 V1 基础上，为已搜索文件建立索引缓存（搜索词→搜索结果），避免重复搜索。

引入知识点：
- 结构体设计：IndexCache、SearchResult
- trait 抽象：IndexStorage trait
- 序列化/反序列化：serde + serde_json
- KV 存储概念：HashMap + JSON 持久化
- 文件元数据：Metadata、时间戳

代码量：~200行

### 阶段四：高级实战

#### V3 Parallel — 并行搜索 + 标签

功能：多线程并行搜索多个文件，给文件打标签，支持按标签过滤搜索。

引入知识点：
- 线程池（rayon::par_iter）
- 并发数据结构 Arc<Mutex<>>
- 标签系统设计
- 性能优化与 benchmark
- 迭代器链式操作

### 阶段五：终极实战

#### V4 Web — 三层架构

将 V3 拆分为三层独立组件：

```
file-toolkit-v4/
├── kv-store/            # KV 存储引擎 crate
│   ├── Cargo.toml
│   └── src/lib.rs       # 持久化 KV 存储（内存 + JSON/SQLite）
├── web-api/             # Web 服务 crate
│   ├── Cargo.toml
│   └── src/main.rs      # axum RESTful API
└── cli/                 # CLI 客户端 crate
    ├── Cargo.toml
    └── src/main.rs      # reqwest HTTP 客户端
```

**KV 存储引擎**（独立 crate，可替换后端）：
- trait Storage 定义接口：get/set/delete/scan
- MemoryStorage：HashMap 实现
- FileStorage：JSON 文件持久化
- （可选扩展）SqliteStorage：rusqlite 实现

**Web API**（axum + tokio）：
- POST /search — 提交搜索任务
- GET /files — 列出已索引文件
- GET /search/:id — 查询搜索结果
- POST /tags — 给文件打标签
- 中间件：日志、错误处理

**CLI 客户端**（clap + reqwest）：
- ftk search <pattern> — 通过 API 搜索
- ftk index <path> — 索引文件
- ftk tag <file> <tag> — 打标签
- ftk list — 列出索引

## 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| Web 框架 | axum | 社区主流、tokio 原生、类型安全 |
| 异步运行时 | tokio | Rust 事实标准 |
| HTTP 客户端 | reqwest | 最流行、与 tokio 兼容 |
| KV 持久化 | serde_json | 零依赖成本、方便查看 |
| 并行计算 | rayon | 零配置并行迭代器 |
| CLI 参数 | clap | 功能完整、文档丰富（V4 引入） |
| HTML 教学 | 纯 HTML + 内联 CSS | 零依赖、零阅读器、直接浏览器打开 |

## 渐进式学习路径

```
开始 ──────▶ 第1章变量     ──▶ 第2章所有权 ──▶ 第3章借用
                                          
第4章字符串 ──▶ 第5章结构体 ──▶ 第6章生命周期 
                                          
第7章泛型   ──▶ 第8章错误   ──▶ 第9章闭包   
                                          
V1 Basic ◀── 第9章完成 ───▶ 第10章智能指针
                                          
V2 Indexed ◀── trait + serde ──▶ 第11章并发
                                          
V3 Parallel ◀── rayon + Arc ──▶ V4 Web ◀── axum + tokio
```

## Git 管理策略

项目使用 git 管理每个阶段的练习：

```
commit 1: 初始化 workspace 结构 + 第01章 HTML + 练习
commit 2: 第02章 所有权
commit 3: 第03章 借用
...
commit N: 第11章 并发
commit N+1: V1 Basic 文件搜索工具
commit N+2: V2 Indexed KV 索引
...
```

顺序提交，每个阶段可独立回溯对比。

## 可扩展性设计

- **新增章节**：在 exercises 下添加新 mod，Cargo.toml 无需修改
- **新增实战版本**：在 file-toolkit 下添加新 crate，注册到 workspace
- **KV 后端替换**：通过 trait Storage，可替换为 SQLite/RocksDB
- **Web API 扩展**：新增路由即可添加功能
- **教学升级**：HTML 可后续用 mdbook 生成静态站，现有内容直接复用
