#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // this is trivial to do in O(n^2)
        let mut result = 0;

        // we don't really care about the order of the pairs
        // we know that we just need to find if two numbers in the vector can sum up
        // to be clamped between lower and upper

        // so it might be a good idea to sort
        nums.sort();

        for i in 0..nums.len() {
            let min_req = lower - nums[i];
            let max_req = upper - nums[i];

            let high = nums[..i].partition_point(|&n| n <= max_req);
            let low = nums[..i].partition_point(|&n| n < min_req);

            result += (high - low) as i32;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_the_number_of_fair_pairs() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;

        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 6);
    }

    #[test]
    pub fn test_count_the_number_of_fair_pairs_2() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;

        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 1);
    }
}
