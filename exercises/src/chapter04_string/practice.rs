// 第04章实践练习：切片与字符串
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

/// 练习1：将 &str 转换为 String
pub fn practice1() -> String {
    todo!("将 \"hello\" 转换为 String")
}

/// 练习2：返回 "hello world" 中 "world" 部分的切片
pub fn practice2() -> &'static str {
    let s = "hello world";
    todo!("返回 world 部分")
}

/// 练习3：使用 format! 拼接 "Hello" 和 "Rust"
pub fn practice3() -> String {
    let a = "Hello";
    let b = "Rust";
    todo!("返回 \"Hello, Rust!\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() { assert_eq!(practice1(), "hello"); }
    #[test]
    #[ignore]
    fn test_practice2() { assert_eq!(practice2(), "world"); }
    #[test]
    #[ignore]
    fn test_practice3() { assert_eq!(practice3(), "Hello, Rust!"); }
}
