//! # Day 1: Trebuchet?!
//!
//! Something is wrong with global snow production, and you've been selected to take a look. The
//! Elves have even given you a map; on it, they've used stars to mark the top fifty locations that
//! are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations, you need to check
//! all **fifty stars** by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
//! calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one
//! star**. Good luck!
//!
//! You try to ask why they can't just use a [weather machine] ("not powerful enough") and where
//! they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of
//! questions") and hang on did you just say the sky ("of course, where do you think snow comes
//! from") when you realize that the Elves are already loading you into a [trebuchet] ("please hold
//! still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their calibration document (your
//! puzzle input) has been **amended** by a very young Elf who was apparently just excited to show
//! off her art skills. Consequently, the Elves are having trouble reading the values on the
//! document.
//!
//! The newly-improved calibration document consists of lines of text; each line originally
//! contained a specific **calibration value** that the Elves now need to recover. On each line, the
//! calibration value can be found by combining the **first digit** and the **last digit** (in that
//! order) to form a single **two-digit number**.
//!
//! For example:
//!
//! ```txt
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//! ```
//!
//! In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`.
//! Adding these together produces **`142`**.
//!
//! Consider your entire calibration document. **What is the sum of all of the calibration values?**
//!
//! [weather machine]: crate::y2015::d01
//! [trebuchet]: https://en.wikipedia.org/wiki/Trebuchet
//!
//! ## Part Two
//!
//! Your calculation isn't quite right. It looks like some of the digits are actually **spelled out
//! with letters**: `one`, `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine`
//! **also** count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and last digit on each
//! line. For example:
//!
//! ```txt
//! two1nine
//! eightwothree
//! abcone2threexyz
//! xtwone3four
//! 4nineeightseven2
//! zoneight234
//! 7pqrstsixteen
//! ```
//!
//! In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding
//! these together produces **`281`**.
//!
//! **What is the sum of all of the calibration values?**

use anyhow::{Context, Result};

pub const INPUT: &str = include_str!("d01.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    Ok(parse_input(input, |_, _, b| b.is_ascii_digit().then_some(b - b'0'))?.into_iter().sum())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    static PATTERNS: &[(&str, u8)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    Ok(parse_input(input, |line, pos, b| {
        if b.is_ascii_digit() {
            Some(b - b'0')
        } else {
            PATTERNS
                .iter()
                .copied()
                .find_map(|(pat, value)| line[pos..].starts_with(pat).then_some(value))
        }
    })?
    .into_iter()
    .sum())
}

pub fn parse_input(
    input: &str,
    parse_byte: impl Fn(&str, usize, u8) -> Option<u8>,
) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .enumerate()
                .find_map(|(pos, b)| parse_byte(line, pos, b))
                .context("missing first digit")?;
            let last = line
                .bytes()
                .enumerate()
                .rev()
                .find_map(|(pos, b)| parse_byte(line, pos, b))
                .context("missing first digit")?;

            Ok(first as u32 * 10 + last as u32)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const INPUT2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_one() {
        assert_eq!(142, solve_part_one(INPUT1).unwrap())
    }

    #[test]
    fn part_two() {
        assert_eq!(281, solve_part_two(INPUT2).unwrap())
    }
}
