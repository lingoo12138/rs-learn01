// 第01章：变量与类型
// 阅读每个练习的说明，理解代码含义，然后运行 cargo test 验证
#![allow(dead_code)]
pub mod practice;

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
