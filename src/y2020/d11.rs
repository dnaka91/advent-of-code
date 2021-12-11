//! # Day 11: Seating System
//!
//! Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that
//! goes directly to the tropical island where you can finally start your vacation. As you reach the
//! waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//!
//! By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
//! pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your
//! puzzle input).
//!
//! The seat layout fits neatly on a grid. Each position is either floor (`.`), an empty seat (`L`),
//! or an occupied seat (`#`). For example, the initial seat layout might look like this:
//!
//! ```txt
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! Now, you just need to model the people who will be arriving shortly. Fortunately, people are
//! entirely predictable and always follow a simple set of rules. All decisions are based on the
//! **number of occupied seats** adjacent to a given seat (one of the eight positions immediately
//! up, down, left, right, or diagonal from the seat). The following rules are applied to every seat
//! simultaneously:
//!
//! - If a seat is **empty** (`L`) and there are **no** occupied seats adjacent to it, the seat
//!   becomes **occupied**.
//! - If a seat is **occupied** (`#`) and **four or more** seats adjacent to it are also occupied,
//!   the seat becomes **empty**.
//! - Otherwise, the seat's state does not change.
//!
//! Floor (`.`) never changes; seats don't move, and nobody sits on the floor.
//!
//! After one round of these rules, every seat in the example layout becomes occupied:
//!
//! ```txt
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! After a second round, the seats with four or more occupied adjacent seats become empty again:
//!
//! ```txt
//! #.LL.L#.##
//! #LLLLLL.L#
//! L.L.L..L..
//! #LLL.LL.L#
//! #.LL.LL.LL
//! #.LLLL#.##
//! ..L.L.....
//! #LLLLLLLL#
//! #.LLLLLL.L
//! #.#LLLL.##
//! ```
//!
//! This process continues for three more rounds:
//!
//! ```txt
//! #.##.L#.##
//! #L###LL.L#
//! L.#.#..#..
//! #L##.##.L#
//! #.##.LL.LL
//! #.###L#.##
//! ..#.#.....
//! #L######L#
//! #.LL###L.L
//! #.#L###.##
//! ```
//!
//! ```txt
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.L.L..#..
//! #LLL.##.L#
//! #.LL.LL.LL
//! #.LL#L#.##
//! ..L.L.....
//! #L#LLLL#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! ```txt
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.#.L..#..
//! #L##.##.L#
//! #.#L.LL.LL
//! #.#L#L#.##
//! ..L.L.....
//! #L#L##L#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! At this point, something interesting happens: the chaos stabilizes and further applications of
//! these rules cause no seats to change state! Once people stop moving around, you count **`37`**
//! occupied seats.
//!
//! Simulate your seating area by applying the seating rules repeatedly until no seats change state.
//! **How many seats end up occupied?**
//!
//! ## Part Two
//!
//! As soon as people start to arrive, you realize your mistake. People don't just care about
//! adjacent seats - they care about **the first seat they can see** in each of those eight
//! directions!
//!
//! Now, instead of considering just the eight immediately adjacent seats, consider the **first
//! seat** in each of those eight directions. For example, the empty seat below would see **eight**
//! occupied seats:
//!
//! ```txt
//! .......#.
//! ...#.....
//! .#.......
//! .........
//! ..#L....#
//! ....#....
//! .........
//! #........
//! ...#.....
//! ```
//!
//! The leftmost empty seat below would only see **one** empty seat, but cannot see any of the
//! occupied ones:
//!
//! ```txt
//! .............
//! .L.L.#.#.#.#.
//! .............
//! ```
//!
//! The empty seat below would see **no** occupied seats:
//!
//! ```txt
//! .##.##.
//! #.#.#.#
//! ##...##
//! ...L...
//! ##...##
//! #.#.#.#
//! .##.##.
//! ```
//!
//! Also, people seem to be more tolerant than you expected: it now takes **five or more** visible
//! occupied seats for an occupied seat to become empty (rather than **four or more** from the
//! previous rules). The other rules still apply: empty seats that see no occupied seats become
//! occupied, seats matching no rule don't change, and floor never changes.
//!
//! Given the same starting layout as above, these new rules cause the seating area to shift around
//! as follows:
//!
//! ```txt
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! ```txt
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! ```txt
//! #.LL.LL.L#
//! #LLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLLL.L
//! #.LLLLL.L#
//! ```
//!
//! ```txt
//! #.L#.##.L#
//! #L#####.LL
//! L.#.#..#..
//! ##L#.##.##
//! #.##.#L.##
//! #.#####.#L
//! ..#.#.....
//! LLL####LL#
//! #.L#####.L
//! #.L####.L#
//! ```
//!
//! ```txt
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##LL.LL.L#
//! L.LL.LL.L#
//! #.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```txt
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.#L.L#
//! #.L####.LL
//! ..#.#.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```txt
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.LL.L#
//! #.LLLL#.LL
//! ..#.L.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once
//! this occurs, you count **`26`** occupied seats.
//!
//! Given the new visibility method and the rule change for occupied seats becoming empty, once
//! equilibrium is reached, **how many seats end up occupied?**

