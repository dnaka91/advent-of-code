use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Advent of Code ===");
    println!("Year 2019");

    println!("  Day 01");
    println!("    Part 1: {}", aoc::y2019::d01::solve_part_one(aoc::y2019::d01::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d01::solve_part_two(aoc::y2019::d01::INPUT)?);

    println!("  Day 02");
    println!("    Part 1: {}", aoc::y2019::d02::solve_part_one(aoc::y2019::d02::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d02::solve_part_two(aoc::y2019::d02::INPUT)?);

    println!("  Day 03");
    println!("    Part 1: {}", aoc::y2019::d03::solve_part_one(aoc::y2019::d03::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d03::solve_part_two(aoc::y2019::d03::INPUT)?);

    Ok(())
}
