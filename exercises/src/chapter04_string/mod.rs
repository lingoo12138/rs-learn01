// 第04章：切片与字符串
#![allow(dead_code)]

/// 练习1：创建 String 和 &str
pub fn exercise1() -> String {
    let s = String::from("hello");
    s
}

/// 练习2：字符串切片，返回 "world"
pub fn slice_world() -> &'static str {
    let s = "hello world";
    &s[6..11]
}

/// 练习3：使用 push_str 追加
pub fn push_hello() -> String {
    let mut s = String::from("hello");
    s.push_str(" world");
    s
}

/// 练习4：使用 format! 拼接字符串
pub fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// 练习5：返回字符串长度
pub fn string_len(s: &str) -> usize {
    s.len()
}

/// 练习6：检查字符串是否包含子串
pub fn contains_rs(s: &str) -> bool {
    s.contains("rs")
}

/// 练习7：替换字符串中的内容
pub fn replace_hello(s: &str) -> String {
    s.replace("hello", "hi")
}

/// 练习8：遍历字符串中的字符，返回字符个数
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1() { assert_eq!(exercise1(), "hello"); }
    #[test]
    fn test_slice_world() { assert_eq!(slice_world(), "world"); }
    #[test]
    fn test_push_hello() { assert_eq!(push_hello(), "hello world"); }
    #[test]
    fn test_format_greeting() { assert_eq!(format_greeting("Rust"), "Hello, Rust!"); }
    #[test]
    fn test_string_len() { assert_eq!(string_len("hello"), 5); }
    #[test]
    fn test_contains_rs() { assert!(contains_rs("rust.rs")); assert!(!contains_rs("hello")); }
    #[test]
    fn test_replace_hello() { assert_eq!(replace_hello("hello world"), "hi world"); }
    #[test]
    fn test_char_count() { assert_eq!(char_count("你好世界"), 4); }
}
