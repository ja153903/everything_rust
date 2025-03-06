use anyhow;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Reindeer {
    name: Rc<String>,
    velocity: i64,
    duration: i64,
    rest: i64,
    // Keep track of the distance traveled
    distance_traveled: i64,
    // Keep track of how long the reindeer has rested up to this point
    resting_ticks: i64,
    // Keep track of how long the user has traveled up to this point
    traveling_ticks: i64,
}

impl Reindeer {
    fn new(name: String, velocity: i64, duration: i64, rest: i64) -> Self {
        Self {
            name: Rc::new(name),
            velocity,
            duration,
            rest,
            distance_traveled: 0,
            resting_ticks: 0,
            traveling_ticks: 0,
        }
    }

    fn update(&mut self) {
        if self.resting_ticks > 0 {
            self.resting_ticks -= 1;
        } else {
            self.traveling_ticks += 1;
            self.distance_traveled += self.velocity;

            if self.traveling_ticks == self.duration {
                self.traveling_ticks = 0;
                self.resting_ticks = self.rest;
            }
        }
    }
}

fn parse_reindeer(line: &str) -> anyhow::Result<Reindeer> {
    let re = Regex::new(
        r"(?P<reindeer>\w+) can fly (?P<velocity>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest>\d+) seconds."
    ).unwrap();
    match re.captures(line) {
        Some(captures) => {
            let reindeer = String::from(&captures["reindeer"]);
            let velocity = &captures["velocity"].parse::<i64>()?;
            let duration = &captures["duration"].parse::<i64>()?;
            let rest = &captures["rest"].parse::<i64>()?;

            Ok(Reindeer::new(reindeer, *velocity, *duration, *rest))
        }
        None => panic!("Did not parse the line correctly!"),
    }
}

fn parse_data() -> Vec<Reindeer> {
    include_str!("./data/yr2015_14.in")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| match parse_reindeer(line) {
            Ok(reindeer) => reindeer,
            Err(_) => panic!("Something went wrong trying to parse reindeer"),
        })
        .collect()
}

pub fn part1() -> i64 {
    let mut reindeers = parse_data();

    (0..2503).for_each(|_| reindeers.iter_mut().for_each(|reindeer| reindeer.update()));

    match reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.distance_traveled)
    {
        Some(reindeer) => reindeer.distance_traveled,
        None => panic!("Could not find the appropriate reindeer"),
    }
}

pub fn part2() -> i64 {
    let mut reindeers = parse_data();
    let mut rankings: HashMap<Rc<String>, i64> = HashMap::new();

    (0..2503).for_each(|_| {
        reindeers.iter_mut().for_each(|reindeer| reindeer.update());

        let winning_reindeers = reindeers
            .iter()
            // NOTE: This was only possible because of the itertools library
            .max_set_by_key(|reindeer| reindeer.distance_traveled);

        winning_reindeers.iter().for_each(|reindeer| {
            rankings
                .entry(reindeer.name.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
    });

    match rankings.values().max() {
        Some(winning_points) => *winning_points,
        None => panic!("Could not find winning number of points"),
    }
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
        info!("Answer for Advent of Code 2015 - Day 14 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 14 - Part 2: {}", ans);
    }
}
