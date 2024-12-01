//! # Day 1: Calorie Counting
//!
//! Santa's reindeer typically eat regular reindeer food, but they need a lot of [magical energy] to
//! deliver presents on Christmas. For that, their favorite snack is a special type of **star**
//! fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition
//! to the grove where the fruit grows.
//!
//! To supply enough magical energy, the expedition needs to retrieve a minimum of **fifty stars**
//! by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide
//! to grab any fruit you see along the way, just in case.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
//! calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one
//! star**. Good luck!
//!
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
//! the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin
//! taking inventory of their supplies. One important consideration is food - in particular, the
//! number of **Calories** each Elf is carrying (your puzzle input).
//!
//! The Elves take turns writing down the number of Calories contained by the various meals, snacks,
//! rations, etc. that they've brought with them, one item per line. Each Elf separates their own
//! inventory from the previous Elf's inventory (if any) by a blank line.
//!
//! For example, suppose the Elves finish writing their items' Calories and end up with the
//! following list:
//!
//! ```txt
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//! ```
//!
//! This list represents the Calories of the food carried by five Elves:
//!
//! - The first Elf is carrying food with `1000`, `2000`, and `3000` Calories, a total of **`6000`**
//!   Calories.
//! - The second Elf is carrying one food item with **`4000`** Calories.
//! - The third Elf is carrying food with `5000` and `6000` Calories, a total of **`11000`**
//!   Calories.
//! - The fourth Elf is carrying food with `7000`, `8000`, and `9000` Calories, a total of
//!   **`24000`** Calories.
//! - The fifth Elf is carrying one food item with **`10000`** Calories.
//!
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd
//! like to know how many Calories are being carried by the Elf carrying the **most** Calories. In
//! the example above, this is **`24000`** (carried by the fourth Elf).
//!
//! Find the Elf carrying the most Calories. **How many total Calories is that Elf carrying?**
//!
//! [magical energy]: crate::y2018::d25
//!
//! ## Part Two
//!
//! By the time you calculate the answer to the Elves' question, they've already realized that the
//! Elf carrying the most Calories of food might eventually **run out of snacks**.
//!
//! To avoid this unacceptable situation, the Elves would instead like to know the total Calories
//! carried by the **top three** Elves carrying the most Calories. That way, even if one of those
//! Elves runs out of snacks, they still have two backups.
//!
//! In the example above, the top three Elves are the fourth Elf (with `24000` Calories), then the
//! third Elf (with `11000` Calories), then the fifth Elf (with `10000` Calories). The sum of the
//! Calories carried by these three elves is **`45000`**.
//!
//! Find the top three Elves carrying the most Calories. **How many Calories are those Elves
//! carrying in total?**

use anyhow::Result;
use itertools::Itertools;

pub const INPUT: &str = include_str!("d01.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    Ok(parse_input(input)?.into_iter().map(|elf| elf.into_iter().sum()).max().unwrap_or(0))
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    Ok(parse_input(input)?
        .into_iter()
        .map(|elf| elf.into_iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum())
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split_terminator('\n').map(|value| value.parse().map_err(Into::into)).collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn part_one() {
        assert_eq!(24000, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(45000, solve_part_two(INPUT).unwrap());
    }
}
