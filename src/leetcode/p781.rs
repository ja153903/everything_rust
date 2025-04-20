#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut counter: HashMap<i32, i32> = HashMap::new();

        for &ans in answers.iter() {
            *counter.entry(ans).or_insert(0) += 1;
        }

        for (&key, &count) in counter.iter() {
            let count_f32 = count as f32;
            let key_f32 = key as f32;
            result += (count_f32 / (key_f32 + 1_f32)).ceil() as i32 * (key + 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_num_rabbits() {
        let v = vec![0, 3, 2, 0, 3, 3, 4, 2, 4, 3, 2, 4, 4, 3, 0, 1, 3, 4, 4, 3];

        assert_eq!(Solution::num_rabbits(v), 26);
    }
}
