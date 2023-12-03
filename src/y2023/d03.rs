//! # Day 3: Gear Ratios
//!
//! You and the Elf eventually reach a [gondola lift] station; he says the gondola lift will take
//! you up to the **water source**, but this is as far as he can bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I
//! wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before
//! I can fix it." You offer to help.
//!
//! The engineer explains that an engine part seems to be missing from the engine, but nobody can
//! figure out which one. If you can **add up all the part numbers** in the engine schematic, it
//! should be easy to work out which part is missing.
//!
//! The engine schematic (your puzzle input) consists of a visual representation of the engine.
//! There are lots of numbers and symbols you don't really understand, but apparently **any number
//! adjacent to a symbol**, even diagonally, is a "part number" and should be included in your sum.
//! (Periods (`.`) do not count as a symbol.)
//!
//! Here is an example engine schematic:
//!
//! ```txt
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, two numbers are **not** part numbers because they are not adjacent to a
//! symbol: `114` (top right) and `58` (middle right). Every other number is adjacent to a symbol
//! and so **is** a part number; their sum is **`4361`**.
//!
//! Of course, the actual engine schematic is much larger. **What is the sum of all of the part
//! numbers in the engine schematic?**
//!
//! [gondola lift]: https://en.wikipedia.org/wiki/Gondola_lift
//!
//! ## Part Two
//!
//! The engineer finds the missing part and installs it in the engine! As the engine springs to
//! life, you jump in the closest gondola, finally ready to ascend to the water source.
//!
//! You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the
//! gondola has a phone labeled "help", so you pick it up and the engineer answers.
//!
//! Before you can explain the situation, she suggests that you look out the window. There stands
//! the engineer, holding a phone in one hand and waving with the other. You're going so slowly that
//! you haven't even left the station. You exit the gondola.
//!
//! The missing part wasn't the only issue - one of the gears in the engine is wrong. A **gear** is
//! any `*` symbol that is adjacent to **exactly two part numbers**. Its **gear ratio** is the
//! result of multiplying those two numbers together.
//!
//! This time, you need to find the gear ratio of every gear and add them all up so that the
//! engineer can figure out which gear needs to be replaced.
//!
//! Consider the same engine schematic again:
//!
//! ```txt
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, there are **two** gears. The first is in the top left; it has part numbers
//! `467` and `35`, so its gear ratio is `16345`. The second gear is in the lower right; its gear
//! ratio is `451490`. (The `*` adjacent to `617` is **not** a gear because it is only adjacent to
//! one part number.) Adding up all of the gear ratios produces **`467835`**.
//!
//! **What is the sum of all of the gear ratios in your engine schematic?**

use std::ops::Range;

use ahash::{AHashMap, AHashSet};
use anyhow::Result;

pub const INPUT: &str = include_str!("d03.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let (numbers, coords) = parse_input(input);

    Ok(numbers
        .into_iter()
        .filter_map(|number| {
            let left = coords.contains_key(&(number.range.start.saturating_sub(1), number.line));
            let right = coords.contains_key(&(number.range.end, number.line));
            let y = number.line.saturating_sub(1);
            let top = (number.range.start.saturating_sub(1)..number.range.end + 1)
                .any(|x| coords.contains_key(&(x, y)));
            let y = number.line + 1;
            let bottom = (number.range.start.saturating_sub(1)..number.range.end + 1)
                .any(|x| coords.contains_key(&(x, y)));

            (left || right || top || bottom).then_some(number.value)
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let (numbers, coords) = parse_input(input);

    Ok(coords
        .into_iter()
        .filter(|&(_, b)| b == b'*')
        .filter_map(|(pos, _)| {
            let mut first = None;
            let mut second = None;

            for number in &numbers {
                let left = (number.range.start.saturating_sub(1), number.line) == pos;
                let right = (number.range.end, number.line) == pos;
                let y = number.line.saturating_sub(1);
                let top = (number.range.start.saturating_sub(1)..number.range.end + 1)
                    .any(|x| (x, y) == pos);
                let y = number.line + 1;
                let bottom = (number.range.start.saturating_sub(1)..number.range.end + 1)
                    .any(|x| (x, y) == pos);

                if left || right || top || bottom {
                    match (first, second) {
                        (None, _) => first = Some(number.value),
                        (Some(_), None) => second = Some(number.value),
                        (Some(_), Some(_)) => return None,
                    }
                }
            }

            first.zip(second).map(|(a, b)| a * b)
        })
        .sum())
}

fn parse_input(input: &str) -> (Vec<Number>, AHashMap<Pos, u8>) {
    let mut numbers = Vec::new();
    let mut symbol_coords = AHashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut number = 0;
        let mut start_x = 0;

        for (x, b) in line.bytes().enumerate() {
            match b {
                b'0'..=b'9' => {
                    if number == 0 {
                        start_x = x;
                    }
                    number = number * 10 + (b - b'0') as u32;
                }
                _ => {
                    if b != b'.' {
                        symbol_coords.insert((x, y), b);
                    }
                    if number > 0 {
                        numbers.push(Number { value: number, line: y, range: start_x..x });
                        number = 0;
                    }
                }
            }
        }

        if number > 0 {
            numbers.push(Number { value: number, line: y, range: start_x..line.len() });
        }
    }

    (numbers, symbol_coords)
}

struct Number {
    value: u32,
    line: usize,
    range: Range<usize>,
}

type Pos = (usize, usize);

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_one() {
        assert_eq!(4361, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(467835, solve_part_two(INPUT).unwrap());
    }
}
