use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn y2020(c: &mut Criterion) {
    use aoc::y2020::*;

    run(c, 1, d01::solve_part_one, d01::solve_part_two, d01::INPUT);
    run(c, 2, d02::solve_part_one, d02::solve_part_two, d02::INPUT);
    run(c, 3, d03::solve_part_one, d03::solve_part_two, d03::INPUT);
    run(c, 4, d04::solve_part_one, d04::solve_part_two, d04::INPUT);
    // run(c, 5, d05::solve_part_one, d05::solve_part_two, d05::INPUT);
    // run(c, 6, d06::solve_part_one, d06::solve_part_two, d06::INPUT);
    // run(c, 7, d07::solve_part_one, d07::solve_part_two, d07::INPUT);
    // run(c, 8, d08::solve_part_one, d08::solve_part_two, d08::INPUT);
    // run(c, 9, d09::solve_part_one, d09::solve_part_two, d09::INPUT);
    // run(c, 10, d10::solve_part_one, d10::solve_part_two, d10::INPUT);
    // run(c, 11, d11::solve_part_one, d11::solve_part_two, d11::INPUT);
    // run(c, 12, d12::solve_part_one, d12::solve_part_two, d12::INPUT);
    // run(c, 13, d13::solve_part_one, d13::solve_part_two, d13::INPUT);
    // run(c, 14, d14::solve_part_one, d14::solve_part_two, d14::INPUT);
    // run(c, 15, d15::solve_part_one, d15::solve_part_two, d15::INPUT);
    // run(c, 16, d16::solve_part_one, d16::solve_part_two, d16::INPUT);
    // run(c, 17, d17::solve_part_one, d17::solve_part_two, d17::INPUT);
    // run(c, 18, d18::solve_part_one, d18::solve_part_two, d18::INPUT);
    // run(c, 19, d19::solve_part_one, d19::solve_part_two, d19::INPUT);
    // run(c, 20, d20::solve_part_one, d20::solve_part_two, d20::INPUT);
    // run(c, 21, d21::solve_part_one, d21::solve_part_two, d21::INPUT);
    // run(c, 22, d22::solve_part_one, d22::solve_part_two, d22::INPUT);
    // run(c, 23, d23::solve_part_one, d23::solve_part_two, d23::INPUT);
    // run(c, 24, d24::solve_part_one, d24::solve_part_two, d24::INPUT);
    // run(c, 25, d25::solve_part_one, d25::solve_part_two, d25::INPUT);
}

fn run<'a, P1, T1, P2, T2>(c: &mut Criterion, day: u8, part1: P1, part2: P2, input: &'a str)
where
    P1: Fn(&'a str) -> T1 + 'a,
    P2: Fn(&'a str) -> T2 + 'a,
{
    c.bench_function(&format!("Y2020 D{:02} P1", day), |b| b.iter(|| part1(black_box(input))));
    c.bench_function(&format!("Y2020 D{:02} P2", day), |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, y2020);
criterion_main!(benches);
