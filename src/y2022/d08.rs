//! # Day 8: Treetop Tree House
//!
//! The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The
//! Elves explain that a previous expedition planted these trees as a reforestation effort. Now,
//! they're curious if this would be a good location for a [tree house].
//!
//! First, determine whether there is enough tree cover here to keep a tree house **hidden**. To do
//! this, you need to count the number of trees that are **visible from outside the grid** when
//! looking directly along a row or column.
//!
//! The Elves have already launched a [quadcopter] to generate a map with the height of each tree
//! (your puzzle input). For example:
//!
//! ```txt
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! Each tree is represented as a single digit whose value is its height, where `0` is the shortest
//! and `9` is the tallest.
//!
//! A tree is **visible** if all of the other trees between it and an edge of the grid are
//! **shorter** than it. Only consider trees in the same row or column; that is, only look up, down,
//! left, or right from any given tree.
//!
//! All of the trees around the edge of the grid are **visible** - since they are already on the
//! edge, there are no trees to block the view. In this example, that only leaves the **interior
//! nine trees** to consider:
//!
//! - The top-left `5` is **visible** from the left and top. (It isn't visible from the right or
//!   bottom since other trees of height `5` are in the way.)
//! - The top-middle `5` is **visible** from the top and right.
//! - The top-right `1` is not visible from any direction; for it to be visible, there would need to
//!   only be trees of height **0** between it and an edge.
//! - The left-middle `5` is **visible**, but only from the right.
//! - The center `3` is not visible from any direction; for it to be visible, there would need to be
//!   only trees of at most height `2` between it and an edge.
//! - The right-middle `3` is **visible** from the right.
//! - In the bottom row, the middle `5` is **visible**, but the `3` and `4` are not.
//!
//! With 16 trees visible on the edge and another 5 visible in the interior, a total of **`21`**
//! trees are visible in this arrangement.
//!
//! Consider your map; **how many trees are visible from outside the grid?**
//!
//! [tree house]: https://en.wikipedia.org/wiki/Tree_house
//! [quadcopter]: https://en.wikipedia.org/wiki/Quadcopter
//!
//! ## Part Two
//!
//! Content with the amount of tree cover available, the Elves just need to know the best spot to
//! build their tree house: they would like to be able to see a lot of **trees**.
//!
//! To measure the viewing distance from a given tree, look up, down, left, and right from that
//! tree; stop if you reach an edge or at the first tree that is the same height or taller than the
//! tree under consideration. (If a tree is right on the edge, at least one of its viewing distances
//! will be zero.)
//!
//! The Elves don't care about distant trees taller than those found by the rules above; the
//! proposed tree house has large [eaves] to keep it dry, so they wouldn't be able to see higher
//! than the tree house anyway.
//!
//! In the example above, consider the middle `5` in the second row:
//!
//! ```txt
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! - Looking up, its view is not blocked; it can see **`1`** tree (of height `3`).
//! - Looking left, its view is blocked immediately; it can see only **`1`** tree (of height `5`,
//!   right next to it).
//! - Looking right, its view is not blocked; it can see **`2`** trees.
//! - Looking down, its view is blocked eventually; it can see **`2`** trees (one of height `3`,
//!   then the tree of height `5` that blocks its view).
//!
//! A tree's **scenic score** is found by **multiplying together** its viewing distance in each of
//! the four directions. For this tree, this is **`4`** (found by multiplying `1 * 1 * 2 * 2`).
//!
//! However, you can do even better: consider the tree of height `5` in the middle of the fourth
//! row:
//!
//! ```txt
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! - Looking up, its view is blocked at **`2`** trees (by another tree with a height of `5`).
//! - Looking left, its view is not blocked; it can see **`2`** trees.
//! - Looking down, its view is also not blocked; it can see **`1`** tree.
//! - Looking right, its view is blocked at **`2`** trees (by a massive tree of height `9`).
//!
//! This tree's scenic score is **`8`** (`2 * 2 * 1 * 2`); this is the ideal spot for the tree
//! house.
//!
//! Consider each tree on your map. **What is the highest scenic score possible for any tree?**
//!
//! [eaves]: https://en.wikipedia.org/wiki/Eaves

