//! # Day 8: Seven Segment Search
//!
//! You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing
//! it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice
//! but to press on.
//!
//! As your submarine slowly makes its way through the cave system, you notice that the four-digit
//! [seven-segment displays] in your submarine are malfunctioning; they must have been damaged
//! during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's
//! wrong.
//!
//! Each digit of a seven-segment display is rendered by turning on or off any of seven segments
//! named `a` through `g`:
//!
//! ```txt
//!   0:      1:      2:      3:      4:
//!  aaaa    ....    aaaa    aaaa    ....
//! b    c  .    c  .    c  .    c  b    c
//! b    c  .    c  .    c  .    c  b    c
//!  ....    ....    dddd    dddd    dddd
//! e    f  .    f  e    .  .    f  .    f
//! e    f  .    f  e    .  .    f  .    f
//!  gggg    ....    gggg    gggg    ....
//!
//!   5:      6:      7:      8:      9:
//!  aaaa    aaaa    aaaa    aaaa    aaaa
//! b    .  b    .  .    c  b    c  b    c
//! b    .  b    .  .    c  b    c  b    c
//!  dddd    dddd    ....    dddd    dddd
//! .    f  e    f  .    f  e    f  .    f
//! .    f  e    f  .    f  e    f  .    f
//!  gggg    gggg    ....    gggg    gggg
//! ```
//!
//! So, to render a `1`, only segments `c` and `f` would be turned on; the rest would be off. To
//! render a `7`, only segments `a`, `c`, and `f` would be turned on.
//!
//! The problem is that the signals which control the segments have been mixed up on each display.
//! The submarine is still trying to display numbers by producing output on signal wires `a` through
//! `g`, but those wires are connected to segments **randomly**. Worse, the wire/segment connections
//! are mixed up separately for each four-digit display! (All of the digits **within** a display use
//! the same connections, though.)
//!
//! So, you might know that only signal wires `b` and `g` are turned on, but that doesn't mean
//! **segments** `b` and `g` are turned on: the only digit that uses two segments is `1`, so it must
//! mean segments `c` and `f` are meant to be on. With just that information, you still can't tell
//! which wire (`b`/`g`) goes to which segment (`c`/`f`). For that, you'll need to collect more
//! information.
//!
//! For each display, you watch the changing signals for a while, make a note of **all ten unique
//! signal patterns** you see, and then write down a single **four digit output value** (your puzzle
//! input). Using the signal patterns, you should be able to work out which pattern corresponds to
//! which digit.
//!
//! For example, here is what you might see in a single entry in your notes:
//!
//! ```txt
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! (The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single
//! line.)
//!
//! Each entry consists of ten **unique signal patterns**, a `|` delimiter, and finally the **four
//! digit output value**. Within an entry, the same wire/segment connections are used (but you don't
//! know what the connections actually are). The unique signal patterns correspond to the ten
//! different ways the submarine tries to render a digit using the current wire/segment connections.
//! Because `7` is the only digit that uses three segments, `dab` in the above example means that to
//! render a `7`, signal lines `d`, `a`, and `b` are on. Because `4` is the only digit that uses
//! four segments, `eafb` means that to render a `4`, signal lines `e`, `a`, `f`, and `b` are on.
//!
//! Using this information, you should be able to work out which combination of signal wires
//! corresponds to each of the ten digits. Then, you can decode the four digit output value.
//! Unfortunately, in the above example, all of the digits in the output value
//! (`cdfeb fcadb cdfeb cdbaf`) use five segments and are more difficult to deduce.
//!
//! For now, **focus on the easy digits**. Consider this larger example:
//!
//! ```txt
//! be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//! fdgacbe cefdb cefbgd gcbe
//! edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//! fcgedb cgb dgebacf gc
//! fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//! cg cg fdcagb cbg
//! fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//! efabcd cedba gadfec cb
//! aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//! gecf egdcabf bgf bfgea
//! fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//! gebdcfa ecba ca fadegcb
//! dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//! cefg dcbef fcge gbcadfe
//! bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//! ed bcgafe cdgba cbgef
//! egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//! gbdfcae bgc cg cgb
//! gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//! fgae cfgab fg bagce
//! ```
//!
//! Because the digits `1`, `4`, `7`, and `8` each use a unique number of segments, you should be
//! able to tell which combinations of signals correspond to those digits. Counting **only digits in
//! the output values** (the part after `|` on each line), in the above example, there are **`26`**
//! instances of digits that use a unique number of segments (highlighted above).
//!
//! **In the output values, how many times do digits `1`, `4`, `7`, or `8` appear?**
//!
//! [seven-segment displays]: https://en.wikipedia.org/wiki/Seven-segment_display
//!
//! ## Part Two
//!
//! Through a little deduction, you should now be able to determine the remaining digits. Consider
//! again the first example above:
//!
//! ```txt
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! After some careful analysis, the mapping between signal wires and segments only make sense in
//! the following configuration:
//!
//! ```txt
//!  dddd
//! e    a
//! e    a
//!  ffff
//! g    b
//! g    b
//!  cccc
//! ```
//!
//! So, the unique signal patterns would correspond to the following digits:
//!
//! - `acedgfb`: `8`
//! - `cdfbe`: `5`
//! - `gcdfa`: `2`
//! - `fbcad`: `3`
//! - `dab`: `7`
//! - `cefabd`: `9`
//! - `cdfgeb`: `6`
//! - `eafb`: `4`
//! - `cagedb`: `0`
//! - `ab`: `1`
//!
//! Then, the four digits of the output value can be decoded:
//!
//! - `cdfeb`: **`5`**
//! - `fcadb`: **`3`**
//! - `cdfeb`: **`5`**
//! - `cdbaf`: **`3`**
//!
//! Therefore, the output value for this entry is **`5353`**.
//!
//! Following this same process for each entry in the second, larger example above, the output value
//! of each entry can be determined:
//!
//! - `fdgacbe cefdb cefbgd gcbe`: `8394`
//! - `fcgedb cgb dgebacf gc`: `9781`
//! - `cg cg fdcagb cbg`: `1197`
//! - `efabcd cedba gadfec cb`: `9361`
//! - `gecf egdcabf bgf bfgea`: `4873`
//! - `gebdcfa ecba ca fadegcb`: `8418`
//! - `cefg dcbef fcge gbcadfe`: `4548`
//! - `ed bcgafe cdgba cbgef`: `1625`
//! - `gbdfcae bgc cg cgb`: `8717`
//! - `fgae cfgab fg bagce`: `4315`
//!
//! Adding all of the output values in this larger example produces **`61229`**.
//!
//! For each entry, determine all of the wire/segment connections and decode the four-digit output
//! values. **What do you get if you add up all of the output values?**

use std::str::FromStr;

use anyhow::{bail, ensure, Context, Ok, Result};
use itertools::Itertools;

pub const INPUT: &str = include_str!("d08.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let entries = parse_input(input)?;

    Ok(entries
        .into_iter()
        .map(|(_, output)| {
            output.iter().filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7)).count()
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let entries = parse_input(input)?;
    let mut digits = [Digit::default(); 10];
    let mut digits_length5 = Vec::with_capacity(3);
    let mut digits_length6 = Vec::with_capacity(3);

    entries
        .into_iter()
        .map(|(input, output)| {
            for digit in &mut digits {
                *digit = Digit::default();
            }
            digits_length5.clear();
            digits_length6.clear();

            for digit_str in input {
                let digit = digit_str.parse::<Digit>()?;
                let mapping = match digit_str.len() {
                    2 => &mut digits[1],
                    4 => &mut digits[4],
                    3 => &mut digits[7],
                    7 => &mut digits[8],
                    5 => {
                        digits_length5.push(digit);
                        continue;
                    }
                    6 => {
                        digits_length6.push(digit);
                        continue;
                    }
                    _ => unreachable!(),
                };

                *mapping = digit;
            }

            for &digit in &digits_length6 {
                if !digit.contains(digits[1]) {
                    digits[6] = digit;
                } else if !digit.contains(digits[4]) {
                    digits[0] = digit;
                } else {
                    digits[9] = digit;
                }
            }

            for &digit in &digits_length5 {
                if digits[6].contains(digit) {
                    digits[5] = digit;
                } else if digits[9].contains(digit) {
                    digits[3] = digit;
                } else {
                    digits[2] = digit;
                }
            }

            output.into_iter().map(|digit| digit.parse::<Digit>()).try_fold(0, |sum, digit| {
                let digit = digit?;
                Ok(sum * 10 + digits.into_iter().position(|d| d == digit).unwrap())
            })
        })
        .try_fold(0, |sum, number| Ok(sum + number?))
}

fn parse_input(input: &str) -> Result<Vec<([&str; 10], [&str; 4])>> {
    input
        .lines()
        .map(|entry| {
            let (input, output) =
                entry.split_once(" | ").context("missing input/output delimiter")?;
            let mut input_array = [""; 10];
            let mut output_array = [""; 4];

            let mut input = input.split_whitespace();
            for item in &mut input_array {
                *item = input.next().context("input data too short")?;
                ensure!((2..=7).contains(&item.len()), "invalid input digit `{}`", item);
            }

            ensure!(input.next().is_none(), "input data too long");

            let mut output = output.split_whitespace();
            for item in &mut output_array {
                *item = output.next().context("output data too short")?;
                ensure!((2..=7).contains(&item.len()), "invalid output digit `{}`", item);
            }

            ensure!(output.next().is_none(), "output data too long");

            Ok((input_array, output_array))
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Digit([u8; 7]);

impl FromStr for Digit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digit = [0; 7];

        for &c in s.as_bytes() {
            match c {
                b'a' => digit[0] = c,
                b'b' => digit[1] = c,
                b'c' => digit[2] = c,
                b'd' => digit[3] = c,
                b'e' => digit[4] = c,
                b'f' => digit[5] = c,
                b'g' => digit[6] = c,
                _ => bail!("invalid character `{}`", c as char),
            }
        }

        Ok(Self(digit))
    }
}

impl Digit {
    fn contains(self, other: Self) -> bool {
        self.0.into_iter().zip(other.0.into_iter()).all(|(a, b)| b == 0 || a == b)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn part_one() {
        assert_eq!(26, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(61229, solve_part_two(INPUT).unwrap());
    }
}