use std::{
    mem,
    ops::{Index, IndexMut},
};

use anyhow::Result;

pub const INPUT: &str = include_str!("d11.txt");

pub fn solve_part_one(input: &str) -> Result<usize> {
    let mut seating = parse_input(input);
    let mut seating_copy = seating.clone();

    loop {
        for y in 0..seating.height() {
            for x in 0..seating.width() {
                let count = count_one(&seating, (x, y));

                if count == 0 && seating[(x, y)] == 'L' {
                    seating_copy[(x, y)] = '#';
                } else if count >= 4 && seating[(x, y)] == '#' {
                    seating_copy[(x, y)] = 'L';
                }
            }
        }

        if seating == seating_copy {
            break;
        }

        seating.copy_from(&seating_copy);
    }

    Ok(seating.count_occupied())
}

pub fn solve_part_two(input: &str) -> Result<usize> {
    let mut seating = parse_input(input);
    let mut seating_copy = seating.clone();

    loop {
        for y in 0..seating.height() {
            for x in 0..seating.width() {
                let count = count_two(&seating, (x, y));

                if count == 0 && seating[(x, y)] == 'L' {
                    seating_copy[(x, y)] = '#';
                } else if count >= 5 && seating[(x, y)] == '#' {
                    seating_copy[(x, y)] = 'L';
                }
            }
        }

        if seating == seating_copy {
            break;
        }

        seating.copy_from(&seating_copy);
    }

    Ok(seating.count_occupied())
}

fn parse_input(input: &str) -> SeatMap {
    let width = input.lines().next().unwrap().len();
    let seats = input.lines().flat_map(|l| l.chars()).collect();

    SeatMap { seats, width }
}

fn count_one(seating: &SeatMap, pos: (usize, usize)) -> u8 {
    let mut count = 0;
    for y in pos.1.saturating_sub(1)..(pos.1 + 2).min(seating.height()) {
        for x in pos.0.saturating_sub(1)..(pos.0 + 2).min(seating.width()) {
            if pos != (x, y) && seating[(x, y)] == '#' {
                count += 1;
            }
        }
    }

    count
}

fn count_two(seating: &SeatMap, pos: (usize, usize)) -> u8 {
    let directions = &[(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut count = 0;

    'outer: for dir in directions {
        let (mut x, mut y) = (pos.0 as i32, pos.1 as i32);

        loop {
            x += dir.0;
            y += dir.1;

            if y < 0 || seating.height() <= y as usize || x < 0 || seating.width() <= x as usize {
                // We walked out of the seating area
                continue 'outer;
            }

            match seating[(x as usize, y as usize)] {
                'L' => continue 'outer,
                '#' => {
                    count += 1;
                    continue 'outer;
                }
                _ => {}
            }
        }
    }

    count
}

#[derive(Eq, Clone)]
struct SeatMap {
    seats: Vec<char>,
    width: usize,
}

impl SeatMap {
    fn copy_from(&mut self, other: &SeatMap) {
        self.seats.copy_from_slice(&other.seats)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.seats.len() / self.width
    }

    fn count_occupied(self) -> usize {
        self.seats.into_iter().filter(|&c| c == '#').count()
    }
}

impl Index<(usize, usize)> for SeatMap {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.seats[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for SeatMap {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.seats[index.1 * self.width + index.0]
    }
}

impl PartialEq for SeatMap {
    fn eq(&self, other: &Self) -> bool {
        self.seats.eq(&other.seats)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "};

    #[test]
    fn part_one() {
        assert_eq!(37, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(26, solve_part_two(INPUT).unwrap());
    }
}
