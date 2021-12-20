use std::{cell::Ref, collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq)]
pub enum CaveType {
    Start,
    End,
    Small,
    Large,
}

impl<'a> From<&'a str> for CaveType {
    fn from(s: &'a str) -> Self {
        match s {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            s => CaveType::End,
        }
    }
}

struct Cave<'a> {
    name: &'a str,
}

impl<'a> Cave<'a> {
    fn cave_type(&self) -> CaveType {
        self.name.into()
    }
}

struct Path<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(s: &'a str) -> Self {
        let mut split = s.split('-');
        Path {
            from: split.next().unwrap(),
            to: split.next().unwrap(),
        }
    }
}

pub struct CaveSystem<'a> {
    paths: HashMap<&'a str, Vec<&'a str>>,
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> From<&'a str> for CaveSystem<'a> {
    fn from(s: &'a str) -> Self {
        CaveSystem {
            paths: todo!(),
            caves: todo!(),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Option<CaveSystem> {
    Some(CaveSystem {
        paths: todo!(),
        caves: todo!(),
    })
}

#[aoc(day12, part1)]
pub fn solve_part1<'a>(input: Box<CaveSystem<'a>>) -> usize {
    0
}
