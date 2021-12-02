//! # Day 3: Squares With Three Sides
//!
//! Now that you can think clearly, you move deeper into the labyrinth of hallways and office
//! furniture that makes up this part of Easter Bunny HQ. This must be a graphic design department;
//! the walls are covered in specifications for triangles.
//!
//! Or are they?
//!
//! The design document gives the side lengths of each triangle it describes, but... `5 10 25`? Some
//! of these aren't triangles. You can't help but mark the impossible ones.
//!
//! In a valid triangle, the sum of any two sides must be larger than the remaining side. For
//! example, the "triangle" given above is impossible, because `5 + 10` is not larger than `25`.
//!
//! In your puzzle input, **how many** of the listed triangles are **possible**?
//!
//! ## Part Two
//!
//! Now that you've helpfully marked up their design documents, it occurs to you that triangles are
//! specified in groups of three **vertically**. Each set of three numbers in a column specifies a
//! triangle. Rows are unrelated.
//!
//! For example, given the following specification, numbers with the same hundreds digit would be
//! part of the same triangle:
//!
//! ```txt
//! 101 301 501
//! 102 302 502
//! 103 303 503
//! 201 401 601
//! 202 402 602
//! 203 403 603
//! ```
//!
//! In your puzzle input, and instead reading by columns, **how many** of the listed triangles are
//! **possible**?

use anyhow::{ensure, Context, Result};

pub const INPUT: &str = include_str!("d03.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let triangles = parse_input(input)?;

    Ok(triangles.into_iter().filter(valid_triangle).count())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let triangles = transform_input(parse_input(input)?)?;

    Ok(triangles.into_iter().filter(valid_triangle).count())
}

fn parse_input(input: &str) -> Result<Vec<[u32; 3]>> {
    input
        .lines()
        .map(|triangle| {
            let mut positions = triangle.split_whitespace().map(|pos| pos.parse());
            let p1 = positions.next().context("missing 1st position")??;
            let p2 = positions.next().context("missing 2nd position")??;
            let p3 = positions.next().context("missing 3rd position")??;
            ensure!(positions.next().is_none(), "no 4th position expected");

            Ok([p1, p2, p3])
        })
        .collect()
}

fn transform_input(input: Vec<[u32; 3]>) -> Result<Vec<[u32; 3]>> {
    ensure!(input.len() % 3 == 0, "input rows must be sets of 3");

    Ok(input
        .chunks_exact(3)
        .flat_map(|c| {
            [[c[0][0], c[1][0], c[2][0]], [c[0][1], c[1][1], c[2][1]], [c[0][2], c[1][2], c[2][2]]]
        })
        .collect())
}

fn valid_triangle(triangle: &[u32; 3]) -> bool {
    triangle[0] < triangle[1] + triangle[2]
        && triangle[1] < triangle[2] + triangle[0]
        && triangle[2] < triangle[0] + triangle[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
