// lib.rs

/// 计算平方，如果输入是负数则 panic
pub fn square_panic(n: i32) -> i32 {
    if n < 0 {
        panic!("cannot square negative number");
    } else {
        n * n
    }
}

/// 计算平方，如果输入是负数则返回错误
pub fn square(n: i32) -> Result<i32, String> {
    if n < 0 {
        Err("cannot square negative number".into())
    } else {
        Ok(n * n)
    }
}

/// 把字符串转为大写，如果输入为空则返回错误
pub fn to_uppercase(s: &str) -> Result<String, String> {
    if s.is_empty() {
        Err("input cannot be empty".into())
    } else {
        Ok(s.to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- assert 系列 ----------
    #[test]
    fn test_square_with_asserts() {
        let result = square(3).unwrap();
        assert_eq!(result, 9); // 相等
        assert_ne!(result, 10); // 不相等
        assert!(result > 5); // 自定义布尔表达式
    }

    #[test]
    fn test_to_uppercase_with_asserts() {
        let result = to_uppercase("rust").unwrap();
        assert_eq!(result, "RUST");
        assert_ne!(result, "Rust");
        assert!(result.chars().all(|c| c.is_ascii_uppercase()));
    }

    // ---------- 使用 Result<T, E> ----------
    #[test]
    fn test_square_ok() -> Result<(), String> {
        let result = square(4)?; // 使用 ? 传播错误
        assert_eq!(result, 16);
        Ok(())
    }

    #[test]
    fn test_square_err() -> Result<(), String> {
        let result = square(-2);
        assert!(result.is_err()); // 确认错误
        Ok(())
    }

    #[test]
    fn test_to_uppercase_ok() -> Result<(), String> {
        let result = to_uppercase("hello")?;
        assert_eq!(result, "HELLO");
        Ok(())
    }

    #[test]
    fn test_to_uppercase_err() -> Result<(), String> {
        let result = to_uppercase("");
        assert!(result.is_err());
        Ok(())
    }

    // ---------- should_panic ----------
    #[test]
    #[should_panic]
    fn test_square_panic_default() {
        square_panic(-5); // 必然触发 panic
    }

    #[test]
    #[should_panic(expected = "cannot square negative number")]
    fn test_square_panic_with_message() {
        square_panic(-10); // 预期触发特定 panic 消息
    }
}
