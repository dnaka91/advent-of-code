//! # Day 9: All in a Single Night
//!
//! Every year, Santa manages to deliver all of his presents in a single night.
//!
//! This year, however, he has some new locations to visit; his elves have provided him the
//! distances between every pair of locations. He can start and end at any two (different) locations
//! he wants, but he must visit each location exactly once. What is the **shortest distance** he can
//! travel to achieve this?
//!
//! For example, given the following distances:
//!
//! ```txt
//! London to Dublin = 464
//! London to Belfast = 518
//! Dublin to Belfast = 141
//! ```
//!
//! The possible routes are therefore:
//!
//! ```txt
//! Dublin -> London -> Belfast = 982
//! London -> Dublin -> Belfast = 605
//! London -> Belfast -> Dublin = 659
//! Dublin -> Belfast -> London = 659
//! Belfast -> Dublin -> London = 605
//! Belfast -> London -> Dublin = 982
//! ```
//!
//! The shortest of these is `London -> Dublin -> Belfast = 605`, and so the answer is `605` in this
//! example.
//!
//! What is the distance of the shortest route?

use anyhow::Result;

pub const INPUT: &str = include_str!("d09.txt");

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
