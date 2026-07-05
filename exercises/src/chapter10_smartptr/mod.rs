// 第10章：智能指针
#![allow(dead_code)]
pub mod practice;

/// 练习1：Box 堆分配
pub fn box_value(x: i32) -> Box<i32> { Box::new(x) }

/// 练习2：Box 递归类型
#[derive(Debug, PartialEq)]
pub enum List { Cons(i32, Box<List>), Nil }
pub fn create_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

/// 练习3：Rc 引用计数
use std::rc::Rc;
pub fn share_with_rc() -> Rc<String> {
    let s = Rc::new(String::from("hello"));
    let _s2 = Rc::clone(&s);
    s
}

/// 练习4：Rc 共享所有权
pub fn rc_strong_count() -> usize {
    let s = Rc::new(5);
    let _a = Rc::clone(&s);
    let _b = Rc::clone(&s);
    Rc::strong_count(&s)
}

/// 练习5：RefCell 内部可变性
use std::cell::RefCell;
pub fn interior_mutability() -> i32 {
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 1;
    let val = *cell.borrow();
    val
}

/// 练习6：Rc<RefCell<T>> 组合
pub fn shared_mutable_value() -> i32 {
    let value = Rc::new(RefCell::new(10));
    let _v2 = Rc::clone(&value);
    *value.borrow_mut() += 5;
    let val = *value.borrow();
    val
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::*;
    #[test] fn test_box() { assert_eq!(*box_value(42), 42); }
    #[test] fn test_list() { assert_eq!(create_list(), Cons(1, Box::new(Cons(2, Box::new(Nil))))); }
    #[test] fn test_rc() { assert_eq!(*share_with_rc(), "hello"); }
    #[test] fn test_rc_count() { assert_eq!(rc_strong_count(), 3); }
    #[test] fn test_refcell() { assert_eq!(interior_mutability(), 6); }
    #[test] fn test_shared_mutable() { assert_eq!(shared_mutable_value(), 15); }
}
