//! # Day 13: Transparent Origami
//!
//! You reach another volcanically active part of the cave. It would be nice if you could do some
//! kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.
//!
//! Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you
//! are greeted with:
//!
//! ```txt
//! Congratulations on your purchase! To activate this infrared thermal imaging
//! camera system, please enter the code found on page 1 of the manual.
//! ```
//!
//! Apparently, the Elves have never used this feature. To your surprise, you manage to find the
//! manual; as you go to open it, page 1 falls out. It's a large sheet of [transparent paper]! The
//! transparent paper is marked with random dots and includes instructions on how to fold it up
//! (your puzzle input). For example:
//!
//! ```txt
//! 6,10
//! 0,14
//! 9,10
//! 0,3
//! 10,4
//! 4,11
//! 6,0
//! 6,12
//! 4,1
//! 0,13
//! 10,12
//! 3,4
//! 3,0
//! 8,4
//! 1,10
//! 2,14
//! 8,10
//! 9,0
//!
//! fold along y=7
//! fold along x=5
//! ```
//!
//! The first section is a list of dots on the transparent paper. `0,0` represents the top-left
//! coordinate. The first value, `x`, increases to the right. The second value, `y`, increases
//! downward. So, the coordinate `3,0` is to the right of `0,0`, and the coordinate `0,7` is below
//! `0,0`. The coordinates in this example form the following pattern, where `#` is a dot on the
//! paper and `.` is an empty, unmarked position:
//!
//! ```txt
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//! ```
//!
//! Then, there is a list of **fold instructions**. Each instruction indicates a line on the
//! transparent paper and wants you to fold the paper **up** (for horizontal `y=...` lines) or
//! **left* (for vertical `x=...` lines). In this example, the first fold instruction is
//! `fold along y=7`, which designates the line formed by all of the positions where `y` is `7`
//! (marked here with `-`):
//!
//! ```txt
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! -----------
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//! ```
//!
//! Because this is a horizontal line, fold the bottom half **up**. Some of the dots might end up
//! overlapping after the fold is complete, but dots will never appear exactly on a fold line. The
//! result of doing this fold looks like this:
//!
//! ```txt
//! #.##..#..#.
//! #...#......
//! ......#...#
//! #...#......
//! .#.#..#.###
//! ...........
//! ...........
//! ```
//!
//! Now, only `17` dots are visible.
//!
//! Notice, for example, the two dots in the bottom left corner before the transparent paper is
//! folded; after the fold is complete, those dots appear in the top left corner (at `0,0` and
//! `0,1`). Because the paper is transparent, the dot just below them in the result (at `0,3`)
//! remains visible, as it can be seen through the transparent paper.
//!
//! Also notice that some dots can end up **overlapping**; in this case, the dots merge together and
//! become a single dot.
//!
//! The second fold instruction is `fold along x=5`, which indicates this line:
//!
//! ```txt
//! #.##.|#..#.
//! #...#|.....
//! .....|#...#
//! #...#|.....
//! .#.#.|#.###
//! .....|.....
//! .....|.....
//! ```
//!
//! Because this is a vertical line, fold **left**:
//!
//! ```txt
//! #####
//! #...#
//! #...#
//! #...#
//! #####
//! .....
//! .....
//! ```
//!
//! The instructions made a square!
//!
//! The transparent paper is pretty big, so for now, focus on just completing the first fold. After
//! the first fold in the example above, **`17`** dots are visible - dots that end up overlapping
//! after the fold is completed count as a single dot.
//!
//! **How many dots are visible after completing just the first fold instruction on your transparent
//! paper?**
//!
//! [transparent paper]: https://en.wikipedia.org/wiki/Transparency_(projection)
//!
//! ## Part Two
//!
//! Finish folding the transparent paper according to the instructions. The manual says the code is
//! always **eight capital letters**.
//!
//! **What code do you use to activate the infrared thermal imaging camera system?**

use ahash::AHashSet;
use anyhow::{bail, Context, Result};

pub const INPUT: &str = include_str!("d13.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let (mut coordinates, folds) = parse_input(input)?;
    let fold = folds.first().context("no folds")?;

    fold_paper(&mut coordinates, fold);

    Ok(coordinates.len())
}

pub fn solve_part_two(input: &str) -> Result<String> {
    let (mut coordinates, folds) = parse_input(input)?;

    for fold in folds {
        fold_paper(&mut coordinates, &fold);
    }

    Ok(render(coordinates))
}

fn parse_input(input: &str) -> Result<(AHashSet<(u16, u16)>, Vec<Fold>)> {
    let (coordinates, folds) =
        input.split_once("\n\n").context("missing coordinates and folds separator")?;

    let coordinates = coordinates
        .lines()
        .map(|coordinate| {
            let (x, y) = coordinate.split_once(',').context("missing X and Y delimiter")?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect::<Result<_>>()?;

    let folds = folds
        .lines()
        .map(|fold| {
            let fold = fold.strip_prefix("fold along ").context("missing fold prefix")?;
            let (direction, position) =
                fold.split_once('=').context("missing fold instruction delimiter")?;

            Ok(Fold {
                direction: match direction {
                    "x" => Direction::Horizontal,
                    "y" => Direction::Vertical,
                    _ => bail!("unknown direction `{}`", direction),
                },
                position: position.parse()?,
            })
        })
        .collect::<Result<_>>()?;

    Ok((coordinates, folds))
}

struct Fold {
    direction: Direction,
    position: u16,
}

enum Direction {
    Horizontal,
    Vertical,
}

fn fold_paper(coordinates: &mut AHashSet<(u16, u16)>, fold: &Fold) {
    match fold.direction {
        Direction::Horizontal => {
            let targets = coordinates
                .iter()
                .copied()
                .filter(|(x, _)| *x > fold.position)
                .collect::<AHashSet<_>>();

            coordinates.retain(|coord| !targets.contains(coord));
            coordinates.extend(targets.into_iter().map(|(x, y)| (fold.position * 2 - x, y)));
        }
        Direction::Vertical => {
            let targets = coordinates
                .iter()
                .copied()
                .filter(|(_, y)| *y > fold.position)
                .collect::<AHashSet<_>>();

            coordinates.retain(|coord| !targets.contains(coord));
            coordinates.extend(targets.into_iter().map(|(x, y)| (x, fold.position * 2 - y)));
        }
    }
}

fn render(coordinates: AHashSet<(u16, u16)>) -> String {
    let (max_x, max_y) = coordinates
        .iter()
        .copied()
        .fold((0, 0), |acc, (x, y)| (acc.0.max(x as usize + 1), acc.1.max(y as usize)));
    let mut buf = vec![b'.'; (max_x + 1) * (max_y + 1)];

    for y in 0..=max_y {
        buf[(max_x + 1) * y] = b'\n';
    }

    for (x, y) in coordinates {
        buf[(max_x + 1) * y as usize + x as usize + 1] = b'#';
    }

    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn part_one() {
        assert_eq!(17, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        let expect = indoc! {"

            #####
            #...#
            #...#
            #...#
            #####"};

        assert_eq!(expect, solve_part_two(INPUT).unwrap());
    }
}
