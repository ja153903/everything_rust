use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
enum Switch {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Instruction {
    command: Switch,
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug)]
struct ParseInstructionError {
    pub message: String,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<command>turn off|turn on|toggle) (?P<start_x>\d+),(?P<start_y>\d+) through (?P<end_x>\d+),(?P<end_y>\d+)").unwrap();

        if let Some(captures) = re.captures(s) {
            let command = &captures["command"];
            let start_x = &captures["start_x"];
            let start_y = &captures["start_y"];
            let end_x = &captures["end_x"];
            let end_y = &captures["end_y"];

            Ok(Instruction {
                command: match command {
                    "turn off" => Switch::TurnOff,
                    "turn on" => Switch::TurnOn,
                    "toggle" => Switch::Toggle,
                    _ => panic!("Invalid option for command is parsed"),
                },
                start: Coordinate {
                    x: start_x
                        .parse::<usize>()
                        .map_err(|_| ParseInstructionError {
                            message: String::from("start_x could not be parsed"),
                        })?,
                    y: start_y
                        .parse::<usize>()
                        .map_err(|_| ParseInstructionError {
                            message: String::from("start_y could not be parsed"),
                        })?,
                },
                end: Coordinate {
                    x: end_x.parse::<usize>().map_err(|_| ParseInstructionError {
                        message: String::from("end_x could not be parsed"),
                    })?,
                    y: end_y.parse::<usize>().map_err(|_| ParseInstructionError {
                        message: String::from("end_y could not be parsed"),
                    })?,
                },
            })
        } else {
            panic!("Something went wrong parsing to Instruction from str");
        }
    }
}

fn parse_data() -> impl Iterator<Item = Result<Instruction, ParseInstructionError>> {
    include_str!("./data/yr2015_06.in")
        .lines()
        .map(Instruction::from_str)
}

pub fn part1() -> i64 {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; 1000]; 1000];

    parse_data().for_each(|instruction| match instruction {
        Ok(instr) => {
            let command = instr.command;
            let start = instr.start;
            let end = instr.end;

            (start.x..=end.x).for_each(|x| {
                (start.y..=end.y).for_each(|y| match command {
                    Switch::Toggle => {
                        grid[y][x] ^= 1;
                    }
                    Switch::TurnOn => {
                        grid[y][x] = 1;
                    }
                    Switch::TurnOff => {
                        grid[y][x] = 0;
                    }
                });
            });
        }
        Err(e) => panic!("Instruction is malformed. Why? {}", e.message),
    });

    let mut result = 0;

    grid.iter().for_each(|row| {
        row.iter().for_each(|&light| {
            if light == 1 {
                result += 1;
            }
        });
    });

    result
}

pub fn part2() -> i64 {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; 1000]; 1000];

    parse_data().for_each(|instruction| match instruction {
        Ok(instr) => {
            let command = instr.command;
            let start = instr.start;
            let end = instr.end;

            (start.x..=end.x).for_each(|x| {
                (start.y..=end.y).for_each(|y| match command {
                    Switch::Toggle => {
                        grid[y][x] += 2;
                    }
                    Switch::TurnOn => {
                        grid[y][x] += 1;
                    }
                    Switch::TurnOff => {
                        grid[y][x] = 0.max(grid[y][x] - 1);
                    }
                });
            });
        }
        Err(e) => panic!("Instruction is malformed. Why? {}", e.message),
    });

    let mut result = 0;

    grid.iter().for_each(|row| {
        row.iter().for_each(|&light| {
            result += light;
        });
    });

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
        info!("Answer for Advent of Code 2015 - Day 06 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 06 - Part 2: {}", ans);
    }
}
