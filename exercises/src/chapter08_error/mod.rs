// 第08章：错误处理
#![allow(dead_code)]

/// 练习1：panic 使用场景
pub fn will_panic(n: i32) -> i32 {
    if n < 0 {
        panic!("negative number!")
    }
    n
}

/// 练习2：使用 Result 处理可恢复错误
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// 练习3：? 运算符传播错误
pub fn divide_and_double(a: f64, b: f64) -> Result<f64, String> {
    let result = safe_divide(a, b)?;
    Ok(result * 2.0)
}

/// 练习4：unwrap 与 expect
pub fn unwrap_or_default(input: Option<i32>) -> i32 {
    input.unwrap_or(0)
}

/// 练习5：map 与 and_then
pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|n| n * 2)
}

/// 练习6：自定义错误类型
#[derive(Debug)]
enum CalcError {
    DivideByZero,
    NegativeSquareRoot,
}
impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
            CalcError::NegativeSquareRoot => write!(f, "cannot sqrt negative"),
        }
    }
}
impl std::error::Error for CalcError {}

/// 练习7：使用自定义错误
fn sqrt_and_divide(a: f64, b: f64) -> Result<f64, CalcError> {
    if a < 0.0 {
        return Err(CalcError::NegativeSquareRoot);
    }
    if b == 0.0 {
        return Err(CalcError::DivideByZero);
    }
    Ok((a.sqrt()) / b)
}

/// 练习8：综合：多错误处理
fn process_numbers(nums: &[&str]) -> Result<Vec<i32>, String> {
    nums.iter()
        .map(|s| {
            s.parse::<i32>()
                .map_err(|_| format!("cannot parse '{}' as i32", s))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_panic() {
        assert_eq!(will_panic(5), 5);
    }
    #[test]
    fn test_safe_divide_ok() {
        assert!(safe_divide(10.0, 2.0).is_ok());
    }
    #[test]
    fn test_safe_divide_err() {
        assert!(safe_divide(1.0, 0.0).is_err());
    }
    #[test]
    fn test_question_mark() {
        assert_eq!(divide_and_double(10.0, 2.0).unwrap(), 10.0);
    }
    #[test]
    fn test_unwrap() {
        assert_eq!(unwrap_or_default(Some(5)), 5);
        assert_eq!(unwrap_or_default(None), 0);
    }
    #[test]
    fn test_map() {
        assert_eq!(parse_and_double("21"), Some(42));
        assert_eq!(parse_and_double("x"), None);
    }
    #[test]
    fn test_calc_err() {
        assert!(sqrt_and_divide(-1.0, 1.0).is_err());
        assert!(sqrt_and_divide(4.0, 0.0).is_err());
        assert_eq!(sqrt_and_divide(4.0, 2.0).unwrap(), 1.0);
    }
    #[test]
    fn test_process() {
        let r = process_numbers(&["1", "2", "3"]);
        assert_eq!(r.unwrap(), vec![1, 2, 3]);
    }
    #[test]
    fn test_process_err() {
        let r = process_numbers(&["1", "x", "3"]);
        assert!(r.is_err());
    }
}
