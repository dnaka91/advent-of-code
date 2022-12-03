//! # Day 8: I Heard You Like Registers
//!
//! You receive a signal directly from the CPU. Because of your recent assistance with
//! [jump instructions], it would like you to compute the result of a series of unusual register
//! instructions.
//!
//! Each instruction consists of several parts: the register to modify, whether to increase or
//! decrease that register's value, the amount by which to increase or decrease it, and a condition.
//! If the condition fails, skip the instruction without modifying the register. The registers all
//! start at `0`. The instructions look like this:
//!
//! ```txt
//! b inc 5 if a > 1
//! a inc 1 if b < 5
//! c dec -10 if a >= 1
//! c inc -20 if c == 10
//! ```
//!
//! These instructions would be processed as follows:
//!
//! - Because `a` starts at `0`, it is not greater than `1`, and so `b` is not modified.
//! - `a` is increased by `1` (to `1`) because `b` is less than `5` (it is `0`).
//! - `c` is decreased by `-10` (to `10`) because `a` is now greater than or equal to `1` (it is
//!   `1`).
//! - `c` is increased by `-20` (to `-10`) because `c` is equal to `10`.
//!
//! After this process, the largest value in any register is `1`.
//!
//! You might also encounter `<=` (less than or equal to) or `!=` (not equal to). However, the CPU
//! doesn't have the bandwidth to tell you what all the registers are named, and leaves that to you
//! to determine.
//!
//! **What is the largest value in any register** after completing the instructions in your puzzle
//! input?
//!
//! [jump instructions]: super::d05
//!
//! ## Part Two
//!
//! To be safe, the CPU also needs to know **the highest value held in any register during this
//! process** so that it can decide how much memory to allocate to these operations. For example, in
//! the above instructions, the highest value ever held was `10` (in register `c` after the third
//! instruction was evaluated).

use std::{cmp, collections::BTreeMap, str::FromStr};

use anyhow::{bail, ensure, Result};

pub const INPUT: &str = include_str!("d08.txt");

pub fn solve_part_one(input: &str) -> Result<i32> {
    let input = parse_input(input)?;
    let mut registers = BTreeMap::new();

    for instr in input {
        if instr.condition.cmp(*registers.entry(instr.condition.register).or_default()) {
            *registers.entry(instr.register).or_default() +=
                if instr.increase { instr.value } else { -instr.value }
        }
    }

    Ok(*registers.values().max().unwrap())
}

pub fn solve_part_two(input: &str) -> Result<i32> {
    let input = parse_input(input)?;
    let mut registers = BTreeMap::new();
    let mut max = 0;

    for instr in input {
        if instr.condition.cmp(*registers.entry(instr.condition.register).or_default()) {
            let reg = registers.entry(instr.register).or_default();
            *reg += if instr.increase { instr.value } else { -instr.value };
            max = cmp::max(max, *reg);
        }
    }

    Ok(max)
}

fn parse_input(input: &str) -> Result<Vec<Instruction<'_>>> {
    input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<_>>();
            ensure!(parts.len() == 7, "instruction must consist of exactly 7 components");
            ensure!(parts[3] == "if", "expected 'if' keyword at 4th position");

            Ok(Instruction {
                register: parts[0],
                increase: parts[1] == "inc",
                value: parts[2].parse()?,
                condition: Condition {
                    register: parts[4],
                    operator: parts[5].parse()?,
                    value: parts[6].parse()?,
                },
            })
        })
        .collect()
}

struct Instruction<'a> {
    register: &'a str,
    increase: bool,
    value: i32,
    condition: Condition<'a>,
}

struct Condition<'a> {
    register: &'a str,
    operator: Op,
    value: i32,
}

impl<'a> Condition<'a> {
    fn cmp(&self, reg: i32) -> bool {
        match self.operator {
            Op::GreaterThan => reg > self.value,
            Op::LessThan => reg < self.value,
            Op::GreaterThanEquals => reg >= self.value,
            Op::LessThanEquals => reg <= self.value,
            Op::Equals => reg == self.value,
            Op::NotEquals => reg != self.value,
        }
    }
}

enum Op {
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Equals,
    NotEquals,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            ">=" => Self::GreaterThanEquals,
            "<=" => Self::LessThanEquals,
            "==" => Self::Equals,
            "!=" => Self::NotEquals,
            _ => bail!("unsupported operator '{}'", s),
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10
    "};

    #[test]
    fn part_one() {
        assert_eq!(1, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(10, solve_part_two(INPUT).unwrap());
    }
}
