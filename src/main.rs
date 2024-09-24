use Math::algebra::advanced_algebra::polynomial::Polynomial;
use Math::arithmetic_and_basic_math::factorial::factorial;
use Math::calculus::differentiation::differentiate::differentiate;
use Math::discrete_mathematics::combination::combination;
use Math::discrete_mathematics::permutation::permutation;

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

    // 微分
    let poly = Polynomial::new(vec![2.0, 5.0, 3.0]); // 2 + 5x + 3x^2
    let derivative = differentiate(&poly);
    println!("The derivative of {:?} is: {:?}", poly, derivative);
}
