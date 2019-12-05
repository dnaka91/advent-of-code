//! # Day 11: Hex Ed
//!
//! Crossing the bridge, you've barely reached the other side of the stream when a program comes up
//! to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite
//! grid!"
//!
//! Fortunately for her, you have plenty of experience with infinite grids.
//!
//! Unfortunately for you, it's a [hex grid].
//!
//! The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the
//! north, northeast, southeast, south, southwest, and northwest:
//!
//! ```txt
//!   \ n  /
//! nw +--+ ne
//!   /    \
//! -+      +-
//!   \    /
//! sw +--+ se
//!   / s  \
//! ```
//!
//! You have the path the child process took. Starting where he started, you need to determine the
//! fewest number of steps required to reach him. (A "step" means to move from the hex you are in to
//! any adjacent hex.)
//!
//! For example:
//!
//! - `ne,ne,ne` is `3` steps away.
//! - `ne,ne,sw,sw` is `0` steps away (back where you started).
//! - `ne,ne,s,s` is `2` steps away (`se,se`).
//! - `se,sw,se,sw,sw` is `3` steps away (`s,s,sw`).
//!
//! [hex grid]: https://en.wikipedia.org/wiki/Hexagonal_tiling

use anyhow::Result;

pub const INPUT: &str = include_str!("d11.txt");

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
