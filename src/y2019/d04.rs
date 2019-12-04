//! # Day 4: Secure Container
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had
//! written the password on a sticky note, but someone threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! - It is a six-digit number.
//! - The value is within the range given in your puzzle input.
//! - Two adjacent digits are the same (like `22` in `122345`).
//! - Going from left to right, the digits **never decrease**; they only ever increase or stay the
//!   same (like `111123` or `135679`).
//!
//! Other than the range rule, the following are true:
//!
//! - `111111` meets these criteria (double `11`, never decreases).
//! - `223450` does not meet these criteria (decreasing pair of digits `50`).
//! - `123789` does not meet these criteria (no double).
//!
//! **How many different passwords** within the range given in your puzzle input meet these
//! criteria?
//!
//! # Day Two
//!
//! An Elf just remembered one more important detail: the two adjacent matching digits **are not
//! part of a larger group of matching digits**.
//
//! Given this additional criterion, but still ignoring the range rule, the following are now true:
//!
//! - `112233` meets these criteria because the digits never decrease and all repeated digits are
//!   exactly two digits long.
//! - `123444` no longer meets the criteria (the repeated `44` is part of a larger group of `444`).
//! - `111122` meets the criteria (even though `1` is repeated more than twice, it still contains a
//!   double `22`).
//!
//! **How many different passwords** within the range given in your puzzle input meet all of the
//! criteria?

use anyhow::ensure;
use anyhow::Result;
use itertools::Itertools;

pub const INPUT: &str = include_str!("d04.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    let (min, max) = parse_input(input)?;

    Ok((min..=max).filter(|v| verify_password(*v, false)).count() as i64)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    let (min, max) = parse_input(input)?;

    Ok((min..=max).filter(|v| verify_password(*v, true)).count() as i64)
}

fn parse_input(input: &str) -> Result<(u32, u32)> {
    let mut input = input.lines().next().unwrap().split('-');
    let min = input.next().unwrap().parse()?;
    let max = input.next().unwrap().parse()?;

    ensure!(111_111 <= min && max <= 999_999, "min and max must be six digits long");

    Ok((min, max))
}

fn verify_password(pw: u32, part2: bool) -> bool {
    if 111_111 <= pw && pw <= 999_999 {
        let chars = pw.to_string().bytes().collect::<Vec<u8>>();
        return has_pairs(&chars) && is_rising(&chars) && (!part2 || has_unique_pair(&chars));
    }
    false
}

fn has_pairs(chars: &[u8]) -> bool {
    chars.iter().tuple_windows().any(|(l, r)| l == r)
}

fn is_rising(chars: &[u8]) -> bool {
    chars.iter().tuple_windows().all(|(l, r)| l <= r)
}

fn has_unique_pair(chars: &[u8]) -> bool {
    chars.iter().tuple_windows().enumerate().any(|(i, (l, r))| {
        l == r && (i == 0 || *l != chars[i - 1]) && (i == 4 || *l != chars[i + 2])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert!(verify_password(111111, false));
        assert!(!verify_password(223450, false));
        assert!(!verify_password(123789, false));
    }

    #[test]
    fn part_two() {
        assert!(verify_password(112233, true));
        assert!(!verify_password(123444, true));
        assert!(verify_password(111122, true));
    }
}
