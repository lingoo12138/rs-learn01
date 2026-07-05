// 第06章实践练习：生命周期
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

pub fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!("如果 x.len() > y.len() 返回 x，否则返回 y")
}

struct Holder<'a> {
    value: &'a str,
}

impl<'a> Holder<'a> {
    pub fn get_value(&self) -> &str {
        todo!("返回 self.value")
    }
}

pub fn make_holder() -> String {
    let h = Holder { value: "hello" };
    h.get_value().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() {
        assert_eq!(longer("abc", "de"), "abc");
    }
    #[test]
    #[ignore]
    fn test_practice2() {
        assert_eq!(make_holder(), "hello");
    }
}
