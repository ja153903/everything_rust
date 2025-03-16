fn parse_data() -> Vec<Vec<char>> {
    include_str!("./data/yr2016_06.in")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1() -> String {
    let mut result = String::new();
    let characters = parse_data();
    for column in 0..characters[0].len() {
        let mut counter: Vec<i32> = vec![0; 26];
        for row in 0..characters.len() {
            counter[(characters[row][column] as u8 - 97) as usize] += 1;
        }

        let mut most_common_char: usize = 0;
        let mut most_common_char_freq: i32 = 0;

        (0..counter.len()).for_each(|i| {
            if most_common_char_freq < counter[i] {
                most_common_char = i;
                most_common_char_freq = counter[i];
            }
        });

        result.push((most_common_char as u8 + 97) as char);
    }

    result
}

pub fn part2() -> String {
    let mut result = String::new();
    let characters = parse_data();
    for column in 0..characters[0].len() {
        let mut counter: Vec<i32> = vec![0; 26];
        for row in 0..characters.len() {
            counter[(characters[row][column] as u8 - 97) as usize] += 1;
        }

        let mut least_common_char: usize = 0;
        let mut least_common_char_freq: i32 = i32::MAX;

        (0..counter.len()).for_each(|i| {
            if least_common_char_freq > counter[i] {
                least_common_char = i;
                least_common_char_freq = counter[i];
            }
        });

        result.push((least_common_char as u8 + 97) as char);
    }

    result
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
        info!("Answer for Advent of Code 2016 - Day 06 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 06 - Part 2: {}", ans);
    }
}
