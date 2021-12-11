//! # Day 17: Conway Cubes
//!
//! As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at
//! the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy
//! source aboard one of their super-secret imaging satellites.
//!
//! The experimental energy source is based on cutting-edge technology: a set of Conway Cubes
//! contained in a pocket dimension! When you hear it's having problems, you can't help but agree to
//! take a look.
//!
//! The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional
//! coordinate (`x,y,z`), there exists a single cube which is either **active** or **inactive**.
//!
//! In the initial state of the pocket dimension, almost all cubes start **inactive**. The only
//! exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
//! start in the specified **active** (`#`) or **inactive** (`.`) state.
//!
//! The energy source then proceeds to boot up by executing six **cycles**.
//!
//! Each cube only ever considers its **neighbors**: any of the 26 other cubes where any of their
//! coordinates differ by at most `1`. For example, given the cube at `x=1,y=2,z=3`, its neighbors
//! include the cube at `x=2,y=2,z=2`, the cube at `x=0,y=2,z=3`, and so on.
//!
//! During a cycle, **all** cubes **simultaneously** change their state according to the following
//! rules:
//!
//! - If a cube is **active** and **exactly `2` or `3`** of its neighbors are also active, the cube
//!   remains **active**. Otherwise, the cube becomes **inactive**.
//! - If a cube is **inactive** but **exactly `3`** of its neighbors are active, the cube becomes
//!   **active**. Otherwise, the cube remains **inactive**.
//!
//! The engineers responsible for this experimental energy source would like you to simulate the
//! pocket dimension and determine what the configuration of cubes should be at the end of the
//! six-cycle boot process.
//!
//! For example, consider the following initial state:
//!
//! ```txt
//! .#.
//! ..#
//! ###
//! ```
//!
//! Even though the pocket dimension is 3-dimensional, this initial state represents a small
//! 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the
//! 3-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the
//! result of each cycle is shown layer-by-layer at each given `z` coordinate (and the frame of view
//! follows the active cells in each cycle):
//!
//! ```txt
//! Before any cycles:
//!
//! z=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1
//! #..
//! ..#
//! .#.
//!
//! z=0
//! #.#
//! .##
//! .#.
//!
//! z=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=0
//! ##...
//! ##...
//! #....
//! ....#
//! .###.
//!
//! z=1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//!
//! After 3 cycles:
//!
//! z=-2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//!
//! z=-1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=0
//! ...#...
//! .......
//! #......
//! .......
//! .....##
//! .##.#..
//! ...#...
//!
//! z=1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//! ```
//!
//! After the full six-cycle boot process completes, **`112`** cubes are left in the **active**
//! state.
//!
//! Starting with your given initial configuration, simulate six cycles. **How many cubes are left
//! in the active state after the sixth cycle?**
//!
//! ## Part Two
//!
//! For some reason, your simulated results don't match what the experimental energy source
//! engineers expected. Apparently, the pocket dimension actually has **four spatial dimensions**,
//! not three.
//!
//! The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional
//! coordinate (`x,y,z,w`), there exists a single cube (really, a **hypercube**) which is still
//! either **active** or **inactive**.
//!
//! Each cube only ever considers its **neighbors**: any of the 80 other cubes where any of their
//! coordinates differ by at most `1`. For example, given the cube at `x=1,y=2,z=3,w=4`, its
//! neighbors include the cube at `x=2,y=2,z=3,w=3`, the cube at `x=0,y=2,z=3,w=4`, and so on.
//!
//! The initial state of the pocket dimension still consists of a small flat region of cubes.
//! Furthermore, the same rules for cycle updating still apply: during each cycle, consider the
//! **number of active neighbors** of each cube.
//!
//! For example, consider the same initial state as in the example above. Even though the pocket
//! dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In
//! particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the
//! result of each cycle is shown layer-by-layer at each given `z` and `w` coordinate:
//!
//! ```txt
//! Before any cycles:
//!
//! z=0, w=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=0, w=0
//! #.#
//! .##
//! .#.
//!
//! z=1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//! ```
//!
//! After the full six-cycle boot process completes, **`848`** cubes are left in the **active**
//! state.
//!
//! Starting with your given initial configuration, simulate six cycles in a 4-dimensional space.
//! **How many cubes are left in the active state after the sixth cycle?**

use std::{
    hash::Hash,
    iter,
    ops::{Index, IndexMut},
};

use anyhow::Result;
use fnv::FnvHashMap;

pub const INPUT: &str = include_str!("d17.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    solve::<PosGen3>(input)
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    solve::<PosGen4>(input)
}

fn parse_input<T>(input: &str) -> FnvHashMap<T, bool>
where
    T: From<(i16, i16)> + Eq + Hash,
{
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| ((x as i16, y as i16).into(), c == '#'))
        })
        .collect()
}

fn solve<T>(input: &str) -> Result<usize>
where
    T: From<(i16, i16)>
        + Eq
        + Hash
        + Index<usize, Output = i16>
        + IndexMut<usize>
        + Copy
        + Bounds
        + Length
        + Iter,
{
    let mut coordinates = parse_input::<T>(input);

    let (min, max) = bounding_box(&coordinates);
    expand(&mut coordinates, min, max);

    let mut copy = coordinates.clone();

    for _ in 0..6 {
        for (&pos, &active) in &coordinates {
            let count = count_active(&coordinates, pos);
            copy.insert(pos, if active { count == 2 || count == 3 } else { count == 3 });
        }

        let (min, max) = bounding_box(&copy);
        expand(&mut copy, min, max);
        coordinates.clone_from(&copy);
    }

    Ok(coordinates.values().into_iter().filter(|&&active| active).count())
}

