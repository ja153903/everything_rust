#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();

        let mut w1_itr = word1.chars();
        let mut w2_itr = word2.chars();

        loop {
            match (w1_itr.next(), w2_itr.next()) {
                (Some(w1), Some(w2)) => {
                    result.push(w1);
                    result.push(w2);
                }
                (Some(w1), None) => {
                    result.push(w1);
                }
                (None, Some(w2)) => {
                    result.push(w2);
                }
                (None, None) => {
                    return result;
                }
            }
        }
    }
}
