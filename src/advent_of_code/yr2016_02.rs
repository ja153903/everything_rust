fn parse_data() -> Vec<String> {
    include_str!("./data/yr2016_02.in")
        .lines()
        .map(String::from)
        .collect()
}

fn coordinate_to_digit(row: i64, col: i64) -> i64 {
    match (row, col) {
        (0, 0) => 1,
        (0, 1) => 2,
        (0, 2) => 3,
        (1, 0) => 4,
        (1, 1) => 5,
        (1, 2) => 6,
        (2, 0) => 7,
        (2, 1) => 8,
        (2, 2) => 9,
        _ => panic!("Invalid row-col pair provided"),
    }
}

fn coordinate_to_digit_diamond(row: i64, col: i64) -> Option<char> {
    match (row, col) {
        (0, 2) => Some('1'),
        (1, 1) => Some('2'),
        (1, 2) => Some('3'),
        (1, 3) => Some('4'),
        (2, 0) => Some('5'),
        (2, 1) => Some('6'),
        (2, 2) => Some('7'),
        (2, 3) => Some('8'),
        (2, 4) => Some('9'),
        (3, 1) => Some('A'),
        (3, 2) => Some('B'),
        (3, 3) => Some('C'),
        (4, 2) => Some('D'),
        _ => None,
    }
}

pub fn part1() -> String {
    let mut row = 1;
    let mut col = 1;

    parse_data()
        .iter()
        .map(|sequence| {
            for ch in sequence.chars() {
                match ch {
                    'U' => {
                        if row > 0 {
                            row -= 1;
                        }
                    }
                    'D' => {
                        if row < 2 {
                            row += 1;
                        }
                    }
                    'L' => {
                        if col > 0 {
                            col -= 1;
                        }
                    }
                    'R' => {
                        if col < 2 {
                            col += 1;
                        }
                    }
                    _ => panic!("Invalid instruction provided"),
                }
            }

            coordinate_to_digit(row, col).to_string()
        })
        .collect::<String>()
}

pub fn part2() -> String {
    let mut row = 2;
    let mut col = 0;

    parse_data()
        .iter()
        .map(|sequence| {
            for ch in sequence.chars() {
                match ch {
                    'U' => {
                        if row > 0 && coordinate_to_digit_diamond(row - 1, col).is_some() {
                            row -= 1;
                        }
                    }
                    'D' => {
                        if row < 4 && coordinate_to_digit_diamond(row + 1, col).is_some() {
                            row += 1;
                        }
                    }
                    'L' => {
                        if col > 0 && coordinate_to_digit_diamond(row, col - 1).is_some() {
                            col -= 1;
                        }
                    }
                    'R' => {
                        if col < 4 && coordinate_to_digit_diamond(row, col + 1).is_some() {
                            col += 1;
                        }
                    }
                    _ => panic!("Invalid instruction provided"),
                }
            }

            match coordinate_to_digit_diamond(row, col) {
                Some(char) => char.to_string(),
                None => panic!("Uh oh"),
            }
        })
        .collect::<String>()
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
        info!("Answer for Advent of Code 2016 - Day 02 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 02 - Part 2: {}", ans);
    }
}
