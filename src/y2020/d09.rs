//! # Day 9: Encoding Error
//!
//! With your neighbor happily enjoying their video game, you turn your attention to an open data
//! port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through the clever
//! use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle
//! input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which,
//! conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a **preamble** of 25 numbers. After that, each number you receive
//! should be the sum of any two of the 25 immediately previous numbers. The two numbers will have
//! different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers `1` through `25` in a random order.
//! To be valid, the next number must be the sum of two of those numbers:
//!
//! - `26` would be a **valid** next number, as it could be `1` plus `25` (or many other pairs, like
//!   `2` and `24`).
//! - `49` would be a **valid** next number, as it is the sum of `24` and `25`.
//! - `100` would **not** be valid; no two of the previous 25 numbers sum to `100`.
//! - `50` would also **not** be valid; although `25` appears in the previous 25 numbers, the two
//!   numbers in the pair must be different.
//!
//! Suppose the 26th number is `45`, and the first number (no longer an option, as it is more than
//! 25 numbers ago) was `20`. Now, for the next number to be valid, there needs to be some pair of
//! numbers among `1`-`19`, `21`-`25`, or `45` that add up to it:
//!
//! - `26` would still be a **valid** next number, as `1` and `25` are still within the previous 25
//!   numbers.
//! - `65` would **not** be valid, as no two of the available numbers sum to it.
//! - `64` and `66` would both be **valid**, as they are the result of `19+45` and `21+45`
//!   respectively.
//!
//! Here is a larger example which only considers the previous **5** numbers (and has a preamble of
//! length 5):
//!
//! ```txt
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum of two of the
//! previous 5 numbers; the only number that does not follow this rule is **`127`**.
//!
//! The first step of attacking the weakness in the XMAS data is to find the first number in the
//! list (after the preamble) which is **not** the sum of two of the 25 numbers before it. **What is
//! the first number that does not have this property?**
//!
//! ## Part Two
//!
//! The final step in breaking the XMAS encryption relies on the invalid number you just found: you
//! must **find a contiguous set of at least two numbers** in your list which sum to the invalid
//! number from step 1.
//!
//! Again consider the above example:
//!
//! ```txt
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from `15` through `40` produces the invalid number
//! from step 1, `127`. (Of course, the contiguous set of numbers in your actual list might be much
//! longer.)
//!
//! To find the **encryption weakness**, add together the **smallest** and **largest** number in
//! this contiguous range; in this example, these are `15` and `47`, producing **`62`**.
//!
//! **What is the encryption weakness in your XMAS-encrypted list of numbers?**

use anyhow::{bail, Context, Result};
use itertools::{Itertools, MinMaxResult};
use rayon::prelude::*;

pub const INPUT: &str = include_str!("d09.txt");

const PREAMBLE: usize = if cfg!(test) { 5 } else { 25 };

pub fn solve_part_one(input: &str) -> Result<u64> {
    let numbers = parse_input(input)?;

    find_outlier(&numbers)
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let numbers = parse_input(input)?;
    let target = find_outlier(&numbers)?;

    (2..numbers.len() - 1)
        .into_par_iter()
        .find_map_any(|i| {
            numbers.par_windows(i).find_any(|w| w.iter().sum::<u64>() == target).and_then(|w| {
                match w.iter().minmax() {
                    MinMaxResult::MinMax(min, max) => Some(min + max),
                    _ => None,
                }
            })
        })
        .context("failed finding the combination")
}

fn parse_input(input: &str) -> Result<Vec<u64>> {
    input.lines().map(|l| l.parse().map_err(Into::into)).collect()
}

fn find_outlier(numbers: &[u64]) -> Result<u64> {
    numbers
        .par_windows(PREAMBLE + 1)
        .find_any(|w| !w.iter().take(PREAMBLE).combinations(2).any(|p| p[0] + p[1] == w[PREAMBLE]))
        .map(|w| w[PREAMBLE])
        .context("failed to find the number")
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "};

    #[test]
    fn part_one() {
        assert_eq!(127, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(62, solve_part_two(INPUT).unwrap());
    }
}
