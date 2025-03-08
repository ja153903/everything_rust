use std::collections::HashSet;

fn parse_data() -> Vec<String> {
    include_str!("./data/yr2015_19.in")
        .lines()
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}

pub fn part1() -> usize {
    let lines = parse_data();
    let mappings = lines.iter().take(lines.len() - 1);
    let input = lines.last().unwrap();

    let mut replacements: Vec<(String, String)> = Vec::new();

    for mapping in mappings {
        let splits: Vec<&str> = mapping.split(" => ").collect();
        let pre = String::from(splits[0]);
        let post = String::from(splits[1]);
        replacements.push((pre, post));
    }

    let mut unique_molecules: HashSet<String> = HashSet::new();

    for (pre, post) in replacements.iter() {
        // find every instance of pre and replace with post
        for (start, _str) in input.match_indices(pre) {
            unique_molecules.insert(format!(
                "{}{}{}",
                &input[0..start],
                post,
                &input[start + pre.len()..],
            ));
        }
    }

    unique_molecules.len()
}

pub fn part2() -> usize {
    let lines = parse_data();
    let mappings = lines.iter().take(lines.len() - 1);
    let input = lines.last().unwrap();

    let mut replacements: Vec<(String, String)> = Vec::new();

    for mapping in mappings {
        let splits: Vec<&str> = mapping.split(" => ").collect();
        let pre = String::from(splits[0]);
        let post = String::from(splits[1]);
        replacements.push((pre, post));
    }

    // NOTE: This approach is too slow; I'm going to try something else
    // let mut queue: VecDeque<(String, i64)> = VecDeque::new();
    // let mut visited: HashSet<String> = HashSet::new();
    //
    // queue.push_back((input.clone(), 0));
    //
    // while let Some(front) = queue.pop_front() {
    //     let (s, steps) = front;
    //
    //     if s == "e" {
    //         return steps;
    //     }
    //
    //     for (pre, post) in replacements.iter() {
    //         for (start, _str) in s.match_indices(post) {
    //             let mut transformed = s.clone();
    //             transformed.replace_range(start..(start + post.len()), pre);
    //
    //             if visited.contains(&transformed) {
    //                 continue;
    //             }
    //
    //             visited.insert(transformed.clone());
    //             queue.push_back((transformed.clone(), steps + 1));
    //         }
    //     }
    // }

    let molecule = input.clone();
    // let mut steps = 0;
    //
    // replacements.sort_by(|(_a_to, a_from), (_b_to, b_from)| b_from.len().cmp(&a_from.len()));
    //
    // while molecule != "e" {
    //     for (pre, post) in replacements.iter() {
    //         let start = molecule.find(post);
    //         match start {
    //             None => continue,
    //             Some(i) => {
    //                 let mut transformed = molecule.clone();
    //                 transformed.replace_range(i..(i + post.len()), pre);
    //                 steps += 1;
    //                 molecule = transformed;
    //                 println!("{molecule}");
    //             }
    //         }
    //     }
    // }
    //
    // steps

    // NOTE: Stole this from the internet, no clue why my solutions above didn't work
    let elements = molecule.chars().filter(char::is_ascii_uppercase).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches('Y').count();

    elements - ar - rn - 2 * y - 1
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
        info!("Answer for Advent of Code 2015 - Day 19 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 19 - Part 2: {}", ans);
    }
}
