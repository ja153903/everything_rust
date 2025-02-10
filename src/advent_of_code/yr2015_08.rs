fn parse_data() -> impl Iterator<Item = &'static str> {
    include_str!("./data/yr2015_08.in")
        .split("\n")
        .filter(|line| !line.is_empty())
}

pub fn part1() -> usize {
    parse_data()
        .map(|line| {
            let mut code_representation: usize = 0;
            let mut in_memory_len: usize = 0;

            let mut i = 0;

            while i < line.len() {
                match &line[i..i + 1] {
                    "\\" => {
                        if &line[i..i + 2] == "\\x" {
                            code_representation += 4;
                            i += 4;
                        } else {
                            code_representation += 2;
                            i += 2;
                        }
                    }
                    _ => {
                        code_representation += 1;
                        i += 1;
                    }
                }
                in_memory_len += 1;
            }

            code_representation - (in_memory_len - 2)
        })
        .sum()
}

pub fn part2() -> usize {
    parse_data()
        .map(|line| {
            let mut expanded_representation: usize = 2;
            let mut code_representation: usize = 0;

            let mut i = 0;

            while i < line.len() {
                match &line[i..i + 1] {
                    "\\" => {
                        expanded_representation += 2;

                        if &line[i..i + 2] == "\\x" {
                            // this accounts for the characters
                            expanded_representation += 3;
                            code_representation += 4;
                            i += 4;
                        } else {
                            expanded_representation += 2;
                            code_representation += 2;
                            i += 2;
                        }
                    }
                    "\"" => {
                        expanded_representation += 2;
                        code_representation += 1;
                        i += 1;
                    }
                    _ => {
                        expanded_representation += 1;
                        code_representation += 1;
                        i += 1;
                    }
                }
            }

            expanded_representation - code_representation
        })
        .sum()
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
        info!("Answer for Advent of Code 2015 - Day 08 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 08 - Part 2: {}", ans);
    }
}
