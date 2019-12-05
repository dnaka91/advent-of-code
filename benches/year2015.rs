use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn y2015_d01(c: &mut Criterion) {
    c.bench_function("Y2015 D01 P1", |b| {
        b.iter(|| aoc::y2015::d01::solve_part_one(black_box(aoc::y2015::d01::INPUT)))
    });
    c.bench_function("Y2015 D01 P2", |b| {
        b.iter(|| aoc::y2015::d01::solve_part_two(black_box(aoc::y2015::d01::INPUT)))
    });
}

fn y2015_d02(c: &mut Criterion) {
    c.bench_function("Y2015 D02 P1", |b| {
        b.iter(|| aoc::y2015::d02::solve_part_one(black_box(aoc::y2015::d02::INPUT)))
    });
    c.bench_function("Y2015 D02 P2", |b| {
        b.iter(|| aoc::y2015::d02::solve_part_two(black_box(aoc::y2015::d02::INPUT)))
    });
}

fn y2015_d03(c: &mut Criterion) {
    c.bench_function("Y2015 D03 P1", |b| {
        b.iter(|| aoc::y2015::d03::solve_part_one(black_box(aoc::y2015::d03::INPUT)))
    });
    c.bench_function("Y2015 D03 P2", |b| {
        b.iter(|| aoc::y2015::d03::solve_part_two(black_box(aoc::y2015::d03::INPUT)))
    });
}

fn y2015_d04(c: &mut Criterion) {
    c.bench_function("Y2015 D04 P1", |b| {
        b.iter(|| aoc::y2015::d04::solve_part_one(black_box(aoc::y2015::d04::INPUT)))
    });
    c.bench_function("Y2015 D04 P2", |b| {
        b.iter(|| aoc::y2015::d04::solve_part_two(black_box(aoc::y2015::d04::INPUT)))
    });
}

fn y2015_d05(c: &mut Criterion) {
    c.bench_function("Y2015 D05 P1", |b| {
        b.iter(|| aoc::y2015::d05::solve_part_one(black_box(aoc::y2015::d05::INPUT)))
    });
    c.bench_function("Y2015 D05 P2", |b| {
        b.iter(|| aoc::y2015::d05::solve_part_two(black_box(aoc::y2015::d05::INPUT)))
    });
}

fn y2015_d06(c: &mut Criterion) {
    c.bench_function("Y2015 D06 P1", |b| {
        b.iter(|| aoc::y2015::d06::solve_part_one(black_box(aoc::y2015::d06::INPUT)))
    });
    c.bench_function("Y2015 D06 P2", |b| {
        b.iter(|| aoc::y2015::d06::solve_part_two(black_box(aoc::y2015::d06::INPUT)))
    });
}

fn y2015_d07(c: &mut Criterion) {
    c.bench_function("Y2015 D07 P1", |b| {
        b.iter(|| aoc::y2015::d07::solve_part_one(black_box(aoc::y2015::d07::INPUT)))
    });
    c.bench_function("Y2015 D07 P2", |b| {
        b.iter(|| aoc::y2015::d07::solve_part_two(black_box(aoc::y2015::d07::INPUT)))
    });
}

fn y2015_d08(c: &mut Criterion) {
    c.bench_function("Y2015 D08 P1", |b| {
        b.iter(|| aoc::y2015::d08::solve_part_one(black_box(aoc::y2015::d08::INPUT)))
    });
    c.bench_function("Y2015 D08 P2", |b| {
        b.iter(|| aoc::y2015::d08::solve_part_two(black_box(aoc::y2015::d08::INPUT)))
    });
}

fn y2015_d09(c: &mut Criterion) {
    c.bench_function("Y2015 D09 P1", |b| {
        b.iter(|| aoc::y2015::d09::solve_part_one(black_box(aoc::y2015::d09::INPUT)))
    });
    c.bench_function("Y2015 D09 P2", |b| {
        b.iter(|| aoc::y2015::d09::solve_part_two(black_box(aoc::y2015::d09::INPUT)))
    });
}

fn y2015_d10(c: &mut Criterion) {
    c.bench_function("Y2015 D10 P1", |b| {
        b.iter(|| aoc::y2015::d10::solve_part_one(black_box(aoc::y2015::d10::INPUT)))
    });
    c.bench_function("Y2015 D10 P2", |b| {
        b.iter(|| aoc::y2015::d10::solve_part_two(black_box(aoc::y2015::d10::INPUT)))
    });
}

fn y2015_d11(c: &mut Criterion) {
    c.bench_function("Y2015 D11 P1", |b| {
        b.iter(|| aoc::y2015::d11::solve_part_one(black_box(aoc::y2015::d11::INPUT)))
    });
    c.bench_function("Y2015 D11 P2", |b| {
        b.iter(|| aoc::y2015::d11::solve_part_two(black_box(aoc::y2015::d11::INPUT)))
    });
}

fn y2015_d12(c: &mut Criterion) {
    c.bench_function("Y2015 D12 P1", |b| {
        b.iter(|| aoc::y2015::d12::solve_part_one(black_box(aoc::y2015::d12::INPUT)))
    });
    c.bench_function("Y2015 D12 P2", |b| {
        b.iter(|| aoc::y2015::d12::solve_part_two(black_box(aoc::y2015::d12::INPUT)))
    });
}

fn y2015_d13(c: &mut Criterion) {
    c.bench_function("Y2015 D13 P1", |b| {
        b.iter(|| aoc::y2015::d13::solve_part_one(black_box(aoc::y2015::d13::INPUT)))
    });
    c.bench_function("Y2015 D13 P2", |b| {
        b.iter(|| aoc::y2015::d13::solve_part_two(black_box(aoc::y2015::d13::INPUT)))
    });
}

