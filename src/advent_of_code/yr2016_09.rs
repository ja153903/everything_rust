#![allow(dead_code)]

fn parse_data() -> &'static str {
    include_str!("./data/yr2016_09.in")
}

pub fn part1() -> i64 {
    todo!("Not implemented")
}

pub fn part2() -> i64 {
    todo!("Not implemented")
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
        info!("Answer for Advent of Code 2016 - Day 09 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 09 - Part 2: {}", ans);
    }
}
