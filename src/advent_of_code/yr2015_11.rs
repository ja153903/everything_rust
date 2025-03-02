use std::collections::HashMap;

const INPUT: &str = "hepxcrrq";

// easiest thing to do here is to make sure that
// we process each string as bytes rather than characters
// making sure that we keep it within the appropriate ascii range
// for lower case characters, that means we should keep it within
// this is 97 to 122

pub fn increment(bytes: &mut Vec<u8>) {
    bytes[0] += 1;
    let mut carry = 0;
    if bytes[0] > 122 {
        bytes[0] -= 26;
        carry = 1;
    }

    let mut i = 1;
    while carry > 0 && i < bytes.len() {
        bytes[i] += carry;
        if bytes[i] > 122 {
            bytes[i] -= 26;
            carry = 1;
        } else {
            carry = 0;
        }

        i += 1;
    }

    if carry > 0 {
        // push 97 here because that's the start of the alphabet
        bytes.push(97);
    }
}

pub fn contains_nonoverlapping_pairs(bytes: &[u8]) -> bool {
    let mut counter: HashMap<u8, i32> = HashMap::new();

    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            counter
                .entry(bytes[i])
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }
    }

    counter.len() >= 2
}

pub fn contains_forbidden_words(bytes: &[u8]) -> bool {
    // i -> 105; o -> 111; u -> 117
    bytes.contains(&105) || bytes.contains(&111) || bytes.contains(&117)
}

pub fn contains_three_straight(bytes: &[u8]) -> bool {
    let mut count = 0;

    for i in 1..bytes.len() - 1 {
        if bytes[i - 1] + 1 == bytes[i] && bytes[i] + 1 == bytes[i + 1] {
            count += 1;
        }
    }

    count >= 1
}

fn cycle_password(password: &str) -> String {
    let mut bytes = password.as_bytes().to_vec();

    loop {
        bytes.reverse();
        increment(&mut bytes);
        bytes.reverse();

        if contains_three_straight(&bytes)
            && !contains_forbidden_words(&bytes)
            && contains_nonoverlapping_pairs(&bytes)
        {
            break;
        }
    }

    match String::from_utf8(bytes) {
        Ok(result) => result,
        Err(_) => {
            panic!("Invalid stuff was parsed");
        }
    }
}

pub fn part1() -> String {
    cycle_password(INPUT)
}

pub fn part2() -> String {
    cycle_password(part1().as_str())
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
        info!("Answer for Advent of Code 2015 - Day 11 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 11 - Part 2: {}", ans);
    }
}
