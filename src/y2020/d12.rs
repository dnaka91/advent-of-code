//! # Day 12: Rain Risk
//!
//! Your ferry made decent progress toward the island, but the storm came in faster than anyone
//! expected. The ferry needs to take **evasive actions**!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a
//! route directly to safety, it produced extremely circuitous instructions. When the captain uses
//! the [PA system] to ask if anyone can help, you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of single-character
//! **actions** paired with integer input **values**. After staring at them for a few minutes, you
//! work out what they probably mean:
//!
//! - Action **`N`** means to move **north** by the given value.
//! - Action **`S`** means to move **south** by the given value.
//! - Action **`E`** means to move **east** by the given value.
//! - Action **`W`** means to move **west** by the given value.
//! - Action **`L`** means to turn **left** the given number of degrees.
//! - Action **`R`** means to turn **right** the given number of degrees.
//! - Action **`F`** means to move **forward** by the given value in the direction the ship is
//!   currently facing.
//!
//! The ship starts by facing **east**. Only the `L` and `R` actions change the direction the ship
//! is facing. (That is, if the ship is facing east and the next instruction is `N10`, the ship
//! would move north 10 units, but would still move east if the following action were `F`.)
//!
//! For example:
//!
//! ```txt
//! F10
//! N3
//! F7
//! R90
//! F11
//! ```
//!
//! These instructions would be handled as follows:
//!
//! - `F10` would move the ship 10 units east (because the ship starts by facing east) to **east 10,
//!   north 0**.
//! - `N3` would move the ship 3 units north to **east 10, north 3**.
//! - `F7` would move the ship another 7 units east (because the ship is still facing east) to
//!   **east 17, north 3**.
//! - `R90` would cause the ship to turn right by 90 degrees and face **south**; it remains at
//!   **east 17, north 3**.
//! - `F11` would move the ship 11 units south to **east 17, south 8**.
//!
//! At the end of these instructions, the ship's [Manhattan distance] (sum of the absolute values of
//! its east/west position and its north/south position) from its starting position is `17 + 8` =
//! **`25`**.
//!
//! Figure out where the navigation instructions lead. **What is the Manhattan distance between that
//! location and the ship's starting position?**
//!
//! [PA system]: https://en.wikipedia.org/wiki/Public_address_system
//! [Manhattan distance]: https://en.wikipedia.org/wiki/Manhattan_distance
//!
//! ## Part Two
//!
//! Before you can give the destination to the captain, you realize that the actual action meanings
//! were printed on the back of the instructions the whole time.
//!
//! Almost all of the actions indicate how to move a **waypoint** which is relative to the ship's
//! position:
//!
//! - Action **`N`** means to move the waypoint **north** by the given value.
//! - Action **`S`** means to move the waypoint **south** by the given value.
//! - Action **`E`** means to move the waypoint **east** by the given value.
//! - Action **`W`** means to move the waypoint **west** by the given value.
//! - Action **`L`** means to rotate the waypoint around the ship **left** (**counter-clockwise**)
//!   the given number of degrees.
//! - Action **`R`** means to rotate the waypoint around the ship **right** (**clockwise**) the
//!   given number of degrees.
//! - Action **`F`** means to move **forward** to the waypoint a number of times equal to the given
//!   value.
//!
//! The waypoint starts **10 units east and 1 unit north** relative to the ship. The waypoint is
//! relative to the ship; that is, if the ship moves, the waypoint moves with it.
//!
//! For example, using the same instructions as above:
//!
//! - `F10` moves the ship to the waypoint 10 times (a total of **100 units east and 10 units
//!   north**), leaving the ship at **east 100, north 10**. The waypoint stays 10 units east and 1
//!   unit north of the ship.
//! - `N3` moves the waypoint 3 units north to **10 units east and 4 units north of the ship**. The
//!   ship remains at **east 100, north 10**.
//! - `F7` moves the ship to the waypoint 7 times (a total of **70 units east and 28 units north**),
//!   leaving the ship at **east 170, north 38**. The waypoint stays 10 units east and 4 units north
//!   of the ship.
//! - `R90` rotates the waypoint around the ship clockwise 90 degrees, moving it to **4 units east
//!   and 10 units south of the ship**. The ship remains at **east 170, north 38**.
//! - `F11` moves the ship to the waypoint 11 times (a total of **44 units east and 110 units
//!   south**), leaving the ship at **east 214, south 72**. The waypoint stays 4 units east and 10
//!   units south of the ship.
//!
//! After these operations, the ship's Manhattan distance from its starting position is `214 + 72` =
//! **`286`**.
//!
//! Figure out where the navigation instructions actually lead. **What is the Manhattan distance
//! between that location and the ship's starting position?**

