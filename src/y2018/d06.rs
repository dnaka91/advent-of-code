//! # Day 6: Chronal Coordinates
//!
//! The device on your wrist beeps several times, and once again you feel like you're falling.
//!
//! "Situation critical," the device announces. "Destination indeterminate. Chronal interference
//! detected. Please specify new target coordinates."
//!
//! The device then produces a list of coordinates (your puzzle input). Are they places it thinks
//! are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a
//! manual.
//!
//! **If they're dangerous,** maybe you can minimize the danger by finding the coordinate that gives
//! the largest distance from the other points.
//!
//! Using only the [Manhattan distance], determine the **area** around each coordinate by counting
//! the number of [integer] X,Y locations that are **closest** to that coordinate (and aren't **tied
//! in distance** to any other coordinate).
//!
//! Your goal is to find the size of the **largest area** that isn't infinite. For example, consider
//! the following list of coordinates:
//!
//! ```txt
//! 1, 1
//! 1, 6
//! 8, 3
//! 3, 4
//! 5, 5
//! 8, 9
//! ```
//!
//! If we name these coordinates `A` through `F`, we can draw them on a grid, putting` 0,0` at the
//! top left:
//!
//! ```txt
//! ..........
//! .A........
//! ..........
//! ........C.
//! ...D......
//! .....E....
//! .B........
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan
//! distance, each location's closest coordinate can be determined, shown here in lowercase:
//!
//! ```txt
//! aaaaa.cccc
//! aAaaa.cccc
//! aaaddecccc
//! aadddeccCc
//! ..dDdeeccc
//! bb.deEeecc
//! bBb.eeee..
//! bbb.eeefff
//! bbb.eeffff
//! bbb.ffffFf
//! ```
//!
//! Locations shown as `.` are equally far from two or more coordinates, and so they don't count as
//! being closest to any.
//!
//! In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here,
//! their areas extend forever outside the visible grid. However, the areas of coordinates D and E
//! are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's
//! location itself). Therefore, in this example, the size of the largest area is **17**.
//!
//! **What is the size of the largest area** that isn't infinite?
//!
//! [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
//! [integer]: https://en.wikipedia.org/wiki/Integer

use anyhow::{anyhow, Context, Error, Result};
use std::ops::Sub;
use std::str::FromStr;

pub const INPUT: &str = include_str!("d06.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(2, ',');
        Ok(Point(
            s.next().ok_or_else(|| anyhow!("no x value"))?.trim().parse()?,
            s.next().ok_or_else(|| anyhow!("no y value"))?.trim().parse()?,
        ))
    }
}

impl Point {
    fn distance(self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

pub fn solve_part_one(input: &str) -> Result<i64> {
    let points = parse_input(input)?;

    let bounded = points
        .iter()
        .filter(|p| {
            // TODO: Optimize with fold
            let t = points.iter().find(|p2| p2 != p && p2.1 > p.1);
            let r = points.iter().find(|p2| p2 != p && p2.0 > p.0);
            let b = points.iter().find(|p2| p2 != p && p2.1 < p.1);
            let l = points.iter().find(|p2| p2 != p && p2.0 < p.0);
            t.is_some() && r.is_some() && b.is_some() && l.is_some()
        })
        .cloned()
        .collect::<Vec<Point>>();

    let m = bounded
        .iter()
        .map(|p| {
            let mut count = 0;

            'outer_up: for y in p.1.. {
                for x in p.0.. {
                    let (b1, b2) = calc(x, y, *p, &points);
                    if b1 {
                        if b2 {
                            break 'outer_up;
                        }
                        break;
                    }

                    count += 1;

                    if x >= 400 {
                        count = 0;
                        break;
                    }
                }
                for x in (0..p.0).rev() {
                    let (b1, b2) = calc(x, y, *p, &points);
                    if b1 {
                        if b2 {
                            break 'outer_up;
                        }
                        break;
                    }

                    count += 1;

                    if x <= 0 {
                        count = 0;
                    }
                }

                if y >= 400 {
                    count = 0;
                    break;
                }
            }
            'outer_down: for y in (0..p.1).rev() {
                for x in p.0.. {
                    let (b1, b2) = calc(x, y, *p, &points);
                    if b1 {
                        if b2 {
                            break 'outer_down;
                        }
                        break;
                    }

                    count += 1;

                    if x >= 400 {
                        count = 0;
                        break;
                    }
                }
                for x in (0..p.0).rev() {
                    let (b1, b2) = calc(x, y, *p, &points);
                    if b1 {
                        if b2 {
                            break 'outer_down;
                        }
                        break;
                    }

                    count += 1;

                    if x <= 0 {
                        count = 0;
                    }
                }

                if y <= 0 {
                    count = 0;
                }
            }
            count
        })
        .max();

    Ok(m.unwrap())
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

fn parse_input(input: &str) -> Result<Vec<Point>> {
    input.lines().map(|l| l.parse()).collect()
}

fn calc(x: i32, y: i32, p: Point, points: &[Point]) -> (bool, bool) {
    let pos = Point(x, y);
    let dist = p.distance(pos);
    let mut dists = points.iter().map(|p| p.distance(pos)).collect::<Vec<i32>>();
    dists.sort();

    assert!(dists.len() >= 2);

    (dists[0] == dists[1] || dist != dists[0], x == p.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
