// 第03章实践练习：借用与引用
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

/// 练习1：创建对 i32 的不可变引用，解引用并返回
pub fn practice1() -> i32 {
    let x = 42;
    todo!("创建 &x 并解引用返回")
}

/// 练习2：通过可变引用将值加 10
pub fn practice2(x: &mut i32) {
    todo!("通过 *x 解引用，加 10")
}

/// 练习3：返回字符串的第一个字符
pub fn practice3(s: &str) -> char {
    todo!("return s.chars().next().unwrap()")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_practice1() {
        assert_eq!(practice1(), 42);
    }

    #[test]
    #[ignore]
    fn test_practice2() {
        let mut v = 5;
        practice2(&mut v);
        assert_eq!(v, 15);
    }

    #[test]
    #[ignore]
    fn test_practice3() {
        assert_eq!(practice3("Rust"), 'R');
    }
}
