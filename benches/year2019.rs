use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn y2019(c: &mut Criterion) {
    use aoc::y2019::*;

    run(c, 1, d01::solve_part_one, d01::solve_part_two, d01::INPUT);
    run(c, 2, d02::solve_part_one, d02::solve_part_two, d02::INPUT);
    run(c, 3, d03::solve_part_one, d03::solve_part_two, d03::INPUT);
    run(c, 4, d04::solve_part_one, d04::solve_part_two, d04::INPUT);
    run(c, 5, d05::solve_part_one, d05::solve_part_two, d05::INPUT);
    run(c, 6, d06::solve_part_one, d06::solve_part_two, d06::INPUT);
    run(c, 7, d07::solve_part_one, d07::solve_part_two, d07::INPUT);
    run(c, 8, d08::solve_part_one, d08::solve_part_two, d08::INPUT);
    run(c, 9, d09::solve_part_one, d09::solve_part_two, d09::INPUT);
}

fn run<P1, T1, P2, T2>(c: &mut Criterion, day: u8, part1: P1, part2: P2, input: &str)
where
    P1: Fn(&str) -> T1,
    P2: Fn(&str) -> T2,
{
    c.bench_function(&format!("Y2019 D{:02} P1", day), |b| b.iter(|| part1(black_box(input))));
    c.bench_function(&format!("Y2019 D{:02} P2", day), |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, y2019);
criterion_main!(benches);
