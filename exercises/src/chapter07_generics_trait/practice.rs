// 第07章实践练习：泛型与 trait
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

pub fn practice1<T>(x: T) -> T {
    todo!("返回 x")
}

trait Describe {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
}

impl Describe for Person {
    fn describe(&self) -> String {
        todo!("返回 name")
    }
}

pub fn print_description(item: impl Describe) -> String {
    item.describe()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() { assert_eq!(practice1(42), 42); }
    #[test]
    #[ignore]
    fn test_practice2() {
        let p = Person { name: "Alice".into() };
        assert_eq!(p.describe(), "Alice");
    }
    #[test]
    #[ignore]
    fn test_practice3() {
        let p = Person { name: "Bob".into() };
        assert_eq!(print_description(p), "Bob");
    }
}
