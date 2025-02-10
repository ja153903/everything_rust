use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    pub fn update(&mut self, direction: char) {
        let (dx, dy) = match direction {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };

        self.x += dx;
        self.y += dy;
    }
}

fn parse_data() -> &'static str {
    include_str!("./data/yr2015_03.in")
}

pub fn part1() -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut current = Point { x: 0, y: 0 };
    visited.insert(current);

    parse_data().chars().for_each(|direction| {
        current.update(direction);
        visited.insert(current);
    });

    visited.len()
}

pub fn part2() -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut current = Point { x: 0, y: 0 };
    let mut robo_current = Point { x: 0, y: 0 };
    visited.insert(current);

    parse_data().char_indices().for_each(|(i, direction)| {
        if i % 2 == 0 {
            current.update(direction);
            visited.insert(current);
        } else {
            robo_current.update(direction);
            visited.insert(robo_current);
        }
    });

    visited.len()
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
        info!("Answer for Advent of Code 2015 - Day 03 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 03 - Part 2: {}", ans);
    }
}
