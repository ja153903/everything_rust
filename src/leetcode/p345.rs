#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut rev_str: Vec<Option<char>> = Vec::new();
        let mut vowels: Vec<char> = Vec::new();

        for ch in s.chars() {
            if Self::is_vowel(ch) {
                vowels.push(ch);
                rev_str.push(None);
            } else {
                rev_str.push(Some(ch));
            }
        }

        let mut i = vowels.len() - 1;

        for j in 0..rev_str.len() {
            if rev_str[j].is_none() {
                rev_str[j] = Some(vowels[i]);
                i -= 1;
            }
        }

        rev_str.iter().map(|item| item.unwrap()).collect()
    }

    pub fn is_vowel(ch: char) -> bool {
        ch == 'a'
            || ch == 'A'
            || ch == 'e'
            || ch == 'E'
            || ch == 'i'
            || ch == 'I'
            || ch == 'o'
            || ch == 'O'
            || ch == 'u'
            || ch == 'U'
    }
}
