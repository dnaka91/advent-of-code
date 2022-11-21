//! # Day 3: Binary Diagnostic
//!
//! The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic
//! report just in case.
//!
//! The diagnostic report (your puzzle input) consists of a list of binary numbers which, when
//! decoded properly, can tell you many useful things about the conditions of the submarine. The
//! first parameter to check is the **power consumption**.
//!
//! You need to use the binary numbers in the diagnostic report to generate two new binary numbers
//! (called the **gamma rate** and the **epsilon rate**). The power consumption can then be found by
//! multiplying the gamma rate by the epsilon rate.
//!
//! Each bit in the gamma rate can be determined by finding the **most common bit in the
//! corresponding position** of all numbers in the diagnostic report. For example, given the
//! following diagnostic report:
//!
//! ```txt
//! 00100
//! 11110
//! 10110
//! 10111
//! 10101
//! 01111
//! 00111
//! 11100
//! 10000
//! 11001
//! 00010
//! 01010
//! ```
//!
//! Considering only the first bit of each number, there are five `0` bits and seven `1` bits. Since
//! the most common bit is `1`, the first bit of the gamma rate is `1`.
//!
//! The most common second bit of the numbers in the diagnostic report is `0`, so the second bit of
//! the gamma rate is `0`.
//!
//! The most common value of the third, fourth, and fifth bits are `1`, `1`, and `0`, respectively,
//! and so the final three bits of the gamma rate are `110`.
//!
//! So, the gamma rate is the binary number `10110`, or **`22`** in decimal.
//!
//! The epsilon rate is calculated in a similar way; rather than use the most common bit, the least
//! common bit from each position is used. So, the epsilon rate is `01001`, or **`9`** in decimal.
//! Multiplying the gamma rate (`22`) by the epsilon rate (`9`) produces the power consumption,
//! **`198`**.
//!
//! Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate,
//! then multiply them together. **What is the power consumption of the submarine?** (Be sure to
//! represent your answer in decimal, not binary.)
//!
//! ## Part Two
//!
//! Next, you should verify the **life support rating**, which can be determined by multiplying the
//! **oxygen generator rating** by the **CO2 scrubber rating**.
//!
//! Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in
//! your diagnostic report - finding them is the tricky part. Both values are located using a
//! similar process that involves filtering out values until only one remains. Before searching for
//! either rating value, start with the full list of binary numbers from your diagnostic report and
//! **consider just the first bit** of those numbers. Then:
//!
//! - Keep only numbers selected by the **bit criteria** for the type of rating value for which you
//!   are searching. Discard numbers which do not match the bit criteria.
//! - If you only have one number left, stop; this is the rating value for which you are searching.
//! - Otherwise, repeat the process, considering the next bit to the right.
//!
//! The **bit criteria** depends on which type of rating value you want to find:
//!
//! - To find **oxygen generator rating**, determine the **most common** value (`0` or `1`) in the
//!   current bit position, and keep only numbers with that bit in that position. If `0` and `1` are
//!   equally common, keep values with a **`1`** in the position being considered.
//! - To find **CO2 scrubber rating**, determine the **least common** value (`0` or `1`) in the
//!   current bit position, and keep only numbers with that bit in that position. If `0` and `1` are
//!   equally common, keep values with a **`0`** in the position being considered.
//!
//! For example, to determine the **oxygen generator rating** value using the same example
//! diagnostic report from above:
//!
//! - Start with all 12 numbers and consider only the first bit of each number. There are more `1`
//!   bits (7) than `0` bits (5), so keep only the 7 numbers with a `1` in the first position:
//!   `11110`, `10110`, `10111`, `10101`, `11100`, `10000`, and `11001`.
//! - Then, consider the second bit of the 7 remaining numbers: there are more `0` bits (4) than `1`
//!   bits (3), so keep only the 4 numbers with a `0` in the second position: `10110`, `10111`,
//!   `10101`, and `10000`.
//! - In the third position, three of the four numbers have a `1`, so keep those three: `10110`,
//!   `10111`, and `10101`.
//! - In the fourth position, two of the three numbers have a `1`, so keep those two: `10110` and
//!   `10111`.
//! - In the fifth position, there are an equal number of `0` bits and `1` bits (one each). So, to
//!   find the **oxygen generator rating**, keep the number with a `1` in that position: `10111`.
//! - As there is only one number left, stop; the **oxygen generator rating** is `10111`, or
//!   **`23`** in decimal.
//!
//! Then, to determine the **CO2 scrubber rating** value from the same example above:
//!
//! - Start again with all 12 numbers and consider only the first bit of each number. There are
//!   fewer `0` bits (5) than `1` bits (7), so keep only the 5 numbers with a `0` in the first
//!   position: `00100`, `01111`, `00111`, `00010`, and `01010`.
//! - Then, consider the second bit of the 5 remaining numbers: there are fewer `1` bits (2) than
//!   `0` bits (3), so keep only the 2 numbers with a `1` in the second position: `01111` and
//!   `01010`.
//! - In the third position, there are an equal number of `0` bits and `1` bits (one each). So, to
//!   find the **CO2 scrubber rating**, keep the number with a `0` in that position: `01010`.
//! - As there is only one number left, stop; the **CO2 scrubber rating** is `01010`, or **`10`** in
//!   decimal.
//!
//! Finally, to find the life support rating, multiply the oxygen generator rating (`23`) by the CO2
//! scrubber rating (`10`) to get **`230`**.
//!
//! Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and
//! CO2 scrubber rating, then multiply them together. **What is the life support rating of the
//! submarine?** (Be sure to represent your answer in decimal, not binary.)

