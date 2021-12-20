use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let input = &mut input.clone();
    (0..100).map(|_| step(input)).sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    let input = &mut input.clone();
    (1..10000)
        .find(|_| {
            step(input);
            let first = input[0][0];
            input.iter().flatten().all(|&i| i == first)
        })
        .unwrap()
}

fn step(input: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes = Vec::new();
    let mut flash_count: usize = 0;

    let row_count = input.len();
    let col_count = input[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if increase_energy_level(input, row, col) {
                flashes.push((row, col));
                flash_count += 1;
            }
        }
    }

    while flashes.len() > 0 {
        let (row, col) = flashes.pop().unwrap();

        let up = row - 1;
        let down = row + 1;
        let left = col - 1;
        let right = col + 1;

        let is_upper_edge = row == 0;
        let is_lower_edge = row == row_count - 1;
        let is_left_edge = col == 0;
        let is_right_edge = col == col_count - 1;

        if !is_upper_edge {
            if increase_energy_level(input, up, col) {
                flashes.push((up, col));
                flash_count += 1;
            }

            if !is_left_edge {
                if increase_energy_level(input, up, left) {
                    flashes.push((up, left));
                    flash_count += 1;
                }
            }

            if !is_right_edge {
                if increase_energy_level(input, up, right) {
                    flashes.push((up, right));
                    flash_count += 1;
                }
            }
        }

        if !is_lower_edge {
            if increase_energy_level(input, down, col) {
                flashes.push((down, col));
                flash_count += 1;
            }

            if !is_left_edge {
                if increase_energy_level(input, down, left) {
                    flashes.push((down, left));
                    flash_count += 1;
                }
            }

            if !is_right_edge {
                if increase_energy_level(input, down, right) {
                    flashes.push((down, right));
                    flash_count += 1;
                }
            }
        }

        if !is_left_edge {
            if increase_energy_level(input, row, left) {
                flashes.push((row, left));
                flash_count += 1;
            }
        }

        if !is_right_edge {
            if increase_energy_level(input, row, right) {
                flashes.push((row, right));
                flash_count += 1;
            }
        }
    }

    for row in 0..row_count {
        for col in 0..col_count {
            if input[row][col] > 9 {
                input[row][col] = 0;
            }
        }
    }

    flash_count
}

fn increase_energy_level(input: &mut Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    if input[row][col] > 9 {
        return false;
    }

    input[row][col] += 1;
    input[row][col] > 9
}
