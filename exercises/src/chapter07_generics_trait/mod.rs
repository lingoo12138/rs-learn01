// 第07章：泛型与 trait
#![allow(dead_code)]

/// 练习1：泛型函数
pub fn identity<T>(x: T) -> T {
    x
}

/// 练习2：泛型结构体
pub struct Pair<T, U> {
    first: T,
    second: U,
}
pub fn make_pair() -> Pair<i32, String> {
    Pair {
        first: 42,
        second: String::from("hello"),
    }
}

/// 练习3：定义 trait
pub trait Greet {
    fn greet(&self) -> String;
}

/// 练习4：为类型实现 trait
struct Dog {
    name: String,
}
impl Greet for Dog {
    fn greet(&self) -> String {
        format!("Woof! I'm {}", self.name)
    }
}

/// 练习5：trait 默认实现
pub trait Hello {
    fn hello(&self) -> String {
        "Hello!".to_string()
    }
}
struct Cat;
impl Hello for Cat {}

/// 练习6：trait 作为参数（impl Trait 语法）
pub fn say_greet(item: impl Greet) -> String {
    item.greet()
}

/// 练习7：trait bound（where 子句）
pub fn greet_twice<T>(item: T) -> String
where
    T: Greet,
{
    format!("{} and again {}", item.greet(), item.greet())
}

/// 练习8：返回实现 trait 的类型
pub fn make_greeter() -> impl Greet {
    Dog {
        name: String::from("Rusty"),
    }
}

/// 练习9：派生 trait
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
pub fn compare_points() -> bool {
    Point { x: 1, y: 2 } == Point { x: 1, y: 2 }
}

/// 练习10：综合：泛型 + trait bound
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
pub fn max_i32() -> i32 {
    max_of(3, 7)
}
pub fn max_str() -> &'static str {
    max_of("abc", "xyz")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(identity(5), 5);
    }
    #[test]
    fn test_pair() {
        let p = make_pair();
        assert_eq!(p.first, 42);
    }
    #[test]
    fn test_dog_greet() {
        let d = Dog {
            name: "Buddy".into(),
        };
        assert_eq!(d.greet(), "Woof! I'm Buddy");
    }
    #[test]
    fn test_cat_hello() {
        assert_eq!(Cat.hello(), "Hello!");
    }
    #[test]
    fn test_say_greet() {
        assert_eq!(
            say_greet(Dog {
                name: "A".into()
            }),
            "Woof! I'm A"
        );
    }
    #[test]
    fn test_greet_twice() {
        let d = Dog {
            name: "Rusty".into(),
        };
        assert_eq!(
            greet_twice(d),
            "Woof! I'm Rusty and again Woof! I'm Rusty"
        );
    }
    #[test]
    fn test_make_greeter() {
        let g = make_greeter();
        assert_eq!(g.greet(), "Woof! I'm Rusty");
    }
    #[test]
    fn test_compare() {
        assert!(compare_points());
    }
    #[test]
    fn test_max_i32() {
        assert_eq!(max_i32(), 7);
    }
    #[test]
    fn test_max_str() {
        assert_eq!(max_str(), "xyz");
    }
}
