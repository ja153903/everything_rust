fn parse_data() -> Vec<Vec<char>> {
    include_str!("./data/yr2015_18.in")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1() -> usize {
    let mut grid = parse_data();

    (0..100).for_each(|_| {
        // on each iteration, we need a vector of state updates to make
        // we also need to iterate over the entire grid to make sure that
        // neighbors are accounted for
        let mut state_updates: Vec<(usize, usize, char)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let mut neighbors = 0;

                // up
                if row > 0 && grid[row - 1][col] == '#' {
                    neighbors += 1;
                }

                // down
                if row < grid.len() - 1 && grid[row + 1][col] == '#' {
                    neighbors += 1;
                }

                // left
                if col > 0 && grid[row][col - 1] == '#' {
                    neighbors += 1;
                }

                // right
                if col < grid[0].len() - 1 && grid[row][col + 1] == '#' {
                    neighbors += 1;
                }

                // up-left
                if row > 0 && col > 0 && grid[row - 1][col - 1] == '#' {
                    neighbors += 1;
                }

                // up-right
                if row > 0 && col < grid[0].len() - 1 && grid[row - 1][col + 1] == '#' {
                    neighbors += 1;
                }

                // down-left
                if row < grid.len() - 1 && col > 0 && grid[row + 1][col - 1] == '#' {
                    neighbors += 1;
                }

                // down-right
                if row < grid.len() - 1 && col < grid[0].len() - 1 && grid[row + 1][col + 1] == '#'
                {
                    neighbors += 1;
                }

                match grid[row][col] {
                    '#' => {
                        if neighbors != 2 && neighbors != 3 {
                            state_updates.push((row, col, '.'));
                        }
                    }
                    '.' => {
                        if neighbors == 3 {
                            state_updates.push((row, col, '#'));
                        }
                    }
                    _ => {
                        panic!("Unknown value parsed in grid")
                    }
                }
            }
        }

        state_updates.iter().for_each(|update| {
            grid[update.0][update.1] = update.2;
        });
    });

    grid.iter().flatten().filter(|&&ch| ch == '#').count()
}

pub fn part2() -> usize {
    let mut grid = parse_data();

    // NOTE: we have a 100x100 grid
    grid[0][0] = '#';
    grid[0][99] = '#';
    grid[99][0] = '#';
    grid[99][99] = '#';

    (0..100).for_each(|_| {
        // on each iteration, we need a vector of state updates to make
        // we also need to iterate over the entire grid to make sure that
        // neighbors are accounted for
        let mut state_updates: Vec<(usize, usize, char)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                match (row, col) {
                    (0, 0) | (0, 99) | (99, 0) | (99, 99) => continue,
                    _ => {
                        let mut neighbors = 0;

                        // up
                        if row > 0 && grid[row - 1][col] == '#' {
                            neighbors += 1;
                        }

                        // down
                        if row < grid.len() - 1 && grid[row + 1][col] == '#' {
                            neighbors += 1;
                        }

                        // left
                        if col > 0 && grid[row][col - 1] == '#' {
                            neighbors += 1;
                        }

                        // right
                        if col < grid[0].len() - 1 && grid[row][col + 1] == '#' {
                            neighbors += 1;
                        }

                        // up-left
                        if row > 0 && col > 0 && grid[row - 1][col - 1] == '#' {
                            neighbors += 1;
                        }

                        // up-right
                        if row > 0 && col < grid[0].len() - 1 && grid[row - 1][col + 1] == '#' {
                            neighbors += 1;
                        }

                        // down-left
                        if row < grid.len() - 1 && col > 0 && grid[row + 1][col - 1] == '#' {
                            neighbors += 1;
                        }

                        // down-right
                        if row < grid.len() - 1
                            && col < grid[0].len() - 1
                            && grid[row + 1][col + 1] == '#'
                        {
                            neighbors += 1;
                        }

                        match grid[row][col] {
                            '#' => {
                                if neighbors != 2 && neighbors != 3 {
                                    state_updates.push((row, col, '.'));
                                }
                            }
                            '.' => {
                                if neighbors == 3 {
                                    state_updates.push((row, col, '#'));
                                }
                            }
                            _ => {
                                panic!("Unknown value parsed in grid")
                            }
                        }
                    }
                }
            }
        }

        state_updates.iter().for_each(|update| {
            grid[update.0][update.1] = update.2;
        });
    });

    grid.iter().flatten().filter(|&&ch| ch == '#').count()
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
        info!("Answer for Advent of Code 2015 - Day 18 - Part 1: {}", ans);
    }

    #[ignore]
    #[test]
    pub fn run_part2() {
        let ans = part2();
        info!("Answer for Advent of Code 2015 - Day 18 - Part 2: {}", ans);
    }
}
