//! # Day 3: Spiral Memory
//!
//! You come across an experimental new kind of memory stored on an infinite two-dimensional grid.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a location marked `1` and
//! then counting up while spiraling outward. For example, the first few squares are allocated like
//! this:
//!
//! ```txt
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//! ```
//!
//! While this is very space-efficient (no squares are skipped), requested data must be carried back
//! to square `1` (the location of the only access port for this memory system) by programs that can
//! only move up, down, left, or right. They always take the shortest path: the [Manhattan Distance]
//! between the location of the data and square `1`.
//!
//! For example:
//!
//! - Data from square `1` is carried `0` steps, since it's at the access port.
//! - Data from square `12` is carried `3` steps, such as: down, left, left.
//! - Data from square `23` is carried only `2` steps: up twice.
//! - Data from square `1024` must be carried `31` steps.
//!
//! **How many steps** are required to carry the data from the square identified in your puzzle
//! input all the way to the access port?
//!
//! [Manhattan Distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
//!
//! ## Part Two
//!
//! As a stress test on the system, the programs here clear the grid and then store the value `1` in
//! square `1`. Then, in the same allocation order as shown above, they store the sum of the values
//! in all adjacent squares, including diagonals.
//!
//! So, the first few squares' values are chosen as follows:
//!
//! - Square `1` starts with the value `1`.
//! - Square `2` has only one adjacent filled square (with value `1`), so it also stores `1`.
//! - Square `3` has both of the above squares as neighbors and stores the sum of their values, `2`.
//! - Square `4` has all three of the aforementioned squares as neighbors and stores the sum of their values, `4`.
//! - Square `5` only has the first and fourth squares as neighbors, so it gets the value `5`.
//!
//! Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:
//!
//! ```txt
//! 147  142  133  122   59
//! 304    5    4    2   57
//! 330   10    1    1   54
//! 351   11   23   25   26
//! 362  747  806--->   ...
//! ```
//!
//! What is the **first value written** that is **larger** than your puzzle input?

use std::collections::HashMap;

use anyhow::{Context, Result};

pub const INPUT: &str = include_str!("d03.txt");

#[derive(Default)]
struct GridIterator {
    pos: (i32, i32),
    round: i32,
    step: u32,
}

impl Iterator for GridIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;

        if self.step == 1 {
            return Some(self.pos);
        }

        if self.pos.0 == self.round && self.pos.1 == self.round {
            self.round += 1;
            self.pos.0 += 1;
            return Some(self.pos);
        }

        if self.pos.0 == self.round && self.pos.1 > -self.round {
            self.pos.1 -= 1;
            return Some(self.pos);
        }

        if self.pos.1 == -self.round && self.pos.0 > -self.round {
            self.pos.0 -= 1;
            return Some(self.pos);
        }

        if self.pos.0 == -self.round && self.pos.1 < self.round {
            self.pos.1 += 1;
            return Some(self.pos);
        }

        if self.pos.1 == self.round && self.pos.0 < self.round {
            self.pos.0 += 1;
            return Some(self.pos);
        }

        None
    }
}

pub fn solve_part_one(input: &str) -> Result<i32> {
    Ok(GridIterator::default()
        .nth(parse_input(input)? - 1)
        .map(|(x, y)| x.abs() + y.abs())
        .unwrap())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let input = parse_input(input)?;
    let iter = GridIterator::default();
    let mut grid = HashMap::new();

    grid.insert((0, 0), 1);

    for pos in iter.skip(1) {
        let mut sum = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == dy && dx == 0 {
                    continue;
                }

                sum += *grid.entry((pos.0 + dx, pos.1 + dy)).or_default();
            }
        }

        if sum > input as u32 {
            return Ok(sum);
        }

        grid.insert(pos, sum);
    }
    Ok(0)
}

fn parse_input(input: &str) -> Result<usize> {
    input.lines().next().context("expected a line with digit")?.parse().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(0, solve_part_one("1").unwrap());
        assert_eq!(3, solve_part_one("12").unwrap());
        assert_eq!(2, solve_part_one("23").unwrap());
        assert_eq!(31, solve_part_one("1024").unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(2, solve_part_two("1").unwrap());
        assert_eq!(4, solve_part_two("2").unwrap());
        assert_eq!(4, solve_part_two("3").unwrap());
        assert_eq!(5, solve_part_two("4").unwrap());
        assert_eq!(10, solve_part_two("5").unwrap());
    }
}
