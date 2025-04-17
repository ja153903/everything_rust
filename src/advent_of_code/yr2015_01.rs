fn parse_data() -> &'static str {
    include_str!("./data/yr2015_01.in")
}

pub fn part1() -> i64 {
    parse_data().chars().fold(0, |acc, ch| {
        acc + match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}

pub fn part2() -> usize {
    let mut level = 0;

    for (i, ch) in parse_data().char_indices() {
        level += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if level < 0 {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
pub mod tests {
    use super::{part1, part2};

    #[test]
    pub fn run_part1() {
        let ans = part1();
        assert_eq!(ans, 74);
    }

    #[test]
    pub fn run_part2() {
        let ans = part2();
        assert_eq!(ans, 1795);
    }
}
