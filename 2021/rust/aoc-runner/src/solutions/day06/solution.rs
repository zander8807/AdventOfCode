use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut fishies = vec![0; 9];

    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .for_each(|i| fishies[i.parse::<usize>().unwrap()] += 1);

    fishies
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut fishies = input.to_vec();
    for _i in 0..80 {
        age(&mut fishies);
    }
    return fishies.iter().sum();
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut fishies = input.to_vec();
    for _i in 0..256 {
        age(&mut fishies);
    }
    return fishies.iter().sum();
}

fn age(fishies: &mut Vec<usize>) {
    let aged_fish = fishies[0];
    for i in 1..fishies.len() {
        fishies[i - 1] = fishies[i];
    }

    fishies[6] += aged_fish;
    fishies[8] = aged_fish;
}
