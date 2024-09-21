pub fn factorial(n: u64) -> u64 {
    (1..=n).product() // Rust的語法糖，從1到n相乘
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
}
