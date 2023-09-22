//! # Day 1: No Time for a Taxicab
//!
//! Santa's sleigh uses a very high-precision clock to guide its movements, and the clock's
//! oscillator is regulated by stars. Unfortunately, the stars have been stolen... by the Easter
//! Bunny. To save Christmas, Santa needs you to retrieve all **fifty stars** by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
//! calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one
//! star**. Good luck!
//!
//! You're airdropped near **Easter Bunny Headquarters** in a city somewhere. "Near", unfortunately,
//! is as close as you can get - the instructions on the Easter Bunny Recruiting Document the Elves
//! intercepted start here, and nobody had time to work them out further.
//!
//! The Document indicates that you should start at the given coordinates (where you just landed)
//! and face North. Then, follow the provided sequence: either turn left (`L`) or right (`R`) 90
//! degrees, then walk forward the given number of blocks, ending at a new intersection.
//!
//! There's no time to follow such ridiculous instructions on foot, though, so you take a moment and
//! work out the destination. Given that you can only walk on the [street grid of the city], how far
//! is the shortest path to the destination?
//!
//! For example:
//!
//! - Following `R2, L3` leaves you `2` blocks East and `3` blocks North, or `5` blocks away.
//! - `R2, R2, R2` leaves you `2` blocks due South of your starting position, which is `2` blocks
//!   away.
//! - `R5, L5, R5, R3` leaves you `12` blocks away.
//!
//! **How many blocks away** is Easter Bunny HQ?
//!
//! [street grid of the city]: https://en.wikipedia.org/wiki/Taxicab_geometry
//!
//! ## Part Two
//!
//! Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny
//! HQ is actually at the first location you visit twice.
//!
//! For example, if your instructions are `R8, R4, R4, R8`, the first location you visit twice is
//! `4` blocks away, due East.
//!
//! How many blocks away is the **first location you visit twice?**

use std::collections::HashSet;

use anyhow::{bail, Context, Result};

pub const INPUT: &str = include_str!("d01.txt");

pub fn solve_part_one(input: &str) -> Result<i32> {
    let instructions = parse_input(input)?;
    let mut direction = Direction::North;
    let mut position = (0, 0);

    for (turn, steps) in instructions {
        direction = direction.turn(turn);
        position = match direction {
            Direction::North => (position.0, position.1 - steps),
            Direction::East => (position.0 + steps, position.1),
            Direction::South => (position.0, position.1 + steps),
            Direction::West => (position.0 - steps, position.1),
        }
    }

    Ok(position.0.abs() + position.1.abs())
}

pub fn solve_part_two(input: &str) -> Result<i32> {
    let instructions = parse_input(input)?;
    let mut direction = Direction::North;
    let mut position = (0, 0);
    let mut history = HashSet::new();

    for (turn, steps) in instructions {
        direction = direction.turn(turn);
        let stepper: fn((i32, i32)) -> (i32, i32) = match direction {
            Direction::North => |p| (p.0, p.1 - 1),
            Direction::East => |p| (p.0 + 1, p.1),
            Direction::South => |p| (p.0, p.1 + 1),
            Direction::West => |p| (p.0 - 1, p.1),
        };

        for _ in 0..steps {
            position = stepper(position);
            if history.contains(&position) {
                return Ok(position.0.abs() + position.1.abs());
            } else {
                history.insert(position);
            }
        }
    }

    bail!("no location visited twice")
}

fn parse_input(input: &str) -> Result<Vec<(Turn, i32)>> {
    input
        .lines()
        .next()
        .context("input is empty")?
        .split(", ")
        .map(|instruction| {
            let direction = match instruction.chars().next() {
                Some('L') => Turn::Left,
                Some('R') => Turn::Right,
                Some(s) => bail!("invalid direction `{}`", s),
                None => bail!("missing direction"),
            };
            let steps = instruction[1..].parse().context("invalid steps number")?;

            Ok((direction, steps))
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Self::North => Self::West,
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
            },
            Turn::Right => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
