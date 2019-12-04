//! # Day 3: Crossed Wires
//!
//! The gravity assist was successful, and you're well on your way to the Venus refuelling station.
//! During the rush back on Earth, the fuel management system wasn't completely installed, so that's
//! next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, **two wires** are connected to
//! a central port and extend outward on a grid. You trace the path each wire takes as it leaves the
//! central port, one wire per line of text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you
//! need to **find the intersection point closest to the central port**. Because the wires are on a
//! grid, use the [Manhattan distance] for this measurement. While the wires do technically cross
//! right at the central port where they both start, this point does not count, nor does a wire
//! count as crossing with itself.
//!
//! For example, if the first wire's path is `R8,U5,L5,D3`, then starting from the central port
//! (`o`), it goes right `8`, up `5`, left `5`, and finally down `3`:
//!
//! ```txt
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is `U7,R6,D4,L4`, it goes up `7`, right `6`, down `4`, and left
//! `4`:
//!
//! ```txt
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked `X`), but the lower-left one is closer to the central
//! port: its distance is `3 + 3 = 6`.
//!
//! Here are a few more examples:
//!
//! - `R75,D30,R83,U83,L12,D49,R71,U7,L72`<br/>
//!   `U62,R66,U55,R34,D71,R55,D58,R83   ` = distance `159`
//! - `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51`<br/>
//!   `U98,R91,D20,R16,D67,R40,U7,R15,U6,R7       ` = distance `135`
//!
//! **What is the Manhattan distance** from the central port to the closest intersection?
//!
//! [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
//!
//! ## Part Two
//!
//! It turns out that this circuit is very timing-sensitive; you actually need to **minimize the
//! signal delay**.
//!
//! To do this, calculate the **number of steps** each wire takes to reach each intersection; choose
//! the intersection where the **sum of both wires' steps** is lowest. If a wire visits a position
//! on the grid multiple times, use the steps value from the **first** time it visits that position
//! when calculating the total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the wire has entered to get
//! to that location, including the intersection being considered. Again consider the example from
//! above:
//!
//! ```txt
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is reached after
//! `8+5+5+2 = 20` steps by the first wire and `7+6+4+3 = 20` steps by the second wire for a total
//! of `20+20 = 40` steps.
//!
//! However, the top-right intersection is better: the first wire takes only` 8+5+2 = 15` and the
//! second wire takes only `7+6+2 = 15`, a total of `15+15 = 30` steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! - `R75,D30,R83,U83,L12,D49,R71,U7,L72`<br/>
//!   `U62,R66,U55,R34,D71,R55,D58,R83   ` = `610` steps
//! - `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51`<br/>
//!   `U98,R91,D20,R16,D67,R40,U7,R15,U6,R7       ` = `410` steps
//!
//! **What is the fewest combined steps the wires must take to reach an intersection?**

use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::Result;
use anyhow::{bail, ensure};

pub const INPUT: &str = include_str!("d03.txt");

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => bail!("Unknown direction {}", s),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    dir: Direction,
    amount: i64,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.len() >= 2, "input must be at least 2 chars long");
        Ok(Self { dir: s[0..1].parse()?, amount: s[1..].parse()? })
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Point(i64, i64);

impl Point {
    fn manhattan_distance(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }

    fn move_in(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.1 += 1,
            Direction::Right => self.0 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
        }
    }
}

pub fn solve_part_one(input: &str) -> Result<i64> {
    let mut input = input.lines();
    let wire_a: HashSet<_> = to_points(input.next().unwrap()).into_iter().collect();
    let wire_b: HashSet<_> = to_points(input.next().unwrap()).into_iter().collect();

    Ok(wire_a.intersection(&wire_b).map(|p| p.manhattan_distance()).min().unwrap())
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    let mut input = input.lines();
    let points_a = to_points(input.next().unwrap());
    let points_b = to_points(input.next().unwrap());
    let wire_a: HashSet<_> = points_a.iter().collect();
    let wire_b: HashSet<_> = points_b.iter().collect();

    Ok(wire_a
        .intersection(&wire_b)
        .map(|p| {
            points_a.iter().position(|o| o == *p).unwrap()
                + points_b.iter().position(|o| o == *p).unwrap()
                + 2
        })
        .min()
        .unwrap() as i64)
}

fn to_points(input: &str) -> Vec<Point> {
    let moves = input.split(',').map(|m| m.parse::<Move>().unwrap());

    let mut pos = Point::default();
    let mut points = Vec::new();

    for m in moves {
        for _ in 0..m.amount {
            pos.move_in(m.dir);
            points.push(pos);
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let tests = [
            (6, vec!["R8,U5,L5,D3", "U7,R6,D4,L4"]),
            (159, vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"]),
            (
                135,
                vec![
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                ],
            ),
        ];

        for (expect, input) in tests.iter() {
            assert_eq!(*expect, solve_part_one(&input.join("\n")).unwrap());
        }
    }

    #[test]
    fn part_two() {
        let tests = [
            (30, vec!["R8,U5,L5,D3", "U7,R6,D4,L4"]),
            (610, vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"]),
            (
                410,
                vec![
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                ],
            ),
        ];

        for (expect, input) in tests.iter() {
            assert_eq!(*expect, solve_part_two(&input.join("\n")).unwrap());
        }
    }
}
