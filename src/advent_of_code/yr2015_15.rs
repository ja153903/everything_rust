use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn scale(&self, n: i64, with_calorie: bool) -> Ingredient {
        Ingredient {
            capacity: self.capacity * n,
            durability: self.durability * n,
            flavor: self.flavor * n,
            texture: self.texture * n,
            calories: if with_calorie {
                self.calories * n
            } else {
                self.calories
            },
        }
    }
}

#[derive(Debug)]
struct ParseIngredientError;

impl FromStr for Ingredient {
    type Err = ParseIngredientError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)"
        ).unwrap();

        match re.captures(line) {
            Some(captures) => {
                let capacity = captures["capacity"]
                    .parse::<i64>()
                    .map_err(|_| ParseIngredientError)?;
                let durability = captures["durability"]
                    .parse::<i64>()
                    .map_err(|_| ParseIngredientError)?;
                let flavor = captures["flavor"]
                    .parse::<i64>()
                    .map_err(|_| ParseIngredientError)?;
                let texture = captures["texture"]
                    .parse::<i64>()
                    .map_err(|_| ParseIngredientError)?;
                let calories = captures["calories"]
                    .parse::<i64>()
                    .map_err(|_| ParseIngredientError)?;

                Ok(Self {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                })
            }
            None => {
                panic!("Could not parse line properly!")
            }
        }
    }
}

fn parse_data() -> Vec<Result<Ingredient, ParseIngredientError>> {
    include_str!("./data/yr2015_15.in")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(Ingredient::from_str)
        .collect()
}

pub fn part1() -> i64 {
    let ingredients = parse_data();
    let mut result = 0;

    for i in 1..100 {
        for j in 1..100 - i {
            for k in 1..(100 - j) {
                for l in 1..(100 - k) {
                    if i + j + k + l != 100 {
                        continue;
                    }

                    let mut current_perm: HashMap<String, i64> = HashMap::new();

                    for (ingredient, factor) in ingredients.iter().zip([i, j, k, l].iter()) {
                        match ingredient {
                            Ok(ingredient) => {
                                let scaled = ingredient.scale(*factor, false);
                                current_perm
                                    .entry(String::from("capacity"))
                                    .and_modify(|e| *e += scaled.capacity)
                                    .or_insert(scaled.capacity);

                                current_perm
                                    .entry(String::from("durability"))
                                    .and_modify(|e| *e += scaled.durability)
                                    .or_insert(scaled.durability);

                                current_perm
                                    .entry(String::from("flavor"))
                                    .and_modify(|e| *e += scaled.flavor)
                                    .or_insert(scaled.flavor);

                                current_perm
                                    .entry(String::from("texture"))
                                    .and_modify(|e| *e += scaled.texture)
                                    .or_insert(scaled.texture);
                            }
                            Err(_) => {
                                panic!("Something went wrong!")
                            }
                        }
                    }

                    let mut product = 1;
                    let mut contains_negative = false;

                    for &value in current_perm.values() {
                        if value <= 0 {
                            contains_negative = true;
                            break;
                        }
                        product *= value;
                    }

                    if !contains_negative && product > 0 {
                        result = result.max(product);
                    }
                }
            }
        }
    }

    result
}

pub fn part2() -> i64 {
    let ingredients = parse_data();

    let mut result = 0;

    for i in 1..100 {
        for j in 1..100 - i {
            for k in 1..(100 - j) {
                for l in 1..(100 - k) {
                    if i + j + k + l != 100 {
                        continue;
                    }

                    let mut current_perm: HashMap<String, i64> = HashMap::new();

                    for (ingredient, factor) in ingredients.iter().zip([i, j, k, l].iter()) {
                        match ingredient {
                            Ok(ingredient) => {
                                let scaled = ingredient.scale(*factor, true);
                                current_perm
                                    .entry(String::from("capacity"))
                                    .and_modify(|e| *e += scaled.capacity)
                                    .or_insert(scaled.capacity);

                                current_perm
                                    .entry(String::from("durability"))
                                    .and_modify(|e| *e += scaled.durability)
                                    .or_insert(scaled.durability);

                                current_perm
                                    .entry(String::from("flavor"))
                                    .and_modify(|e| *e += scaled.flavor)
                                    .or_insert(scaled.flavor);

                                current_perm
                                    .entry(String::from("texture"))
                                    .and_modify(|e| *e += scaled.texture)
                                    .or_insert(scaled.texture);

                                current_perm
                                    .entry(String::from("calories"))
                                    .and_modify(|e| *e += scaled.calories)
                                    .or_insert(scaled.calories);
                            }
                            Err(_) => {
                                panic!("Something went wrong!")
                            }
                        }
                    }

                    let mut product = 1;
                    let mut contains_negative = false;

                    if let Some(&calories) = current_perm.get("calories") {
                        if calories == 500 {
                            for &value in current_perm.values() {
                                if value <= 0 {
                                    contains_negative = true;
                                    break;
                                }
                                product *= value;
                            }

                            product /= 500;
                            if !contains_negative && product > 0 {
                                result = result.max(product);
                            }
                        }
                    }
                }
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
        info!("Answer for Advent of Code 2015 - Day 01 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 01 - Part 2: {}", ans);
    }
}
