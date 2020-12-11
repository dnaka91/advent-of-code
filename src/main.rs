use std::fmt::Display;

use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Advent of Code ===");

    run_y2020()?;
    run_y2019()?;
    run_y2018()?;
    run_y2017()?;
    run_y2016()?;
    run_y2015()?;

    Ok(())
}

fn run_y2020() -> Result<()> {
    use aoc::y2020::*;

    println!("Year 2020");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    // run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    // run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    // run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    // run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    // run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    // run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    // run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    // run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    // run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    // run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    // run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    // run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    // run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    // run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    // run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

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
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

    Ok(())
}

fn run_y2018() -> Result<()> {
    use aoc::y2018::*;

    println!("Year 2018");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

    Ok(())
}

fn run_y2017() -> Result<()> {
    use aoc::y2017::*;

    println!("Year 2017");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

    Ok(())
}

fn run_y2016() -> Result<()> {
    use aoc::y2016::*;

    println!("Year 2016");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

    Ok(())
}

fn run_y2015() -> Result<()> {
    use aoc::y2015::*;

    println!("Year 2015");

    run(1, d01::solve_part_one, d01::solve_part_two, d01::INPUT)?;
    run(2, d02::solve_part_one, d02::solve_part_two, d02::INPUT)?;
    run(3, d03::solve_part_one, d03::solve_part_two, d03::INPUT)?;
    run(4, d04::solve_part_one, d04::solve_part_two, d04::INPUT)?;
    run(5, d05::solve_part_one, d05::solve_part_two, d05::INPUT)?;
    run(6, d06::solve_part_one, d06::solve_part_two, d06::INPUT)?;
    run(7, d07::solve_part_one, d07::solve_part_two, d07::INPUT)?;
    run(8, d08::solve_part_one, d08::solve_part_two, d08::INPUT)?;
    run(9, d09::solve_part_one, d09::solve_part_two, d09::INPUT)?;
    run(10, d10::solve_part_one, d10::solve_part_two, d10::INPUT)?;
    run(11, d11::solve_part_one, d11::solve_part_two, d11::INPUT)?;
    run(12, d12::solve_part_one, d12::solve_part_two, d12::INPUT)?;
    run(13, d13::solve_part_one, d13::solve_part_two, d13::INPUT)?;
    run(14, d14::solve_part_one, d14::solve_part_two, d14::INPUT)?;
    run(15, d15::solve_part_one, d15::solve_part_two, d15::INPUT)?;
    run(16, d16::solve_part_one, d16::solve_part_two, d16::INPUT)?;
    run(17, d17::solve_part_one, d17::solve_part_two, d17::INPUT)?;
    run(18, d18::solve_part_one, d18::solve_part_two, d18::INPUT)?;
    run(19, d19::solve_part_one, d19::solve_part_two, d19::INPUT)?;
    run(20, d20::solve_part_one, d20::solve_part_two, d20::INPUT)?;
    run(21, d21::solve_part_one, d21::solve_part_two, d21::INPUT)?;
    run(22, d22::solve_part_one, d22::solve_part_two, d22::INPUT)?;
    run(23, d23::solve_part_one, d23::solve_part_two, d23::INPUT)?;
    run(24, d24::solve_part_one, d24::solve_part_two, d24::INPUT)?;
    run(25, d25::solve_part_one, d25::solve_part_two, d25::INPUT)?;

    Ok(())
}

fn run<'a, P1, T1, P2, T2>(day: u8, part1: P1, part2: P2, input: &'a str) -> Result<()>
where
    P1: Fn(&'a str) -> Result<T1>,
    P2: Fn(&'a str) -> Result<T2>,
    T1: Display + 'a,
    T2: Display + 'a,
{
    println!("  Day {:02}", day);
    println!("    Part 1: {}", part1(input)?);
    println!("    Part 2: {}", part2(input)?);
    Ok(())
}
