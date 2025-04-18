/// `sum_natural_numbers` serves as a formula for calculating the sum of the first n natural numbers
pub fn sum_natural_numbers(n: i64) -> i64 {
    n * (n + 1) / 2
}

/// `sum_squared_numbers` serves as a formula for calculating the sum of the squares of the first n natural numbers
pub fn sum_squared_numbers(n: i64) -> i64 {
    n * (n + 1) * (2 * n + 1) / 6
}

/// `sum_arithmetic_progression` serves as a formula for calculating the sum of an arithmetic progression
pub fn sum_arithmetic_progression(first: i64, last: i64, len: i64) -> i64 {
    len * (first + last) / 2
}

/// `binet` serves as a closed-form formula for calculating Fibonacci numbers
pub fn binet(n: i32) -> f64 {
    let sqrt5 = (5 as f64).sqrt();

    ((1_f64 + sqrt5).powi(n) - (1_f64 - sqrt5).powi(n)) / (2_f64.powi(n) * sqrt5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_natural_numbers() {
        assert_eq!(sum_natural_numbers(1), 1);
        assert_eq!(sum_natural_numbers(2), 3);
        assert_eq!(sum_natural_numbers(3), 6);
        assert_eq!(sum_natural_numbers(4), 10);
        assert_eq!(sum_natural_numbers(5), 15);
        assert_eq!(sum_natural_numbers(100), 5050);
    }

    #[test]
    fn test_sum_squared_numbers() {
        assert_eq!(sum_squared_numbers(1), 1);
        assert_eq!(sum_squared_numbers(2), 5);
        assert_eq!(sum_squared_numbers(3), 14);
        assert_eq!(sum_squared_numbers(4), 30);
        assert_eq!(sum_squared_numbers(5), 55);
        assert_eq!(sum_squared_numbers(10), 385);
    }

    #[test]
    fn test_sum_arithmetic_progression() {
        assert_eq!(sum_arithmetic_progression(1, 5, 5), 15);
        assert_eq!(sum_arithmetic_progression(2, 10, 5), 30);
        assert_eq!(sum_arithmetic_progression(1, 9, 5), 25);
        assert_eq!(sum_arithmetic_progression(1, 100, 100), 5050);
        assert_eq!(sum_arithmetic_progression(0, 0, 1), 0);
        assert_eq!(sum_arithmetic_progression(-5, 5, 11), 0);
    }

    #[test]
    fn test_binet() {
        // Test first few Fibonacci numbers
        assert!((binet(0) - 0.0).abs() < 1e-10);
        assert!((binet(1) - 1.0).abs() < 1e-10);
        assert!((binet(2) - 1.0).abs() < 1e-10);
        assert!((binet(3) - 2.0).abs() < 1e-10);
        assert!((binet(4) - 3.0).abs() < 1e-10);
        assert!((binet(5) - 5.0).abs() < 1e-10);
        assert!((binet(6) - 8.0).abs() < 1e-10);
        assert!((binet(7) - 13.0).abs() < 1e-10);

        // Test some larger numbers
        assert!((binet(12) - 144.0).abs() < 1e-10);
        assert!((binet(15) - 610.0).abs() < 1e-10);

        // Test negative numbers (which should follow Fibonacci pattern backwards)
        assert!((binet(-1) - 1.0).abs() < 1e-10);
        assert!((binet(-2) - (-1.0)).abs() < 1e-10);
        assert!((binet(-3) - 2.0).abs() < 1e-10);
        assert!((binet(-4) - (-3.0)).abs() < 1e-10);
    }
}
