//! # Day 9: Sensor Boost
//!
//! You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress
//! signal coming from the asteroid belt. It must be the Ceres monitoring station!
//!
//! In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the
//! latest **BOOST** program - Basic Operation Of System Test.
//!
//! While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons,
//! it refuses to do so until the computer it runs on passes some checks to demonstrate it is a
//! **complete Intcode computer**.
//!
//! [Your existing Intcode computer] is missing one key feature: it needs support for parameters in
//! **relative mode**.
//!
//! Parameters in mode `2`, **relative mode**, behave very similarly to parameters in **position
//! mode**: the parameter is interpreted as a position. Like position mode, parameters in relative
//! mode can be read from or written to.
//!
//! The important difference is that relative mode parameters don't count from address `0`. Instead,
//! they count from a value called the **relative base**. The **relative base** starts at `0`.
//!
//! The address a relative mode parameter refers to is itself **plus** the current **relative
//! base**. When the relative base is `0`, relative mode parameters and position mode parameters
//! with the same value refer to the same address.
//!
//! For example, given a relative base of `50`, a relative mode parameter of `-7` refers to memory
//! address `50 + -7 = 43`.
//!
//! The relative base is modified with the **relative base offset** instruction:
//!
//! - Opcode `9` **adjusts the relative base** by the value of its only parameter. The relative base
//!   increases (or decreases, if the value is negative) by the value of the parameter.
//!
//! For example, if the relative base is `2000`, then after the instruction `109,19`, the relative
//! base would be `2019`. If the next instruction were `204,-34`, then the value at address `1985`
//! would be output.
//!
//! Your Intcode computer will also need a few other capabilities:
//!
//! - The computer's available memory should be much larger than the initial program. Memory beyond
//!   the initial program starts with the value `0` and can be read or written like any other
//!   memory. (It is invalid to try to access memory at a negative address, though.)
//! - The computer should have support for large numbers. Some instructions near the beginning of
//!   the BOOST program will verify this capability.
//!
//! Here are some example programs that use these features:
//!
//! - `109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99` takes no input and produces a
//!   [copy of itself] as output.
//! - `1102,34915192,34915192,7,4,7,99,0` should output a 16-digit number.
//! - `104,1125899906842624,99` should output the large number in the middle.
//!
//! The BOOST program will ask for a single input; run it in test mode by providing it the value
//! `1`. It will perform a series of checks on each opcode, output any opcodes (and the associated
//! parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
//!
//! Once your Intcode computer is fully functional, the BOOST program should report no
//! malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST
//! keycode. **What BOOST keycode does it produce?**
//!
//! [Your existing Intcode computer]: https://adventofcode.com/2019/day/5
//! [copy of itself]: https://en.wikipedia.org/wiki/Quine_(computing)
//!
//! ## Part Two
//!
//! **You now have a complete Intcode computer.**
//!
//! Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors using
//! the BOOST program.
//!
//! The program runs in sensor boost mode by providing the input instruction the value `2`. Once
//! run, it will boost the sensors automatically, but it might take a few seconds to complete the
//! operation on slower hardware. In sensor boost mode, the program will output a single value:
//! **the coordinates of the distress signal**.
//!
//! Run the BOOST program in sensor boost mode. **What are the coordinates of the distress signal?**

use std::convert::{TryFrom, TryInto};

use anyhow::bail;
use anyhow::Result;

pub const INPUT: &str = include_str!("d09.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustBase,
    Exit,
}

impl TryFrom<i64> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            9 => Self::AdjustBase,
            99 => Self::Exit,
            _ => bail!("Unkown opcode {}", value),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => bail!("Unkown mode {}", value),
        })
    }
}

pub fn solve_part_one(input: &str) -> Result<i64> {
    let mut cmds = parse_input(input);

    Ok(process(&mut cmds, 1)?)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    let mut cmds = parse_input(input);

    Ok(process(&mut cmds, 2)?)
}

