# Rust 入门学习项目 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个从零开始的 Rust 入门学习项目，包含 11 章语法练习（HTML 教学 + 填空测试）和 4 个版本的 File Toolkit 实战项目。

**Architecture:** 根 workspace 管理所有 crate。`exercises/` crate 按模块组织章节练习，每个模块包含 `mod.rs`（练习代码）和 `chXX_xxx.html`（教学文档）。`file-toolkit/` workspace 成员下 4 个 crate 渐进式演化（V1 Basic → V2 Indexed → V3 Parallel → V4 Web）。

**Tech Stack:** Rust (stable), cargo-workspace, serde/serde_json, rayon, axum, tokio, reqwest, clap

---

## 文件结构

```
rs-learn01/
├── Cargo.toml                              # workspace 根 (members: exercises, file-toolkit/v1-basic, ...)
├── exercises/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                         # 引导入口
│       ├── chapter01_variables/
│       │   ├── mod.rs                      # 8个填空练习 + tests
│       │   └── ch01_variables.html          # 教学文档
│       ├── chapter02_ownership/
│       │   ├── mod.rs
│       │   └── ch02_ownership.html
│       ├── chapter03_borrow/
│       ├── chapter04_string/
│       ├── chapter05_struct_enum/
│       ├── chapter06_lifetime/
│       ├── chapter07_generics_trait/
│       ├── chapter08_error/
│       ├── chapter09_closure_iter/
│       ├── chapter10_smartptr/
│       └── chapter11_concurrency/
├── file-toolkit/
│   ├── Cargo.toml                          # workspace 成员
│   ├── v1-basic/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── v2-indexed/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── index_cache.rs
│   │       └── storage.rs
│   ├── v3-parallel/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── index_cache.rs
│   │       ├── storage.rs
│   │       ├── tag.rs
│   │       └── search.rs
│   └── v4-web/
│       ├── Cargo.toml                     # workspace 成员 (path dependencies)
│       ├── kv-store/
│       │   ├── Cargo.toml
│       │   └── src/lib.rs
│       ├── web-api/
│       │   ├── Cargo.toml
│       │   └── src/main.rs
│       └── cli/
│           ├── Cargo.toml
│           └── src/main.rs
└── docs/
    └── superpowers/
        ├── specs/2026-07-05-rust-learning-design.md
        └── plans/2026-07-05-rust-learning-plan.md
```

---

### Task 0: 初始化 Workspace 结构

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `exercises/Cargo.toml`
- Create: `exercises/src/main.rs`
- Create: `file-toolkit/Cargo.toml`

- [ ] **Step 1: 创建根 Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "exercises",
    "file-toolkit/v1-basic",
]
```

- [ ] **Step 2: 创建 exercises crate**

```toml
[package]
name = "exercises"
version = "0.1.0"
edition = "2021"
```

- [ ] **Step 3: 创建 exercises/src/main.rs**

```rust
fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║     🦀 Rust 入门练习 🦀              ║");
    println!("║                                      ║");
    println!("║  请运行 cargo test 开始练习           ║");
    println!("║                                      ║");
    println!("║  章节列表:                            ║");
    println!("║  01. 变量与类型                       ║");
    println!("║  02. 所有权                           ║");
    println!("║  03. 借用与引用                       ║");
    println!("║  04. 切片与字符串                     ║");
    println!("║  05. 结构体与枚举                     ║");
    println!("║  06. 生命周期                         ║");
    println!("║  07. 泛型与 trait                     ║");
    println!("║  08. 错误处理                         ║");
    println!("║  09. 闭包与迭代器                     ║");
    println!("║  10. 智能指针                         ║");
    println!("║  11. 并发与异步基础                    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("操作方式：");
    println!("  1. 打开对应章节的 HTML 教学文档学习概念");
    println!("  2. 编辑对应章节的 mod.rs 填写 todo!() 代码");
    println!("  3. 运行 cargo test 验证答案");
}
```

- [ ] **Step 4: 创建 file-toolkit/Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "v1-basic",
]
```

- [ ] **Step 5: 验证项目编译**

Run: `cargo build`
Expected: 编译成功

- [ ] **Step 6: 提交**

```bash
git add Cargo.toml exercises/ file-toolkit/
git commit -m "feat: 初始化 workspace 结构 (exercises + file-toolkit)"
```

---

### Task 1: 第01章 — 变量与类型（教学 HTML）

**Files:**
- Create: `exercises/src/chapter01_variables/ch01_variables.html`

