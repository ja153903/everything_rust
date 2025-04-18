#![allow(dead_code)]

struct Solution;

impl Solution {
    // TODO: Create a more optimized solution for this problem.
    // There's a constraint we did not fulfill which requires us to
    // use constant space
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() <= 1 {
            return chars.len() as i32;
        }

        let mut res: Vec<char> = Vec::new();

        let mut count = 1;
        let mut current = chars[0];

        for i in 1..chars.len() {
            if current == chars[i] {
                count += 1;
            } else {
                res.push(current);
                if count > 1 {
                    count.to_string().chars().for_each(|ch| res.push(ch));
                }

                current = chars[i];
                count = 1;
            }

            if i == chars.len() - 1 {
                res.push(current);
                if count > 1 {
                    count.to_string().chars().for_each(|ch| res.push(ch));
                }
            }
        }

        for (i, &ch) in res.iter().enumerate() {
            chars[i] = ch;
        }

        res.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);

        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);

        let mut chars = vec!['a', 'b', 'c'];
        assert_eq!(Solution::compress(&mut chars), 3);

        let mut chars = vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 3);

        let mut chars = vec![];
        assert_eq!(Solution::compress(&mut chars), 0);

        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 6);
    }
}
