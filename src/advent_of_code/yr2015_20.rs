use std::collections::HashSet;

const INPUT: i64 = 29000000;

pub fn part1() -> i64 {
    for i in 1..=(INPUT / 10) {
        let mut unique_factors = HashSet::new();

        for j in 1..i.isqrt() + 1 {
            if i % j == 0 {
                unique_factors.insert(j);

                if j != i / j {
                    unique_factors.insert(i / j);
                }
            }
        }

        let mut sum = 0;

        for item in unique_factors.iter() {
            sum += item * 10;
        }

        if sum >= INPUT {
            return i;
        }
    }

    0
}

pub fn part2() -> i64 {
    for house in 665280..=(INPUT / 11) {
        let mut unique_factors = HashSet::new();

        // elf_i will deliver at most to 50 * i
        // so we have to remove factors that aren't applicable for house_j
        // house_j should receive presents from factors of j
        // however we should check if house_j is a valid target for elf_i

        for elf in 1..house.isqrt() + 1 {
            if house % elf == 0 {
                if elf * 50 >= house {
                    unique_factors.insert(elf);
                }

                if elf != house / elf && (house / elf) * 50 >= house {
                    unique_factors.insert(house / elf);
                }
            }
        }

        let mut sum = 0;

        for elf in unique_factors.iter() {
            sum += elf * 11;
        }

        if sum >= INPUT {
            return house;
        }
    }

    0
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
        info!("Answer for Advent of Code 2015 - Day 20 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 20 - Part 2: {}", ans);
    }
}
