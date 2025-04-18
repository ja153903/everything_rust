#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            String::from("1")
        } else {
            Self::rle(Self::count_and_say(n - 1))
        }
    }

    pub fn rle(s: String) -> String {
        if s.is_empty() {
            String::default()
        } else {
            let mut result = String::new();
            let mut itr = s.chars();
            let mut count = 1;
            let mut current = itr.next().unwrap();

            for ch in itr {
                if current == ch {
                    count += 1;
                } else {
                    result.push_str(&count.to_string());
                    result.push(current);
                    current = ch;
                    count = 1;
                }
            }

            result.push_str(&count.to_string());
            result.push(current);

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle() {
        assert_eq!(Solution::rle(String::from("")), String::from(""));
        assert_eq!(Solution::rle(String::from("1")), String::from("11"));
        assert_eq!(Solution::rle(String::from("11")), String::from("21"));
        assert_eq!(Solution::rle(String::from("21")), String::from("1211"));
        assert_eq!(Solution::rle(String::from("1211")), String::from("111221"));
        assert_eq!(
            Solution::rle(String::from("111221")),
            String::from("312211")
        );
    }
}
