use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Assign(String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u32, String),
    RShift(String, u32, String),
    Not(String, String),
}

#[derive(Debug)]
struct ParseOperationError {
    pub message: String,
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operations_re = Regex::new(
            r"(?P<lhs>\w+) (?P<command>AND|OR|LSHIFT|RSHIFT) (?P<rhs>\w+) -> (?P<register>\w+)",
        )
        .unwrap();
        if let Some(captures) = operations_re.captures(s) {
            let lhs = &captures["lhs"];
            let rhs = &captures["rhs"];
            let command = &captures["command"];
            let register = &captures["register"];

            match command {
                "AND" => {
                    return Ok(Operation::And(
                        String::from(lhs),
                        String::from(rhs),
                        String::from(register),
                    ));
                }
                "OR" => {
                    return Ok(Operation::Or(
                        String::from(lhs),
                        String::from(rhs),
                        String::from(register),
                    ));
                }
                "LSHIFT" => {
                    return Ok(Operation::LShift(
                        String::from(lhs),
                        rhs.parse::<u32>().map_err(|_| ParseOperationError {
                            message: format!("Could not parse rhs with value {}", rhs),
                        })?,
                        String::from(register),
                    ));
                }
                "RSHIFT" => {
                    return Ok(Operation::RShift(
                        String::from(lhs),
                        rhs.parse::<u32>().map_err(|_| ParseOperationError {
                            message: format!("Could not parse rhs with value {}", rhs),
                        })?,
                        String::from(register),
                    ));
                }
                _ => panic!("Operation::from_str received invalid command: {}", command),
            };
        }

        let negation_re = Regex::new(r"NOT (?P<value>\w+) -> (?P<register>\w+)").unwrap();
        if let Some(captures) = negation_re.captures(s) {
            let value = &captures["value"];
            let register = &captures["register"];

            return Ok(Operation::Not(String::from(value), String::from(register)));
        }

        // NOTE: This regex is too generic so we do this last
        let assignment_re = Regex::new(r"(?P<value>\w+) -> (?P<register>\w+)").unwrap();
        if let Some(captures) = assignment_re.captures(s) {
            let value = &captures["value"];
            let register = &captures["register"];

            return Ok(Operation::Assign(
                String::from(value),
                String::from(register),
            ));
        }

        panic!(
            "Operation::from_str could not parse the following input: {}",
            s
        )
    }
}

fn parse_data() -> impl Iterator<Item = Result<Operation, ParseOperationError>> {
    include_str!("./data/yr2015_07.in")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(Operation::from_str)
}

