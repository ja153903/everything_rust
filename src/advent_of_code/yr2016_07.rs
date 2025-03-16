#![allow(unused_variables)]

fn parse_data() -> Vec<(Vec<String>, Vec<String>)> {
    include_str!("./data/yr2016_07.in")
        .lines()
        .map(|line| {
            let mut within_hypernet: Vec<String> = Vec::new();
            let mut ip_addrs: Vec<String> = Vec::new();

            let mut current = String::new();

            for ch in line.chars() {
                if ch == '[' {
                    ip_addrs.push(current);
                    current = String::new();
                } else if ch == ']' {
                    within_hypernet.push(current);
                    current = String::new();
                } else {
                    current.push(ch);
                }
            }

            if !current.is_empty() {
                ip_addrs.push(current);
            }

            (within_hypernet, ip_addrs)
        })
        .collect()
}

fn has_abba(line: &str) -> bool {
    let mut i = 0;

    while i < line.len() - 3 {
        if line[i..i + 1] == line[i + 3..i + 4]
            && line[i + 1..i + 2] == line[i + 2..i + 3]
            && line[i..i + 1] != line[i + 1..i + 2]
        {
            return true;
        }

        i += 1;
    }

    false
}

pub fn part1() -> usize {
    parse_data()
        .into_iter()
        .filter(|(within_hypernet, ip_addrs)| {
            for ip_addr in within_hypernet {
                if has_abba(ip_addr) {
                    return false;
                }
            }

            for ip_addr in ip_addrs {
                if has_abba(ip_addr) {
                    return true;
                }
            }

            false
        })
        .count()
}

pub fn get_aba(ip_addr: &str) -> Vec<String> {
    let mut aba: Vec<String> = Vec::new();

    let mut i = 0;

    while i < ip_addr.len() - 2 {
        if ip_addr[i..i + 1] == ip_addr[i + 2..i + 3] && ip_addr[i..i + 1] != ip_addr[i + 1..i + 2]
        {
            aba.push(String::from(&ip_addr[i..i + 3]));
        }

        i += 1;
    }

    aba
}

pub fn part2() -> usize {
    parse_data()
        .into_iter()
        .filter(|(within_hypernet, ip_addrs)| {
            for ip_addr in ip_addrs {
                let abas = get_aba(ip_addr);
                for aba in abas {
                    let bab = format!("{}{}{}", &aba[1..2], &aba[0..1], &aba[1..2]);
                    if within_hypernet
                        .iter()
                        .any(|hypernet| hypernet.contains(&bab))
                    {
                        return true;
                    }
                }
            }

            false
        })
        .count()
}

#[cfg(test)]
pub mod tests {
    use log::info;

    use super::{has_abba, part1, part2};
    use crate::advent_of_code::test_logger;

    #[ctor::ctor]
    fn init() {
        test_logger::setup();
    }

    #[test]
    pub fn test_has_abba() {
        assert!(has_abba("abba"));
        assert!(!has_abba("aaaa"));
    }

    #[ignore]
    #[test]
    pub fn run_part1() {
        let ans = part1();
        info!("Answer for Advent of Code 2016 - Day 07 - Part 1: {}", ans);
    }

    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2016 - Day 07 - Part 2: {}", ans);
    }
}
