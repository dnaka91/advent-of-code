use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn y2019_d01(c: &mut Criterion) {
    c.bench_function("Y2019 D01 P1", |b| {
        b.iter(|| aoc::y2019::d01::solve_part_one(black_box(aoc::y2019::d01::INPUT)))
    });
    c.bench_function("Y2019 D01 P2", |b| {
        b.iter(|| aoc::y2019::d01::solve_part_two(black_box(aoc::y2019::d01::INPUT)))
    });
}

criterion_group!(benches, y2019_d01);
criterion_main!(benches);
