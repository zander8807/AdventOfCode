use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    let mut scores = Vec::new();

    for line in input {
        let mut stack: Vec<String> = Vec::new();
        let chars = line.chars().map(|c| c.to_string());

        for char in chars {
            let char = char.as_str();
            if let Some(s) = get_pair(char) {
                stack.push(s.to_string());
                continue;
            }

            if stack.last().unwrap_or(&"not_a_value".to_string()) != &char {
                scores.push(get_corrupt_score(char));
                break;
            }

            stack.pop();
        }
    }

    scores.iter().sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    let mut scores = Vec::new();

    for line in input {
        let chars = line.chars().map(|c| c.to_string());

        let mut stack: Vec<String> = Vec::new();
        for char in chars {
            let char = char.as_str();
            if let Some(s) = get_pair(char) {
                stack.push(s.to_string());
                continue;
            }

            if stack.last().unwrap_or(&"not_a_value".to_string()) != &char {
                stack.clear();
                break;
            }

            stack.pop();
        }

        if stack.len() == 0 {
            continue;
        }

        let score = stack.iter().rev().fold(0usize, |mut sum, s| {
            sum = sum * 5;
            sum += get_incomplete_score(s.as_str());
            sum
        });

        scores.push(score);
    }

    scores.sort();

    scores[scores.len() / 2]
}

fn get_pair(s: &str) -> Option<&str> {
    match s {
        "(" => Some(")"),
        "[" => Some("]"),
        "{" => Some("}"),
        "<" => Some(">"),
        _ => None,
    }
}

fn get_corrupt_score(s: &str) -> usize {
    match s {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("oops!"),
    }
}

fn get_incomplete_score(s: &str) -> usize {
    match s {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        _ => panic!("oops!"),
    }
}
