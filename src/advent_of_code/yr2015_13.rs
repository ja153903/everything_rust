use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

fn parse_data() -> HashMap<Arc<String>, HashMap<Arc<String>, i64>> {
    let mut graph: HashMap<Arc<String>, HashMap<Arc<String>, i64>> = HashMap::new();
    let re = Regex::new(
        r"(?P<u>\w+) would (?P<gain_or_lose>gain|lose) (?P<happiness>\d+) happiness units by sitting next to (?P<v>\w+).",
    ).unwrap();

    include_str!("./data/yr2015_13.in")
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some(captures) = re.captures(line) {
                let u = &captures["u"];
                let v = &captures["v"];
                let gain_or_lose = &captures["gain_or_lose"];
                let mut happiness = captures["happiness"].parse::<i64>().unwrap();
                happiness = if gain_or_lose == "gain" {
                    happiness
                } else {
                    -happiness
                };

                let u_arc = Arc::new(String::from(u));
                let v_arc = Arc::new(String::from(v));

                graph
                    .entry(u_arc)
                    .and_modify(|e| {
                        e.entry(v_arc.clone())
                            .and_modify(|e| {
                                *e += happiness;
                            })
                            .or_insert(happiness);
                    })
                    .or_insert_with(|| {
                        let mut hash = HashMap::new();
                        hash.insert(v_arc.clone(), happiness);
                        hash
                    });
            } else {
                panic!("Something went wrong with parsing the data")
            }
        });

    graph
}

pub fn part1() -> i64 {
    let graph = parse_data();
    let unique_users: Vec<Arc<String>> = graph.keys().cloned().collect();
    let k = unique_users.len();

    let mut result = 0;

    for permutation in unique_users.into_iter().permutations(k) {
        // for each permutation, find the appropriate path value
        let mut current = 0;

        for i in 1..permutation.len() {
            let user_a = permutation[i - 1].clone();
            let user_b = permutation[i].clone();

            if let Some(inner_map) = graph.get(&*user_a) {
                if let Some(value) = inner_map.get(&*user_b) {
                    current += value;
                }
            }

            if let Some(inner_map) = graph.get(&*user_b) {
                if let Some(value) = inner_map.get(&*user_a) {
                    current += value;
                }
            }

            if i == permutation.len() - 1 {
                let user_a = permutation[0].clone();
                let user_b = permutation[i].clone();

                if let Some(inner_map) = graph.get(&*user_a) {
                    if let Some(value) = inner_map.get(&*user_b) {
                        current += value;
                    }
                }

                if let Some(inner_map) = graph.get(&*user_b) {
                    if let Some(value) = inner_map.get(&*user_a) {
                        current += value;
                    }
                }
            }

            result = result.max(current);
        }
    }

    result
}

pub fn part2() -> i64 {
    let mut graph = parse_data();
    let mut unique_users: Vec<Arc<String>> = graph.keys().cloned().collect();
    let k = unique_users.len();

    let me_arc = Arc::new(String::from("Jaime"));

    (0..k).for_each(|i| {
        graph
            .entry(me_arc.clone())
            .and_modify(|e| {
                e.entry(unique_users[i].clone()).or_insert(0);
            })
            .or_insert_with(|| {
                let mut hash = HashMap::new();
                hash.insert(unique_users[i].clone(), 0);
                hash
            });

        graph
            .entry(unique_users[i].clone())
            .and_modify(|e| {
                e.entry(me_arc.clone()).or_insert(0);
            })
            .or_insert_with(|| {
                let mut hash = HashMap::new();
                hash.insert(me_arc.clone(), 0);
                hash
            });
    });

    unique_users.push(me_arc.clone());

    let mut result = 0;

    for permutation in unique_users.into_iter().permutations(k + 1) {
        // for each permutation, find the appropriate path value
        let mut current = 0;

        for i in 1..permutation.len() {
            let user_a = permutation[i - 1].clone();
            let user_b = permutation[i].clone();

            if let Some(inner_map) = graph.get(&*user_a) {
                if let Some(value) = inner_map.get(&*user_b) {
                    current += value;
                }
            }

            if let Some(inner_map) = graph.get(&*user_b) {
                if let Some(value) = inner_map.get(&*user_a) {
                    current += value;
                }
            }

            if i == permutation.len() - 1 {
                let user_a = permutation[0].clone();
                let user_b = permutation[i].clone();

                if let Some(inner_map) = graph.get(&*user_a) {
                    if let Some(value) = inner_map.get(&*user_b) {
                        current += value;
                    }
                }

                if let Some(inner_map) = graph.get(&*user_b) {
                    if let Some(value) = inner_map.get(&*user_a) {
                        current += value;
                    }
                }
            }

            result = result.max(current);
        }
    }

    result
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
        info!("Answer for Advent of Code 2015 - Day 13 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 13 - Part 2: {}", ans);
    }
}
