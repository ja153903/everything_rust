#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .filter(|word| !word.is_empty())
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
