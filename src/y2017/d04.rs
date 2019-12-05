//! # Day 4: High-Entropy Passphrases
//!
//! A new system policy has been put in place that requires all accounts to use a **passphrase**
//! instead of simply a pass**word**. A passphrase consists of a series of words (lowercase letters)
//! separated by spaces.
//!
//! To ensure security, a valid passphrase must contain no duplicate words.
//!
//! For example:
//!
//! - `aa bb cc dd ee` is valid.
//! - `aa bb cc dd aa` is not valid - the word `aa` appears more than once.
//! - `aa bb cc dd aaa` is valid - `aa` and `aaa` count as different words.
//!
//! The system's full passphrase list is available as your puzzle input. **How many passphrases are
//! valid?**

use anyhow::Result;

pub const INPUT: &str = include_str!("d04.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    Ok(0)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
