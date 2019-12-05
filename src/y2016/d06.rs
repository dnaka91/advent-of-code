//! # Day 6: Signals and Noise
//!
//! Something is jamming your communications with Santa. Fortunately, your signal is only partially
//! jammed, and protocol in situations like this is to switch to a simple [repetition code] to get
//! the message through.
//!
//! In this model, the same message is sent repeatedly. You've recorded the repeating message signal
//! (your puzzle input), but the data seems quite corrupted - almost too badly to recover.
//! **Almost**.
//!
//! All you need to do is figure out which character is most frequent for each position. For
//! example, suppose you had recorded the following messages:
//!
//! ```txt
//! eedadn
//! drvtee
//! eandsr
//! raavrd
//! atevrs
//! tsrnev
//! sdttsa
//! rasrtv
//! nssdts
//! ntnada
//! svetve
//! tesnvt
//! vntsnd
//! vrdear
//! dvrsen
//! enarar
//! ```
//!
//! The most common character in the first column is `e`; in the second, `a`; in the third, `s`, and
//! so on. Combining these characters returns the error-corrected message, `easter`.
//!
//! Given the recording in your puzzle input, **what is the error-corrected version** of the message
//! being sent?
//!
//! [repetition code]: https://en.wikipedia.org/wiki/Repetition_code

use anyhow::Result;

pub const INPUT: &str = include_str!("d06.txt");

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
