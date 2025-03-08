use itertools::Itertools;

fn parse_data() -> Vec<i64> {
    include_str!("./data/yr2015_17.in")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn part1() -> usize {
    let containers = parse_data();
    let mut total = 0;

    (1..=containers.len()).for_each(|len| {
        total += containers
            .iter()
            .combinations(len)
            .filter(|row| {
                let mut sum = 0;
                row.iter().for_each(|&&item| {
                    sum += item;
                });

                sum == 150
            })
            .count();
    });

    total
}

pub fn part2() -> usize {
    let containers = parse_data();

    for len in 1..=containers.len() {
        let count = containers
            .iter()
            .combinations(len)
            .filter(|row| {
                let mut sum = 0;
                row.iter().for_each(|&&item| {
                    sum += item;
                });

                sum == 150
            })
            .count();

        if count > 0 {
            return len;
        }
    }

    panic!("Could not find the length?")
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
        info!("Answer for Advent of Code 2015 - Day 17 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 17 - Part 2: {}", ans);
    }
}
