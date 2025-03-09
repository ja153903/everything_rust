use std::collections::HashSet;

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    rotation: Rotation,
    steps: i64,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(direction: &Direction, rotation: &Rotation) -> Direction {
        match rotation {
            Rotation::Left => match direction {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Rotation::Right => match direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        }
    }

    fn get_delta(direction: &Direction) -> (i64, i64) {
        match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

fn parse_data() -> Vec<Instruction> {
    include_str!("./data/yr2016_01.in")
        .split(", ")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let rotation = if &line[0..1] == "L" {
                Rotation::Left
            } else {
                Rotation::Right
            };

            // NOTE: `trim` is necessary here because we have a case where spaces are not trimmed
            // beforehand
            let steps = &line[1..].trim().parse::<i64>().unwrap();

            Instruction {
                rotation,
                steps: *steps,
            }
        })
        .collect()
}

pub fn part1() -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut direction = Direction::North;

    parse_data().iter().for_each(|instruction| {
        direction = Direction::rotate(&direction, &instruction.rotation);
        let (dx, dy) = Direction::get_delta(&direction);

        x += instruction.steps * dx;
        y += instruction.steps * dy;
    });

    x.abs() + y.abs()
}

pub fn part2() -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut direction = Direction::North;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    visited.insert((0, 0));

    for instruction in parse_data().iter() {
        direction = Direction::rotate(&direction, &instruction.rotation);
        let (dx, dy) = Direction::get_delta(&direction);

        for _ in 0..instruction.steps {
            x += dx;
            y += dy;

            if visited.contains(&(x, y)) {
                return x.abs() + y.abs();
            }

            visited.insert((x, y));
        }
    }

    panic!("Could not find the twice visited location")
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
        info!("Answer for Advent of Code 2016 - Day 01 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 01 - Part 2: {}", ans);
    }
}
