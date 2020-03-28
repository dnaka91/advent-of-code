//! # Day 5: Doesn't He Have Intern-Elves For This?
//!
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//!
//! A **nice string** is one with all of the following properties:
//!
//! - It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
//! - It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or
//!   `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
//! - It does **not** contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of
//!   the other requirements.
//!
//! For example:
//!
//! - `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double
//!   letter (`...dd...`), and none of the disallowed substrings.
//! - `aaa` is nice because it has at least three vowels and a double letter, even though the
//!   letters used by different rules overlap.
//! - `jchzalrnumimnmhp` is naughty because it has no double letter.
//! - `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! - `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
//!
//! How many strings are nice?

use anyhow::Result;

pub const INPUT: &str = include_str!("d05.txt");

const VOWELS: &[char] = &['a', 'i', 'e', 'o', 'u'];
const BAD_STRINGS: &[[char; 2]] = &[['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

pub fn solve_part_one(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|l| {
            let mut vowels = 0;
            let mut double = false;
            let mut bad = false;

            l.chars().fold(None, |prev, c| {
                if VOWELS.contains(&c) {
                    vowels += 1;
                }

                if let Some(p) = prev {
                    double = double || p == c;
                    bad = bad || BAD_STRINGS.contains(&[p, c])
                }

                Some(c)
            });

            if vowels >= 3 && double && !bad {
                1
            } else {
                0
            }
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(1, solve_part_one("ugknbfddgicrmopn").unwrap());
        assert_eq!(1, solve_part_one("aaa").unwrap());
        assert_eq!(0, solve_part_one("jchzalrnumimnmhp").unwrap());
        assert_eq!(0, solve_part_one("haegwjzuvuyypxyu").unwrap());
        assert_eq!(0, solve_part_one("dvszwmarrgswjxmb").unwrap());
    }

    #[test]
    fn part_two() {}
}
