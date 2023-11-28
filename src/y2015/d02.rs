//! # Day 2: I Was Told There Would Be No Math
//!
//! The elves are running low on wrapping paper, and so they need to submit an order for more. They
//! have a list of the dimensions (length `l`, width `w`, and height `h`) of each present, and only
//! want to order exactly as much as they need.
//!
//! Fortunately, every present is a box (a perfect [right rectangular prism]), which makes
//! calculating the required wrapping paper for each gift a little easier: find the surface area of
//! the box, which is `2*l*w + 2*w*h + 2*h*l`. The elves also need a little extra paper for each
//! present: the area of the smallest side.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square feet of wrapping
//!   paper plus `6` square feet of slack, for a total of `58` square feet.
//! - A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square feet of wrapping
//!   paper plus `1` square foot of slack, for a total of `43` square feet.
//!
//! All numbers in the elves' list are in feet. How many total **square feet of wrapping paper**
//! should they order?
//!
//! [right rectangular prism]: https://en.wikipedia.org/wiki/Cuboid#Rectangular_cuboid
//!
//! ## Part Two
//!
//! The elves are also running low on ribbon. Ribbon is all the same width, so they only have to
//! worry about the length they need to order, which they would again like to be exact.
//!
//! The ribbon required to wrap a present is the shortest distance around its sides, or the smallest
//! perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet
//! of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
//! Don't ask how they tie the bow, though; they'll never tell.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the present
//!   plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet.
//! - A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the present
//!   plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet.
//!
//! How many total **feet of ribbon** should they order?

use anyhow::{anyhow, ensure, Result};

pub const INPUT: &str = include_str!("d02.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let boxes = parse_input(input)?;

    Ok(boxes
        .iter()
        .map(|b| {
            let lw = b.0 * b.1;
            let wh = b.1 * b.2;
            let hl = b.2 * b.0;
            let slack = lw.min(wh).min(hl);
            2 * (lw + wh + hl) + slack
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let boxes = parse_input(input)?;

    Ok(boxes
        .iter()
        .map(|b| {
            // By subtracting the biggest value, we get the sum of the two smallest ones.
            let biggest = b.0.max(b.1).max(b.2);
            2 * (b.0 + b.1 + b.2 - biggest) + b.0 * b.1 * b.2
        })
        .sum())
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32, u32)>> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('x');
            let l = parts.next().ok_or_else(|| anyhow!("missing length"))?.parse()?;
            let w = parts.next().ok_or_else(|| anyhow!("missing width"))?.parse()?;
            let h = parts.next().ok_or_else(|| anyhow!("missing height"))?.parse()?;
            ensure!(parts.next().is_none(), "too many parts");
            Ok((l, w, h))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(58, solve_part_one("2x3x4").unwrap());
        assert_eq!(43, solve_part_one("1x1x10").unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(34, solve_part_two("2x3x4").unwrap());
        assert_eq!(14, solve_part_two("1x1x10").unwrap());
    }
}
