#![allow(dead_code)] // Remove this when you're actually implementing the solution

fn parse_data() -> &'static str {
    include_str!("./data/yr2015_01.in")
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
        info!("Answer for Advent of Code 2015 - Day 01 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 01 - Part 2: {}", ans);
    }
}
