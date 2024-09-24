#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial {
    coeffs: Vec<f64>, // 使用向量儲存多項式係數
}

impl Polynomial {
    // 創建多項式
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }

    // 獲取係數
    pub fn coeffs(&self) -> &Vec<f64> {
        &self.coeffs
    }

    // 多項式加法
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = usize::max(self.coeffs.len(), other.coeffs.len());
        let mut result_coeffs = vec![0.0; max_len];

        for i in 0..self.coeffs.len() {
            result_coeffs[i] += self.coeffs[i];
        }

        for i in 0..other.coeffs.len() {
            result_coeffs[i] += other.coeffs[i];
        }

        Polynomial::new(result_coeffs)
    }

    // 多項式減法
    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        let max_len = usize::max(self.coeffs.len(), other.coeffs.len());
        let mut result_coeffs = vec![0.0; max_len];

        for i in 0..self.coeffs.len() {
            result_coeffs[i] += self.coeffs[i];
        }

        for i in 0..other.coeffs.len() {
            result_coeffs[i] -= other.coeffs[i];
        }

        Polynomial::new(result_coeffs)
    }

    // 多項式乘法
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let result_len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result_coeffs = vec![0.0; result_len];

        for (i, &self_coeff) in self.coeffs.iter().enumerate() {
            for (j, &other_coeff) in other.coeffs.iter().enumerate() {
                result_coeffs[i + j] += self_coeff * other_coeff;
            }
        }

        Polynomial::new(result_coeffs)
    }

    // 多項式微分
    pub fn differentiate(&self) -> Polynomial {
        let mut result_coeffs = Vec::new();

        for (exp, &coeff) in self.coeffs.iter().enumerate().skip(1) {
            result_coeffs.push(coeff * exp as f64);
        }

        Polynomial::new(result_coeffs)
    }

    // 打印多項式 (格式化)
    pub fn to_string(&self) -> String {
        let mut terms = Vec::new();

        for (i, &coeff) in self.coeffs.iter().enumerate() {
            if coeff != 0.0 {
                let term = if i == 0 {
                    format!("{}", coeff)
                } else if i == 1 {
                    format!("{}x", coeff)
                } else {
                    format!("{}x^{}", coeff, i)
                };
                terms.push(term);
            }
        }

        if terms.is_empty() {
            "0".to_string()
        } else {
            terms.reverse();
            terms.join(" + ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let poly1 = Polynomial::new(vec![1.0, 2.0, 3.0]); // 3x^2 + 2x + 1
        let poly2 = Polynomial::new(vec![3.0, 4.0]); // 4x + 3
        let sum = poly1.add(&poly2);
        assert_eq!(sum, Polynomial::new(vec![4.0, 6.0, 3.0])); // 3x^2 + 6x + 4
    }

    #[test]
    fn test_subtract() {
        let poly1 = Polynomial::new(vec![1.0, 2.0, 3.0]); // 3x^2 + 2x + 1
        let poly2 = Polynomial::new(vec![3.0, 4.0]); // 4x + 3
        let difference = poly1.subtract(&poly2);
        assert_eq!(difference, Polynomial::new(vec![-2.0, -2.0, 3.0])); // 3x^2 - 2x - 2
    }

    #[test]
    fn test_multiply() {
        let poly1 = Polynomial::new(vec![1.0, 2.0]); // 2x + 1
        let poly2 = Polynomial::new(vec![1.0, 3.0]); // 3x + 1
        let product = poly1.multiply(&poly2);
        assert_eq!(product, Polynomial::new(vec![1.0, 5.0, 6.0])); // 6x^2 + 5x + 1
    }

    #[test]
    fn test_differentiate() {
        let poly = Polynomial::new(vec![2.0, 5.0, 3.0]); // 3x^2 + 5x + 2
        let derivative = poly.differentiate();
        assert_eq!(derivative, Polynomial::new(vec![5.0, 6.0])); // 6x + 5
    }

    #[test]
    fn test_to_string() {
        let poly = Polynomial::new(vec![2.0, 5.0, 3.0]); // 3x^2 + 5x + 2
        assert_eq!(poly.to_string(), "3x^2 + 5x + 2");

        let poly2 = Polynomial::new(vec![0.0, 0.0, 0.0]); // 0
        assert_eq!(poly2.to_string(), "0");
    }
}
