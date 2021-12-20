use std::collections::{HashMap, LinkedList};

#[derive(PartialEq, Debug)]
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
            s => {
                if s.chars().next().unwrap().is_uppercase() {
                    CaveType::Large
                } else {
                    CaveType::Small
                }
            }
        }
    }
}

#[derive(Debug)]
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
        let (original_from, original_to) = (split.next().unwrap(), split.next().unwrap());

        let should_switch = original_from == "end" || original_to == "start";
        let from = if !should_switch {
            original_from
        } else {
            original_to
        };
        let to = if !should_switch {
            original_to
        } else {
            original_from
        };

        Path { from, to }
    }
}

#[derive(Debug)]
pub struct CaveSystem<'a> {
    paths: HashMap<&'a str, Vec<&'a str>>,
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> CaveSystem<'a> {
    fn paths_from(&self, start: &'a str) -> Option<&Vec<&'a str>> {
        self.paths.get(start)
    }

    fn get_cave(&self, label: &'a str) -> &'a Cave {
        self.caves
            .get(label)
            .expect(format!("Couldn't find label {}", label).as_str())
    }
}

impl<'a> From<&'a str> for CaveSystem<'a> {
    fn from(s: &'a str) -> Self {
        let lines = s.lines();

        let mut paths: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        let mut caves: HashMap<&'a str, Cave<'a>> = HashMap::new();

        for line in lines {
            let path: Path = line.into();
            paths.entry(path.from).or_insert(Vec::new()).push(path.to);
            if path.from != "start" && path.from != "end" {
                paths.entry(path.to).or_insert(Vec::new()).push(path.from);
            }

            if !caves.contains_key(path.from) {
                caves.insert(path.from, Cave { name: path.from });
            }
            if !caves.contains_key(path.to) {
                caves.insert(path.to, Cave { name: path.to });
            }
        }

        CaveSystem { paths, caves }
    }
}

struct Visit<'a> {
    label: &'a str,
    path: Vec<&'a str>,
    visited: HashMap<&'a str, usize>,
}

struct Exploration<'a> {
    complete_paths: Vec<Vec<&'a str>>,
    to_visit: LinkedList<Visit<'a>>,
    cave_system: &'a CaveSystem<'a>,
    can_visit_small_cave_twice: bool,
}

impl<'a> Exploration<'a> {
    pub fn new(cave_system: &'a CaveSystem<'a>, can_visit_small_cave_twice: bool) -> Self {
        Exploration {
            complete_paths: Vec::new(),
            to_visit: LinkedList::new(),
            cave_system,
            can_visit_small_cave_twice: can_visit_small_cave_twice,
        }
    }
    pub fn explore(&mut self) {
        self.to_visit.push_back(Visit {
            label: "start",
            path: Vec::new(),
            visited: HashMap::new(),
        });

        while !self.to_visit.is_empty() {
            let visit = self.to_visit.pop_back().unwrap();
            self.visit(&visit);
        }
    }

    pub fn visit(&mut self, v: &Visit<'a>) {
        if !self.can_visit(v) {
            return;
        }

        let mut path = v.path.clone();
        path.push(&v.label);

        if self.cave_system.get_cave(v.label).cave_type() == CaveType::End {
            self.complete_paths.push(path);
            return;
        }

        let mut visited = v.visited.clone();

        *visited.entry(v.label).or_insert(0) += 1;

        let paths = self.cave_system.paths_from(v.label);
        if paths.is_none() {
            return;
        }
        let paths = paths.unwrap();

        for to_label in paths {
            self.to_visit.push_back(Visit {
                label: to_label,
                path: path.clone(),
                visited: visited.clone(),
            });
        }
    }

    fn can_visit(&self, visit: &Visit<'a>) -> bool {
        let visit_count = visit.visited.get(visit.label).unwrap_or(&0);

        match self.cave_system.get_cave(visit.label).cave_type() {
            CaveType::Start | CaveType::End | CaveType::Large => true,
            CaveType::Small => {
                if visit_count == &0 {
                    true
                } else if self.can_visit_small_cave_twice {
                    for (label, visits) in &visit.visited {
                        if self.cave_system.get_cave(label).cave_type() != CaveType::Small {
                            continue;
                        }

                        if visits > &1 {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn process_input<'a>(s: &'a str) -> CaveSystem {
    s.into()
}

pub fn part_1<'a>(input: &CaveSystem) -> usize {
    let mut exploration = Exploration::new(input.to_owned(), false);
    exploration.explore();
    exploration.complete_paths.len()
}

pub fn part_2<'a>(input: &CaveSystem) -> usize {
    let mut exploration = Exploration::new(input.to_owned(), true);
    exploration.explore();
    exploration.complete_paths.len()
}
