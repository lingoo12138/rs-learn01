// 第02章：所有权
// 阅读代码理解所有权机制，然后运行 cargo test 验证
#![allow(dead_code)]
pub mod practice;

/// 练习1：将 String 的所有权从 s1 移动到 s2，返回 s2
/// 练习2 与本练习代码相同，但侧重于理解为什么移动后 s1 不能使用
pub fn exercise1() -> String {
    let s1 = String::from("hello");
    let s2 = s1;
    s2
}

/// 练习2：下面的代码为什么不能编译？请把原因写在注释中
/// 答案：s1 的所有权已经移动到 s2，s1 已失效
pub fn exercise2() -> String {
    let s1 = String::from("hello");
    let s2 = s1;
    s2
}

/// 练习3：使用 clone() 让 s1 和 s2 都有效
pub fn exercise3() -> String {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    s2
}

/// 练习4：整数是 Copy 类型，验证移动后 x 仍然可用
pub fn exercise4() -> i32 {
    let x = 42;
    let y = x;
    let _ = y;
    x
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
    s
}

/// 练习8：判断类型是 Copy 还是 Move
pub fn classify_copy_or_move() -> &'static str {
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
        assert_eq!(exercise3(), "hello");
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
        assert!(result.contains("i32: Copy"));
        assert!(result.contains("String: Move"));
        assert!(result.contains("bool: Copy"));
    }
}
