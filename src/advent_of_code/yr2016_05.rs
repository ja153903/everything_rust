const INPUT: &str = "ffykfhsq";

pub fn part1() -> String {
    let mut i = 1;
    let mut result = String::new();

    loop {
        let mut to_hash = String::new();
        to_hash.push_str(INPUT);
        to_hash.push_str(&i.to_string());

        let digest = md5::compute(to_hash.as_bytes());
        let as_hex = format!("{:x}", digest);

        if as_hex.starts_with("00000") {
            result.push(as_hex.chars().nth(5).unwrap());
            if result.len() == 8 {
                break;
            }
        }

        i += 1;
    }

    result
}

pub fn part2() -> String {
    let mut i = 1;
    let mut valid_inserts = 0;
    let mut result: Vec<String> = vec![String::new(); 8];

    loop {
        let mut to_hash = String::new();
        to_hash.push_str(INPUT);
        to_hash.push_str(&i.to_string());

        let digest = md5::compute(to_hash.as_bytes());
        let as_hex = format!("{:x}", digest);

        if as_hex.starts_with("00000") {
            if let Some(position) = as_hex.chars().nth(5) {
                if let Some(position_as_digit) = position.to_digit(10) {
                    if position_as_digit < 8 && result[position_as_digit as usize].is_empty() {
                        if let Some(character) = as_hex.chars().nth(6) {
                            result[position_as_digit as usize] = String::from(character);
                            valid_inserts += 1;
                            if valid_inserts == 8 {
                                break;
                            }
                        }
                    }
                }
            }
        }

        i += 1;
    }

    result.join("")
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
        info!("Answer for Advent of Code 2016 - Day 05 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 05 - Part 2: {}", ans);
    }
}
