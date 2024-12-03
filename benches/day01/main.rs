use rand::seq::SliceRandom;
use rand::thread_rng;

use criterion::{Criterion, criterion_group, criterion_main};

fn bench_task2(c: &mut Criterion) {
    let mut items1: Vec<i32> = (0..1000).collect();
    items1.shuffle(&mut thread_rng());
    let mut items2: Vec<i32> = (0..1000).collect();
    items2.shuffle(&mut thread_rng());

    c.bench_function("bench_task2", |b| {
        b.iter(|| {
            std::hint::black_box(aoc2024::day01::task2(&mut items1, &mut items2));
        });
    });
    c.bench_function("bench_task2_faster", |b| {
        b.iter(|| {
            std::hint::black_box(aoc2024::day01::task2_faster(&mut items1, &mut items2));
        });
    });
    c.bench_function("bench_task2_fastest", |b| {
        b.iter(|| {
            std::hint::black_box(aoc2024::day01::task2_fastest(&mut items1, &mut items2));
        });
    });
}

criterion_group!(benches, bench_task2);
criterion_main!(benches);
