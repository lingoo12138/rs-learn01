// 第03章：借用与引用
// 阅读代码理解引用机制，然后运行 cargo test 验证
#![allow(dead_code)]
pub mod practice;

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
        assert_eq!(s.len(), 5); // s still valid!
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
