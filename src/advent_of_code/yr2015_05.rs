mod nice_string_util {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const FORBIDDEN_WORDS_V1: [&str; 4] = ["ab", "cd", "pq", "xy"];

    fn count_vowels(s: &str) -> i64 {
        let mut count: i64 = 0;

        s.chars().for_each(|ch| {
            if VOWELS.contains(&ch) {
                count += 1;
            }
        });

        count
    }

    fn has_repeating_chars(s: &str) -> bool {
        let mut i: usize = 1;

        while i < s.len() {
            if s[i..(i + 1)] == s[(i - 1)..i] {
                return true;
            }

            i += 1;
        }

        false
    }

    fn has_forbidden_word(s: &str) -> bool {
        let mut i: usize = 1;

        while i < s.len() {
            if FORBIDDEN_WORDS_V1.contains(&&s[(i - 1)..(i + 1)]) {
                return true;
            }

            i += 1;
        }

        false
    }

    fn has_nonoverlapping_pair(s: &str) -> bool {
        let mut i: usize = 1;

        while i < s.len() {
            let pair = &s[(i - 1)..(i + 1)];
            let mut j: usize = i + 2;

            while j < s.len() {
                let another_pair = &s[(j - 1)..(j + 1)];

                if pair == another_pair {
                    return true;
                }

                j += 1;
            }

            i += 1;
        }

        false
    }

    fn has_repeating_chars_between_single_char(s: &str) -> bool {
        let mut i: usize = 1;

        while i < s.len() - 1 {
            if s[(i - 1)..i] == s[(i + 1)..(i + 2)] {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn is_nice(s: &str, v1: Option<bool>) -> bool {
        if v1.is_some() {
            count_vowels(s) >= 3 && has_repeating_chars(s) && !has_forbidden_word(s)
        } else {
            has_nonoverlapping_pair(s) && has_repeating_chars_between_single_char(s)
        }
    }
}

fn parse_data() -> impl Iterator<Item = &'static str> {
    include_str!("./data/yr2015_05.in")
        .split("\n")
        .filter(|line| !line.is_empty())
}

pub fn part1() -> usize {
    parse_data()
        .filter(|line| nice_string_util::is_nice(line, Some(true)))
        .count()
}

pub fn part2() -> usize {
    parse_data()
        .filter(|line| nice_string_util::is_nice(line, None))
        .count()
}

#[cfg(test)]
pub mod tests {
    use log::info;

    use super::{part1, part2};
    use crate::advent_of_code::test_logger;

    #[ctor::ctor]
    fn init() {
        test_logger::setup();
    }

    #[ignore]
    #[test]
    pub fn run_part1() {
        let ans = part1();
        info!("Answer for Advent of Code 2015 - Day 05 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 05 - Part 2: {}", ans);
    }
}
