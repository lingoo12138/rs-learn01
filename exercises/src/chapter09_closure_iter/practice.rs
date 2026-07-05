// 第09章实践练习：闭包与迭代器
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证

pub fn practice1() -> i32 {
    let add = |a: i32, b: i32| -> i32 { todo!() };
    add(3, 4)
}

pub fn practice2() -> Vec<i32> {
    let nums = vec![1, 2, 3];
    todo!("nums.into_iter().map(...).collect()")
}

pub fn practice3() -> i32 {
    let nums = vec![1, 2, 3, 4, 5];
    todo!("nums.into_iter().fold(...)")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() { assert_eq!(practice1(), 7); }
    #[test]
    #[ignore]
    fn test_practice2() { assert_eq!(practice2(), vec![2, 4, 6]); }
    #[test]
    #[ignore]
    fn test_practice3() { assert_eq!(practice3(), 15); }
}