fn y2015_d14(c: &mut Criterion) {
    c.bench_function("Y2015 D14 P1", |b| {
        b.iter(|| aoc::y2015::d14::solve_part_one(black_box(aoc::y2015::d14::INPUT)))
    });
    c.bench_function("Y2015 D14 P2", |b| {
        b.iter(|| aoc::y2015::d14::solve_part_two(black_box(aoc::y2015::d14::INPUT)))
    });
}

fn y2015_d15(c: &mut Criterion) {
    c.bench_function("Y2015 D15 P1", |b| {
        b.iter(|| aoc::y2015::d15::solve_part_one(black_box(aoc::y2015::d15::INPUT)))
    });
    c.bench_function("Y2015 D15 P2", |b| {
        b.iter(|| aoc::y2015::d15::solve_part_two(black_box(aoc::y2015::d15::INPUT)))
    });
}

fn y2015_d16(c: &mut Criterion) {
    c.bench_function("Y2015 D16 P1", |b| {
        b.iter(|| aoc::y2015::d16::solve_part_one(black_box(aoc::y2015::d16::INPUT)))
    });
    c.bench_function("Y2015 D16 P2", |b| {
        b.iter(|| aoc::y2015::d16::solve_part_two(black_box(aoc::y2015::d16::INPUT)))
    });
}

fn y2015_d17(c: &mut Criterion) {
    c.bench_function("Y2015 D17 P1", |b| {
        b.iter(|| aoc::y2015::d17::solve_part_one(black_box(aoc::y2015::d17::INPUT)))
    });
    c.bench_function("Y2015 D17 P2", |b| {
        b.iter(|| aoc::y2015::d17::solve_part_two(black_box(aoc::y2015::d17::INPUT)))
    });
}

fn y2015_d18(c: &mut Criterion) {
    c.bench_function("Y2015 D18 P1", |b| {
        b.iter(|| aoc::y2015::d18::solve_part_one(black_box(aoc::y2015::d18::INPUT)))
    });
    c.bench_function("Y2015 D18 P2", |b| {
        b.iter(|| aoc::y2015::d18::solve_part_two(black_box(aoc::y2015::d18::INPUT)))
    });
}

fn y2015_d19(c: &mut Criterion) {
    c.bench_function("Y2015 D19 P1", |b| {
        b.iter(|| aoc::y2015::d19::solve_part_one(black_box(aoc::y2015::d19::INPUT)))
    });
    c.bench_function("Y2015 D19 P2", |b| {
        b.iter(|| aoc::y2015::d19::solve_part_two(black_box(aoc::y2015::d19::INPUT)))
    });
}

fn y2015_d20(c: &mut Criterion) {
    c.bench_function("Y2015 D20 P1", |b| {
        b.iter(|| aoc::y2015::d20::solve_part_one(black_box(aoc::y2015::d20::INPUT)))
    });
    c.bench_function("Y2015 D20 P2", |b| {
        b.iter(|| aoc::y2015::d20::solve_part_two(black_box(aoc::y2015::d20::INPUT)))
    });
}

fn y2015_d21(c: &mut Criterion) {
    c.bench_function("Y2015 D21 P1", |b| {
        b.iter(|| aoc::y2015::d21::solve_part_one(black_box(aoc::y2015::d21::INPUT)))
    });
    c.bench_function("Y2015 D21 P2", |b| {
        b.iter(|| aoc::y2015::d21::solve_part_two(black_box(aoc::y2015::d21::INPUT)))
    });
}

fn y2015_d22(c: &mut Criterion) {
    c.bench_function("Y2015 D22 P1", |b| {
        b.iter(|| aoc::y2015::d22::solve_part_one(black_box(aoc::y2015::d22::INPUT)))
    });
    c.bench_function("Y2015 D22 P2", |b| {
        b.iter(|| aoc::y2015::d22::solve_part_two(black_box(aoc::y2015::d22::INPUT)))
    });
}

fn y2015_d23(c: &mut Criterion) {
    c.bench_function("Y2015 D23 P1", |b| {
        b.iter(|| aoc::y2015::d23::solve_part_one(black_box(aoc::y2015::d23::INPUT)))
    });
    c.bench_function("Y2015 D23 P2", |b| {
        b.iter(|| aoc::y2015::d23::solve_part_two(black_box(aoc::y2015::d23::INPUT)))
    });
}

fn y2015_d24(c: &mut Criterion) {
    c.bench_function("Y2015 D24 P1", |b| {
        b.iter(|| aoc::y2015::d24::solve_part_one(black_box(aoc::y2015::d24::INPUT)))
    });
    c.bench_function("Y2015 D24 P2", |b| {
        b.iter(|| aoc::y2015::d24::solve_part_two(black_box(aoc::y2015::d24::INPUT)))
    });
}

fn y2015_d25(c: &mut Criterion) {
    c.bench_function("Y2015 D25 P1", |b| {
        b.iter(|| aoc::y2015::d25::solve_part_one(black_box(aoc::y2015::d25::INPUT)))
    });
    c.bench_function("Y2015 D25 P2", |b| {
        b.iter(|| aoc::y2015::d25::solve_part_two(black_box(aoc::y2015::d25::INPUT)))
    });
}

criterion_group!(
    benches, y2015_d01, y2015_d02, y2015_d03, y2015_d04, y2015_d05, y2015_d06, y2015_d07,
    y2015_d08, y2015_d09, y2015_d10, y2015_d11, y2015_d12, y2015_d13, y2015_d14, y2015_d15,
    y2015_d16, y2015_d17, y2015_d18, y2015_d19, y2015_d20, y2015_d21, y2015_d22, y2015_d23,
    y2015_d24, y2015_d25,
);
criterion_main!(benches);
