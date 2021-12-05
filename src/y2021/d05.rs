//! # Day 5: Hydrothermal Venture
//!
//! You come across a field of [hydrothermal vents] on the ocean floor! These vents constantly
//! produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in **lines**; the submarine helpfully produces a list of nearby lines of vents
//! (your puzzle input) for you to review. For example:
//!
//! ```txt
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! ```
//!
//! Each line of vents is given as a line segment in the format `x1,y1 -> x2,y2` where `x1`,`y1` are
//! the coordinates of one end the line segment and `x2`,`y2` are the coordinates of the other end.
//! These line segments include the points at both ends. In other words:
//!
//! - An entry like `1,1 -> 1,3` covers points `1,1`, `1,2`, and `1,3`.
//! - An entry like `9,7 -> 7,7` covers points `9,7`, `8,7`, and `7,7`.
//!
//! For now, **only consider horizontal and vertical lines**: lines where either `x1 = x2` or
//! `y1 = y2`.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! ```txt
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! ```
//!
//! In this diagram, the top left corner is `0,0` and the bottom right corner is `9,9`. Each
//! position is shown as **the number of lines which cover that point** or `.` if no line covers
//! that point. The top-left pair of `1`s, for example, comes from `2,2 -> 2,1`; the very bottom row
//! is formed by the overlapping lines `0,9 -> 5,9` and `0,9 -> 2,9`.
//!
//! To avoid the most dangerous areas, you need to determine **the number of points where at least
//! two lines overlap**. In the above example, this is anywhere in the diagram with a `2` or larger
//! - a total of **`5`** points.
//!
//! Consider only horizontal and vertical lines. **At how many points do at least two lines
//! overlap?**
//!
//! [hydrothermal vents]: https://en.wikipedia.org/wiki/Hydrothermal_vent
//!
//! ## Part Two
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture;
//! you need to also consider **diagonal lines**.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only
//! ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! - An entry like `1,1 -> 3,3` covers points `1,1`, `2,2`, and `3,3`.
//! - An entry like `9,7 -> 7,9` covers points `9,7`, `8,8`, and `7,9`.
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! ```txt
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! ```
//!
//! You still need to determine **the number of points where at least two lines overlap**. In the
//! above example, this is still anywhere in the diagram with a `2` or larger - now a total of
//! **`12`** points.
//!
//! Consider all of the lines. **At how many points do at least two lines overlap?**

use std::mem;

use anyhow::{Context, Result};
use fnv::FnvHashMap;

pub const INPUT: &str = include_str!("d05.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let lines = parse_input(input)?;
    let mut grid = FnvHashMap::default();

    for line in lines {
        if line.start.0 == line.end.0 {
            walk_horizontal(&mut grid, line);
        } else if line.start.1 == line.end.1 {
            walk_vertical(&mut grid, line);
        }
    }

    Ok(grid.into_values().filter(|&v| v >= 2).count())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let lines = parse_input(input)?;
    let mut grid = FnvHashMap::default();

    for line in lines {
        if line.start.0 == line.end.0 {
            walk_horizontal(&mut grid, line);
        } else if line.start.1 == line.end.1 {
            walk_vertical(&mut grid, line);
        } else {
            walk_diagonal(&mut grid, line);
        }
    }

    Ok(grid.into_values().filter(|&v| v >= 2).count())
}

fn parse_input(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").context("missing line point delimiter")?;
            let (s1, s2) = start.split_once(',').context("missing start value delimiter")?;
            let (e1, e2) = end.split_once(',').context("missing end value delimiter")?;

            Ok(Line { start: (s1.parse()?, s2.parse()?), end: (e1.parse()?, e2.parse()?) })
        })
        .collect()
}

type Point = (i16, i16);

#[derive(Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

fn walk_horizontal(grid: &mut FnvHashMap<Point, u32>, mut line: Line) {
    if line.start.1 > line.end.1 {
        mem::swap(&mut line.start, &mut line.end);
    }

    for point in (line.start.1..=line.end.1).map(|y| (line.start.0, y)) {
        *grid.entry(point).or_default() += 1;
    }
}

fn walk_vertical(grid: &mut FnvHashMap<Point, u32>, mut line: Line) {
    if line.start.0 > line.end.0 {
        mem::swap(&mut line.start, &mut line.end);
    }

    for point in (line.start.0..=line.end.0).map(|x| (x, line.start.1)) {
        *grid.entry(point).or_default() += 1;
    }
}

fn walk_diagonal(grid: &mut FnvHashMap<Point, u32>, line: Line) {
    let diff_x = line.start.0 - line.end.0;
    let diff_y = line.start.1 - line.end.1;

    if diff_x.abs() != diff_y.abs() {
        return;
    }

    let mut pos = line.start;
    let mod_x = if line.start.0 < line.end.0 { 1 } else { -1 };
    let mod_y = if line.start.1 < line.end.1 { 1 } else { -1 };

    loop {
        *grid.entry(pos).or_default() += 1;
        if pos == line.end {
            break;
        }

        pos.0 += mod_x;
        pos.1 += mod_y;
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn part_one() {
        assert_eq!(5, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(12, solve_part_two(INPUT).unwrap());
    }
}
