// 第05章实践练习：结构体与枚举
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

struct Book {
    title: String,
    author: String,
}

impl Book {
    pub fn is_author(&self, name: &str) -> bool {
        todo!("检查 self.author 是否等于 name")
    }
}

pub fn practice1() -> Book {
    todo!("创建 Book 实例，标题 Rust编程，作者 Alice")
}

pub fn practice3(x: Option<i32>) -> i32 {
    todo!("match x，Some(v) 返回 v，None 返回 0")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() {
        let b = practice1();
        assert_eq!(b.title, "Rust编程");
        assert_eq!(b.author, "Alice");
    }
    #[test]
    #[ignore]
    fn test_practice2() {
        let b = Book { title: "Rust".into(), author: "Alice".into() };
        assert!(b.is_author("Alice"));
        assert!(!b.is_author("Bob"));
    }
    #[test]
    #[ignore]
    fn test_practice3() {
        assert_eq!(practice3(Some(42)), 42);
        assert_eq!(practice3(None), 0);
    }
}
