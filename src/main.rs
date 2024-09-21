mod combination;
mod factorial;
mod permutation;

use combination::combination;
use factorial::factorial;
use permutation::permutation;

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
