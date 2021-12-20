use aoc_solution::{
    fetch_input,
    solution::{part_1, part_2, process_input},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "advent-of-code",
    about = "Some options for running advent-of-code"
)]
struct AdventOfCodeOpt {
    #[structopt(short, long)]
    parts: Option<Vec<String>>,
}
fn main() {
    let opt = AdventOfCodeOpt::from_args();

    let input = fetch_input();

    let input = &process_input(&input);

    let parts: Option<&Vec<String>> = opt.parts.as_ref();

    if parts.is_none() || parts.unwrap().contains(&"1".to_string()) {
        println!("part 1: {}", part_1(input));
    }

    if parts.is_none() || parts.unwrap().contains(&"2".to_string()) {
        println!("part 2: {}", part_2(input));
    }
}
