//! # Day 4: The Ideal Stocking Stuffer
//!
//! Santa needs help [mining] some AdventCoins (very similar to [bitcoins]) to use as gifts for all
//! the economically forward-thinking little girls and boys.
//!
//! To do this, he needs to find [MD5] hashes which, in [hexadecimal], start with at least **five
//! zeroes**. The input to the MD5 hash is some secret key (your puzzle input, given below) followed
//! by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no
//! leading zeroes: `1`, `2`, `3`, ...) that produces such a hash.
//!
//! For example:
//!
//! - If your secret key is `abcdef`, the answer is `609043`, because the MD5 hash of `abcdef609043`
//!   starts with five zeroes (`000001dbbfa...`), and it is the lowest such number to do so.
//! - If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
//!   starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970` looks like
//!   `000006136ef...`.
//!
//! [mining]: https://en.wikipedia.org/wiki/Bitcoin#Mining
//! [bitcoins]: https://en.wikipedia.org/wiki/Bitcoin
//! [MD5]: https://en.wikipedia.org/wiki/MD5
//! [hexadecimal]: https://en.wikipedia.org/wiki/Hexadecimal
//!
//! ## Part Two
//!
//! Now find one that starts with **six zeroes**.

use anyhow::{bail, Result};
use md5::{Digest, Md5};

pub const INPUT: &str = include_str!("d04.txt");

pub fn solve_part_one(input: &str) -> Result<u64> {
    let key = parse_input(input);

    for i in 1..u64::MAX {
        let hash = Md5::digest(format!("{}{}", key, i).as_bytes());

        if hex::encode(hash).starts_with("00000") {
            return Ok(i);
        }
    }

    bail!("no value found")
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let key = parse_input(input);

    for i in 1..u64::MAX {
        let hash = Md5::digest(format!("{}{}", key, i).as_bytes());

        if hex::encode(hash).starts_with("000000") {
            return Ok(i);
        }
    }

    bail!("no value found")
}

fn parse_input(input: &str) -> &str {
    input.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(609_043, solve_part_one("abcdef").unwrap());
        assert_eq!(1_048_970, solve_part_one("pqrstuv").unwrap());
    }

    #[test]
    fn part_two() {}
}
