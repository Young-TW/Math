pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

pub fn divide(a: i64, b: i64) -> i64 {
    a / b
}

pub fn power(a: i64, b: i64) -> i64 {
    a.pow(b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(3, 2), 1);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 2), 6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 2), 3);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
    }
}
