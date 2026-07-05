// 第08章实践练习：错误处理
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

pub fn practice1(n: i32) -> Result<i32, String> {
    todo!("如果 n > 0 返回 Ok(n)，否则 Err(\"negative\".into())")
}

pub fn practice2(a: i32, b: i32) -> Result<i32, String> {
    let result = practice1(a)?;
    todo!("返回 Ok(result + b)")
}

pub fn practice3(s: &str) -> Option<i32> {
    todo!("s.parse().ok()")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1_ok() { assert!(practice1(5).is_ok()); }
    #[test]
    #[ignore]
    fn test_practice1_err() { assert!(practice1(-1).is_err()); }
    #[test]
    #[ignore]
    fn test_practice2() { assert_eq!(practice2(3, 4).unwrap(), 7); }
    #[test]
    #[ignore]
    fn test_practice3() { assert_eq!(practice3("42"), Some(42)); }
    #[test]
    #[ignore]
    fn test_practice3_invalid() { assert_eq!(practice3("abc"), None); }
}
