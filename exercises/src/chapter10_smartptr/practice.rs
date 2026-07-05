// 第10章实践练习：智能指针
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证
use std::cell::RefCell;
use std::rc::Rc;

pub fn practice1() -> Box<i32> {
    todo!("Box::new(100)")
}

pub fn practice2() -> usize {
    let s = Rc::new(String::from("share me"));
    let _a = Rc::clone(&s);
    let _b = Rc::clone(&s);
    todo!("返回 Rc::strong_count(&s)")
}

pub fn practice3() -> i32 {
    let cell = RefCell::new(10);
    todo!("通过 borrow_mut 加 5，然后通过 borrow 返回值")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() { assert_eq!(*practice1(), 100); }
    #[test]
    #[ignore]
    fn test_practice2() { assert_eq!(practice2(), 3); }
    #[test]
    #[ignore]
    fn test_practice3() { assert_eq!(practice3(), 15); }
}
