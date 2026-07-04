// 第02章实践练习：所有权
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

/// 练习1：将 String 的所有权从 s1 移动到 s2，返回 s2
pub fn practice1() -> String {
    let s1 = String::from("rust");
    todo!("将 s1 所有权移动到 s2，返回 s2")
}

/// 练习2：使用 clone 让 s1 和 s2 都有效，返回 s1（练习 clone）
pub fn practice2() -> String {
    let s1 = String::from("hello");
    todo!("克隆 s1 到 s2，然后返回 s1")
}

/// 练习3：实现函数接收 String，返回它的长度（不获取所有权？用引用！）
pub fn string_len(s: &String) -> usize {
    todo!("返回字符串长度，使用 s.len()")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_practice1() {
        assert_eq!(practice1(), "rust");
    }

    #[test]
    #[ignore]
    fn test_practice2() {
        assert_eq!(practice2(), "hello");
    }

    #[test]
    #[ignore]
    fn test_practice3() {
        let s = String::from("hello");
        assert_eq!(string_len(&s), 5);
        // 传入引用后 s 仍然可用
        assert_eq!(s.len(), 5);
    }
}
