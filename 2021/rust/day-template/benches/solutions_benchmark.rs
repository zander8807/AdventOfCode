use aoc_solution::{
    fetch_input,
    solution::{part_1, part_2, process_input},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = fetch_input();
    let input = &process_input(&input);

    c.bench_function("part_1", |b| b.iter(|| part_1(black_box(input))));
    c.bench_function("part_2", |b| b.iter(|| part_2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