pub fn part1() -> u32 {
    let mut registers: HashMap<String, u32> = HashMap::new();
    let mut circular_queue: VecDeque<Operation> = VecDeque::new();

    parse_data().for_each(|operation| match operation {
        Ok(op) => circular_queue.push_back(op),
        Err(e) => panic!("Somethign went wrong parsing the operation: {}", e.message),
    });

    while !circular_queue.is_empty() {
        // process each operation
        let front = circular_queue.pop_front();

        match front {
            Some(operation) => match operation {
                Operation::Assign(v, register) => {
                    if let Ok(v_u32) = v.parse::<u32>() {
                        registers.insert(register, v_u32);
                    } else if let Some(&v_u32) = registers.get(&v) {
                        registers.insert(register, v_u32);
                    } else {
                        circular_queue.push_back(Operation::Assign(v, register));
                    }
                }
                Operation::Not(v, register) => {
                    if let Ok(v_u32) = v.parse::<u32>() {
                        registers.insert(register, !v_u32);
                    } else if let Some(&v_u32) = registers.get(&v) {
                        registers.insert(register, !v_u32);
                    } else {
                        circular_queue.push_back(Operation::Not(v, register));
                    }
                }
                Operation::And(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs.parse::<u32>();

                    match (lhs_u32, rhs_u32) {
                        (Ok(lhs_u32_v), Ok(rhs_u32_v)) => {
                            registers.insert(register, lhs_u32_v & rhs_u32_v);
                        }
                        (Ok(lhs_u32_v), Err(_)) => {
                            if let Some(&rhs_u32_v) = registers.get(&rhs) {
                                registers.insert(register, lhs_u32_v & rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::And(lhs, rhs, register));
                            }
                        }
                        (Err(_), Ok(rhs_u32_v)) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v & rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::And(lhs, rhs, register));
                            }
                        }
                        (Err(_), Err(_)) => {
                            let lhs_u32 = registers.get(&lhs);
                            let rhs_u32 = registers.get(&rhs);

                            match (lhs_u32, rhs_u32) {
                                (Some(&lhs_u32_v), Some(&rhs_u32_v)) => {
                                    registers.insert(register, lhs_u32_v & rhs_u32_v);
                                }
                                _ => {
                                    circular_queue.push_back(Operation::And(lhs, rhs, register));
                                }
                            }
                        }
                    }
                }
                Operation::Or(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs.parse::<u32>();

                    match (lhs_u32, rhs_u32) {
                        (Ok(lhs_u32_v), Ok(rhs_u32_v)) => {
                            registers.insert(register, lhs_u32_v | rhs_u32_v);
                        }
                        (Ok(lhs_u32_v), Err(_)) => {
                            if let Some(&rhs_u32_v) = registers.get(&rhs) {
                                registers.insert(register, lhs_u32_v | rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::Or(lhs, rhs, register));
                            }
                        }
                        (Err(_), Ok(rhs_u32_v)) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v | rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::Or(lhs, rhs, register));
                            }
                        }
                        (Err(_), Err(_)) => {
                            let lhs_u32 = registers.get(&lhs);
                            let rhs_u32 = registers.get(&rhs);

                            match (lhs_u32, rhs_u32) {
                                (Some(&lhs_u32_v), Some(&rhs_u32_v)) => {
                                    registers.insert(register, lhs_u32_v | rhs_u32_v);
                                }
                                _ => {
                                    circular_queue.push_back(Operation::Or(lhs, rhs, register));
                                }
                            }
                        }
                    }
                }
                Operation::RShift(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs;

                    match lhs_u32 {
                        Ok(lhs_u32_v) => {
                            registers.insert(register, lhs_u32_v >> rhs_u32);
                        }
                        Err(_) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v >> rhs_u32);
                            } else {
                                circular_queue.push_back(Operation::RShift(lhs, rhs, register));
                            }
                        }
                    }
                }
                Operation::LShift(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs;

                    match lhs_u32 {
                        Ok(lhs_u32_v) => {
                            registers.insert(register, lhs_u32_v << rhs_u32);
                        }
                        Err(_) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v << rhs_u32);
                            } else {
                                circular_queue.push_back(Operation::LShift(lhs, rhs, register));
                            }
                        }
                    }
                }
            },
            None => panic!("Operation could not be deqeueued from deque"),
        }
    }

    *registers
        .get("a")
        .expect("Could not get the proper value for a")
}

