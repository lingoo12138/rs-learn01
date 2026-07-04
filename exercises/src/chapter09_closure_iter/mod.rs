// 第09章：闭包与迭代器
#![allow(dead_code)]

/// 练习1：闭包基础语法
pub fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// 练习2：闭包捕获环境变量
pub fn capture_example() -> i32 {
    let x = 10;
    let add_x = |y| y + x;
    add_x(5)
}

/// 练习3：Fn/FnMut/FnOnce
pub fn call_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

/// 练习4：迭代器 next
pub fn first_element() -> i32 {
    let v = vec![1, 2, 3];
    *v.iter().next().unwrap()
}

/// 练习5：map 适配器
pub fn square_all(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().map(|n| n * n).collect()
}

/// 练习6：filter 适配器
pub fn keep_even(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().filter(|n| n % 2 == 0).collect()
}

/// 练习7：fold 归约
pub fn sum_all(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| acc + n)
}

/// 练习8：collect 收集
pub fn double_and_collect(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|n| n * 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_adder() {
        assert_eq!(make_adder(5)(3), 8);
    }
    #[test]
    fn test_capture() {
        assert_eq!(capture_example(), 15);
    }
    #[test]
    fn test_call_fn() {
        assert_eq!(call_fn(|x| x * 2, 5), 10);
    }
    #[test]
    fn test_first() {
        assert_eq!(first_element(), 1);
    }
    #[test]
    fn test_square() {
        assert_eq!(square_all(vec![1, 2, 3]), vec![1, 4, 9]);
    }
    #[test]
    fn test_filter() {
        assert_eq!(keep_even(vec![1, 2, 3, 4]), vec![2, 4]);
    }
    #[test]
    fn test_fold() {
        assert_eq!(sum_all(vec![1, 2, 3]), 6);
    }
    #[test]
    fn test_collect() {
        assert_eq!(double_and_collect(vec![1, 2, 3]), vec![2, 4, 6]);
    }
}
