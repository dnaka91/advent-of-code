//! # Day 2: Password Philosophy
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast
//! from here is via [toboggan].
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong
//! with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been
//! allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of **passwords**
//! (according to the corrupted database) and **the corporate policy when that password was set**.
//!
//! For example, suppose you have the following list:
//!
//! ```txt
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password policy indicates the
//! lowest and highest number of times a given letter must appear for the password to be valid. For
//! example, `1-3 a` means that the password must contain a at least `1` time and at most `3` times.
//!
//! In the above example, **`2`** passwords are valid. The middle password, **cdefg**, is not; it
//! contains no instances of `b`, but needs at least `1`. The first and third passwords are valid:
//! they contain one `a` or nine `c`, both within the limits of their respective policies.
//!
//! **How many passwords are valid** according to their policies?
//!
//! [toboggan]: https://en.wikipedia.org/wiki/Toboggan
//!
//! ## Part Two
//!
//! While it appears you validated the passwords correctly, they don't seem to be what the Official
//! Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules
//! from his old job at the sled rental place down the street! The Official Toboggan Corporate
//! Policy actually works a little differently.
//!
//! Each policy actually describes two **positions in the password**, where `1` means the first
//! character, `2` means the second character, and so on. (Be careful; Toboggan Corporate Policies
//! have no concept of "index zero"!) **Exactly one of these positions** must contain the given
//! letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! - `1-3 a: abcde` is **valid**: position `1` contains `a` and position `3` does not.
//! - `1-3 b: cdefg` is **invalid**: neither position `1` nor position `3` contains `b`.
//! - `2-9 c: ccccccccc` is **invalid**: both position `2` and position `9` contain `c`.
//!
//! **How many passwords are valid** according to the new interpretation of the policies?

use anyhow::{Context, Result};

pub const INPUT: &str = include_str!("d02.txt");

struct Policy {
    min: usize,
    max: usize,
    c: char,
}

pub fn solve_part_one(input: &str) -> Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .filter(|(policy, pw)| {
            let count = pw.chars().filter(|&c| c == policy.c).count();
            policy.min <= count && count <= policy.max
        })
        .count())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .filter(|(policy, pw)| {
            let a = pw.chars().nth(policy.min - 1);
            let b = pw.chars().nth(policy.max - 1);
            a != b && (a == Some(policy.c) || b == Some(policy.c))
        })
        .count())
}

fn parse_input(input: &str) -> Result<Vec<(Policy, &str)>> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.splitn(2, ": ");
            let policy = parts.next().context("policy missing")?;
            let password = parts.next().context("password missing")?;

            let mut parts = policy.split_whitespace();
            let range = parts.next().context("range missing")?;
            let c = parts
                .next()
                .context("character missing")?
                .chars()
                .next()
                .context("character missing")?;

            let mut parts = range.splitn(2, '-');
            let min = parts.next().context("min missing")?.parse()?;
            let max = parts.next().context("max missing")?.parse()?;

            Ok((Policy { min, max, c }, password))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
    1-3 a: abcde\n\
    1-3 b: cdefg\n\
    2-9 c: ccccccccc\
    ";

    #[test]
    fn part_one() {
        assert_eq!(2, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(1, solve_part_two(INPUT).unwrap());
    }
}