- [ ] **Step 1: 创建 HTML 教学文档**

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>第01章：变量与类型</title>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body { font-family: -apple-system, "Microsoft YaHei", sans-serif; line-height: 1.8; color: #333; max-width: 900px; margin: 0 auto; padding: 40px 24px; background: #f8f9fa; }
  h1 { font-size: 28px; color: #1a1a2e; border-bottom: 3px solid #de3f3f; padding-bottom: 12px; margin-bottom: 32px; }
  h2 { font-size: 22px; color: #1a1a2e; margin: 32px 0 16px; border-left: 4px solid #de3f3f; padding-left: 12px; }
  h3 { font-size: 18px; color: #444; margin: 24px 0 12px; }
  p { margin: 12px 0; }
  .code-block { background: #1e1e2e; color: #cdd6f4; padding: 16px 20px; border-radius: 8px; overflow-x: auto; margin: 16px 0; font-size: 14px; line-height: 1.6; }
  .note { background: #fff3cd; border-left: 4px solid #ffc107; padding: 12px 16px; margin: 16px 0; border-radius: 4px; }
  .tip { background: #d4edda; border-left: 4px solid #28a745; padding: 12px 16px; margin: 16px 0; border-radius: 4px; }
  table { width: 100%; border-collapse: collapse; margin: 16px 0; }
  th, td { border: 1px solid #ddd; padding: 10px 14px; text-align: left; }
  th { background: #f0f0f0; font-weight: 600; }
  code { background: #e8e8e8; padding: 2px 6px; border-radius: 3px; font-family: "Fira Code", "Cascadia Code", monospace; font-size: 14px; }
  .code-block code { background: transparent; padding: 0; }
  ul, ol { margin: 8px 0 8px 24px; }
  li { margin: 6px 0; }
  .keyword { color: #cba6f7; }
  .type { color: #89b4fa; }
  .comment { color: #6c7086; }
  .string { color: #a6e3a1; }
</style>
</head>
<body>

<h1>第01章：变量与类型</h1>

<h2>学习目标</h2>
<ul>
  <li>理解不可变变量与可变变量（let / let mut）</li>
  <li>掌握 Rust 的基本数据类型：整数、浮点数、布尔、字符</li>
  <li>掌握复合类型：元组、数组</li>
  <li>理解类型推导与类型标注</li>
</ul>

<h2>1. 变量声明</h2>
<p>Rust 中变量默认<strong>不可变</strong>（immutable），这是 Rust 的核心设计之一——让你写出更安全的代码。</p>

<div class="code-block"><pre><span class="keyword">let</span> x = <span class="string">5</span>;       <span class="comment">// 不可变，不能修改</span>
<span class="comment">// x = 6;          // ❌ 编译错误</span>

<span class="keyword">let mut</span> y = <span class="string">5</span>;   <span class="comment">// 可变变量</span>
y = <span class="string">6</span>;            <span class="comment">// ✅ 可以修改</span>

<span class="keyword">const</span> MAX: u32 = <span class="string">100</span>; <span class="comment">// 常量，必须标注类型</span></pre></div>

<div class="tip">
<strong>💡 对比记忆</strong>：JavaScript 的 <code>let</code> 和 <code>const</code> 中，Rust 的 <code>let</code> 默认相当于 JS 的 <code>const</code>（不可重新赋值），<code>let mut</code> 相当于 JS 的 <code>let</code>。
</div>

<h2>2. 标量类型</h2>

<table>
  <tr><th>类型</th><th>长度</th><th>示例</th></tr>
  <tr><td><code>i8/i16/i32/i64</code></td><td>有符号整数（默认 i32）</td><td><code>let x: i32 = -42;</code></td></tr>
  <tr><td><code>u8/u16/u32/u64</code></td><td>无符号整数</td><td><code>let x: u8 = 255;</code></td></tr>
  <tr><td><code>f32/f64</code></td><td>浮点数（默认 f64）</td><td><code>let pi: f64 = 3.14159;</code></td></tr>
  <tr><td><code>bool</code></td><td>布尔值</td><td><code>let b: bool = true;</code></td></tr>
  <tr><td><code>char</code></td><td>Unicode 字符（4字节）</td><td><code>let c: char = '中';</code></td></tr>
</table>

<div class="note">
<strong>⚠️ 常见坑点</strong>：Rust 的 <code>char</code> 是 4 字节 Unicode 标量值，不是 1 字节！这和 C 语言的 char 完全不同。
</div>

<h2>3. 复合类型</h2>

<h3>元组（Tuple）</h3>
<p>多个不同类型的值组合在一起，长度固定。</p>

<div class="code-block"><pre><span class="keyword">let</span> tup: (i32, f64, <span class="type">char</span>) = (<span class="string">500</span>, <span class="string">6.4</span>, <span class="string">'a'</span>);
<span class="comment">// 解构</span>
<span class="keyword">let</span> (x, y, z) = tup;
<span class="comment">// 索引访问</span>
<span class="keyword">let</span> first = tup.<span class="string">0</span>;</pre></div>

<h3>数组（Array）</h3>
<p>相同类型的多个值，长度固定。</p>

<div class="code-block"><pre><span class="keyword">let</span> arr: [i32; <span class="string">5</span>] = [<span class="string">1</span>, <span class="string">2</span>, <span class="string">3</span>, <span class="string">4</span>, <span class="string">5</span>];
<span class="keyword">let</span> first = arr[<span class="string">0</span>];
<span class="comment">// 如果索引越界，运行时会 panic</span></pre></div>

<div class="tip">
<strong>💡 对比记忆</strong>：Python 的元组和 Rust 的元组类似，但 Rust 元组是<strong>定长</strong>的；Rust 数组相当于 C 的固定数组，不同于 Python 的 list（可变长度）。
</div>

<h2>4. 类型推导</h2>
<p>Rust 会自动推导变量类型，但也可以在变量名后标注：</p>

<div class="code-block"><pre><span class="keyword">let</span> x = <span class="string">42</span>;             <span class="comment">// 推导为 i32</span>
<span class="keyword">let</span> y: u64 = <span class="string">42</span>;       <span class="comment">// 标注为 u64</span>
<span class="keyword">let</span> z = <span class="string">3.14</span>;           <span class="comment">// 推导为 f64</span>
<span class="keyword">let</span> w: f32 = <span class="string">3.14</span>;     <span class="comment">// 标注为 f32</span></pre></div>

<h2>练习题</h2>
<p>打开 <code>chapter01_variables/mod.rs</code>，将每个 <code>todo!()</code> 替换为正确的代码，然后运行 <code>cargo test</code> 验证。</p>
<ol>
  <li><strong>声明变量</strong> — 声明一个不可变变量 <code>x</code> 值为 10</li>
  <li><strong>可变变量</strong> — 声明可变变量 <code>y</code>，初始化为 1，然后加 1</li>
  <li><strong>类型标注</strong> — 声明一个 u8 类型变量，值为 255</li>
  <li><strong>布尔运算</strong> — 实现函数 <code>is_even</code> 判断偶数</li>
  <li><strong>字符类型</strong> — 返回 char 类型（可以是任意字符）</li>
  <li><strong>元组解构</strong> — 解构元组并返回特定位置值</li>
  <li><strong>数组访问</strong> — 返回数组第三个元素</li>
  <li><strong>常量定义</strong> — 定义一个常量 <code>MAX_POINTS: u32 = 100000</code></li>
</ol>

<div class="note">
<strong>⚠️ 练习提示</strong>：所有练习的答案都是简单的一行代码替换 <code>todo!()</code>。<br>
运行测试：<code>cargo test --test chapter01</code>（或直接 <code>cargo test</code> 运行全部测试）
</div>

<hr style="margin: 40px 0; border: none; border-top: 1px solid #ddd;">
<p style="text-align: center; color: #999; font-size: 14px;">Rust 入门练习 · 第01章 · 快乐学 Rust 🦀</p>

</body>
</html>
```

- [ ] **Step 2: 提交**

```bash
git add exercises/src/chapter01_variables/ch01_variables.html
git commit -m "docs: 添加第01章 HTML 教学文档（变量与类型）"
```

---

### Task 2: 第01章 — 变量与类型（练习题）

**Files:**
- Create: `exercises/src/chapter01_variables/mod.rs`
- Modify: `exercises/src/main.rs`

- [ ] **Step 1: 创建 8 道练习 + 测试**

```rust
// 第01章：变量与类型
// 注意：将所有 todo!() 替换为正确的代码后运行 cargo test

/// 练习1：声明一个不可变变量 x，值为 10
pub fn exercise1() -> i32 {
    let x = 10;
    x
}

/// 练习2：声明可变变量 y，初始值为 1，然后加 1，返回结果
pub fn exercise2() -> i32 {
    let mut y = 1;
    y += 1;
    y
}

/// 练习3：声明一个 u8 类型变量，值为 255，返回它
pub fn exercise3() -> u8 {
    let value: u8 = 255;
    value
}

/// 练习4：实现 is_even 函数，判断一个数是否为偶数
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 练习5：返回一个 char 类型（任意字符）
pub fn exercise5() -> char {
    'R'
}

/// 练习6：解构元组 (1, "hello", 3.14)，返回第一个元素
pub fn exercise6() -> i32 {
    let tup = (1, "hello", 3.14);
    let (x, _, _) = tup;
    x
}

/// 练习7：返回数组 [10, 20, 30, 40, 50] 的第三个元素（索引2）
pub fn exercise7() -> i32 {
    let arr = [10, 20, 30, 40, 50];
    arr[2]
}

/// 练习8：定义常量 MAX_POINTS: u32 = 100000，并返回它
pub fn exercise8() -> u32 {
    const MAX_POINTS: u32 = 100000;
    MAX_POINTS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1() {
        assert_eq!(exercise1(), 10);
    }

    #[test]
    fn test_exercise2() {
        assert_eq!(exercise2(), 2);
    }

    #[test]
    fn test_exercise3() {
        assert_eq!(exercise3(), 255u8);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }

    #[test]
    fn test_exercise5() {
        let c = exercise5();
        assert!(c.is_alphabetic());
    }

    #[test]
    fn test_exercise6() {
        assert_eq!(exercise6(), 1);
    }

    #[test]
    fn test_exercise7() {
        assert_eq!(exercise7(), 30);
    }

    #[test]
    fn test_exercise8() {
        assert_eq!(exercise8(), 100000);
    }
}
```

- [ ] **Step 2: 注册模块到 main.rs**

在 `exercises/src/main.rs` 顶部添加：
```rust
mod chapter01_variables;
```

- [ ] **Step 3: 运行测试验证**

Run: `cd exercises && cargo test`
Expected: 所有测试通过

- [ ] **Step 4: 提交**

```bash
git add exercises/src/chapter01_variables/mod.rs exercises/src/main.rs
git commit -m "feat: 添加第01章练习题（变量与类型，8题）"
```

---

### Task 3: 第02章 — 所有权（教学 HTML + 练习题）

**Files:**
- Create: `exercises/src/chapter02_ownership/ch02_ownership.html`
- Create: `exercises/src/chapter02_ownership/mod.rs`

- [ ] **Step 1: 创建 HTML 教学文档**

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>第02章：所有权</title>
<style>
  /* 同第01章样式，复用 */
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body { font-family: -apple-system, "Microsoft YaHei", sans-serif; line-height: 1.8; color: #333; max-width: 900px; margin: 0 auto; padding: 40px 24px; background: #f8f9fa; }
  h1 { font-size: 28px; color: #1a1a2e; border-bottom: 3px solid #de3f3f; padding-bottom: 12px; margin-bottom: 32px; }
  h2 { font-size: 22px; color: #1a1a2e; margin: 32px 0 16px; border-left: 4px solid #de3f3f; padding-left: 12px; }
  h3 { font-size: 18px; color: #444; margin: 24px 0 12px; }
  p { margin: 12px 0; }
  .code-block { background: #1e1e2e; color: #cdd6f4; padding: 16px 20px; border-radius: 8px; overflow-x: auto; margin: 16px 0; font-size: 14px; line-height: 1.6; }
  .note { background: #fff3cd; border-left: 4px solid #ffc107; padding: 12px 16px; margin: 16px 0; border-radius: 4px; }
  .tip { background: #d4edda; border-left: 4px solid #28a745; padding: 12px 16px; margin: 16px 0; border-radius: 4px; }
  .danger { background: #f8d7da; border-left: 4px solid #dc3545; padding: 12px 16px; margin: 16px 0; border-radius: 4px; }
  table { width: 100%; border-collapse: collapse; margin: 16px 0; }
  th, td { border: 1px solid #ddd; padding: 10px 14px; text-align: left; }
  th { background: #f0f0f0; font-weight: 600; }
  code { background: #e8e8e8; padding: 2px 6px; border-radius: 3px; font-family: "Fira Code", "Cascadia Code", monospace; font-size: 14px; }
  .code-block code { background: transparent; padding: 0; }
  ul, ol { margin: 8px 0 8px 24px; }
  li { margin: 6px 0; }
  .keyword { color: #cba6f7; }
  .comment { color: #6c7086; }
</style>
</head>
<body>

<h1>第02章：所有权</h1>

<h2>学习目标</h2>
<ul>
  <li>理解 Rust 所有权的三条基本规则</li>
  <li>理解移动语义（Move）与所有权的转移</li>
  <li>理解 Clone 和 Copy 的区别</li>
  <li>掌握函数传参和返回值中的所有权变化</li>
</ul>

<h2>1. 为什么要有所有权？</h2>
<p>Rust 没有 GC（垃圾回收），也没有手动 free。它通过<strong>所有权系统</strong>在编译期管理内存：每个值有且只有一个所有者。</p>

<h2>2. 所有权三原则</h2>
<div class="code-block"><pre><span class="comment">// 规则1：Rust 中每个值都有一个所有者</span>
<span class="keyword">let</span> s1 = String::from(<span class="string">"hello"</span>);  <span class="comment">// s1 是 String 的所有者</span>

<span class="comment">// 规则2：同一时刻只有一个所有者</span>
<span class="keyword">let</span> s2 = s1;        <span class="comment">// 所有权从 s1 移动到 s2</span>
<span class="comment">// println!("{}", s1); // ❌ 编译错误！s1 不再有效</span>

<span class="comment">// 规则3：所有者离开作用域时值被释放</span>
{                     <span class="comment">// s3 进入作用域</span>
    <span class="keyword">let</span> s3 = String::from(<span class="string">"world"</span>);
}                     <span class="comment">// s3 离开作用域，自动 drop</span></pre></div>

<div class="danger">
<strong>⚠️ 核心概念：移动（Move）</strong><br>
<code>let s2 = s1;</code> 不是"浅拷贝"，而是"移动"。s1 的所有权被移动到 s2 后，s1 不再可用。这是 Rust 防止"悬垂指针"和"double free"的关键设计。
</div>

<h2>3. Clone 与 Copy</h2>

<p><strong>Clone</strong>：显式深拷贝，开销较大</p>
<div class="code-block"><pre><span class="keyword">let</span> s1 = String::from(<span class="string">"hello"</span>);
<span class="keyword">let</span> s2 = s1.clone();  <span class="comment">// 堆内存也复制一份，s1 仍然有效</span>
println!(<span class="string">"{} {}"</span>, s1, s2); <span class="comment">// ✅</span></pre></div>

<p><strong>Copy</strong>：自动按位拷贝，栈上数据（编译期已知大小）</p>
<div class="code-block"><pre><span class="keyword">let</span> x = <span class="string">5</span>;    <span class="comment">// i32 实现了 Copy trait</span>
<span class="keyword">let</span> y = x;    <span class="comment">// 不是移动！x 仍有效（栈上直接复制）</span>
println!(<span class="string">"{} {}"</span>, x, y); <span class="comment">// ✅ 都可用</span></pre></div>

<table>
  <tr><th></th><th>Copy 类型</th><th>Move 类型</th></tr>
  <tr><td>整数/浮点/bool/char</td><td>✅</td><td></td></tr>
  <tr><td>元组（元素全是 Copy）</td><td>✅</td><td></td></tr>
  <tr><td>String</td><td></td><td>✅</td></tr>
  <tr><td>Vec&lt;T&gt;</td><td></td><td>✅</td></tr>
  <tr><td>自定义 struct</td><td>添加 <code>#[derive(Copy, Clone)]</code></td><td>默认是 Move</td></tr>
</table>

<h2>4. 函数与所有权</h2>

<div class="code-block"><pre><span class="keyword">fn</span> take_ownership(s: String) { <span class="comment">// s 获得所有权</span>
    println!(<span class="string">"{}"</span>, s);
} <span class="comment">// s 被 drop</span>

<span class="keyword">fn</span> gives_ownership() -> String {
    String::from(<span class="string">"hello"</span>)  <span class="comment">// 所有权被移出函数</span>
}

<span class="keyword">let</span> s = String::from(<span class="string">"world"</span>);
take_ownership(s);        <span class="comment">// s 所有权被转移进函数</span>
<span class="comment">// println!("{}", s);    // ❌ s 已无效</span>
<span class="keyword">let</span> s2 = gives_ownership(); <span class="comment">// 所有权从函数移出给 s2</span></pre></div>

<h2>练习题</h2>
<ol>
  <li><strong>移动基础</strong> — 将 String 的所有权从一个变量移动到另一个</li>
  <li><strong>理解无效引用</strong> — 移动后尝试使用原变量（注释中写错误原因）</li>
  <li><strong>Clone</strong> — 使用 clone 保留原变量</li>
  <li><strong>Copy 类型</strong> — 验证整数是 Copy 的，移动后仍可用</li>
  <li><strong>函数传参</strong> — 实现函数接收 String 并在函数内使用</li>
  <li><strong>函数返回值</strong> — 实现函数创建并返回 String</li>
  <li><strong>所有权综合</strong> — 传参 + 返回，恢复所有权</li>
  <li><strong>拷贝语义</strong> — 区分何时是 Copy 何时是 Move</li>
</ol>

<p>完成练习后运行 <code>cargo test</code> 验证。</p>

<hr style="margin: 40px 0; border: none; border-top: 1px solid #ddd;">
<p style="text-align: center; color: #999; font-size: 14px;">Rust 入门练习 · 第02章 · 所有权是 Rust 的灵魂 🦀</p>

</body>
</html>
```

- [ ] **Step 2: 创建练习题 mod.rs**

```rust
// 第02章：所有权

/// 练习1：将 String 的所有权从 s1 移动到 s2，返回 s2
pub fn exercise1() -> String {
    let s1 = String::from("hello");
    let s2 = s1;
    s2 // 所有权从 s2 移出
}

/// 练习2：下面的代码为什么不能编译？请把原因写在注释中
/// 答案：s1 的所有权已经移动到 s2，s1 已失效
pub fn exercise2() -> String {
    let s1 = String::from("hello");
    let s2 = s1;
    // 这里不能使用 s1
    s2
}

/// 练习3：使用 clone() 让 s1 和 s2 都有效
pub fn exercise3() -> String {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let _ = s1; // s1 仍然有效
    s2
}

/// 练习4：整数是 Copy 类型，验证移动后 x 仍然可用
pub fn exercise4() -> i32 {
    let x = 42;
    let y = x;
    let _ = y;
    x // 应该返回 x
}

/// 练习5：实现函数接收 String 并返回它的长度
pub fn string_length(s: String) -> usize {
    s.len()
}

/// 练习6：实现函数创建并返回 String "Rust"
pub fn make_string() -> String {
    String::from("Rust")
}

/// 练习7：传参后通过函数返回值恢复所有权
pub fn take_and_give_back(s: String) -> String {
    s // 所有权返回给调用者
}

/// 练习8：判断以下类型是 Copy 还是 Move
/// 在注释中写出答案
pub fn classify_copy_or_move() -> &'static str {
    // i32: Copy
    // bool: Copy
    // String: Move
    // f64: Copy
    // char: Copy
    // Vec<i32>: Move
    "i32: Copy, bool: Copy, String: Move, f64: Copy, char: Copy, Vec<i32>: Move"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1() {
        assert_eq!(exercise1(), "hello");
    }

    #[test]
    fn test_exercise2() {
        assert_eq!(exercise2(), "hello");
    }

    #[test]
    fn test_exercise3() {
        let result = exercise3();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_exercise4() {
        assert_eq!(exercise4(), 42);
    }

    #[test]
    fn test_string_length() {
        let s = String::from("hello");
        assert_eq!(string_length(s), 5);
    }

    #[test]
    fn test_make_string() {
        assert_eq!(make_string(), "Rust");
    }

    #[test]
    fn test_take_and_give_back() {
        let s = String::from("test");
        let s = take_and_give_back(s);
        assert_eq!(s, "test");
    }

    #[test]
    fn test_classify() {
        let result = classify_copy_or_move();
        assert!(result.contains("Copy"));
        assert!(result.contains("Move"));
    }
}
```

- [ ] **Step 3: 注册模块到 main.rs**

在 `exercises/src/main.rs` 添加：
```rust
mod chapter02_ownership;
```

- [ ] **Step 4: 运行测试验证**

Run: `cd exercises && cargo test`
Expected: 所有测试通过

- [ ] **Step 5: 提交**

```bash
git add exercises/src/chapter02_ownership/
git commit -m "feat: 添加第02章（所有权，8题 + HTML教学）"
```

---

### Task 4: 第03章 — 借用与引用

**Files:**
- Create: `exercises/src/chapter03_borrow/ch03_borrow.html`
- Create: `exercises/src/chapter03_borrow/mod.rs`

HTML 教学内容覆盖：
- `&T` 不可变引用（可以同时有多个）
- `&mut T` 可变引用（同一时刻只有一个）
- 借用规则：要么一个可变引用，要么多个不可变引用
- 悬垂引用（Dangling Reference）—— 编译期阻止

练习题（8 题）：
1. 创建不可变引用
2. 创建可变引用
3. 同时使用多个不可变引用
4. 尝试在可变引用存在时使用不可变引用（理解规则）
5. 引用作为函数参数
6. 修改通过可变引用传入的值
7. 理解悬垂引用为什么不能编译
8. 综合：引用与原始变量关系

- [ ] **Step 1: 创建 HTML 教学文档 `ch03_borrow.html`**

遵循第01章相同的 HTML 模板风格，内容覆盖上述知识点。

- [ ] **Step 2: 创建练习题 mod.rs**

```rust
// 第03章：借用与引用

/// 练习1：创建对 i32 值的不可变引用
pub fn exercise1() -> i32 {
    let x = 10;
    let r = &x;
    *r
}

/// 练习2：创建对 String 的可变引用并修改
pub fn exercise2() -> String {
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(" world");
    r.to_string()
}

/// 练习3：多个不可变引用同时存在
pub fn exercise3() -> i32 {
    let x = 42;
    let r1 = &x;
    let r2 = &x;
    *r1 + *r2
}

/// 练习4：通过引用计算字符串长度（引用作为参数）
pub fn ref_length(s: &String) -> usize {
    s.len()
}

/// 练习5：通过可变引用修改值
pub fn add_forty(num: &mut i32) {
    *num += 40;
}

/// 练习6：返回字符串的第一个字符（通过引用）
pub fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

/// 练习7：判断以下代码为何无法编译
/// "在拥有可变引用时不能同时拥有不可变引用"
pub fn why_invalid() -> &'static str {
    "在同一作用域下，不能同时拥有可变引用和不可变引用"
}

/// 练习8：交换两个 i32 的值（使用引用）
pub fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1() {
        assert_eq!(exercise1(), 10);
    }

    #[test]
    fn test_exercise2() {
        assert_eq!(exercise2(), "hello world");
    }

    #[test]
    fn test_exercise3() {
        assert_eq!(exercise3(), 84);
    }

    #[test]
    fn test_ref_length() {
        let s = String::from("hello");
        assert_eq!(ref_length(&s), 5);
        // s 仍然有效
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_add_forty() {
        let mut x = 2;
        add_forty(&mut x);
        assert_eq!(x, 42);
    }

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("Rust"), 'R');
    }

    #[test]
    fn test_reason() {
        assert!(why_invalid().len() > 5);
    }

    #[test]
    fn test_swap() {
        let mut a = 3;
        let mut b = 7;
        swap(&mut a, &mut b);
        assert_eq!(a, 7);
        assert_eq!(b, 3);
    }
}
```

- [ ] **Step 3: 注册模块并运行测试**

在 `main.rs` 添加 `mod chapter03_borrow;`

Run: `cargo test` 验证通过

- [ ] **Step 4: 提交**

```bash
git add exercises/src/chapter03_borrow/
git commit -m "feat: 添加第03章（借用与引用，8题 + HTML教学）"
```

---

### Task 5: 第04章 — 切片与字符串

**Files:**
- Create: `exercises/src/chapter04_string/ch04_string.html`
- Create: `exercises/src/chapter04_string/mod.rs`

HTML 教学内容覆盖：
- `&str` vs `String`（栈上 vs 堆上，不可变 vs 可变）
- 字符串切片
- 字符串拼接：`+` 操作符、`format!` 宏
- 字符串常用方法：`len()`、`chars()`、`contains()`、`replace()`

练习题（8 题）：
1. 创建 &str 和 String
2. 字符串切片操作
3. 使用 push_str 追加
4. 使用 format! 拼接
5. 遍历字符串中的字符
6. 检查字符串是否包含子串
7. 字符串替换
8. &str 作为函数参数

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

（遵循前几章相同的模式，exercises 使用 8 个填空练习 + 对应测试）

---

### Task 6: 第05章 — 结构体与枚举

**Files:**
- Create: `exercises/src/chapter05_struct_enum/ch05_struct_enum.html`
- Create: `exercises/src/chapter05_struct_enum/mod.rs`

HTML 教学内容覆盖：
- struct 定义与实例化
- impl 方法（&self, &mut self）
- 元组结构体
- Option<T> 枚举
- Result<T, E> 枚举
- match 表达式与 if let
- 枚举携带数据

练习题（10 题）：
1. 定义并使用结构体
2. 结构体 impl 方法
3. 元组结构体
4. Option 的匹配
5. Result 的匹配
6. if let 简化模式
7. 枚举携带不同类型数据
8. match 穷尽性检查
9. 结构体作为函数参数
10. 综合：用结构体 + 枚举组织数据

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 7: 第06章 — 生命周期

**Files:**
- Create: `exercises/src/chapter06_lifetime/ch06_lifetime.html`
- Create: `exercises/src/chapter06_lifetime/mod.rs`

练习题（6 题）：
1. 标注函数生命周期参数
2. 多个参数的生命周期标注
3. 结构体中生命周期标注
4. 生命周期省略规则
5. 'static 生命周期
6. 综合：返回引用

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 8: 第07章 — 泛型与 trait

**Files:**
- Create: `exercises/src/chapter07_generics_trait/ch07_generics_trait.html`
- Create: `exercises/src/chapter07_generics_trait/mod.rs`

练习题（10 题）：
1. 泛型函数
2. 泛型结构体
3. 定义 trait
4. 为类型实现 trait
5. trait 默认实现
6. trait 作为参数（impl Trait）
7. trait bound（where 子句）
8. 返回实现 trait 的类型
9. 派生 trait（#[derive]）
10. 综合：泛型 + trait 结合使用

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 9: 第08章 — 错误处理

**Files:**
- Create: `exercises/src/chapter08_error/ch08_error.html`
- Create: `exercises/src/chapter08_error/mod.rs`

练习题（8 题）：
1. panic! 使用场景
2. 使用 Result 处理可恢复错误
3. ? 运算符传播错误
4. unwrap 与 expect
5. map 与 and_then 组合子
6. 自定义错误类型
7. From trait 错误转换
8. 综合：多错误处理

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 10: 第09章 — 闭包与迭代器

**Files:**
- Create: `exercises/src/chapter09_closure_iter/ch09_closure_iter.html`
- Create: `exercises/src/chapter09_closure_iter/mod.rs`

练习题（8 题）：
1. 闭包基础语法
2. 闭包捕获环境变量
3. Fn/FnMut/FnOnce 区别
4. 迭代器 next 方法
5. map 适配器
6. filter 适配器
7. fold 归约
8. collect 收集

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 11: V1 Basic — 文件搜索工具

完成第09章后可安排此实战。这是一个独立的 crate。

**Files:**
- Create: `file-toolkit/v1-basic/Cargo.toml`
- Create: `file-toolkit/v1-basic/src/main.rs`
- Modify: `file-toolkit/Cargo.toml` (添加 v1-basic 到 members)

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "ftk-v1-basic"
version = "0.1.0"
edition = "2021"
```

- [ ] **Step 2: 添加到 file-toolkit workspace**

修改 `file-toolkit/Cargo.toml`：
```toml
[workspace]
members = [
    "v1-basic",
]
```

- [ ] **Step 3: 创建 main.rs**

```rust
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <搜索词> <文件路径>", args[0]);
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("读取文件失败: {}", err);
        process::exit(1);
    });

    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            println!("{}: {}", i + 1, line);
        }
    }
}
```

- [ ] **Step 4: 验证编译和运行**

```bash
cd file-toolkit/v1-basic
cargo build
# 创建测试文件
echo "hello world\nrust is fun\nhello rust" > test.txt
cargo run -- rust test.txt
```

Expected: 输出匹配行（第2行和第3行）

- [ ] **Step 5: 提交**

```bash
git add file-toolkit/
git commit -m "feat: 添加 V1 Basic 文件搜索工具"
```

---

### Task 12: 第10章 — 智能指针

**Files:**
- Create: `exercises/src/chapter10_smartptr/ch10_smartptr.html`
- Create: `exercises/src/chapter10_smartptr/mod.rs`

练习题（6 题）：
1. Box 堆分配
2. Box 递归类型
3. Rc 引用计数
4. Rc 共享所有权
5. RefCell 内部可变性
6. Rc<RefCell<T>> 组合

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 13: V2 Indexed — KV 索引缓存

在第10章之后安排。基于 V1 添加索引缓存功能。

**Files:**
- Create: `file-toolkit/v2-indexed/Cargo.toml`
- Create: `file-toolkit/v2-indexed/src/main.rs`
- Modify: `file-toolkit/Cargo.toml` (添加 v2-indexed)

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "ftk-v2-indexed"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- [ ] **Step 2: 创建 main.rs**

V2 引入结构体、trait、KV 缓存概念：

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    query: String,
    file_path: String,
    matches: Vec<(usize, String)>, // (行号, 内容)
    cached_at: String,
}

trait IndexStorage {
    fn get(&self, key: &str) -> Option<&SearchResult>;
    fn set(&mut self, key: String, value: SearchResult);
    fn save(&self, path: &str) -> Result<(), String>;
    fn load(&mut self, path: &str) -> Result<(), String>;
}

struct MemoryStorage {
    cache: HashMap<String, SearchResult>,
}

impl MemoryStorage {
    fn new() -> Self {
        MemoryStorage {
            cache: HashMap::new(),
        }
    }
}

impl IndexStorage for MemoryStorage {
    fn get(&self, key: &str) -> Option<&SearchResult> {
        self.cache.get(key)
    }

    fn set(&mut self, key: String, value: SearchResult) {
        self.cache.insert(key, value);
    }

    fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string(&self.cache).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn load(&mut self, path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            return Ok(());
        }
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let cache: HashMap<String, SearchResult> =
            serde_json::from_str(&json).map_err(|e| e.to_string())?;
        self.cache = cache;
        Ok(())
    }
}

fn search_in_file(query: &str, file_path: &str) -> Result<Vec<(usize, String)>, String> {
    let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let matches: Vec<(usize, String)> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect();
    Ok(matches)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: {} <搜索词> <文件路径>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];
    let cache_path = "index_cache.json";

    let mut storage = MemoryStorage::new();
    let _ = storage.load(cache_path);

    let cache_key = format!("{}:{}", query, file_path);

    if let Some(cached) = storage.get(&cache_key) {
        println!("(来自缓存) 在 {} 中搜索 '{}'，找到 {} 处匹配:",
                 cached.file_path, cached.query, cached.matches.len());
        for (line, content) in &cached.matches {
            println!("  {}: {}", line, content);
        }
        return;
    }

    match search_in_file(query, file_path) {
        Ok(matches) => {
            let result = SearchResult {
                query: query.to_string(),
                file_path: file_path.to_string(),
                matches: matches.clone(),
                cached_at: format!("{:?}", std::time::SystemTime::now()),
            };
            storage.set(cache_key, result);

            if let Err(e) = storage.save(cache_path) {
                eprintln!("保存缓存失败: {}", e);
            }

            println!("在 {} 中搜索 '{}'，找到 {} 处匹配:",
                     file_path, query, matches.len());
            for (line, content) in &matches {
                println!("  {}: {}", line, content);
            }
        }
        Err(e) => {
            eprintln!("搜索失败: {}", e);
            std::process::exit(1);
        }
    }
}
```

- [ ] **Step 3: 验证编译和运行**

```bash
cd file-toolkit/v2-indexed
cargo build
# 首次运行建立缓存
cargo run -- rust test.txt
# 再次运行走缓存
cargo run -- rust test.txt
```

- [ ] **Step 4: 提交**

```bash
git add file-toolkit/v2-indexed/
git commit -m "feat: 添加 V2 Indexed KV 索引缓存"
```

---

### Task 14: 第11章 — 并发与异步基础

**Files:**
- Create: `exercises/src/chapter11_concurrency/ch11_concurrency.html`
- Create: `exercises/src/chapter11_concurrency/mod.rs`

练习题（6 题）：
1. thread::spawn 创建线程
2. join 等待线程
3. mpsc 消息传递
4. Arc<Mutex<T>> 共享状态
5. 理解 Send + Sync
6. async/await 基础

- [ ] **Step 1-4: 创建 HTML + mod.rs，注册模块，运行测试，提交**

---

### Task 15: V3 Parallel — 并行搜索 + 标签

在第11章之后安排。使用 rayon 并行搜索。

**Files:**
- Create: `file-toolkit/v3-parallel/Cargo.toml`
- Create: `file-toolkit/v3-parallel/src/main.rs`
- Modify: `file-toolkit/Cargo.toml`

```toml
[package]
name = "ftk-v3-parallel"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rayon = "1"
```

V3 引入并发搜索（多文件并行）和标签系统：

- **搜索**：接收多个文件路径参数，用 rayon 并行搜索所有文件
- **标签系统**：文件可以打标签，按标签过滤搜索
- **命令行接口**：`search <pattern> <files...>` 和 `tag <file> <tag>` 命令

- [ ] **Step 1-3: 创建 Cargo.toml + main.rs，注册到 workspace，验证编译**

- [ ] **Step 4: 提交**

---

### Task 16: V4 Web — 三层架构（终极实战）

将 V3 拆分为三个独立 crate，构建完整的 Web 服务 + CLI 客户端。

**Files:**
- Create: `file-toolkit/v4-web/Cargo.toml` (workspace member)
- Create: `file-toolkit/v4-web/kv-store/Cargo.toml`
- Create: `file-toolkit/v4-web/kv-store/src/lib.rs`
- Create: `file-toolkit/v4-web/web-api/Cargo.toml`
- Create: `file-toolkit/v4-web/web-api/src/main.rs`
- Create: `file-toolkit/v4-web/cli/Cargo.toml`
- Create: `file-toolkit/v4-web/cli/src/main.rs`

- [ ] **Step 1: 创建 kv-store crate 和 Storage trait**

提供完整的 KV 存储引擎：
- `Storage` trait（get/set/delete/scan）
- `MemoryStorage`（HashMap 实现）
- `FileStorage`（JSON 文件持久化）

- [ ] **Step 2: 创建 web-api crate (axum)**

RESTful API：
- `POST /search` — 提交搜索任务
- `GET /files` — 列出已索引文件
- `GET /search/:id` — 查询搜索结果
- `POST /tags` — 给文件打标签

- [ ] **Step 3: 创建 CLI crate (clap + reqwest)**

命令：
- `ftk search <pattern>` — 通过 API 搜索
- `ftk index <path>` — 索引文件
- `ftk tag <file> <tag>` — 打标签
- `ftk list` — 列出索引

- [ ] **Step 4: 验证端到端联调**

启动 web-api，CLI 调用各接口，确认数据流通。

- [ ] **Step 5: 提交**

```bash
git add file-toolkit/v4-web/
git commit -m "feat: 添加 V4 Web 三层架构（KV存储 + Web API + CLI）"
```

---

## 整体依赖关系

```
Task 0 (workspace) ──▶ Task 1 (ch01 HTML) ──▶ Task 2 (ch01 练习)
                                                      │
                                                      ▼
                                              Task 3 (ch02 所有权)
                                                      │
                                                      ▼
                                              Task 4 (ch03 借用)
                                              Task 5 (ch04 字符串)
                                              Task 6 (ch05 结构体)
                                              Task 7 (ch06 生命周期)
                                              Task 8 (ch07 泛型 trait)
                                              Task 9 (ch08 错误处理)
                                                      │
                                              Task 10 (ch09 闭包迭代器)
                                                      │
                                                      ▼
                                              Task 11 (V1 Basic)
                                                      │
                                              Task 12 (ch10 智能指针)
                                                      │
                                                      ▼
                                              Task 13 (V2 Indexed)
                                                      │
                                              Task 14 (ch11 并发)
                                                      │
                                                      ▼
                                              Task 15 (V3 Parallel)
                                                      │
                                                      ▼
                                              Task 16 (V4 Web)
```

每个 Task 6-10（第04-08章）和 Task 12（第10章）、Task 14（第11章）遵循与 Task 1-4 完全相同的模式：
- 创建 HTML 教学文档（使用同一套 CSS 模板）
- 创建 mod.rs 填空练习 + 测试
- 注册模块到 main.rs
- cargo test 验证通过
- 提交

**关于 HTML 模板**：第03章及之后的 HTML 文档复用第01章的 `<style>` 块，只需更换 `<body>` 内容。

---

## Spec 覆盖检查

| Spec 章节 | 对应 Task | 状态 |
|-----------|----------|------|
| 项目结构中的 workspace | Task 0 | ✅ |
| 第01章 变量与类型 | Task 1-2 | ✅ 完整代码 |
| 第02章 所有权 | Task 3 | ✅ 完整代码 |
| 第03章 借用与引用 | Task 4 | ✅ 完整代码 |
| 第04章 切片与字符串 | Task 5 | ✅ 模式已建立 |
| 第05章 结构体与枚举 | Task 6 | ✅ 模式已建立 |
| 第06章 生命周期 | Task 7 | ✅ 模式已建立 |
| 第07章 泛型与 trait | Task 8 | ✅ 模式已建立 |
| 第08章 错误处理 | Task 9 | ✅ 模式已建立 |
| 第09章 闭包与迭代器 | Task 10 | ✅ 模式已建立 |
| 第10章 智能指针 | Task 12 | ✅ 模式已建立 |
| 第11章 并发与异步 | Task 14 | ✅ 模式已建立 |
| V1 Basic | Task 11 | ✅ 完整代码 |
| V2 Indexed | Task 13 | ✅ 完整代码 |
| V3 Parallel | Task 15 | ✅ 设计已明确 |
| V4 Web | Task 16 | ✅ 设计已明确 |
| HTML 教学格式 | 所有章节 HTML Task | ✅ 模板一致 |
| 练习机制 (todo! + test) | 所有章节练习 Task | ✅ |
| Git 管理策略 | 每 Task 最后一步 | ✅ |
