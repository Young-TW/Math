// 導入 Polynomial 結構體
use crate::polynomial::Polynomial;

pub fn differentiate(poly: &Polynomial) -> Polynomial {
    let mut result_coeffs = Vec::new();

    // 微分公式: 對每一項 (exp, coeff) 做 exp * coeff 的運算，並將 exp 減一
    for (exp, &coeff) in poly.coeffs().iter().enumerate().skip(1) {
        result_coeffs.push(coeff * exp as f64);
    }

    Polynomial::new(result_coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomial::Polynomial;

    #[test]
    fn test_differentiate() {
        // 測試多項式: 3x^2 + 5x + 2 的微分
        let poly = Polynomial::new(vec![2.0, 5.0, 3.0]); // 2 + 5x + 3x^2
        let derivative = differentiate(&poly);

        // 預期結果為: 6x + 5 對應於 [5.0, 6.0]
        assert_eq!(derivative, Polynomial::new(vec![5.0, 6.0]));
    }

    #[test]
    fn test_differentiate_zero() {
        // 測試常數多項式的微分 (應該為 0)
        let poly = Polynomial::new(vec![5.0]); // 5
        let derivative = differentiate(&poly);

        // 預期結果為: 0
        assert_eq!(derivative, Polynomial::new(vec![]));
    }

    #[test]
    fn test_differentiate_linear() {
        // 測試一次多項式的微分: 5x + 2
        let poly = Polynomial::new(vec![2.0, 5.0]); // 5x + 2
        let derivative = differentiate(&poly);

        // 預期結果為: 5
        assert_eq!(derivative, Polynomial::new(vec![5.0]));
    }
}
