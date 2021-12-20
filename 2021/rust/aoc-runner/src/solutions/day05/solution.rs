use std::{
    cmp::{max, min},
    ops::Sub,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

trait IsDiagonal {
    fn is_diagonal(&self) -> bool;
}

#[derive(Debug)]
pub struct Segment {
    from: Point,
    to: Point,
}

impl IsDiagonal for Segment {
    fn is_diagonal(&self) -> bool {
        !(self.from.x == self.to.x || self.from.y == self.to.y)
    }
}

trait DrawSegment {
    fn draw(&mut self, segment: &Segment);
}

trait IntersectionCount {
    fn intersection_count(&self) -> usize;
}
struct Map {
    map: Vec<Vec<usize>>,
}

impl Map {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            map: vec![vec![0; rows + 1]; cols + 1],
        }
    }
}

impl DrawSegment for Map {
    fn draw(&mut self, s: &Segment) {
        let x_diff = abs_difference(s.from.x, s.to.x);
        let y_diff = abs_difference(s.from.y, s.to.y);
        let x_min = min(s.from.x, s.to.x);
        let x_max = max(s.from.x, s.to.x);
        let y_min = min(s.from.y, s.to.y);
        let y_max = max(s.from.y, s.to.y);

        let mut x_range: Vec<usize> = if x_diff == 0 {
            vec![x_min; y_diff + 1]
        } else {
            (x_min..x_max + 1).collect()
        };

        if s.from.x > s.to.x {
            x_range.reverse();
        }

        let mut y_range = if y_diff == 0 {
            vec![y_min; x_diff + 1]
        } else {
            (y_min..y_max + 1).collect()
        };

        if s.from.y > s.to.y {
            y_range.reverse()
        }

        x_range
            .iter()
            .zip(y_range.iter())
            .for_each(|t| self.map[*t.1][*t.0] += 1);
    }
}

impl IntersectionCount for Map {
    fn intersection_count(&self) -> usize {
        self.map.iter().flatten().filter(|c| c > &&1).count()
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Segment> {
    input
        .lines()
        .map(|l| {
            let mut segment_split = l.trim().split(" -> ");
            let mut from_split = segment_split.next().unwrap().split(",");
            let mut to_split = segment_split.next().unwrap().split(",");

            Segment {
                from: Point {
                    x: from_split.next().unwrap().parse().unwrap(),
                    y: from_split.next().unwrap().parse().unwrap(),
                },
                to: Point {
                    x: to_split.next().unwrap().parse().unwrap(),
                    y: to_split.next().unwrap().parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Segment]) -> usize {
    let max_x: usize = input
        .iter()
        .flat_map(|i| [i.from.x, i.to.x])
        .max()
        .unwrap()
        .into();

    let max_y: usize = input
        .iter()
        .flat_map(|i| [i.from.y, i.to.y])
        .max()
        .unwrap()
        .into();

    let mut map = Map::new(max_y, max_x);

    for segment in input {
        if !segment.is_diagonal() {
            map.draw(segment);
        }
    }

    return map.intersection_count();
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Segment]) -> usize {
    let max_x: usize = input
        .iter()
        .flat_map(|i| [i.from.x, i.to.x])
        .max()
        .unwrap()
        .into();

    let max_y: usize = input
        .iter()
        .flat_map(|i| [i.from.y, i.to.y])
        .max()
        .unwrap()
        .into();

    let mut map = Map::new(max_y, max_x);

    for segment in input {
        map.draw(segment);
    }

    return map.intersection_count();
}

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}
