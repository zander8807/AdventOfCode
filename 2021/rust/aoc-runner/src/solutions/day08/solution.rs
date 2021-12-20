use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct InputOutput {
    signals: Vec<String>,
    output: Vec<String>,
}

#[derive(Debug)]
pub struct Display {
    decoder: HashMap<String, char>,
}

impl Display {
    pub fn new(signals: &Vec<String>) -> Self {
        let mut signals: HashSet<String> = signals
            .iter()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort();
                chars.iter().collect::<String>()
            })
            .collect();

        let mut decoder = HashMap::new();

        let one = &signals
            .iter()
            .filter(|s| s.len() == 2)
            .next()
            .unwrap()
            .clone();
        signals.iter().position(|s| s == one).unwrap();
        signals.remove(one);
        decoder.insert(one.to_string(), '1');

        let four = &signals
            .iter()
            .filter(|s| s.len() == 4)
            .next()
            .unwrap()
            .clone();
        signals.remove(four);
        decoder.insert(four.to_string(), '4');

        let seven = &signals
            .iter()
            .filter(|s| s.len() == 3)
            .next()
            .unwrap()
            .clone();
        signals.remove(seven);
        decoder.insert(seven.to_string(), '7');

        let eight = &signals
            .iter()
            .filter(|s| s.len() == 7)
            .next()
            .unwrap()
            .clone();
        signals.remove(eight);
        decoder.insert(eight.to_string(), '8');

        let nine = &signals
            .iter()
            .filter(|s| four.chars().all(|c| s.contains(c)))
            .next()
            .unwrap()
            .clone();
        signals.remove(nine);
        decoder.insert(nine.to_string(), '9');

        let zero = &signals
            .iter()
            .filter(|s| one.chars().all(|c| s.contains(c) && s.len() == 6))
            .next()
            .unwrap()
            .clone();
        signals.remove(zero);
        decoder.insert(zero.to_string(), '0');

        let three = &signals
            .iter()
            .filter(|s| one.chars().all(|c| s.contains(c)))
            .next()
            .unwrap()
            .clone();
        signals.remove(three);
        decoder.insert(three.to_string(), '3');

        let six = &signals
            .iter()
            .filter(|s| s.len() == 6)
            .next()
            .unwrap()
            .clone();
        signals.remove(six);
        decoder.insert(six.to_string(), '6');

        let five = &signals
            .iter()
            .filter(|s| s.chars().all(|c| six.contains(c)))
            .next()
            .unwrap()
            .clone();
        signals.remove(five);
        decoder.insert(five.to_string(), '5');

        let two = &signals.iter().next().unwrap().clone();
        signals.remove(two);
        decoder.insert(two.to_string(), '2');

        Display { decoder }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<InputOutput> {
    input
        .lines()
        .map(|s| {
            let mut split = s.split(" | ");
            let signals: Vec<String> = split
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();

            let output: Vec<String> = split
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();

            InputOutput { signals, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<InputOutput>) -> usize {
    let counts: Vec<usize> = input
        .iter()
        .map(|i| {
            i.output
                .iter()
                .map(|c| c.len())
                .filter(|c| c == &2 || c == &4 || c == &3 || c == &7)
                .count()
        })
        .collect();

    counts.iter().sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<InputOutput>) -> usize {
    input
        .iter()
        .map(|i| {
            let display = Display::new(&i.signals);
            let value: String = i
                .output
                .iter()
                .map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort();
                    let s = chars.iter().collect::<String>();
                    display.decoder.get(&s).unwrap()
                })
                .collect();

            let value: usize = value.parse().unwrap();

            value
        })
        .sum()
}
