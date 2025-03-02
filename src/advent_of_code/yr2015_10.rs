#![allow(dead_code)]

fn run(times: usize) -> usize {
    let mut itr_count = 0;
    let mut current_state = String::from("1113222113");

    loop {
        if itr_count == times {
            return current_state.len();
        }

        let mut count = 1;
        let mut current_char = &current_state[0..1];
        // NOTE: that this will timeout if we try to use format!
        // to update next_state. The easiest way to solve this is by
        // using push_str and mutating instead
        let mut next_state = String::new();

        for i in 1..current_state.len() {
            if current_char == &current_state[i..i+1] {
                count += 1;
            } else {
                next_state.push_str(count.to_string().as_str());
                next_state.push_str(current_char);

                current_char = &current_state[i..i+1];
                count = 1;
            }
        }

        next_state.push_str(count.to_string().as_str());
        next_state.push_str(current_char);

        current_state = next_state;
        itr_count += 1;
    }
}

pub fn part1() -> usize {
    run(40)
}

pub fn part2() -> usize {
    run(50)
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
        info!("Answer for Advent of Code 2015 - Day 10 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 10 - Part 2: {}", ans);
    }
}
