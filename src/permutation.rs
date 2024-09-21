use crate::factorial::factorial;

pub fn permutation(n: u64, r: u64) -> u64 {
    factorial(n) / factorial(n - r) // P(n, r) = n! / (n - r)!
}

#[test]
fn test_permutation() {
    assert_eq!(permutation(5, 3), 60);
    assert_eq!(permutation(10, 2), 90);
}
