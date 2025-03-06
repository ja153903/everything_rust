use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

fn parse_data() -> HashMap<i64, HashMap<Rc<String>, i64>> {
    let mut data: HashMap<i64, HashMap<Rc<String>, i64>> = HashMap::new();
    let re = Regex::new(
        r"Sue (?P<id>\d+): (?P<compound1>\w+): (?P<compound1_value>\d+), (?P<compound2>\w+): (?P<compound2_value>\d+), (?P<compound3>\w+): (?P<compound3_value>\d+)"
    ).unwrap();
    include_str!("./data/yr2015_16.in")
        .lines()
        .for_each(|line| match re.captures(line) {
            None => panic!("Line was not parsed correctly"),
            Some(captures) => {
                let id = captures["id"].parse::<i64>().unwrap();

                let compound1 = Rc::new(String::from(&captures["compound1"]));
                let compound1_value = captures["compound1_value"].parse::<i64>().unwrap();

                let compound2 = Rc::new(String::from(&captures["compound2"]));
                let compound2_value = captures["compound2_value"].parse::<i64>().unwrap();

                let compound3 = Rc::new(String::from(&captures["compound3"]));
                let compound3_value = captures["compound3_value"].parse::<i64>().unwrap();

                let mut inner_map = HashMap::new();
                inner_map.insert(compound1, compound1_value);
                inner_map.insert(compound2, compound2_value);
                inner_map.insert(compound3, compound3_value);

                data.insert(id, inner_map);
            }
        });

    data
}

fn get_mfcsam() -> HashMap<Rc<String>, i64> {
    let mut mfcsam: HashMap<Rc<String>, i64> = HashMap::new();
    mfcsam.insert(Rc::new(String::from("children")), 3);
    mfcsam.insert(Rc::new(String::from("cats")), 7);
    mfcsam.insert(Rc::new(String::from("samoyeds")), 2);
    mfcsam.insert(Rc::new(String::from("pomeranians")), 3);
    mfcsam.insert(Rc::new(String::from("akitas")), 0);
    mfcsam.insert(Rc::new(String::from("vizslas")), 0);
    mfcsam.insert(Rc::new(String::from("goldfish")), 5);
    mfcsam.insert(Rc::new(String::from("trees")), 3);
    mfcsam.insert(Rc::new(String::from("cars")), 2);
    mfcsam.insert(Rc::new(String::from("perfumes")), 1);

    mfcsam
}

pub fn part1() -> i64 {
    let mfcsam = get_mfcsam();
    let data = parse_data();

    for (&id, map) in data.iter() {
        let mut has_all_matching = true;
        for (key, value) in map.iter() {
            if let Some(mfcsam_value) = mfcsam.get(key) {
                if value != mfcsam_value {
                    has_all_matching = false;
                    break;
                }
            }
        }

        if has_all_matching {
            return id;
        }
    }

    panic!("Why did you not find the proper gift?")
}

pub fn part2() -> i64 {
    let mfcsam = get_mfcsam();
    let data = parse_data();

    for (&id, map) in data.iter() {
        let mut has_all_matching = true;
        for (key, value) in map.iter() {
            if let Some(mfcsam_value) = mfcsam.get(key) {
                match key.as_str() {
                    "cats" | "trees" => {
                        if value <= mfcsam_value {
                            has_all_matching = false;
                            break;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if value >= mfcsam_value {
                            has_all_matching = false;
                            break;
                        }
                    }
                    _ => {
                        if value != mfcsam_value {
                            has_all_matching = false;
                            break;
                        }
                    }
                }
            }
        }

        if has_all_matching {
            return id;
        }
    }

    panic!("Why did you not find the proper gift?")
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
        info!("Answer for Advent of Code 2015 - Day 16 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 16 - Part 2: {}", ans);
    }
}
