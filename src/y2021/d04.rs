//! # Day 4: Giant Squid
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that
//! you can't see any sunlight. What you **can** see, however, is a giant squid that has attached
//! itself to the outside of your submarine.
//!
//! Maybe it wants to play [bingo]?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen
//! at random, and the chosen number is **marked** on all boards on which it appears. (Numbers may
//! not appear on all boards.) If all numbers in any row or any column of a board are marked, that
//! board **wins**. (Diagonals don't count.)
//!
//! The submarine has a **bingo subsystem** to help passengers (currently, you and the giant squid)
//! pass the time. It automatically generates a random order in which to draw numbers and a random
//! set of boards (your puzzle input). For example:
//!
//! ```txt
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//! ```
//!
//! After the first five numbers are drawn (`7`, `4`, `9`, `5`, and `11`), there are no winners, but
//! the boards are marked as follows (shown here adjacent to each other to save space):
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! After the next six numbers are drawn (`17`, `23`, `2`, `0`, `14`, and `21`), there are still no
//! winners:
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! Finally, `24` is drawn:
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! At this point, the third board **wins** because it has at least one complete row or column of
//! marked numbers (in this case, the entire top row is marked: **`14 21 17 24 4`**).
//!
//! The **score** of the winning board can now be calculated. Start by finding the **sum of all
//! unmarked numbers** on that board; in this case, the sum is `188`. Then, multiply that sum by
//! **the number that was just called** when the board won, `24`, to get the final score,
//! `188 * 24 = 4512`.
//!
//! To guarantee victory against the giant squid, figure out which board will win first. **What will
//! your final score be if you choose that board?**
//!
//! [bingo]: https://en.wikipedia.org/wiki/Bingo_(American_version)
//!
//! ## Part Two
//!
//! On the other hand, it might be wise to try a different strategy: let the giant squid win.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than waste
//! time counting its arms, the safe thing to do is to **figure out which board will win last** and
//! choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after `13` is
//! eventually called and its middle column is completely marked. If you were to keep playing until
//! this point, the second board would have a sum of unmarked numbers equal to `148` for a final
//! score of `148 * 13 = 1924`.
//!
//! Figure out which board will win last. **Once it wins, what would its final score be?**

use anyhow::{bail, Context, Result};

pub const INPUT: &str = include_str!("d04.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let (numbers, mut boards) = parse_input(input)?;

    for number in numbers {
        mark_boards(&mut boards, number);

        if let Some(board) = boards.iter().find(|board| check_board(board)) {
            return Ok(sum_unmarked(board) * number as u32);
        }
    }

    bail!("no winner")
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let (numbers, mut boards) = parse_input(input)?;

    for number in numbers {
        mark_boards(&mut boards, number);

        if boards.len() == 1 && check_board(&boards[0]) {
            return Ok(sum_unmarked(&boards[0]) * number as u32);
        }

        boards.retain(|board| !check_board(board));
    }

    bail!("no winner")
}

type Board = [(u8, bool); 25];

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<Board>)> {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .context("missing draw number list ")?
        .split(',')
        .map(|number| number.parse().map_err(Into::into))
        .collect::<Result<Vec<_>>>()?;

    let mut boards = Vec::new();

    while lines.next() == Some("") {
        let mut board = [(0, false); 25];

        for i in 0..5 {
            let mut line = lines
                .next()
                .context("missing line in 5x5 board")?
                .split_whitespace()
                .map(|number| number.parse());

            for j in 0..5 {
                board[5 * i + j].0 = line.next().context("missing number in 5x5 board")??;
            }
        }

        boards.push(board);
    }

    Ok((numbers, boards))
}

fn mark_boards(boards: &mut [Board], number: u8) {
    boards
        .iter_mut()
        .flat_map(|board| board.iter_mut())
        .filter(|(value, _)| *value == number)
        .for_each(|(_, marked)| *marked = true);
}

fn check_board(board: &Board) -> bool {
    (0..5).any(|i| {
        let horz = (i * 5..i * 5 + 5).all(|i| board[i].1);
        let vert = (i..21 + i).step_by(5).all(|i| board[i].1);

        horz || vert
    })
}

fn sum_unmarked(board: &Board) -> u32 {
    board.iter().filter_map(|(value, marked)| (!marked).then(|| *value as u32)).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};

    #[test]
    fn part_one() {
        assert_eq!(4512, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(1924, solve_part_two(INPUT).unwrap());
    }
}
