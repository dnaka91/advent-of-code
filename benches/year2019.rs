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

fn y2019_d04(c: &mut Criterion) {
    c.bench_function("Y2019 D04 P1", |b| {
        b.iter(|| aoc::y2019::d04::solve_part_one(black_box(aoc::y2019::d04::INPUT)))
    });
    c.bench_function("Y2019 D04 P2", |b| {
        b.iter(|| aoc::y2019::d04::solve_part_two(black_box(aoc::y2019::d04::INPUT)))
    });
}

fn y2019_d05(c: &mut Criterion) {
    c.bench_function("Y2019 D05 P1", |b| {
        b.iter(|| aoc::y2019::d05::solve_part_one(black_box(aoc::y2019::d05::INPUT)))
    });
    c.bench_function("Y2019 D05 P2", |b| {
        b.iter(|| aoc::y2019::d05::solve_part_two(black_box(aoc::y2019::d05::INPUT)))
    });
}

fn y2019_d06(c: &mut Criterion) {
    c.bench_function("Y2019 D06 P1", |b| {
        b.iter(|| aoc::y2019::d06::solve_part_one(black_box(aoc::y2019::d06::INPUT)))
    });
    c.bench_function("Y2019 D06 P2", |b| {
        b.iter(|| aoc::y2019::d06::solve_part_two(black_box(aoc::y2019::d06::INPUT)))
    });
}

fn y2019_d08(c: &mut Criterion) {
    c.bench_function("Y2019 D08 P1", |b| {
        b.iter(|| aoc::y2019::d08::solve_part_one(black_box(aoc::y2019::d08::INPUT)))
    });
    c.bench_function("Y2019 D08 P2", |b| {
        b.iter(|| aoc::y2019::d08::solve_part_two(black_box(aoc::y2019::d08::INPUT)))
    });
}

criterion_group!(
    benches, y2019_d01, y2019_d02, y2019_d03, y2019_d04, y2019_d05, y2019_d06, y2019_d08
);
criterion_main!(benches);