trait Bounds {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

trait Length {
    fn len() -> usize;
}

trait Iter {
    fn iter_neighbors(&self) -> Box<dyn Iterator<Item = Self> + '_>;
    fn iter_bounding_box<'a>(&'a self, other: &'a Self) -> Box<dyn Iterator<Item = Self> + 'a>;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct PosGen3([i16; 3]);

impl Bounds for PosGen3 {
    fn min_value() -> Self {
        Self([i16::MIN; 3])
    }

    fn max_value() -> Self {
        Self([i16::MAX; 3])
    }
}

impl Length for PosGen3 {
    fn len() -> usize {
        3
    }
}

impl Iter for PosGen3 {
    fn iter_neighbors(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(
            (self.0[0] - 1..=self.0[0] + 1)
                .flat_map(move |x| iter::repeat(x).zip(self.0[1] - 1..=self.0[1] + 1))
                .flat_map(move |xy| iter::repeat(xy).zip(self.0[2] - 1..=self.0[2] + 1))
                .map(|((x, y), z)| Self([x, y, z])),
        )
    }

    fn iter_bounding_box<'a>(&'a self, other: &'a Self) -> Box<dyn Iterator<Item = Self> + 'a> {
        Box::new(
            (self.0[0]..=other.0[0])
                .flat_map(move |x| iter::repeat(x).zip(self.0[1]..=other.0[1]))
                .flat_map(move |xy| iter::repeat(xy).zip(self.0[2]..=other.0[2]))
                .map(|((x, y), z)| Self([x, y, z])),
        )
    }
}

impl Index<usize> for PosGen3 {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PosGen3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<(i16, i16)> for PosGen3 {
    fn from((x, y): (i16, i16)) -> Self {
        Self([x, y, 0])
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct PosGen4([i16; 4]);

impl Bounds for PosGen4 {
    fn min_value() -> Self {
        Self([i16::MIN; 4])
    }

    fn max_value() -> Self {
        Self([i16::MAX; 4])
    }
}

impl Length for PosGen4 {
    fn len() -> usize {
        4
    }
}

impl Iter for PosGen4 {
    fn iter_neighbors(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(
            (self.0[0] - 1..=self.0[0] + 1)
                .flat_map(move |x| iter::repeat(x).zip(self.0[1] - 1..=self.0[1] + 1))
                .flat_map(move |xy| iter::repeat(xy).zip(self.0[2] - 1..=self.0[2] + 1))
                .flat_map(move |xyz| iter::repeat(xyz).zip(self.0[3] - 1..=self.0[3] + 1))
                .map(|(((x, y), z), w)| Self([x, y, z, w])),
        )
    }

    fn iter_bounding_box<'a>(&'a self, other: &'a Self) -> Box<dyn Iterator<Item = Self> + 'a> {
        Box::new(
            (self.0[0]..=other.0[0])
                .flat_map(move |x| iter::repeat(x).zip(self.0[1]..=other.0[1]))
                .flat_map(move |xy| iter::repeat(xy).zip(self.0[2]..=other.0[2]))
                .flat_map(move |xyz| iter::repeat(xyz).zip(self.0[3]..=other.0[3]))
                .map(|(((x, y), z), w)| Self([x, y, z, w])),
        )
    }
}

impl Index<usize> for PosGen4 {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PosGen4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<(i16, i16)> for PosGen4 {
    fn from((x, y): (i16, i16)) -> Self {
        Self([x, y, 0, 0])
    }
}

fn count_active<T>(coords: &FnvHashMap<T, bool>, pos: T) -> u8
where
    T: Eq + Copy + Hash + Iter,
{
    let mut count = 0;

    for p in pos.iter_neighbors() {
        if p == pos {
            continue;
        }

        if coords.get(&p).copied().unwrap_or_default() {
            count += 1;
        }
    }

    count
}

fn bounding_box<T>(coords: &FnvHashMap<T, bool>) -> (T, T)
where
    T: Bounds + Length + Index<usize, Output = i16> + IndexMut<usize>,
{
    let mut min = T::max_value();
    let mut max = T::min_value();

    for pos in coords.keys() {
        for i in 0..T::len() {
            min[i] = min[i].min(pos[i]);
            max[i] = max[i].max(pos[i]);
        }
    }

    (min, max)
}

fn expand<T>(coords: &mut FnvHashMap<T, bool>, mut min: T, mut max: T)
where
    T: Length + IndexMut<usize, Output = i16> + Iter + Hash + Eq,
{
    for i in 0..T::len() {
        min[i] -= 1;
        max[i] += 1;
    }

    for p in min.iter_bounding_box(&max) {
        coords.entry(p).or_insert(false);
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        .#.
        ..#
        ###
    "};

    #[test]
    fn part_one() {
        assert_eq!(112, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(848, solve_part_two(INPUT).unwrap());
    }
}
