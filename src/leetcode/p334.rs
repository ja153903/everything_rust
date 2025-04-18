#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums.iter() {
            if *num <= first {
                first = *num;
            } else if *num <= second {
                second = *num;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing_triplet() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1, 6, 7]));
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert!(!Solution::increasing_triplet(vec![2, 1, 1, 1]));
        assert!(!Solution::increasing_triplet(vec![1, 1, 1]));
        assert!(!Solution::increasing_triplet(vec![1, 2]));
        assert!(!Solution::increasing_triplet(vec![1]));
        assert!(!Solution::increasing_triplet(vec![]));
        assert!(Solution::increasing_triplet(vec![1, 5, 0, 4, 1, 3]));
    }
}
