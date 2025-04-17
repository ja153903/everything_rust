use regex::Regex;

#[derive(Debug)]
enum InstructionType {
    Rect {
        l: usize,
        r: usize,
    },
    Rotate {
        x_or_y: String,
        value: usize,
        delta: usize,
    },
}

fn parse_data() -> Vec<Option<InstructionType>> {
    let rect_re = Regex::new(r"rect (?P<l>\d+)x(?P<r>\d+)").unwrap();
    let rotation_re =
        Regex::new(r"rotate (column|row) (?P<x_or_y>x|y)=(?P<value>\d+) by (?P<delta>\d+)")
            .unwrap();

    include_str!("./data/yr2016_08.in")
        .lines()
        .map(|line| {
            if let Some(captures) = rect_re.captures(line) {
                Some(InstructionType::Rect {
                    l: captures["l"].parse().unwrap(),
                    r: captures["r"].parse().unwrap(),
                })
            } else {
                rotation_re
                    .captures(line)
                    .map(|captures| InstructionType::Rotate {
                        x_or_y: String::from(&captures["x_or_y"]),
                        value: captures["value"].parse().unwrap(),
                        delta: captures["delta"].parse().unwrap(),
                    })
            }
        })
        .collect()
}

pub fn part1() -> usize {
    // We initialize the screen here with the appropriate dimensions
    let mut screen = vec![vec![false; 50]; 6];
    let data = parse_data();

    for data in data.iter() {
        match data {
            None => panic!("Why is this empty?"),
            Some(instruction) => match instruction {
                InstructionType::Rect { l, r } => {
                    (0..*r).for_each(|row| {
                        for col in 0..*l {
                            screen[row][col] = true;
                        }
                    });
                }
                InstructionType::Rotate {
                    x_or_y,
                    value,
                    delta,
                } => {
                    // just a note here
                    // what this is saying is that if we get y=A then this means that we're
                    // shifting letters to the right by B
                    // If we get x=A, then this means that we're vertically shifting letters by B
                    if x_or_y == "x" {
                        let shifts = delta % 6;
                        let mut temp_vec = [false; 6];

                        for row in 0..6 {
                            temp_vec[(row + shifts) % 6] = screen[row][*value];
                        }

                        for row in 0..6 {
                            screen[row][*value] = temp_vec[row];
                        }
                    } else {
                        let shifts = delta % 50;
                        screen[*value].rotate_right(shifts);
                    }
                }
            },
        }
    }

    screen
        .iter()
        .fold(0, |acc, row| acc + row.iter().filter(|&b| *b).count())
}

pub fn part2() {
    let mut screen = vec![vec![false; 50]; 6];
    let data = parse_data();

    for data in data.iter() {
        match data {
            None => panic!("Why is this empty?"),
            Some(instruction) => match instruction {
                InstructionType::Rect { l, r } => {
                    (0..*r).for_each(|row| {
                        for col in 0..*l {
                            screen[row][col] = true;
                        }
                    });
                }
                InstructionType::Rotate {
                    x_or_y,
                    value,
                    delta,
                } => {
                    // just a note here
                    // what this is saying is that if we get y=A then this means that we're
                    // shifting letters to the right by B
                    // If we get x=A, then this means that we're vertically shifting letters by B
                    if x_or_y == "x" {
                        let shifts = delta % 6;
                        let mut temp_vec = [false; 6];

                        for row in 0..6 {
                            temp_vec[(row + shifts) % 6] = screen[row][*value];
                        }

                        for row in 0..6 {
                            screen[row][*value] = temp_vec[row];
                        }
                    } else {
                        let shifts = delta % 50;
                        screen[*value].rotate_right(shifts);
                    }
                }
            },
        }
    }

    (0..6).for_each(|row| {
        for col in 0..50 {
            if screen[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    });
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
        part2();
        // info!("Answer for Advent of Code 2015 - Day 01 - Part 2: {}");
    }
}
