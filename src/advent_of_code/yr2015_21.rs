use itertools::Itertools;
use std::str::FromStr;

fn parse_data() -> &'static str {
    include_str!("./data/yr2015_21.in")
}

#[derive(Debug)]
struct Item {
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Item {
    fn new(cost: i64, damage: i64, armor: i64) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

struct User {
    hit_points: i64,
    damage: i64,
    armor: i64,
    current_hit_points: i64,
}

impl User {
    fn new() -> Self {
        Self {
            current_hit_points: 100,
            hit_points: 100,
            damage: 0,
            armor: 0,
        }
    }

    fn update(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    fn restore_health(&mut self) {
        self.current_hit_points = self.hit_points;
    }

    fn reset(&mut self) {
        self.current_hit_points = self.hit_points;
        self.damage = 0;
        self.armor = 0;
    }
}

#[derive(Debug)]
pub struct ParseUserError;

impl FromStr for User {
    type Err = ParseUserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.lines().collect();
        let hit_point_parts: Vec<&str> = parts[0].split(": ").collect();
        let hit_points = hit_point_parts[1]
            .parse::<i64>()
            .map_err(|_| ParseUserError)?;

        let damage_parts: Vec<&str> = parts[1].split(": ").collect();
        let damage = damage_parts[1].parse::<i64>().map_err(|_| ParseUserError)?;

        let armor_parts: Vec<&str> = parts[2].split(": ").collect();
        let armor = armor_parts[1].parse::<i64>().map_err(|_| ParseUserError)?;

        Ok(User {
            current_hit_points: hit_points,
            hit_points,
            damage,
            armor,
        })
    }
}

fn parse_boss() -> Result<User, ParseUserError> {
    User::from_str(parse_data())
}

#[derive(Debug)]
struct Inventory {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

fn get_inventory() -> Inventory {
    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];

    let armors = vec![
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];

    let rings = vec![
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];

    Inventory {
        weapons,
        armors,
        rings,
    }
}

pub fn part1() -> Result<i64, ParseUserError> {
    let mut boss = parse_boss()?;
    let mut me = User::new();

    // armor is optional
    // you have to buy one weapon
    // you can buy 0-2 rings

    let mut min_cost = i64::MAX;
    let inventory = get_inventory();

    // we have to choose one weapon
    for weapon in inventory.weapons.iter() {
        // we can choose to have 0 or 1 armor
        for armor_choice in 0..=1 {
            for armors in inventory.armors.iter().combinations(armor_choice) {
                // we can choose to have 0 to 2 rings
                for ring_choice in 0..=2 {
                    for rings in inventory.rings.iter().combinations(ring_choice) {
                        let mut current_cost = 0;
                        me.update(weapon);
                        current_cost += weapon.cost;

                        armors.iter().for_each(|item| {
                            me.update(item);
                            current_cost += item.cost;
                        });

                        rings.iter().for_each(|item| {
                            me.update(item);
                            current_cost += item.cost;
                        });

                        // figure out if we can win this fight

                        let mut user_turn = true;
                        while boss.current_hit_points > 0 && me.current_hit_points > 0 {
                            if user_turn {
                                boss.current_hit_points -= (me.damage - boss.armor).max(1);
                            } else {
                                me.current_hit_points -= (boss.damage - me.armor).max(1);
                            }

                            user_turn = !user_turn;
                        }

                        if me.current_hit_points > 0 {
                            min_cost = min_cost.min(current_cost);
                        }

                        boss.restore_health();
                        me.reset();
                    }
                }
            }
        }
    }

    Ok(min_cost)
}

pub fn part2() -> Result<i64, ParseUserError> {
    let mut boss = parse_boss()?;
    let mut me = User::new();

    // armor is optional
    // you have to buy one weapon
    // you can buy 0-2 rings

    let mut max_cost = i64::MIN;
    let inventory = get_inventory();

    // we have to choose one weapon
    for weapon in inventory.weapons.iter() {
        // we can choose to have 0 or 1 armor
        for armor_choice in 0..=1 {
            for armors in inventory.armors.iter().combinations(armor_choice) {
                // we can choose to have 0 to 2 rings
                for ring_choice in 0..=2 {
                    for rings in inventory.rings.iter().combinations(ring_choice) {
                        let mut current_cost = 0;
                        me.update(weapon);
                        current_cost += weapon.cost;

                        armors.iter().for_each(|item| {
                            me.update(item);
                            current_cost += item.cost;
                        });

                        rings.iter().for_each(|item| {
                            me.update(item);
                            current_cost += item.cost;
                        });

                        // figure out if we can win this fight

                        let mut user_turn = true;
                        while boss.current_hit_points > 0 && me.current_hit_points > 0 {
                            if user_turn {
                                boss.current_hit_points -= (me.damage - boss.armor).max(1);
                            } else {
                                me.current_hit_points -= (boss.damage - me.armor).max(1);
                            }

                            user_turn = !user_turn;
                        }

                        if me.current_hit_points <= 0 {
                            max_cost = max_cost.max(current_cost);
                        }

                        boss.restore_health();
                        me.reset();
                    }
                }
            }
        }
    }

    Ok(max_cost)
}

#[cfg(test)]
pub mod tests {

    use log::info;

    use super::{part1, part2, ParseUserError};
    use crate::advent_of_code::test_logger;

    #[ctor::ctor]
    fn init() {
        test_logger::setup();
    }

    #[ignore]
    #[test]
    pub fn run_part1() -> Result<(), ParseUserError> {
        let ans = part1()?;
        info!("Answer for Advent of Code 2015 - Day 21 - Part 1: {}", ans);

        Ok(())
    }

    #[ignore]
    #[test]
    pub fn run_part2() -> Result<(), ParseUserError> {
        let ans = part2()?;
        info!("Answer for Advent of Code 2015 - Day 21 - Part 2: {}", ans);

        Ok(())
    }
}
