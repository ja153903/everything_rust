#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();

        let mut i = 0;
        let mut j = 0;

        let mut is_word1 = true;

        while i < word1.len() && j < word2.len() {
            if is_word1 {
                result.push_str(&word1[i..i + 1]);
                i += 1;
            } else {
                result.push_str(&word2[j..j + 1]);
                j += 1;
            }

            is_word1 = !is_word1;
        }

        while i < word1.len() {
            result.push_str(&word1[i..i + 1]);
            i += 1;
        }

        while j < word2.len() {
            result.push_str(&word2[j..j + 1]);
            j += 1;
        }

        result
    }
}
