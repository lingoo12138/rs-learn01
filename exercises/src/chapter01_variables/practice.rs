// 第01章实践练习：变量与类型
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

/// 练习1：声明一个 f64 类型变量 pi，值为 3.14159，返回它
pub fn practice1() -> f64 {
    todo!("声明变量 pi: f64 = 3.14159")
}

/// 练习2：用 let mut 声明计数器 count 从 0 开始，加 5 次 1，返回最终值
pub fn practice2() -> i32 {
    todo!("实现可变变量计数器")
}

/// 练习3：解构元组 ("Alice", 30, "engineer")，返回年龄（第二个元素）
pub fn practice3() -> i32 {
    let person = ("Alice", 30, "engineer");
    todo!("解构 person 并返回 30")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_practice1() {
        let result = practice1();
        assert!((result - 3.14159).abs() < 1e-10);
    }

    #[test]
    #[ignore]
    fn test_practice2() {
        assert_eq!(practice2(), 5);
    }

    #[test]
    #[ignore]
    fn test_practice3() {
        assert_eq!(practice3(), 30);
    }
}
