//! # Day 1: Report Repair
//!
//! After saving Christmas [five years in a row], you've decided to take a vacation at a nice resort
//! on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used there
//! have a little picture of a starfish; the locals just call them **stars**. None of the currency
//! exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by
//! the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all **fifty stars** by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
//! calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one
//! star**. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your **expense report** (your
//! puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to **find the two entries that sum to `2020`** and then multiply
//! those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```txt
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to `2020` are `1721` and `299`. Multiplying them together
//! produces `1721 * 299 = 514579`, so the correct answer is **`514579`**.
//!
//! Of course, your expense report is much larger. **Find the two entries that sum to `2020`; what
//! do you get if you multiply them together?**
//!
//! [five years in a row]: https://adventofcode.com/events
//!
//! ## Part Two
//!
//! The Elves in accounting are thankful for your help; one of them even offers you a starfish coin
//! they had left over from a past vacation. They offer you a second one if you can find **three**
//! numbers in your expense report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to `2020` are `979`, `366`, and `675`.
//! Multiplying them together produces the answer, **`241861950`**.
//!
//! In your expense report, **what is the product of the three entries that sum to `2020`?**

use std::iter;

use anyhow::{Context, Result};

pub const INPUT: &str = include_str!("d01.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let bills = parse_input(input)?;

    bills
        .iter()
        .flat_map(|a| iter::repeat(a).zip(&bills))
        .find(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b)
        .context("no combination sums up to 2020")
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let bills = parse_input(input)?;

    bills
        .iter()
        .flat_map(|a| iter::repeat(a).zip(&bills))
        .flat_map(|a| iter::repeat(a).zip(&bills))
        .find(|((a, b), c)| *a + *b + *c == 2020)
        .map(|((a, b), c)| a * b * c)
        .context("no combination sums up to 2020")
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.lines().map(|i| i.parse().map_err(Into::into)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn part_one() {
        assert_eq!(514579, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(241861950, solve_part_two(INPUT).unwrap());
    }
}
