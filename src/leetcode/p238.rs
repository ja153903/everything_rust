#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut right = 1;

        for j in (0..nums.len()).rev() {
            result[j] *= right;
            right *= nums[j];
        }

        result
    }
}
