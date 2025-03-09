#![allow(dead_code, unused_variables, unused_mut)]

use anyhow;

#[derive(Debug)]
enum Effect {
    Shield { turns: i64, armor_increase: i64 },
    Poison { turns: i64, damage_dealt: i64 },
    Recharge { turns: i64, mana_replenished: i64 },
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Effect::Shield { .. }, Effect::Shield { .. })
                | (Effect::Poison { .. }, Effect::Poison { .. })
                | (Effect::Recharge { .. }, Effect::Recharge { .. })
        )
    }
}

#[derive(Debug)]
enum Spell {
    MagicMissile {
        cost: i64,
        damage_dealt: i64,
    },
    Drain {
        cost: i64,
        damage_dealt: i64,
        health_replenished: i64,
    },
    Shield {
        cost: i64,
        effect: Effect,
    },
    Poison {
        cost: i64,
        effect: Effect,
    },
    Recharge {
        cost: i64,
        effect: Effect,
    },
}

#[derive(Debug)]
struct Wizard {
    base_hp: i64,
    base_mana: i64,

    cur_hp: i64,
    cur_mana: i64,

    armor: i64,
    damage: i64,

    active_effects: Vec<Effect>,
}

impl Wizard {
    fn new(hp: i64, mana: i64, damage: i64) -> Self {
        Self {
            base_hp: hp,
            base_mana: mana,
            cur_hp: hp,
            cur_mana: mana,
            armor: 0,
            damage,
            active_effects: Vec::new(),
        }
    }

    fn cast(&mut self, spell: Spell) -> anyhow::Result<()> {
        match spell {
            Spell::MagicMissile { cost, damage_dealt } => {
                if self.cur_mana - cost < 0 {
                    return Err(anyhow::anyhow!("Not enough mana"));
                }

                self.cur_mana -= cost;
                self.damage = damage_dealt;
            }
            Spell::Drain {
                cost,
                damage_dealt,
                health_replenished,
            } => {
                if self.cur_mana - cost < 0 {
                    return Err(anyhow::anyhow!("Not enough mana"));
                }

                self.cur_mana -= cost;
                self.damage = damage_dealt;
                self.cur_hp += health_replenished;
            }
            Spell::Shield { cost, effect } => {
                if self.cur_mana - cost < 0 {
                    return Err(anyhow::anyhow!("Not enough mana"));
                }

                self.cur_mana -= cost;
                self.register_effect(effect)?;
            }
            Spell::Poison { cost, effect } => {
                if self.cur_mana - cost < 0 {
                    return Err(anyhow::anyhow!("Not enough mana"));
                }

                self.cur_mana -= cost;
                self.register_effect(effect)?;
            }
            Spell::Recharge { cost, effect } => {
                if self.cur_mana - cost < 0 {
                    return Err(anyhow::anyhow!("Not enough mana"));
                }

                self.cur_mana -= cost;
                self.register_effect(effect)?;
            }
        }

        Ok(())
    }

    fn register_effect(&mut self, effect: Effect) -> anyhow::Result<()> {
        if !self.active_effects.contains(&effect) {
            self.active_effects.push(effect);
        }

        Err(anyhow::anyhow!(
            "Could not register effect since already exists"
        ))
    }
}

fn get_spells() -> Vec<Spell> {
    vec![
        Spell::MagicMissile {
            cost: 53,
            damage_dealt: 4,
        },
        Spell::Drain {
            cost: 73,
            damage_dealt: 2,
            health_replenished: 2,
        },
        Spell::Shield {
            cost: 113,
            effect: Effect::Shield {
                turns: 6,
                armor_increase: 7,
            },
        },
        Spell::Poison {
            cost: 173,
            effect: Effect::Poison {
                turns: 6,
                damage_dealt: 3,
            },
        },
        Spell::Recharge {
            cost: 229,
            effect: Effect::Recharge {
                turns: 5,
                mana_replenished: 101,
            },
        },
    ]
}

pub fn part1() -> i64 {
    let mut me = Wizard::new(50, 500, 0);
    // NOTE: the boss does not have a mana pool or spell pool
    let mut boss = Wizard::new(51, 500, 9);

    let spells = get_spells();

    // somehow we want to simulate all the possible ways
    // we can win against the user using all sorts of spells...

    0
}

pub fn part2() -> i64 {
    todo!("Not implemented")
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
        info!("Answer for Advent of Code 2015 - Day 22 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 22 - Part 2: {}", ans);
    }
}