use anyhow::{bail, ensure, Result};

pub const INPUT: &str = include_str!("d12.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let movements = parse_input(input)?;

    let mut pos = (0, 0);
    let mut direction = Direction::East;

    let drive = |pos: &mut (i32, i32), d: Direction, amount: u16| {
        let amount = amount as i32;
        match d {
            Direction::North => pos.1 -= amount,
            Direction::South => pos.1 += amount,
            Direction::East => pos.0 += amount,
            Direction::West => pos.0 -= amount,
        }
    };

    for (movement, amount) in movements {
        match movement {
            Movement::Direction(d) => drive(&mut pos, d, amount),
            Movement::Turn(t) => {
                for _ in 0..amount / 90 {
                    direction = match t {
                        Turn::Left => match direction {
                            Direction::North => Direction::West,
                            Direction::West => Direction::South,
                            Direction::South => Direction::East,
                            Direction::East => Direction::North,
                        },
                        Turn::Right => match direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        },
                    };
                }
            }
            Movement::Forward => drive(&mut pos, direction, amount),
        }
    }

    Ok((pos.0.abs() + pos.1.abs()) as u32)
}

pub fn solve_part_two(input: &str) -> Result<u32> {
    let movements = parse_input(input)?;

    let mut ship_pos = (0, 0);
    let mut waypoint_pos = (10, -1);

    for (movement, amount) in movements {
        let amount = amount as i32;
        match movement {
            Movement::Direction(d) => match d {
                Direction::North => waypoint_pos.1 -= amount,
                Direction::South => waypoint_pos.1 += amount,
                Direction::East => waypoint_pos.0 += amount,
                Direction::West => waypoint_pos.0 -= amount,
            },
            Movement::Turn(t) => {
                for _ in 0..amount / 90 {
                    waypoint_pos = match t {
                        Turn::Left => (waypoint_pos.1, -waypoint_pos.0),
                        Turn::Right => (-waypoint_pos.1, waypoint_pos.0),
                    };
                }
            }
            Movement::Forward => {
                ship_pos.0 += waypoint_pos.0 * amount;
                ship_pos.1 += waypoint_pos.1 * amount;
            }
        }
    }

    Ok((ship_pos.0 + ship_pos.1) as u32)
}

fn parse_input(input: &str) -> Result<Vec<(Movement, u16)>> {
    input
        .lines()
        .map(|l| {
            ensure!(l.len() >= 2, "instruction is too short");

            let movement = match &l[..1] {
                "N" => Movement::Direction(Direction::North),
                "S" => Movement::Direction(Direction::South),
                "E" => Movement::Direction(Direction::East),
                "W" => Movement::Direction(Direction::West),
                "L" => Movement::Turn(Turn::Left),
                "R" => Movement::Turn(Turn::Right),
                "F" => Movement::Forward,
                c => bail!("unknown movement '{}'", c),
            };
            let amount = l[1..].parse()?;

            Ok((movement, amount))
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Movement {
    Direction(Direction),
    Turn(Turn),
    Forward,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        F10
        N3
        F7
        R90
        F11
    "};

    #[test]
    fn part_one() {
        assert_eq!(25, solve_part_one(INPUT).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(286, solve_part_two(INPUT).unwrap());
    }
}