fn process(cmds: &mut Vec<i64>, memory: i64) -> Result<i64> {
    let mut i = 0;
    let mut rel_base = 0;
    let mut memory = memory;

    while i < cmds.len() {
        let (opcode, mode1, mode2, mode3) = parse_opcode(cmds[i])?;
        match opcode {
            Opcode::Add => {
                let x = get_value(cmds, i + 1, rel_base, mode1);
                let y = get_value(cmds, i + 2, rel_base, mode2);
                let out = get_address(cmds, i + 3, rel_base, mode3);
                set_value(cmds, out, x + y);
                i += 4;
            }
            Opcode::Mul => {
                let x = get_value(cmds, i + 1, rel_base, mode1);
                let y = get_value(cmds, i + 2, rel_base, mode2);
                let out = get_address(cmds, i + 3, rel_base, mode3);
                set_value(cmds, out, x * y);
                i += 4;
            }
            Opcode::Input => {
                let out = get_address(cmds, i + 1, rel_base, mode1);
                set_value(cmds, out, memory);
                i += 2
            }
            Opcode::Output => {
                memory = get_value(cmds, i + 1, rel_base, mode1);
                i += 2;
            }
            Opcode::JumpIfTrue => {
                if get_value(cmds, i + 1, rel_base, mode1) != 0 {
                    i = get_value(cmds, i + 2, rel_base, mode2) as usize;
                } else {
                    i += 3;
                }
            }
            Opcode::JumpIfFalse => {
                if get_value(cmds, i + 1, rel_base, mode1) == 0 {
                    i = get_value(cmds, i + 2, rel_base, mode2) as usize;
                } else {
                    i += 3;
                }
            }
            Opcode::LessThan => {
                let x = get_value(cmds, i + 1, rel_base, mode1);
                let y = get_value(cmds, i + 2, rel_base, mode2);
                let out = get_address(cmds, i + 3, rel_base, mode3);
                set_value(cmds, out, if x < y { 1 } else { 0 });
                i += 4;
            }
            Opcode::Equals => {
                let x = get_value(cmds, i + 1, rel_base, mode1);
                let y = get_value(cmds, i + 2, rel_base, mode2);
                let out = get_address(cmds, i + 3, rel_base, mode3);
                set_value(cmds, out, if x == y { 1 } else { 0 });
                i += 4;
            }
            Opcode::AdjustBase => {
                let delta = get_value(cmds, i + 1, rel_base, mode1);
                rel_base += delta;
                i += 2;
            }
            Opcode::Exit => break,
        }
        if cmds.len() <= i {
            cmds.resize(i + 4, 0);
        }
    }

    Ok(memory)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split(',').map(|v| v.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn parse_opcode(opcode: i64) -> Result<(Opcode, Mode, Mode, Mode)> {
    Ok((
        (opcode % 100).try_into()?,
        (opcode / 100 % 10).try_into()?,
        (opcode / 1000 % 10).try_into()?,
        (opcode / 10000 % 10).try_into()?,
    ))
}

fn get_address(cmds: &[i64], pos: usize, rel_base: i64, mode: Mode) -> usize {
    match mode {
        Mode::Immediate => pos,
        Mode::Position => cmds[pos] as usize,
        Mode::Relative => (cmds[pos] + rel_base) as usize,
    }
}

fn get_value(cmds: &mut Vec<i64>, pos: usize, rel_base: i64, mode: Mode) -> i64 {
    let i = get_address(cmds, pos, rel_base, mode);
    if cmds.len() <= i {
        cmds.resize(i + 1, 0);
    }

    cmds[i]
}

fn set_value(cmds: &mut Vec<i64>, pos: usize, value: i64) {
    if cmds.len() <= pos {
        cmds.resize(pos + 1, 0);
    }
    cmds[pos] = value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut input =
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        assert_eq!(99, process(&mut input, 1).unwrap());

        let mut input = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        assert_eq!(1_219_070_632_396_864, process(&mut input, 1).unwrap());

        let mut input = vec![104, 1_125_899_906_842_624, 99];
        assert_eq!(1_125_899_906_842_624, process(&mut input, 1).unwrap());
    }

    #[test]
    fn part_two() {}
}
