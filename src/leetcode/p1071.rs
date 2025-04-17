#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn gcd_of_strings(s: String, t: String) -> String {
        if !Self::is_equal_concatenated(&s, &t) || s.is_empty() || t.is_empty() {
            String::default()
        } else {
            s.chars().take(Self::gcd(s.len(), t.len())).collect()
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn is_equal_concatenated(s: &str, t: &str) -> bool {
        format!("{}{}", s, t) == format!("{}{}", t, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_basic_tests() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("A".to_string(), "A".to_string()),
            "A".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("AAAA".to_string(), "AA".to_string()),
            "AA".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("XYZXYZXYZ".to_string(), "XYZXYZ".to_string()),
            "XYZ".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("".to_string(), "".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABC".to_string(), "".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABAB".to_string(), "AB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABABAB".to_string(), "ABAB".to_string()),
            "ABAB".to_string()
        );
    }
}
