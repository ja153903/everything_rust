use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
pub struct Dimension {
    pub height: i64,
    pub length: i64,
    pub width: i64,
}

#[derive(Debug)]
pub struct ParseDimensionError;

impl FromStr for Dimension {
    type Err = ParseDimensionError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<height>\d+)x(?P<width>\d+)x(?P<length>\d+)").unwrap();
        if let Some(captures) = re.captures(line) {
            let height = captures["height"]
                .parse::<i64>()
                .map_err(|_| ParseDimensionError)?;
            let length = captures["length"]
                .parse::<i64>()
                .map_err(|_| ParseDimensionError)?;
            let width = captures["width"]
                .parse::<i64>()
                .map_err(|_| ParseDimensionError)?;

            Ok(Dimension {
                height,
                length,
                width,
            })
        } else {
            panic!("[Dimension::from_str]: Something went wrong with regex")
        }
    }
}

impl Dimension {
    pub fn surface_area(&self) -> i64 {
        let h = self.height;
        let w = self.width;
        let l = self.length;

        2 * l * w + 2 * w * h + 2 * h * l
    }

    pub fn smallest_area(&self) -> i64 {
        let h = self.height;
        let w = self.width;
        let l = self.length;

        (h * w).min((w * l).min(h * l))
    }

    pub fn volume(&self) -> i64 {
        let h = self.height;
        let w = self.width;
        let l = self.length;

        h * w * l
    }

    pub fn smallest_perimeter(&self) -> i64 {
        let h = self.height;
        let w = self.width;
        let l = self.length;

        (2 * h + 2 * w).min((2 * h + 2 * l).min(2 * w + 2 * l))
    }
}

pub fn parse_data() -> impl Iterator<Item = Result<Dimension, ParseDimensionError>> {
    include_str!("./data/yr2015_02.in")
        .lines()
        .map(Dimension::from_str)
}

pub fn part1() -> i64 {
    parse_data().fold(0, |acc, dimension| {
        if let Ok(dimension) = dimension {
            return acc + dimension.surface_area() + dimension.smallest_area();
        }

        panic!("Something went wrong with the data during iteration")
    })
}

pub fn part2() -> i64 {
    parse_data().fold(0, |acc, dimension| {
        if let Ok(dimension) = dimension {
            return acc + dimension.volume() + dimension.smallest_perimeter();
        }

        panic!("Something went wrong with the data during iteration")
    })
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
        info!("Answer for Advent of Code 2015 - Day 02 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 02 - Part 2: {}", ans);
    }
}
