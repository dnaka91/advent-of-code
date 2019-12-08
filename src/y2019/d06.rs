//! # Day 6: Universal Orbit Map
//!
//! You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often
//! involves transferring between orbits, the orbit maps here are useful for finding efficient
//! routes between, for example, you and Santa. You download a map of the local orbits (your puzzle
//! input).
//!
//! Except for the universal Center of Mass (`COM`), every object in space is in orbit around
//! exactly one other object. An [orbit] looks roughly like this:
//!
//! ```txt
//!                   \
//!                    \
//!                     |
//!                     |
//! AAA--> o            o <--BBB
//!                     |
//!                     |
//!                    /
//!                   /
//! ```
//!
//! In this diagram, the object `BBB` is in orbit around `AAA`. The path that `BBB` takes around
//! `AAA` (drawn with lines) is only partly shown. In the map data, this orbital relationship is
//! written `AAA)BBB`, which means "`BBB` is in orbit around `AAA`".
//!
//! Before you use your map data to plot a course, you need to make sure it wasn't corrupted during
//! the download. To verify maps, the Universal Orbit Map facility uses **orbit count checksums** -
//! the total number of **direct orbits** (like the one shown above) and **indirect orbits**.
//!
//! Whenever `A` orbits `B` and `B` orbits `C`, then `A` **indirectly orbits** `C`. This chain can
//! be any number of objects long: if `A` orbits `B`, `B` orbits `C`, and `C` orbits `D`, then `A`
//! indirectly orbits `D`.
//!
//! For example, suppose you have the following map:
//!
//! ```txt
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```txt
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I
//! ```
//!
//! In this visual representation, when two objects are connected by a line, the one on the right
//! directly orbits the one on the left.
//!
//! Here, we can count the total number of orbits as follows:
//!
//! - `D` directly orbits `C` and indirectly orbits `B` and `COM`, a total of `3` orbits.
//! - `L` directly orbits `K` and indirectly orbits `J`, `E`, `D`, `C`, `B`, and `COM`, a total of
//!   `7` orbits.
//! - `COM` orbits nothing.
//!
//! The total number of direct and indirect orbits in this example is **`42`**.
//!
//! **What is the total number of direct and indirect orbits** in your map data?
//!
//! [orbit]: https://en.wikipedia.org/wiki/Orbit
//!
//! ## Part Two
//!
//! Now, you just need to figure out how many **orbital transfers** you (`YOU`) need to take to get
//! to Santa (`SAN`).
//!
//! You start at the object `YOU` are orbiting; your destination is the object `SAN` is orbiting. An
//! orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//!
//! For example, suppose you have the following map:
//!
//! ```txt
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! K)YOU
//! I)SAN
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```txt
//!                           YOU
//!                          /
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//! ```
//!
//! In this example, `YOU` are in orbit around `K`, and `SAN` is in orbit around `I`. To move from
//! `K` to `I`, a minimum of `4` orbital transfers are required:
//!
//! - `K` to `J`
//! - `J` to `E`
//! - `E` to `D`
//! - `D` to `I`
//!
//! Afterward, the map of orbits looks like this:
//!
//! ```txt
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!                  \
//!                   YOU
//! ```
//!
//! **What is the minimum number of orbital transfers required** to move from the object `YOU` are
//! orbiting to the object `SAN` is orbiting? (Between the objects they are orbiting - **not**
//! between `YOU` and `SAN`.)

use anyhow::{Context, Result};
use fnv::FnvHashMap;

pub const INPUT: &str = include_str!("d06.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    let map = parse_input(input);
    let mut counts = FnvHashMap::with_capacity_and_hasher(map.len(), Default::default());

    let mut total = 0;
    for key in map.keys() {
        let mut count = 0;
        let mut value = key;

        while let Some(p) = map.get(value) {
            if let Some(c) = counts.get(p) {
                count += *c;
                break;
            }
            count += 1;
            value = p;
        }
        counts.insert(key, count + 1);
        total += count;
    }

    Ok(total)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    let map = parse_input(input);

    let you = map.get("YOU").context("YOU not in the map")?;
    let san = map.get("SAN").context("SAN not in the map")?;

    let mut you_count = 0;
    let mut you_parent = map.get(you).unwrap();
    loop {
        let mut san_count = 0;
        let mut san_parent = Some(map.get(san).unwrap());
        while let Some(sp) = san_parent {
            if sp == you_parent {
                return Ok(you_count + san_count + 2);
            }
            san_count += 1;
            san_parent = map.get(sp);
        }

        you_count += 1;
        you_parent = map.get(you_parent).unwrap();
    }
}

fn parse_input(input: &str) -> FnvHashMap<&str, &str> {
    input
        .lines()
        .map(|l| {
            let i = l.find(')').unwrap();
            // orbiter as key, object as value, as we always want to walk bottom up.
            (&l[i + 1..], &l[..i])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input =
            &["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"];
        assert_eq!(42, solve_part_one(&input.join("\n")).unwrap());
    }

    #[test]
    fn part_two() {
        let input = &[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        assert_eq!(4, solve_part_two(&input.join("\n")).unwrap());
    }
}
