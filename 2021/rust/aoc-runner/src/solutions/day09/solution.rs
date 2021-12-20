use std::collections::{BinaryHeap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
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

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let mut low_points = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let curr_value = input[row][col];

            if row > 0 && curr_value >= input[row - 1][col] {
                continue;
            }

            if row < input.len() - 1 && curr_value >= input[row + 1][col] {
                continue;
            }

            if col > 0 && curr_value >= input[row][col - 1] {
                continue;
            }

            if col < input[row].len() - 1 && curr_value >= input[row][col + 1] {
                continue;
            }

            low_points.push(curr_value + 1);
        }
    }

    low_points.iter().sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    let mut input = input.clone();
    let mut curr_label: usize = 10;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let curr_value = input[row][col];

            if curr_value >= 9 {
                continue;
            }

            mark_basin(row, col, &mut input, curr_label);
            curr_label += 1;
        }
    }
    let mut label_counts: HashMap<usize, usize> = HashMap::new();
    input
        .iter()
        .flatten()
        .filter(|label| label > &&9)
        .for_each(|label| *label_counts.entry(*label).or_insert(0) += 1);

    let mut heap = BinaryHeap::new();

    label_counts.values().for_each(|c| heap.push(c));

    let mut res: usize = 1;
    res *= heap.pop().unwrap();
    res *= heap.pop().unwrap();
    res *= heap.pop().unwrap();

    res
}

fn mark_basin(row: usize, col: usize, input: &mut Vec<Vec<usize>>, curr_label: usize) {
    let mut stack = Vec::new();

    stack.push((row, col));

    while stack.len() > 0 {
        let (row, col) = stack.pop().unwrap();

        input[row][col] = curr_label;

        let up = row - 1;
        let down = row + 1;
        let left = col - 1;
        let right = col + 1;

        if row > 0 && input[up][col] < 9 {
            stack.push((up, col));
        }

        if row < input.len() - 1 && input[down][col] < 9 {
            stack.push((down, col));
        }

        if col > 0 && input[row][left] < 9 {
            stack.push((row, left));
        }

        if col < input[row].len() - 1 && input[row][right] < 9 {
            stack.push((row, right));
        }
    }
}
