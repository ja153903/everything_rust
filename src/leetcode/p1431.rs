#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let current_max = candies.iter().max().unwrap();

        candies
            .iter()
            .map(|count| *count + extra_candies >= *current_max)
            .collect()
    }
}
