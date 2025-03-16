use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Room {
    encrypted_name: Rc<String>,
    sector_id: i64,
    checksum: Rc<String>,
}

impl Room {
    fn is_real_room(&self) -> bool {
        let mut counter: HashMap<char, i64> = HashMap::new();
        self.encrypted_name.chars().for_each(|char| {
            *counter.entry(char).or_insert(0) += 1;
        });

        let mut entries: Vec<_> = counter.into_iter().collect();
        entries.sort_by(|a, b| (b.1, a.0).cmp(&(a.1, b.0)));

        let candidate = entries.iter().map(|&tup| tup.0).take(5).join("");

        candidate == *self.checksum
    }

    fn cycle_name(&self, n: i64) -> String {
        let mut real_name = self.encrypted_name.to_string();
        (0..n).for_each(|_| {
            real_name = real_name
                .chars()
                .map(|ch| {
                    if ch == 'z' {
                        'a'
                    } else {
                        ((ch as u8) + 1) as char
                    }
                })
                .collect();
        });

        real_name
    }
}

fn parse_data() -> Vec<Room> {
    let re = Regex::new(r"(?P<sector_id>\d+)\[(?P<checksum>\w+)\]").unwrap();
    include_str!("./data/yr2016_04.in")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            let encrypted_name = parts
                .iter()
                .take(parts.len() - 1)
                .map(|&s| String::from(s))
                .join("");
            let (sector_id, checksum) = match parts.last() {
                None => panic!("Why did this fail?"),
                Some(s) => match re.captures(s) {
                    None => panic!("Could not parse sector_id and checksum"),
                    Some(captures) => (
                        match &captures["sector_id"].parse::<i64>() {
                            Ok(value) => *value,
                            Err(_) => panic!("Something went wrong parsing the value"),
                        },
                        String::from(&captures["checksum"]),
                    ),
                },
            };

            Room {
                encrypted_name: Rc::new(encrypted_name),
                checksum: Rc::new(checksum),
                sector_id,
            }
        })
        .collect()
}

pub fn part1() -> i64 {
    parse_data()
        .into_iter()
        .filter(|room| room.is_real_room())
        .map(|room| room.sector_id)
        .sum()
}

pub fn part2() -> Option<(String, i64)> {
    parse_data()
        .into_iter()
        .filter(|room| room.is_real_room())
        .map(|room| {
            let real_name = room.cycle_name(room.sector_id % 26);
            (real_name, room.sector_id)
        })
        .find(|(real_name, _sector_id)| real_name.contains("north"))
        .or(None)
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
        info!("Answer for Advent of Code 2016 - Day 04 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        match ans {
            None => panic!("Could not find the appropriate answer"),
            Some((_real_name, sector_id)) => {
                info!(
                    "Answer for Advent of Code 2016 - Day 04 - Part 2: {}",
                    sector_id
                );
            }
        }
    }
}