use ahash::AHashSet;
use anyhow::{anyhow, ensure, Context, Result};

pub const INPUT: &str = include_str!("d08.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let (forest, width) = parse_input(input)?;
    let height = forest.len() / width;
    let corners = (width + height - 2) * 2;
    let mut visible = AHashSet::with_capacity(forest.len() - corners);

    for y in 1..height - 1 {
        // left to right
        let mut max = 0;
        for x in 1..width - 1 {
            max = forest[x + y * width - 1].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }

        // right to left
        let mut max = 0;
        for x in (1..width - 1).rev() {
            max = forest[x + y * width + 1].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }
    }

    for x in 1..width - 1 {
        // top to bottom
        let mut max = 0;
        for y in 1..height - 1 {
            max = forest[x + (y - 1) * width].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }

        // bottom to top
        let mut max = 0;
        for y in (1..height - 1).rev() {
            max = forest[x + (y + 1) * width].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }
    }

    Ok(visible.len() + corners)
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let (forest, width) = parse_input(input)?;
    let height = forest.len() / width;
    let mut best = 0;

    for y in 0..height {
        for x in 0..width {
            let left = count_left(&forest, (x, y), (width, height));
            let right = count_right(&forest, (x, y), (width, height));
            let top = count_top(&forest, (x, y), (width, height));
            let bottom = count_bottom(&forest, (x, y), (width, height));

            // println!("{x}x{y}: {top} * {left} * {right} * {bottom} = {}", left * right * top * bottom);
            best = (left * right * top * bottom).max(best);
        }
    }

    Ok(best)
}

fn count_left(forest: &[u8], (x, y): (usize, usize), (w, h): (usize, usize)) -> usize {
    if x == 0 {
        return 0;
    }

    let mut max = 0;
    let mut count = 0;

    for i in (0..x).rev() {
        if forest[i + y * w] >= forest[x + y * w] {
            return count + 1;
        }

        max = forest[i + y * w].max(max);
        count += 1;
    }

    count
}

fn count_right(forest: &[u8], (x, y): (usize, usize), (w, h): (usize, usize)) -> usize {
    if x == w - 1 {
        return 0;
    }

    let mut max = 0;
    let mut count = 0;

    for i in x + 1..w {
        if forest[i + y * w] >= forest[x + y * w] {
            return count + 1;
        }

        max = forest[i + y * w].max(max);
        count += 1;
    }

    count
}

fn count_top(forest: &[u8], (x, y): (usize, usize), (w, h): (usize, usize)) -> usize {
    if y == 0 {
        return 0;
    }

    let mut max = 0;
    let mut count = 0;

    for i in (0..y).rev() {
        if forest[x + i * w] >= forest[x + y * w] {
            return count + 1;
        }

        max = forest[x + i * w].max(max);
        count += 1;
    }

    count
}

fn count_bottom(forest: &[u8], (x, y): (usize, usize), (w, h): (usize, usize)) -> usize {
    if y == h - 1 {
        return 0;
    }

    let mut max = 0;
    let mut count = 0;

    for i in y + 1..h {
        if forest[x + i * w] >= forest[x + y * w] {
            return count + 1;
        }

        max = forest[x + i * w].max(max);
        count += 1;
    }

    count
}

fn parse_input(input: &str) -> Result<(Vec<u8>, usize)> {
    let mut lines = input.lines();
    let width = lines.next().context("empty input")?.len();

    ensure!(lines.all(|line| line.len() == width), "input lines must have the same width");

    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|n| {
                ensure!(n.is_ascii_digit(), "invalid height {n}");
                Ok(n - b'0')
            })
        })
        .collect::<Result<_>>()
        .map(|values| (values, width))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn part_one() {
        assert_eq!(21, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(8, solve_part_two(INPUT).unwrap());
    }
}
