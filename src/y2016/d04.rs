//! # Day 4: Security Through Obscurity
//!
//! Finally, you come across an information kiosk with a list of rooms. Of course, the list is
//! encrypted and full of decoy data, but the instructions to decode the list are barely hidden
//! nearby. Better remove the decoy data first.
//!
//! Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a
//! dash, a sector ID, and a checksum in square brackets.
//!
//! A room is real (not a decoy) if the checksum is the five most common letters in the encrypted
//! name, in order, with ties broken by alphabetization. For example:
//!
//! - `aaaaa-bbb-z-y-x-123[abxyz]` is a real room because the most common letters are `a` (5), `b`
//!   (3), and then a tie between `x`, `y`, and `z`, which are listed alphabetically.
//! - `a-b-c-d-e-f-g-h-987[abcde]` is a real room because although the letters are all tied (1 of
//!   each), the first five are listed alphabetically.
//! - `not-a-real-room-404[oarel]` is a real room.
//! - `totally-real-room-200[decoy]` is not.
//!
//! Of the real rooms from the list above, the sum of their sector IDs is `1514`.
//!
//! What is the **sum of the sector IDs of the real rooms**?
//!
//! ## Part Two
//!
//! With all the decoy data out of the way, it's time to decrypt this list and get moving.
//!
//! The room names are encrypted by a state-of-the-art [shift cipher], which is nearly unbreakable
//! without the right software. However, the information kiosk designers at Easter Bunny HQ were not
//! expecting to deal with a master cryptographer like yourself.
//!
//! To decrypt a room name, rotate each letter forward through the alphabet a number of times equal
//! to the room's sector ID. `A` becomes `B`, `B` becomes `C`, `Z` becomes `A`, and so on. Dashes
//! become spaces.
//!
//! For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.
//!
//! What is the sector ID of the room where North Pole objects are stored?
//!
//! [shift cipher]: https://en.wikipedia.org/wiki/Caesar_cipher

use std::collections::HashMap;

use anyhow::{ensure, Context, Result};

pub const INPUT: &str = include_str!("d04.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let rooms = parse_input(input)?;

    Ok(rooms
        .into_iter()
        .filter_map(|room| {
            let mut chars = room
                .name
                .chars()
                .filter(|&c| c != '-')
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(1) += 1;
                    map
                })
                .into_iter()
                .collect::<Vec<_>>();

            chars.sort_by(|a, b| a.1.cmp(&b.1).reverse().then(a.0.cmp(&b.0)));

            let checksum = chars.into_iter().map(|(c, _)| c).take(5).collect::<String>();

            (checksum == room.checksum).then_some(room.sector)
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let rooms = parse_input(input)?;

    Ok(rooms
        .into_iter()
        .map(|room| {
            let decrypted = room
                .name
                .chars()
                .map(|c| match c {
                    c @ 'a'..='z' => (((c as u32 - 97 + room.sector) % 26) + 97) as u8 as char,
                    '-' | ' ' => ' ',
                    _ => unreachable!(),
                })
                .collect::<String>();

            (decrypted, room.sector)
        })
        .find(|(name, _)| name == "northpole object storage")
        .context("no room named `northpole object storage`")?
        .1)
}

fn parse_input(input: &str) -> Result<Vec<Room<'_>>> {
    input
        .lines()
        .map(|room| {
            let (name, rest) = room.rsplit_once('-').context("no separator in the room")?;
            ensure!(name.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "invalid name");
            ensure!(
                rest.len() == 10
                    && rest.chars().nth(3) == Some('[')
                    && rest.chars().nth(9) == Some(']'),
                "invalid sector ID and checksum"
            );

            Ok(Room { name, sector: rest[0..3].parse()?, checksum: &rest[4..9] })
        })
        .collect()
}

struct Room<'a> {
    name: &'a str,
    sector: u32,
    checksum: &'a str,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        aaaaa-bbb-z-y-x-123[abxyz]
        a-b-c-d-e-f-g-h-987[abcde]
        not-a-real-room-404[oarel]
        totally-real-room-200[decoy]
    "};

    #[test]
    fn part_one() {
        assert_eq!(1514, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {}
}
