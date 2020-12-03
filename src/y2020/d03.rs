//! # Day 3: Toboggan Trajectory
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by
//! toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is
//! covered in trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
//! You make a map (your puzzle input) of the open squares (`.`) and trees (`#`) you can see. For
//! example:
//!
//! ```txt
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal
//! genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ```txt
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (`.`) in the top-left corner and need to reach the bottom (below
//! the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers
//! rational numbers); start by **counting all the trees** you would encounter for the slope **right
//! 3, down 1**:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1.
//! Then, check the position that is right 3 and down 1 from there, and so on until you go past the
//! bottom of the map.
//!
//! The locations you'd check in the above example are marked here with **`O`** where there was an
//! open square and **`X`** where there was a tree:
//!
//! ```txt
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter **`7`** trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, **how
//! many trees would you encounter?**
//!
//! ## Part Two
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal
//! stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you
//! start at the top-left corner and traverse the map all the way to the bottom:
//!
//! - Right 1, down 1.
//! - Right 3, down 1. (This is the slope you already checked.)
//! - Right 5, down 1.
//! - Right 7, down 1.
//! - Right 1, down 2.
//!
//! In the above example, these slopes would find `2`, `7`, `3`, `4`, and `2` tree(s) respectively;
//! multiplied together, these produce the answer **`336`**.
//!
//! **What do you get if you multiply together the number of trees encountered on each of the listed
//! slopes?**

use anyhow::{bail, Result};

pub const INPUT: &str = include_str!("d03.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let forest = parse_input(input);
    let mut pos = (0, 0);
    let mut count = 0;

    loop {
        pos.0 += 3;
        pos.1 += 1;

        if pos.1 >= forest.len() {
            return Ok(count);
        }

        if forest[pos.1][pos.0 % forest[pos.1].len()] {
            count += 1;
        }
    }
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let forest = parse_input(input);

    Ok([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().fold(1, |total, (right, down)| {
        let mut pos = (0, 0);
        let mut count = 0;

        loop {
            pos.0 += right;
            pos.1 += down;

            if pos.1 >= forest.len() {
                return total * count;
            }

            if forest[pos.1][pos.0 % forest[pos.1].len()] {
                count += 1;
            }
        }
    }))
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
    ..##.......\n\
    #...#...#..\n\
    .#....#..#.\n\
    ..#.#...#.#\n\
    .#...##..#.\n\
    ..#.##.....\n\
    .#.#.#....#\n\
    .#........#\n\
    #.##...#...\n\
    #...##....#\n\
    .#..#...#.#\
    ";

    #[test]
    fn part_one() {
        assert_eq!(7, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(336, solve_part_two(INPUT).unwrap());
    }
}
