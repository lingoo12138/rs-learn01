// 第06章：生命周期
#![allow(dead_code)]

/// 练习1：标注函数生命周期参数——返回两个引用中较短的那个
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// 练习2：结构体中的生命周期
struct Excerpt<'a> {
    part: &'a str,
}
pub fn create_excerpt() -> String {
    let novel = String::from("Call me Ishmael...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { part: first_sentence };
    e.part.to_string()
}

/// 练习3：多个生命周期参数
pub fn first_and_second<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
    first
}

/// 练习4：生命周期省略规则（编译器自动推断）
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// 练习5：'static 生命周期
pub fn static_str() -> &'static str {
    "I live forever"
}

/// 练习6：综合：返回结构体中存储的引用
impl<'a> Excerpt<'a> {
    pub fn get_part(&self) -> &'a str { self.part }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "de"), "abc");
    }
    #[test]
    fn test_excerpt() { assert_eq!(create_excerpt(), "Call me Ishmael"); }
    #[test]
    fn test_first_second() { assert_eq!(first_and_second("a", "b"), "a"); }
    #[test]
    fn test_first_word() { assert_eq!(first_word("hello world"), "hello"); }
    #[test]
    fn test_static() { assert_eq!(static_str(), "I live forever"); }
    #[test]
    fn test_get_part() {
        let s = String::from("test");
        let e = Excerpt { part: &s };
        assert_eq!(e.get_part(), "test");
    }
}
