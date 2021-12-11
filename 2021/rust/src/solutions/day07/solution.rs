use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut crabs = input.to_vec();
    crabs.sort();

    let median = crabs[crabs.len() / 2];

    input
        .iter()
        .map(|i| if i < &median { median - i } else { i - median })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut crabs = input.to_vec();
    crabs.sort();

    (0..crabs[crabs.len() - 1])
        .map(|i| {
            crabs
                .iter()
                .map(|j| {
                    if i < j.clone() {
                        (0..j - i + 1).sum::<usize>()
                    } else {
                        (0..i - j + 1).sum::<usize>()
                    }
                })
                .sum()
        })
        .min()
        .unwrap()
}
