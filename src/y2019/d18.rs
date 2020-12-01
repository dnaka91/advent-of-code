//! # Day 18: Many-Worlds Interpretation
//!
//! As you approach Neptune, a planetary security system detects you and activates a giant
//! [tractor beam] on [Triton]! You have no choice but to land.
//!
//! A scan of the local area reveals only one interesting feature: a massive underground vault. You
//! generate a map of the tunnels (your puzzle input). The tunnels are too narrow to move
//! diagonally.
//!
//! Only one **entrance** (marked `@`) is present among the **open passages** (marked `.`) and
//! **stone walls** (`#`), but you also detect an assortment of **keys** (shown as lowercase
//! letters) and **doors** (shown as uppercase letters). Keys of a given letter open the door of the
//! same letter: `a` opens `A`, `b` opens `B`, and so on. You aren't sure which key you need to
//! disable the tractor beam, so you'll need to **collect all of them**.
//!
//! For example, suppose you have the following map:
//!
//! ```.txt
//! #########
//! #b.A.@.a#
//! #########
//! ```
//!
//! Starting from the entrance (`@`), you can only access a large door (`A`) and a key (`a`). Moving
//! toward the door doesn't help you, but you can move `2` steps to collect the key, unlocking `A`
//! in the process:
//!
//! ```txt
//! #########
//! #b.....@#
//! #########
//! ```
//!
//! Then, you can move `6` steps to collect the only other key, `b`:
//!
//! ```txt
//! #########
//! #@......#
//! #########
//! ```
//!
//! So, collecting every key took a total of **`8`** steps.
//!
//! Here is a larger example:
//!
//! ```txt
//! ########################
//! #f.D.E.e.C.b.A.@.a.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! The only reasonable move is to take key a and unlock door `A`:
//!
//! ```txt
//! ########################
//! #f.D.E.e.C.b.....@.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! Then, do the same with key `b`:
//!
//! ```txt
//! ########################
//! #f.D.E.e.C.@.........c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! ...and the same with key `c`:
//!
//! ```txt
//! ########################
//! #f.D.E.e.............@.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! Now, you have a choice between keys `d` and `e`. While key `e` is closer, collecting it now
//! would be slower in the long run than collecting key `d` first, so that's the best choice:
//!
//! ```txt
//! ########################
//! #f...E.e...............#
//! ######################.#
//! #@.....................#
//! ########################
//! ```
//!
//! Finally, collect key `e` to unlock door `E`, then collect key `f`, taking a grand total of
//! **`86`** steps.
//!
//! Here are a few more examples:
//!
//! - ```txt
//!   ########################
//!   #...............b.C.D.f#
//!   #.######################
//!   #.....@.a.B.c.d.A.e.F.g#
//!   ########################
//!   ```
//!
//!   Shortest path is `132` steps: `b`, `a`, `c`, `d`, `f`, `e`, `g`
//!
//! - ```txt
//!   #################
//!   #i.G..c...e..H.p#
//!   ########.########
//!   #j.A..b...f..D.o#
//!   ########@########
//!   #k.E..a...g..B.n#
//!   ########.########
//!   #l.F..d...h..C.m#
//!   #################
//!   ```
//!
//!   Shortest paths are `136` steps;
//!
//!   one is: `a`, `f`, `b`, `j`, `g`, `n`, `h`, `d`, `l`, `o`, `e`, `p`, `c`, `i`, `k`, `m`
//!
//! - ```txt
//!   ########################
//!   #@..............ac.GI.b#
//!   ###d#e#f################
//!   ###A#B#C################
//!   ###g#h#i################
//!   ########################
//!   ```
//!
//!   Shortest paths are `81` steps; one is: `a`, `c`, `f`, `i`, `d`, `g`, `b`, `e`, `h`
//!
//! **How many steps is the shortest path that collects all of the keys?**
//!
//! [tractor beam]: https://en.wikipedia.org/wiki/Tractor_beam
//! [Triton]: https://en.wikipedia.org/wiki/Triton_(moon)

use anyhow::Result;

pub const INPUT: &str = include_str!("d18.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    Ok(0)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
