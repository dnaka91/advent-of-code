//! # Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//!
//! He begins by delivering a present to the house at his starting location, and then an elf at the
//! North Pole calls him via radio and tells him where to move next. Moves are always exactly one
//! house to the north (`^`), south (`v`), east (`>`), or west (`<`). After each move, he delivers
//! another present to the house at his new location.
//!
//! However, the elf back at the north pole has had a little too much eggnog, and so his directions
//! are a little off, and Santa ends up visiting some houses more than once. How many houses receive
//! **at least one present**?
//!
//! For example:
//!
//! - `>` delivers presents to `2` houses: one at the starting location, and one to the east.
//! - `^>v<` delivers presents to `4` houses in a square, including twice to the house at his
//!   starting/ending location.
//! - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only `2` houses.
//!
//! ## Part Two
//!
//! The next year, to speed up the process, Santa creates a robot version of himself,
//! **Robo-Santa**, to deliver presents with him.
//!
//! Santa and Robo-Santa start at the same location (delivering two presents to the same starting
//! house), then take turns moving based on instructions from the elf, who is eggnoggedly reading
//! from the same script as the previous year.
//!
//! This year, how many houses receive *at least one present*?
//!
//! For example:
//!
//! - `^v` delivers presents to `3` houses, because Santa goes north, and then Robo-Santa goes
//!   south.
//! - `^>v<` now delivers presents to `3` houses, and Santa and Robo-Santa end up back where they
//!   started.
//! - `^v^v^v^v^v` now delivers presents to `11` houses, with Santa going one direction and
//!   Robo-Santa going the other.

use std::{collections::HashSet, convert::TryFrom};

use anyhow::{bail, Result};

pub const INPUT: &str = include_str!("d03.txt");

enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => bail!("unknown direction `{}`", value),
        })
    }
}

pub fn solve_part_one(input: &str) -> Result<usize> {
    let directions = parse_input(input)?;
    let mut pos = (0, 0);
    let mut houses = HashSet::new();

    houses.insert(pos);

    for dir in directions {
        match dir {
            Direction::North => pos.0 += 1,
            Direction::East => pos.1 += 1,
            Direction::South => pos.0 -= 1,
            Direction::West => pos.1 -= 1,
        }
        houses.insert(pos);
    }

    Ok(houses.len())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let directions = parse_input(input)?;
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut which = false;
    let mut houses = HashSet::new();

    houses.insert(pos1);

    for dir in directions {
        let pos = if which { &mut pos2 } else { &mut pos1 };
        which = !which;

        match dir {
            Direction::North => pos.0 += 1,
            Direction::East => pos.1 += 1,
            Direction::South => pos.0 -= 1,
            Direction::West => pos.1 -= 1,
        }
        houses.insert(*pos);
    }

    Ok(houses.len())
}

fn parse_input(input: &str) -> Result<Vec<Direction>> {
    input.chars().map(TryFrom::try_from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(2, solve_part_one(">").unwrap());
        assert_eq!(4, solve_part_one("^>v<").unwrap());
        assert_eq!(2, solve_part_one("^v^v^v^v^v").unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(3, solve_part_two("^v").unwrap());
        assert_eq!(3, solve_part_two("^>v<").unwrap());
        assert_eq!(11, solve_part_two("^v^v^v^v^v").unwrap());
    }
}
