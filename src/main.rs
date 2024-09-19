// 計算階乘的函式
fn factorial(n: u64) -> u64 {
    (1..=n).product() // Rust的語法糖，從1到n相乘
}

// 計算排列數 P(n, r) 的函式
fn permutation(n: u64, r: u64) -> u64 {
    factorial(n) / factorial(n - r) // P(n, r) = n! / (n - r)!
}

// 計算組合數 C(n, r) 的函式
fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r)) // C(n, r) = n! / (r! * (n - r)!)
}

fn main() {
    // 範例數據
    let n = 5;
    let r = 3;

    // 計算並顯示排列數 P(n, r)
    let p_result = permutation(n, r);
    println!("P({}, {}) = {}", n, r, p_result);

    // 計算並顯示組合數 C(n, r)
    let c_result = combination(n, r);
    println!("C({}, {}) = {}", n, r, c_result);

    // 顯示階乘結果
    let factorial_result = factorial(n);
    println!("{}! = {}", n, factorial_result);
}
