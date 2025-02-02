fn parse_data() -> &'static str {
    "iwrupvqb"
}

fn find_smallest_number_to_start_with(s: &str) -> i64 {
    let secret_key = parse_data();
    let mut i = 1;

    loop {
        let input = format!("{}{}", secret_key, i);
        let digest = md5::compute(input.as_bytes());
        let as_hex = format!("{:x}", digest);

        if as_hex.starts_with(s) {
            return i;
        }

        i += 1;
    }
}

pub fn part1() -> i64 {
    find_smallest_number_to_start_with("00000")
}

pub fn part2() -> i64 {
    find_smallest_number_to_start_with("000000")
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

    #[ignore = "This test takes way too long to run normally"]
    #[test]
    pub fn run_part1() {
        let ans = part1();
        info!("Answer for Advent of Code 2015 - Day 04 - Part 1: {}", ans);
    }

    #[ignore = "This test takes way too long to run normally"]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 04 - Part 2: {}", ans);
    }
}
