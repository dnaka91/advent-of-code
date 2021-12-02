//! # Day 2: Bathroom Security
//!
//! You arrive at **Easter Bunny Headquarters** under cover of darkness. However, you left in such a
//! rush that you forgot to use the bathroom! Fancy office buildings like this one usually have
//! keypad locks on their bathrooms, so you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes will no longer be
//! written down. Instead, please memorize and follow the procedure below to access the bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be found by starting on the
//! previous button and moving to adjacent buttons on the keypad: `U` moves up, `D` moves down, `L`
//! moves left, and `R` moves right. Each line of instructions corresponds to one button, starting
//! at the previous button (or, for the first line, **the "5" button**); press whatever button
//! you're on at the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you walk to the bathroom.
//! You picture a keypad like this:
//!
//! ```txt
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ```txt
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//! ```
//!
//! - You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"),
//!   so the first button is `1`.
//! - Starting from the previous button ("1"), you move right twice (to "3") and then down three
//!   times (stopping at "9" after two moves and ignoring the third), ending up with `9`.
//! - Continuing from "9", you move left, up, right, down, and left, ending with `8`.
//! - Finally, you move up four times (stopping at "2"), then down once, ending with `5`.
//!
//! So, in this example, the bathroom code is `1985`.
//!
//! Your puzzle input is the instructions from the document you found at the front desk. What is the
//! **bathroom code**?
//!
//! ## Part Two
//!
//! You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors can
//! behold the many fancy conference rooms and water coolers on this floor) and go to punch in the
//! code. Much to your bladder's dismay, the keypad is not at all like you imagined it. Instead, you
//! are confronted with the result of hundreds of man-hours of bathroom-keypad-design meetings:
//!
//! ```txt
//!     1
//!   2 3 4
//! 5 6 7 8 9
//!   A B C
//!     D
//! ```
//!
//! You still start at "5" and stop when you're at an edge, but given the same instructions as
//! above, the outcome is very different:
//!
//! - You start at "5" and don't move at all (up and left are both edges), ending at `5`.
//! - Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D",
//!   "D"), ending at `D`.
//! - Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at `B`.
//! - Finally, after five more moves, you end at `3`.
//!
//! So, given the actual keypad layout, the code would be `5DB3`.
//!
//! Using the same instructions in your puzzle input, what is the correct **bathroom code**?

use anyhow::{bail, Result};

pub const INPUT: &str = include_str!("d02.txt");

pub fn solve_part_one(input: &str) -> Result<String> {
    let directions = parse_input(input)?;
    let mut position = 5;
    let mut buttons = String::with_capacity(directions.len());

    for directions in directions {
        for direction in directions {
            match direction {
                Direction::Up => {
                    if position > 3 {
                        position -= 3;
                    }
                }
                Direction::Down => {
                    if position < 7 {
                        position += 3;
                    }
                }
                Direction::Left => {
                    if (position - 1) % 3 != 0 {
                        position -= 1;
                    }
                }
                Direction::Right => {
                    if position % 3 != 0 {
                        position += 1;
                    }
                }
            }
        }

        buttons.push((b'0' + position) as char);
    }

    Ok(buttons)
}

pub fn solve_part_two(input: &str) -> Result<String> {
    const PAD: [char; 13] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];

    let directions = parse_input(input)?;
    let mut position = 5;
    let mut buttons = String::with_capacity(directions.len());

    for directions in directions {
        for direction in directions {
            match direction {
                Direction::Up => {
                    if !matches!(position, 1 | 2 | 4 | 5 | 9) {
                        position -= if matches!(position, 3 | 13) { 2 } else { 4 };
                    }
                }
                Direction::Down => {
                    if !matches!(position, 5 | 9 | 10 | 12 | 13) {
                        position += if matches!(position, 1 | 11) { 2 } else { 4 };
                    }
                }
                Direction::Left => {
                    if !matches!(position, 1 | 2 | 5 | 10 | 13) {
                        position -= 1;
                    }
                }
                Direction::Right => {
                    if !matches!(position, 1 | 4 | 9 | 12 | 13) {
                        position += 1;
                    }
                }
            }
        }

        buttons.push(PAD[position - 1]);
    }

    Ok(buttons)
}

fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>> {
    input
        .lines()
        .map(|directions| {
            directions
                .chars()
                .map(|c| {
                    Ok(match c {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => bail!("invalid direction `{}`", c),
                    })
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ULL
        RRDDD
        LURDL
        UUUUD
    "};

    #[test]
    fn part_one() {
        assert_eq!("1985", solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!("5DB3", solve_part_two(INPUT).unwrap());
    }
}
