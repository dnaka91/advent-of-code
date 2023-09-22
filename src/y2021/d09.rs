//! # Day 9: Smoke Basin
//!
//! These caves seem to be [lava tubes]. Parts are even still volcanically active; small
//! hydrothermal vents release smoke into the caves that slowly settles like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to avoid it and be
//! that much safer. The submarine generates a heightmap of the floor of the nearby caves for you
//! (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in. For example, consider the following
//! heightmap:
//!
//! ```txt
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Each number corresponds to the height of a particular location, where `9` is the highest and `0`
//! is the lowest a location can be.
//!
//! Your first goal is to find the **low points** - the locations that are lower than any of its
//! adjacent locations. Most locations have four adjacent locations (up, down, left, and right);
//! locations on the edge or corner of the map have three or two adjacent locations, respectively.
//! (Diagonal locations do not count as adjacent.)
//!
//! In the above example, there are **four** low points, all highlighted: two are in the first row
//! (a `1` and a `0`), one is in the third row (a `5`), and one is in the bottom row (also a `5`).
//! All other locations on the heightmap have some lower adjacent location, and so are not low
//! points.
//!
//! The **risk level** of a low point is **1 plus its height**. In the above example, the risk
//! levels of the low points are `2`, `1`, `6`, and `6`. The sum of the risk levels of all low
//! points in the heightmap is therefore **`15`**.
//!
//! Find all of the low points on your heightmap. **What is the sum of the risk levels of all low
//! points on your heightmap?**
//!
//! [lava tubes]: https://en.wikipedia.org/wiki/Lava_tube
//!
//! ## Part Two
//!
//! Next, you need to find the largest basins so you know what areas are most important to avoid.
//!
//! A **basin** is all locations that eventually flow downward to a single low point. Therefore,
//! every low point has a basin, although some basins are very small. Locations of height `9` do not
//! count as being in any basin, and all other locations will always be part of exactly one basin.
//!
//! The **size** of a basin is the number of locations within the basin, including the low point.
//! The example above has four basins.
//!
//! The top-left basin, size `3`:
//!
//! ```txt
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The top-right basin, size `9`:
//!
//! ```txt
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The middle basin, size `14`:
//!
//! ```txt
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The bottom-right basin, size `9`:
//!
//! ```txt
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Find the three largest basins and multiply their sizes together. In the above example, this is
//! `9 * 14 * 9 = 1134`.
//!
//! **What do you get if you multiply together the sizes of the three largest basins?**

use ahash::AHashSet;
use anyhow::{ensure, Result};

pub const INPUT: &str = include_str!("d09.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let (width, field) = parse_input(input)?;

    Ok((0..field.len())
        .map(|i| if is_low_point(width, &field, i) { field[i] as u32 + 1 } else { 0 })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let (width, field) = parse_input(input)?;
    let mut seen = AHashSet::new();

    let basins = (0..field.len())
        .map(|i| {
            if is_low_point(width, &field, i) {
                check_basin(width, &field, i, &mut seen)
            } else {
                0
            }
        })
        .fold([0u32; 4], |mut acc, basin| {
            if acc[0] < basin {
                acc[0] = basin;
                acc.sort_unstable();
            }
            acc
        });

    Ok(basins[1] * basins[2] * basins[3])
}

fn parse_input(input: &str) -> Result<(usize, Vec<u8>)> {
    let width = input.lines().map(|line| line.len()).min().unwrap_or_default();

    let field = input
        .lines()
        .flat_map(|line| {
            line.as_bytes().iter().map(|number| {
                ensure!(number.is_ascii_digit(), "invalid digit");
                Ok(*number - b'0')
            })
        })
        .collect::<Result<_>>()?;

    Ok((width, field))
}

fn is_low_point(width: usize, field: &[u8], i: usize) -> bool {
    let point = field[i];

    if point == 9 {
        return false;
    }

    let top = i < width || field[i - width] > point;
    let bottom = i + width >= field.len() || field[i + width] > point;
    let left = i % width == 0 || field[i - 1] > point;
    let right = (i + 1) % width == 0 || field[i + 1] > point;

    top && bottom && left && right
}

fn check_basin(width: usize, field: &[u8], i: usize, seen: &mut AHashSet<usize>) -> u32 {
    let point = field[i];
    let mut count = 1;

    if point == 9 || seen.contains(&i) {
        return 0;
    }

    seen.insert(i);

    if i >= width && field[i - width] >= point {
        count += check_basin(width, field, i - width, seen);
    }

    if i + width < field.len() && field[i + width] >= point {
        count += check_basin(width, field, i + width, seen);
    }

    if i % width != 0 && field[i - 1] >= point {
        count += check_basin(width, field, i - 1, seen);
    }

    if (i + 1) % width != 0 && field[i + 1] >= point {
        count += check_basin(width, field, i + 1, seen);
    }

    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn part_one() {
        assert_eq!(15, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(1134, solve_part_two(INPUT).unwrap());
    }
}
