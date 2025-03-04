use anyhow::Result;
use serde_json::Value;

pub fn part1() -> Result<i64> {
    let values: Vec<Value> = serde_json::from_str(include_str!("./data/yr2015_12.in"))?;
    Ok(values.iter().map(|it| dfs(it, false)).sum())
}

fn dfs(value: &Value, code_red: bool) -> i64 {
    if value.is_number() {
        value.as_i64().unwrap()
    } else if value.is_array() {
        let mut result = 0;

        for item in value.as_array().unwrap().iter() {
            result += dfs(item, code_red);
        }

        result
    } else if value.is_object() {
        let mut result = 0;

        for item in value.as_object().unwrap().values() {
            if code_red && item.is_string() && value == "red" {
                // Break out of the iteration if any of these values are "red"
                return 0;
            }
            result += dfs(item, code_red);
        }

        result
    } else {
        0
    }
}

pub fn part2() -> Result<i64> {
    let values: Vec<Value> = serde_json::from_str(include_str!("./data/yr2015_12.in"))?;
    Ok(values.iter().map(|it| dfs(it, true)).sum())
}

#[cfg(test)]
pub mod tests {
    use anyhow::Result;
    use log::info;

    use super::{part1, part2};
    use crate::advent_of_code::test_logger;

    #[ctor::ctor]
    fn init() {
        test_logger::setup();
    }

    #[ignore]
    #[test]
    pub fn run_part1() -> Result<()> {
        let ans = part1()?;
        info!("Answer for Advent of Code 2015 - Day 12 - Part 1: {}", ans);

        Ok(())
    }

    #[ignore]
    #[test]
    pub fn run_part2() -> Result<()> {
        let ans = part2()?;
        info!("Answer for Advent of Code 2015 - Day 12 - Part 2: {}", ans);
        Ok(())
    }
}
