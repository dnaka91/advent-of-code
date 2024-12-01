//! # Day 5: Supply Stacks
//!
//! The expedition can depart as soon as the final supplies have been unloaded from the ships.
//! Supplies are stored in stacks of marked **crates**, but because the needed supplies are buried
//! under many other crates, the crates need to be rearranged.
//!
//! The ship has a **giant cargo crane** capable of moving crates between stacks. To ensure none of
//! the crates get crushed or fall over, the crane operator will rearrange them in a series of
//! carefully-planned steps. After the crates are rearranged, the desired crates will be at the top
//! of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they
//! forgot to ask her **which** crate will end up where, and they want to be ready to unload them as
//! soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates **and** the rearrangement
//! procedure (your puzzle input). For example:
//!
//! ```txt
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! ```
//!
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate `Z` is on
//! the bottom, and crate `N` is on top. Stack 2 contains three crates; from bottom to top, they are
//! crates `M`, `C`, and `D`. Finally, stack 3 contains a single crate, `P`.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates
//! is moved from one stack to a different stack. In the first step of the above rearrangement
//! procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! ```txt
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved **one at a
//! time**, so the first crate to be moved (`D`) ends up below the second and third crates:
//!
//! ```txt
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! ```
//!
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved **one at a
//! time**, crate `C` ends up below crate `M`:
//!
//! ```txt
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! ```
//!
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```txt
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! ```
//!
//! The Elves just need to know **which crate will end up on top of each stack**; in this example,
//! the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack 3, so you should combine
//! these together and give the Elves the message **`CMZ`**.
//!
//! **After the rearrangement procedure completes, what crate ends up on top of each stack?**
//!
//! ## Part Two
//!
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't
//! following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The
//! crane isn't a CrateMover 9000 - it's a **CrateMover 9001**.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather
//! seats, an extra cup holder, and **the ability to pick up and move multiple crates at once**.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//! ```txt
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! ```txt
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved
//! crates **stay in the same order**, resulting in this new configuration:
//!
//! ```txt
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//! ```
//!
//! Next, as both crates are moved from stack 2 to stack 1, they **retain their order** as well:
//!
//! ```txt
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//! ```
//!
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate `C` that gets
//! moved:
//!
//! ```txt
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! ```
//!
//! In this example, the CrateMover 9001 has put the crates in a totally different order: **`MCD`**.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where
//! they should stand to be ready to unload the final supplies. **After the rearrangement procedure
//! completes, what crate ends up on top of each stack?**

use anyhow::{Context, Result};
use itertools::Itertools;

pub const INPUT: &str = include_str!("d05.txt");

pub fn solve_part_one(input: &str) -> Result<String> {
    let (mut stacks, steps) = parse_input(input)?;

    for step in steps {
        let (amount, from, to) = step?;
        for _ in 0..amount {
            if let Some(krate) = stacks[from].pop() {
                stacks[to].push(krate);
            }
        }
    }

    Ok(stacks.into_iter().filter_map(|mut stack| stack.pop()).collect())
}

pub fn solve_part_two(input: &str) -> Result<String> {
    let (mut stacks, steps) = parse_input(input)?;
    let mut buf = Vec::new();

    for step in steps {
        let (amount, from, to) = step?;
        let pos = stacks[from].len() - amount;
        buf.extend(stacks[from].drain(pos..));
        stacks[to].extend(buf.iter().cloned());
        buf.clear();
    }

    Ok(stacks.into_iter().filter_map(|mut stack| stack.pop()).collect())
}

type Stack = Vec<char>;
type Step = (usize, usize, usize);

fn parse_input(input: &str) -> Result<(Vec<Stack>, impl Iterator<Item = Result<Step>> + '_)> {
    let (stacks, steps) =
        input.split_once("\n\n").context("no empty line separator between stacks and steps")?;

    let stacks =
        stacks.lines().rev().skip(1).map(CrateIterator).fold(Vec::new(), |mut stacks, crates| {
            for (i, krate) in crates.enumerate().filter(|(_, c)| *c != ' ') {
                if stacks.len() < i + 1 {
                    stacks.resize(i + 1, Vec::new());
                }
                stacks[i].push(krate);
            }
            stacks
        });

    let steps = steps.lines().map(|line| {
        let line = line.strip_prefix("move ").context("missing `move ` keyword")?;
        let (amount, line) = line.split_once(' ').context("missing amount")?;
        let line = line.strip_prefix("from ").context("missing `from ` keyword")?;
        let (from, line) = line.split_once(' ').context("missing from position")?;
        let to = line.strip_prefix("to ").context("missing `to ` keyword")?;

        Ok((amount.parse()?, from.parse::<usize>()? - 1, to.parse::<usize>()? - 1))
    });

    Ok((stacks, steps))
}

struct CrateIterator<'a>(&'a str);

impl Iterator for CrateIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.len() {
            3 => {
                let krate = self.0;
                self.0 = "";
                krate.chars().nth(1)
            }
            v if v >= 4 => {
                let (krate, rest) = self.0.split_at(4);
                self.0 = rest;
                krate.chars().nth(1)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn part_one() {
        assert_eq!("CMZ", solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!("MCD", solve_part_two(INPUT).unwrap());
    }
}
