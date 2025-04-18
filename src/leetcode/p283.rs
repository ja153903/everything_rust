#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i: usize = 0;

        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }

        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}
