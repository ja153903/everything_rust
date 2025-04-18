#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut itr = s.chars().peekable();

        for ch in t.chars() {
            if let Some(&s_ch) = itr.peek() {
                if s_ch == ch {
                    itr.next();
                }
            }
        }

        itr.next().is_none()
    }
}
