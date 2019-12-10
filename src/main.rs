use std::fmt::Display;

use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Advent of Code ===");

    run_y2019()?;

    Ok(())
}

fn run_y2019() -> Result<()> {
    use aoc::y2019::*;

    println!("Year 2019");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;

    Ok(())
}

fn run<P1, T1, P2, T2>(day: u8, part1: P1, part2: P2, input: &str) -> Result<()>
where
    P1: Fn(&str) -> Result<T1>,
    P2: Fn(&str) -> Result<T2>,
    T1: Display,
    T2: Display,
{
    println!("  Day {:02}", day);
    println!("    Part 1: {}", part1(input)?);
    println!("    Part 2: {}", part2(input)?);
    Ok(())
}
