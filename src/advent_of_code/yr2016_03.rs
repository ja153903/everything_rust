fn parse_data() -> Vec<(i64, i64, i64)> {
    include_str!("./data/yr2016_03.in")
        .lines()
        .map(|line| {
            let sides: Vec<i64> = line
                .split_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect();

            (sides[0], sides[1], sides[2])
        })
        .collect()
}

fn parse_data_as_matrix() -> Vec<Vec<i64>> {
    include_str!("./data/yr2016_03.in")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_triangle(sides: &(i64, i64, i64)) -> bool {
    sides.0 + sides.1 > sides.2 && sides.0 + sides.2 > sides.1 && sides.1 + sides.2 > sides.0
}

pub fn part1() -> usize {
    parse_data().into_iter().filter(is_valid_triangle).count()
}

pub fn part2() -> usize {
    let grid = parse_data_as_matrix();
    let mut sides: Vec<(i64, i64, i64)> = Vec::new();

    for col in 0..3_usize {
        for row in (0..grid.len() - 2).step_by(3) {
            sides.push((grid[row][col], grid[row + 1][col], grid[row + 2][col]));
        }
    }

    sides.into_iter().filter(is_valid_triangle).count()
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
