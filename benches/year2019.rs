use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn y2019_d01(c: &mut Criterion) {
    c.bench_function("Y2019 D01 P1", |b| {
        b.iter(|| aoc::y2019::d01::solve_part_one(black_box(aoc::y2019::d01::INPUT)))
    });
    c.bench_function("Y2019 D01 P2", |b| {
        b.iter(|| aoc::y2019::d01::solve_part_two(black_box(aoc::y2019::d01::INPUT)))
    });
}

fn y2019_d02(c: &mut Criterion) {
    c.bench_function("Y2019 D02 P1", |b| {
        b.iter(|| aoc::y2019::d02::solve_part_one(black_box(aoc::y2019::d02::INPUT)))
    });
    c.bench_function("Y2019 D02 P2", |b| {
        b.iter(|| aoc::y2019::d02::solve_part_two(black_box(aoc::y2019::d02::INPUT)))
    });
}

fn y2019_d03(c: &mut Criterion) {
    c.bench_function("Y2019 D03 P1", |b| {
        b.iter(|| aoc::y2019::d03::solve_part_one(black_box(aoc::y2019::d03::INPUT)))
    });
    c.bench_function("Y2019 D03 P2", |b| {
        b.iter(|| aoc::y2019::d03::solve_part_two(black_box(aoc::y2019::d03::INPUT)))
    });
}

criterion_group!(benches, y2019_d01, y2019_d02, y2019_d03);
criterion_main!(benches);
