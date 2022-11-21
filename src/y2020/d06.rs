//! # Day 6: Custom Customs
//!
//! As your flight approaches the regional airport where you'll switch to a much larger plane,
//! [customs declaration forms] are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked `a` through `z`. All you need to do is
//! identify the questions for which **anyone in your group** answers "yes". Since your group is
//! just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks if
//! you can help. For each of the people in their group, you write down the questions for which they
//! answer "yes", one per line. For example:
//!
//! ```txt
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are **`6`** questions to which anyone answered "yes": `a`, `b`, `c`, `x`,
//! `y`, and `z`. (Duplicate answers to the same question don't count extra; each question counts at
//! most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from
//! every group on the plane (your puzzle input). Each group's answers are separated by a blank
//! line, and within each group, each person's answers are on a single line. For example:
//!
//! ```txt
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - The first group contains one person who answered "yes" to **`3`** questions: `a`, `b`, and
//!   `c`.
//! - The second group contains three people; combined, they answered "yes" to **`3`** questions:
//!   `a`, `b`, and `c`.
//! - The third group contains two people; combined, they answered "yes" to **`3`** questions: `a`,
//!   `b`, and `c`.
//! - The fourth group contains four people; combined, they answered "yes" to only **`1`** question,
//!   `a`.
//! - The last group contains one person who answered "yes" to only **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1` = **`11`**.
//!
//! For each group, count the number of questions to which anyone answered "yes". **What is the sum
//! of those counts?**
//!
//! [customs declaration forms]: https://en.wikipedia.org/wiki/Customs_declaration
//!
//! ## Part Two
//!
//! As you finish the last group's customs declaration, you notice that you misread one word in the
//! instructions:
//!
//! You don't need to identify the questions to which **anyone** answered "yes"; you need to
//! identify the questions to which **everyone** answered "yes"!
//!
//! Using the same example as above:
//!
//! ```txt
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - In the first group, everyone (all 1 person) answered "yes" to **`3`** questions: `a`, `b`, and
//!   `c`.
//! - In the second group, there is **no** question to which everyone answered "yes".
//! - In the third group, everyone answered yes to only **`1`** question, `a`. Since some people did
//!   not answer "yes" to `b` or `c`, they don't count.
//! - In the fourth group, everyone answered yes to only **`1`** question, `a`.
//! - In the fifth group, everyone (all 1 person) answered "yes" to **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1` = **`6`**.
//!
//! For each group, count the number of questions to which **everyone** answered "yes". **What is
//! the sum of those counts?**

use ahash::AHashSet;
use anyhow::Result;

pub const INPUT: &str = include_str!("d06.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    Ok(parse_input(input)
        .into_iter()
        .map(|g| {
            g.into_iter()
                .fold(AHashSet::default(), |mut h, a| {
                    h.extend(a.chars());
                    h
                })
                .len()
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    Ok(parse_input(input)
        .into_iter()
        .map(|g| {
            let base = g.first().map(|s| s.chars().collect::<AHashSet<_>>()).unwrap_or_default();
            g.into_iter()
                .skip(1)
                .fold(base, |mut h, a| {
                    h.retain(|&c| a.contains(c));
                    h
                })
                .len()
        })
        .sum())
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.split_terminator("\n\n").map(|g| g.lines().collect()).collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b
    "};

    #[test]
    fn part_one() {
        let input = indoc! {"
            abcx
            abcy
            abcz
        "};
        assert_eq!(6, solve_part_one(input).unwrap());
        assert_eq!(11, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(6, solve_part_two(INPUT).unwrap());
    }
}
