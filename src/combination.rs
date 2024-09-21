use crate::factorial::factorial;

pub fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r)) // C(n, r) = n! / (r! * (n - r)!)
}

#[test]
fn test_combination() {
    assert_eq!(combination(5, 3), 10);
    assert_eq!(combination(10, 2), 45);
}
