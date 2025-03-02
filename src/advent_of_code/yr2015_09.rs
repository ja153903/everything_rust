use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::Arc;

// using Arc is better because we can use
// a cheaper abstraction that clones by incrementing references rather
// than actually doing a deep clone
#[derive(Debug)]
struct UndirectedEdge {
    u: Arc<String>,
    v: Arc<String>,
    weight: i64,
}

#[derive(Debug)]
struct ParseUndirectedEdgeError;

impl FromStr for UndirectedEdge {
    type Err = ParseUndirectedEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<u>\w+) to (?P<v>\w+) = (?P<weight>\d+)").unwrap();
        match re.captures(s) {
            Some(captures) => {
                let u = &captures["u"];
                let v = &captures["v"];
                let weight = &captures["weight"];

                Ok(UndirectedEdge {
                    u: Arc::new(String::from(u)),
                    v: Arc::new(String::from(v)),
                    weight: weight
                        .parse::<i64>()
                        .map_err(|_| ParseUndirectedEdgeError)?,
                })
            }
            None => panic!("Something went wrong trying to parse the line"),
        }
    }
}

fn parse_data() -> impl Iterator<Item = Result<UndirectedEdge, ParseUndirectedEdgeError>> {
    include_str!("./data/yr2015_09.in")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(UndirectedEdge::from_str)
}

fn build_graph() -> HashMap<Arc<String>, Vec<(Arc<String>, i64)>> {
    let mut graph: HashMap<Arc<String>, Vec<(Arc<String>, i64)>> = HashMap::new();

    parse_data().for_each(|edge| match edge {
        Ok(edge) => {
            graph
                .entry(Arc::clone(&edge.u))
                .or_default()
                .push((Arc::clone(&edge.v), edge.weight));
            graph
                .entry(Arc::clone(&edge.v))
                .or_default()
                .push((Arc::clone(&edge.u), edge.weight));
        }
        Err(_) => panic!("Something went wrong here"),
    });

    graph
}

pub fn part1() -> i64 {
    let _graph = build_graph();
    let mut result: i64 = i64::MAX;

    for starting_point in _graph.keys() {
        let mut queue: VecDeque<(Arc<String>, i64, usize, String)> = VecDeque::new();

        queue.push_back((Arc::clone(starting_point), 0, 1, starting_point.to_string()));

        while !queue.is_empty() {
            let _front = queue.pop_front();
            match _front {
                Some(front) => {
                    let (key, cost, unique_count, path) = front;

                    if unique_count == _graph.keys().len() {
                        result = result.min(cost);
                        continue;
                    }

                    let children = _graph.get(&key);

                    match children {
                        Some(children) => {
                            for child in children.iter() {
                                let (next_node, cost_for_next) = child;

                                if path.contains(&next_node.to_string()) {
                                    continue;
                                }

                                queue.push_back((
                                    Arc::clone(next_node),
                                    cost + cost_for_next,
                                    unique_count + 1,
                                    if path.is_empty() {
                                        format!("{}", next_node)
                                    } else {
                                        format!("{},{}", path, next_node)
                                    },
                                ));
                            }
                        }
                        None => panic!("This key doesn't exist?"),
                    }
                }
                None => panic!("Why is this empty?"),
            }
        }
    }

    result
}

pub fn part2() -> i64 {
    let _graph = build_graph();
    let mut result: i64 = i64::MIN;

    for starting_point in _graph.keys() {
        let mut queue: VecDeque<(Arc<String>, i64, usize, String)> = VecDeque::new();

        queue.push_back((Arc::clone(starting_point), 0, 1, starting_point.to_string()));

        while !queue.is_empty() {
            let _front = queue.pop_front();
            match _front {
                Some(front) => {
                    let (key, cost, unique_count, path) = front;

                    if unique_count == _graph.keys().len() {
                        result = result.max(cost);
                        continue;
                    }

                    let children = _graph.get(&key);

                    match children {
                        Some(children) => {
                            for child in children.iter() {
                                let (next_node, cost_for_next) = child;

                                if path.contains(&next_node.to_string()) {
                                    continue;
                                }

                                queue.push_back((
                                    Arc::clone(next_node),
                                    cost + cost_for_next,
                                    unique_count + 1,
                                    if path.is_empty() {
                                        format!("{}", next_node)
                                    } else {
                                        format!("{},{}", path, next_node)
                                    },
                                ));
                            }
                        }
                        None => panic!("This key doesn't exist?"),
                    }
                }
                None => panic!("Why is this empty?"),
            }
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
        info!("Answer for Advent of Code 2015 - Day 09 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 09 - Part 2: {}", ans);
    }
}