pub fn part2() -> u32 {
    let mut registers: HashMap<String, u32> = HashMap::new();
    let mut circular_queue: VecDeque<Operation> = VecDeque::new();

    parse_data().for_each(|operation| match operation {
        Ok(op) => match op {
            Operation::Assign(v, register) => {
                if &register == "b" {
                    circular_queue.push_back(Operation::Assign(String::from("3176"), register));
                } else {
                    circular_queue.push_back(Operation::Assign(v, register));
                }
            }
            _ => circular_queue.push_back(op),
        },
        Err(e) => panic!("Somethign went wrong parsing the operation: {}", e.message),
    });

    while !circular_queue.is_empty() {
        let front = circular_queue.pop_front();

        match front {
            Some(operation) => match operation {
                Operation::Assign(v, register) => {
                    if let Ok(v_u32) = v.parse::<u32>() {
                        registers.insert(register, v_u32);
                    } else if let Some(&v_u32) = registers.get(&v) {
                        registers.insert(register, v_u32);
                    } else {
                        circular_queue.push_back(Operation::Assign(v, register));
                    }
                }
                Operation::Not(v, register) => {
                    if let Ok(v_u32) = v.parse::<u32>() {
                        registers.insert(register, !v_u32);
                    } else if let Some(&v_u32) = registers.get(&v) {
                        registers.insert(register, !v_u32);
                    } else {
                        circular_queue.push_back(Operation::Not(v, register));
                    }
                }
                Operation::And(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs.parse::<u32>();

                    match (lhs_u32, rhs_u32) {
                        (Ok(lhs_u32_v), Ok(rhs_u32_v)) => {
                            registers.insert(register, lhs_u32_v & rhs_u32_v);
                        }
                        (Ok(lhs_u32_v), Err(_)) => {
                            if let Some(&rhs_u32_v) = registers.get(&rhs) {
                                registers.insert(register, lhs_u32_v & rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::And(lhs, rhs, register));
                            }
                        }
                        (Err(_), Ok(rhs_u32_v)) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v & rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::And(lhs, rhs, register));
                            }
                        }
                        (Err(_), Err(_)) => {
                            let lhs_u32 = registers.get(&lhs);
                            let rhs_u32 = registers.get(&rhs);

                            match (lhs_u32, rhs_u32) {
                                (Some(&lhs_u32_v), Some(&rhs_u32_v)) => {
                                    registers.insert(register, lhs_u32_v & rhs_u32_v);
                                }
                                _ => {
                                    circular_queue.push_back(Operation::And(lhs, rhs, register));
                                }
                            }
                        }
                    }
                }
                Operation::Or(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs.parse::<u32>();

                    match (lhs_u32, rhs_u32) {
                        (Ok(lhs_u32_v), Ok(rhs_u32_v)) => {
                            registers.insert(register, lhs_u32_v | rhs_u32_v);
                        }
                        (Ok(lhs_u32_v), Err(_)) => {
                            if let Some(&rhs_u32_v) = registers.get(&rhs) {
                                registers.insert(register, lhs_u32_v | rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::Or(lhs, rhs, register));
                            }
                        }
                        (Err(_), Ok(rhs_u32_v)) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v | rhs_u32_v);
                            } else {
                                circular_queue.push_back(Operation::Or(lhs, rhs, register));
                            }
                        }
                        (Err(_), Err(_)) => {
                            let lhs_u32 = registers.get(&lhs);
                            let rhs_u32 = registers.get(&rhs);

                            match (lhs_u32, rhs_u32) {
                                (Some(&lhs_u32_v), Some(&rhs_u32_v)) => {
                                    registers.insert(register, lhs_u32_v | rhs_u32_v);
                                }
                                _ => {
                                    circular_queue.push_back(Operation::Or(lhs, rhs, register));
                                }
                            }
                        }
                    }
                }
                Operation::RShift(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs;

                    match lhs_u32 {
                        Ok(lhs_u32_v) => {
                            registers.insert(register, lhs_u32_v >> rhs_u32);
                        }
                        Err(_) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v >> rhs_u32);
                            } else {
                                circular_queue.push_back(Operation::RShift(lhs, rhs, register));
                            }
                        }
                    }
                }
                Operation::LShift(lhs, rhs, register) => {
                    let lhs_u32 = lhs.parse::<u32>();
                    let rhs_u32 = rhs;

                    match lhs_u32 {
                        Ok(lhs_u32_v) => {
                            registers.insert(register, lhs_u32_v << rhs_u32);
                        }
                        Err(_) => {
                            if let Some(&lhs_u32_v) = registers.get(&lhs) {
                                registers.insert(register, lhs_u32_v << rhs_u32);
                            } else {
                                circular_queue.push_back(Operation::LShift(lhs, rhs, register));
                            }
                        }
                    }
                }
            },
            None => panic!("Operation could not be deqeueued from deque"),
        }
    }

    *registers
        .get("a")
        .expect("Could not get the proper value for a")
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
        info!("Answer for Advent of Code 2015 - Day 07 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 07 - Part 2: {}", ans);
    }
}
