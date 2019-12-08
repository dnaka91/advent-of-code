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

    println!("  Day 04");
    println!("    Part 1: {}", aoc::y2019::d04::solve_part_one(aoc::y2019::d04::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d04::solve_part_two(aoc::y2019::d04::INPUT)?);

    println!("  Day 05");
    println!("    Part 1: {}", aoc::y2019::d05::solve_part_one(aoc::y2019::d05::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d05::solve_part_two(aoc::y2019::d05::INPUT)?);

    println!("  Day 06");
    println!("    Part 1: {}", aoc::y2019::d06::solve_part_one(aoc::y2019::d06::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d06::solve_part_two(aoc::y2019::d06::INPUT)?);

    println!("  Day 08");
    println!("    Part 1: {}", aoc::y2019::d08::solve_part_one(aoc::y2019::d08::INPUT)?);
    println!("    Part 2: {}", aoc::y2019::d08::solve_part_two(aoc::y2019::d08::INPUT)?);

    Ok(())
}