use anyhow::Result;

pub const INPUT: &str = include_str!("d03.txt");

pub fn solve_part_one(input: &str) -> Result<u64> {
    let (numbers, bits) = parse_input(input)?;
    let threshold = ((numbers.len() + 1) / 2) as u32;

    let count = numbers.into_iter().fold(vec![0; bits], |mut count, num| {
        (0..bits).for_each(|i| {
            let shift = bits - (i + 1);
            count[i] += u32::from(((num >> shift) & 1) == 1);
        });
        count
    });

    let mut gamma = 0u16;

    (0..bits).for_each(|i| {
        if count[i] >= threshold {
            gamma |= 1 << (bits - (i + 1));
        }
    });

    let epsilon = !gamma & (u16::MAX >> (u16::BITS as usize - bits));

    Ok(gamma as u64 * epsilon as u64)
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let (numbers, bits) = parse_input(input)?;

    let generator = find_rating(numbers.clone(), bits, 1, 0);
    let scrubber = find_rating(numbers, bits, 0, 1);

    Ok(generator as u32 * scrubber as u32)
}

fn parse_input(input: &str) -> Result<(Vec<u16>, usize)> {
    let numbers = input
        .lines()
        .map(|number| u16::from_str_radix(number, 2).map_err(Into::into))
        .collect::<Result<_>>()?;
    let bits = input.lines().map(|number| number.len()).max().unwrap_or_default();

    Ok((numbers, bits))
}

fn bit_count(numbers: &[u16], bits: usize, pos: usize) -> u32 {
    let shift = bits - pos;

    numbers
        .iter()
        .copied()
        .fold(0, |count, num| count + u32::from(((num >> shift) & 1) == 1))
}

fn find_rating(mut numbers: Vec<u16>, bits: usize, most: u16, least: u16) -> u16 {
    let mut generator = 0;

    for i in 1..=bits {
        let count = bit_count(&numbers, bits, i);
        let shift = bits - i;
        let keep = if count as usize >= (numbers.len() + 1) / 2 { most } else { least };

        numbers.retain(|num| (num >> shift) & 1 == keep);

        if numbers.len() == 1 {
            generator = numbers[0];
            break;
        }
    }

    generator
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn part_one() {
        assert_eq!(198, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(230, solve_part_two(INPUT).unwrap());
    }
}
