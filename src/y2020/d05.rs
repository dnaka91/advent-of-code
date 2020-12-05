//! # Day 5: Binary Boarding
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't
//! sure which seat is yours, and all of the flight attendants are busy with the flood of people
//! that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes
//! (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of [zones or groups], this airline uses **binary space partitioning** to seat people. A
//! seat might be specified like `FBFBBFFRLR`, where `F` means "front", `B` means "back", `L` means
//! "left", and `R` means "right".
//!
//! The first 7 characters will either be `F` or `B`; these specify exactly one of the **128 rows**
//! on the plane (numbered `0` through `127`). Each letter tells you which half of a region the
//! given seat is in. Start with the whole list of rows; the first letter indicates whether the seat
//! is in the **front** (`0` through `63`) or the **back** (`64` through `127`). The next letter
//! indicates which half of that region the seat is in, and so on until you're left with exactly one
//! row.
//!
//! For example, consider just the first seven characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, rows `0` through `127`.
//! - `F` means to take the **lower half**, keeping rows `0` through `63`.
//! - `B` means to take the **upper half**, keeping rows `32` through `63`.
//! - `F` means to take the **lower half**, keeping rows `32` through `47`.
//! - `B` means to take the **upper half**, keeping rows `40` through `47`.
//! - `B` keeps rows `44` through `47`.
//! - `F` keeps rows `44` through `45`.
//! - The final `F` keeps the lower of the two, **row `44`**.
//!
//! The last three characters will be either `L` or `R`; these specify exactly one of the **8
//! columns** of seats on the plane (numbered `0` through `7`). The same process as above proceeds
//! again, this time with only three steps. `L` means to keep the **lower half**, while `R` means to
//! keep the **upper half**.
//!
//! For example, consider just the last 3 characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, columns `0` through `7`.
//! - `R` means to take the **upper half**, keeping columns `4` through `7`.
//! - `L` means to take the **lower half**, keeping columns `4` through `5`.
//! - The final `R` keeps the upper of the two, **column `5`**.
//!
//! So, decoding `FBFBBFFRLR` reveals that it is the seat at **row `44`, column `5`**.
//!
//! Every seat also has a unique **seat ID**: multiply the row by 8, then add the column. In this
//! example, the seat has ID `44 * 8 + 5 = 357`.
//!
//! Here are some other boarding passes:
//!
//! - `BFFFBBFRRR`: row `70`, column `7`, seat ID `567`.
//! - `FFFBBBFRRR`: row `14`, column `7`, seat ID `119`.
//! - `BBFFBBFRLL`: row `102`, column `4`, seat ID `820`.
//!
//! As a sanity check, look through your list of boarding passes. **What is the highest seat ID on a
//! boarding pass?**
//!
//! [zones or groups]: https://www.youtube.com/watch?v=oAHbLRjF0vo
//!
//! ## Part Two
//!
//! **Ding!** The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your
//! list. However, there's a catch: some of the seats at the very front and back of the plane don't
//! exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will
//! be in your list.
//!
//! **What is the ID of your seat?**

use anyhow::{ensure, Context, Result};

pub const INPUT: &str = include_str!("d05.txt");

pub fn solve_part_one(input: &str) -> Result<u16> {
    parse_input(input)?
        .into_iter()
        .map(|p| {
            let fb = find_seat(&p[..7]);
            let lr = find_seat(&p[7..]);

            fb as u16 * 8 + lr as u16
        })
        .max()
        .context("no maximum seat ID found")
}

pub fn solve_part_two(input: &str) -> Result<u16> {
    let mut seats = [false; 8 * 128];

    parse_input(input)?.into_iter().for_each(|p| {
        let y = find_seat(&p[..7]);
        let x = find_seat(&p[7..]);

        seats[y as usize * 8 + x as usize] = true;
    });

    seats
        .iter()
        .enumerate()
        .skip_while(|(i, v)| !*v)
        .find(|(i, v)| !*v)
        .map(|(i, v)| i as u16)
        .context("no seat ID found")
}

fn parse_input(input: &str) -> Result<Vec<&str>> {
    input
        .lines()
        .map(|l| {
            ensure!(l.len() == 10, "pass must be 10 characters long");
            ensure!(
                l.chars().all(|c| "FBLR".contains(c)),
                "pass must only contain F, B, L or R characters"
            );
            Ok(l)
        })
        .collect()
}

fn find_seat(values: &str) -> u8 {
    let max = 2u8.pow(values.len() as u32);
    let mut range = (0, max - 1);

    for (i, c) in values.chars().enumerate() {
        let v = max / 2u8.pow(i as u32 + 1);
        match c {
            'F' | 'L' => range.1 -= v,
            'B' | 'R' => range.0 += v,
            _ => {}
        }
    }

    range.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(357, solve_part_one("FBFBBFFRLR").unwrap());
        assert_eq!(567, solve_part_one("BFFFBBFRRR").unwrap());
        assert_eq!(119, solve_part_one("FFFBBBFRRR").unwrap());
        assert_eq!(820, solve_part_one("BBFFBBFRLL").unwrap());
    }

    #[test]
    fn part_two() {}
}
