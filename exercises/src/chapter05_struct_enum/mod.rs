// 第05章：结构体与枚举
#![allow(dead_code)]

pub mod practice;

/// 练习1：定义并实例化一个结构体
pub struct Person {
    name: String,
    age: u8,
}
pub fn create_person() -> Person {
    Person { name: String::from("Alice"), age: 30 }
}

/// 练习2：为结构体实现方法
impl Person {
    pub fn greet(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }
}

/// 练习3：元组结构体
pub struct Color(i32, i32, i32);
pub fn black() -> Color { Color(0, 0, 0) }

/// 练习4：Option 匹配
pub fn maybe_number(flag: bool) -> Option<i32> {
    if flag { Some(42) } else { None }
}
pub fn unwrap_or_zero(x: Option<i32>) -> i32 {
    match x { Some(v) => v, None => 0 }
}

/// 练习5：Result 匹配
pub fn check_positive(n: i32) -> Result<String, String> {
    if n > 0 { Ok("positive".to_string()) } else { Err("not positive".to_string()) }
}

/// 练习6：if let 简化
pub fn if_let_example(x: Option<i32>) -> i32 {
    if let Some(v) = x { v } else { 0 }
}

/// 练习7：枚举携带不同类型数据
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
pub fn process_message(msg: Message) -> String {
    match msg {
        Message::Quit => "quit".to_string(),
        Message::Move { x, y } => format!("move to {},{}", x, y),
        Message::Write(s) => s,
    }
}

/// 练习8：match 穷尽性检查
pub fn describe_number(n: i32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3..=i32::MAX => "many",
        i32::MIN..=0 => "non-positive",
    }
}

/// 练习9：结构体作为函数参数
pub fn person_age(p: &Person) -> u8 { p.age }

/// 练习10：综合：Rectangle 结构体 + 方法
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    pub fn area(&self) -> u32 { self.width * self.height }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_person() { let p = create_person(); assert_eq!(p.name, "Alice"); }
    #[test]
    fn test_greet() { let p = create_person(); assert_eq!(p.greet(), "Hi, I'm Alice"); }
    #[test]
    fn test_black() { let c = black(); assert_eq!(c.0, 0); }
    #[test]
    fn test_option_some() { assert_eq!(unwrap_or_zero(Some(42)), 42); }
    #[test]
    fn test_option_none() { assert_eq!(unwrap_or_zero(None), 0); }
    #[test]
    fn test_result_ok() { assert!(check_positive(5).is_ok()); }
    #[test]
    fn test_result_err() { assert!(check_positive(-1).is_err()); }
    #[test]
    fn test_if_let() { assert_eq!(if_let_example(Some(10)), 10); }
    #[test]
    fn test_message_write() {
        assert_eq!(process_message(Message::Write("hi".into())), "hi");
    }
    #[test]
    fn test_describe() { assert_eq!(describe_number(5), "many"); }
    #[test]
    fn test_person_age() { let p = create_person(); assert_eq!(person_age(&p), 30); }
    #[test]
    fn test_area() {
        let r = Rectangle { width: 3, height: 4 };
        assert_eq!(r.area(), 12);
    }
    #[test]
    fn test_can_hold() {
        let big = Rectangle { width: 5, height: 5 };
        let small = Rectangle { width: 3, height: 3 };
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }
}
