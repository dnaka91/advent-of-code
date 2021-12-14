//! # Day 14: Extended Polymerization
//!
//! The incredible pressures at this depth are starting to put a strain on your submarine. The
//! submarine has [polymerization] equipment that would produce suitable materials to reinforce the
//! submarine, and the nearby volcanically-active caves should even have the necessary input
//! elements in sufficient quantities.
//!
//! The submarine manual contains instructions for finding the optimal polymer formula;
//! specifically, it offers a **polymer template** and a list of **pair insertion** rules (your
//! puzzle input). You just need to work out what polymer would result after repeating the pair
//! insertion process a few times.
//!
//! For example:
//!
//! ```txt
//! NNCB
//!
//! CH -> B
//! HH -> N
//! CB -> H
//! NH -> C
//! HB -> C
//! HC -> B
//! HN -> C
//! NN -> C
//! BH -> H
//! NC -> B
//! NB -> B
//! BN -> B
//! BB -> N
//! BC -> B
//! CC -> N
//! CN -> C
//! ```
//!
//! The first line is the **polymer template** - this is the starting point of the process.
//!
//! The following section defines the **pair insertion** rules. A rule like `AB -> C` means that
//! when elements `A` and `B` are immediately adjacent, element `C` should be inserted between them.
//! These insertions all happen simultaneously.
//!
//! So, starting with the polymer template `NNCB`, the first step simultaneously considers all three
//! pairs:
//!
//! - The first pair (`NN`) matches the rule `NN -> C`, so element **`C`** is inserted between the
//!   first `N` and the second `N`.
//! - The second pair (`NC`) matches the rule `NC -> B`, so element **`B`** is inserted between the
//!   `N` and the `C`.
//! - The third pair (`CB`) matches the rule `CB -> H`, so element **`H`** is inserted between the
//!   `C` and the `B`.
//!
//! Note that these pairs overlap: the second element of one pair is the first element of the next
//! pair. Also, because all pairs are considered simultaneously, inserted elements are not
//! considered to be part of a pair until the next step.
//!
//! After the first step of this process, the polymer becomes `NCNBCHB`.
//!
//! Here are the results of a few steps using the above rules:
//!
//! ```txt
//! Template:     NNCB
//! After step 1: NCNBCHB
//! After step 2: NBCCNBBBCBHCB
//! After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//! After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//! ```
//!
//! This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073.
//! After step 10, `B` occurs 1749 times, `C` occurs 298 times, `H` occurs 161 times, and `N` occurs
//! 865 times; taking the quantity of the most common element (`B`, 1749) and subtracting the
//! quantity of the least common element (`H`, 161) produces `1749 - 161 = 1588`.
//!
//! Apply 10 steps of pair insertion to the polymer template and find the most and least common
//! elements in the result. **What do you get if you take the quantity of the most common element
//! and subtract the quantity of the least common element?**
//!
//! [polymerization]: https://en.wikipedia.org/wiki/Polymerization
//!
//! ## Part Two
//!
//! The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run
//! more steps of the pair insertion process; a total of **40 steps** should do it.
//!
//! In the above example, the most common element is `B` (occurring `2192039569602` times) and the
//! least common element is `H` (occurring `3849876073` times); subtracting these produces
//! **`2188189693529`**.
//!
//! Apply **40** steps of pair insertion to the polymer template and find the most and least common
//! elements in the result. **What do you get if you take the quantity of the most common element
//! and subtract the quantity of the least common element?**

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use ahash::AHashMap;
use anyhow::{bail, ensure, Context, Result};
use itertools::{Itertools, MinMaxResult};

pub const INPUT: &str = include_str!("d14.txt");

pub fn solve_part_one(input: &str) -> Result<u64> {
    let (template, insertions) = parse_input(input)?;
    polymerize(template, insertions, 10)
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let (template, insertions) = parse_input(input)?;
    polymerize(template, insertions, 40)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Result<(Vec<u8>, AHashMap<(u8, u8), u8>)> {
    let mut lines = input.lines();

    let template = lines.next().context("missing polymer template")?.as_bytes().to_owned();
    ensure!(lines.next() == Some(""), "missing separater between polymer and pairs");

    let pairs = lines
        .map(|insertion| {
            let (from, to) =
                insertion.split_once(" -> ").context("missing insertion pair separator")?;
            let mut from = from.bytes();
            let mut to = to.bytes();

            let insertion = (
                (
                    from.next().context("missing first char")?,
                    from.next().context("missing second char")?,
                ),
                to.next().context("missing single char")?,
            );

            ensure!(from.next().is_none(), "too many characters in pair key");
            ensure!(to.next().is_none(), "too many characters in pair value");

            Ok(insertion)
        })
        .collect::<Result<_>>()?;

    Ok((template, pairs))
}

struct Counter<T>(AHashMap<T, u64>);

impl<T: Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let map = iter.into_iter().fold(AHashMap::new(), |mut acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        });
        Self(map)
    }
}

impl<T: Eq + Hash> Counter<T> {
    fn new() -> Self {
        Self(AHashMap::new())
    }

    fn add(&mut self, item: T, count: u64) {
        *self.0.entry(item).or_default() += count;
    }

    fn items(&self) -> impl Iterator<Item = (&T, &u64)> {
        self.0.iter()
    }

    fn reset(&mut self) {
        self.0.clear();
    }

    fn counts(self) -> impl Iterator<Item = u64> {
        let map: HashMap<_, _, _> = self.0.into();
        map.into_values()
    }
}

fn polymerize(template: Vec<u8>, insertions: AHashMap<(u8, u8), u8>, steps: usize) -> Result<u64> {
    let mut elements = Counter::from_iter(template.iter().copied());
    let mut template = Counter::from_iter(template.into_iter().tuple_windows());
    let mut buffer = Counter::new();

    for _ in 0..steps {
        for (pair, &count) in template.items() {
            if let Some(&value) = insertions.get(pair) {
                buffer.add((pair.0, value), count);
                buffer.add((value, pair.1), count);
                elements.add(value, count);
            }
        }

        std::mem::swap(&mut template, &mut buffer);
        buffer.reset();
    }

    match elements.counts().minmax() {
        MinMaxResult::MinMax(min, max) => Ok(max - min),
        _ => bail!("not enough values to find minimum and maximum occurrence"),
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn part_one() {
        assert_eq!(1588, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(2188189693529, solve_part_two(INPUT).unwrap());
    }
}
