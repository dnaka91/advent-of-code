//! The **Intcode Program** used in solutions for day [2], [5], [7] and [9].
//!
//! [2]: crate::y2019::d02
//! [5]: crate::y2019::d05
//! [7]: crate::y2019::d07
//! [9]: crate::y2019::d09

use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    iter::FromIterator,
};

use anyhow::{bail, Error, Result};
use thiserror::Error;

/// A list of errors that may occur during the usage of this intcode [`Program`].
#[derive(Debug, Error)]
pub enum ProgramError {
    /// Parsing error while converting an input string with [`parse_input`].
    #[error("invalid integer")]
    ParseInt(#[from] std::num::ParseIntError),
    /// An opcode couldn't be parse because it's not supported by the program.
    #[error("unknown opcode `{0}`")]
    UnknownOpcode(i64),
    /// A parameter mode couldn't be parsed because it's not supported by the program.
    #[error("unknown mode `{0}`")]
    UnknownMode(i64),
}

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
            _ => bail!(ProgramError::UnknownOpcode(value)),
        })
    }
}

impl From<Opcode> for i64 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Add => 1,
            Opcode::Mul => 2,
            Opcode::Input => 3,
            Opcode::Output => 4,
            Opcode::JumpIfTrue => 5,
            Opcode::JumpIfFalse => 6,
            Opcode::LessThan => 7,
            Opcode::Equals => 8,
            Opcode::AdjustBase => 9,
            Opcode::Exit => 99,
        }
    }
}

impl Opcode {
    fn len(self) -> usize {
        match self {
            Self::Add | Self::Mul | Self::LessThan | Self::Equals => 4,
            Self::Input | Self::Output | Self::AdjustBase => 2,
            Self::JumpIfTrue | Self::JumpIfFalse => 3,
            Self::Exit => 1,
        }
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
            _ => bail!(ProgramError::UnknownMode(value)),
        })
    }
}

/// A program that operates on a set of commands which represent Intcodes.
#[derive(Default)]
pub struct Program {
    cmds: Vec<i64>,
    params: VecDeque<i64>,
    pos: usize,
    rel_base: i64,
}

impl Program {
    /// Create a new program that operates on the provided commands. Start parameters can be given
    /// as well.
    pub fn new(cmds: Vec<i64>, start_params: &[i64]) -> Self {
        Self {
            cmds,
            params: VecDeque::from_iter(start_params.iter().cloned()),
            ..Default::default()
        }
    }

    /// Run one cycle of the program. A cycle is finished when either the position is at the end of
    /// all commands, or an **output** (`4`) or **exit** (`99`) command is reached. In general this
    /// function should be repeatedly called until [`is_finished`] returns `true`.
    ///
    /// The program can still be run even if it's already finished, but the output will always be
    /// zero.
    ///
    /// [`is_finished`]: Program::is_finished
    ///
    /// # Example
    ///
    /// ```rust
    /// use aoc::y2019::intcode::{parse_input, Program};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut program = Program::new(parse_input("3,5,4,5,99,0")?, &[1]);
    ///
    ///     assert_eq!(1, program.run(&[])?);
    ///     assert!(program.is_finished());
    ///
    ///     assert_eq!(0, program.run(&[])?); // Already finished
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn run(&mut self, params: &[i64]) -> Result<i64> {
        self.params.extend(params);

        while self.pos < self.cmds.len() {
            let (opcode, mode1, mode2, mode3) = parse_opcode(self.cmds[self.pos])?;
            match opcode {
                Opcode::Add => {
                    let x = self.get_value(1, mode1);
                    let y = self.get_value(2, mode2);
                    self.set_value(3, mode3, x + y);
                    self.pos += opcode.len();
                }
                Opcode::Mul => {
                    let x = self.get_value(1, mode1);
                    let y = self.get_value(2, mode2);
                    self.set_value(3, mode3, x * y);
                    self.pos += opcode.len();
                }
                Opcode::Input => {
                    let value = self.params.pop_front().unwrap();
                    self.set_value(1, mode1, value);
                    self.pos += opcode.len();
                }
                Opcode::Output => {
                    let value = self.get_value(1, mode1);
                    self.pos += opcode.len();
                    return Ok(value);
                }
                Opcode::JumpIfTrue => {
                    self.pos = if self.get_value(1, mode1) != 0 {
                        self.get_value(2, mode2).try_into()?
                    } else {
                        self.pos + opcode.len()
                    };
                }
                Opcode::JumpIfFalse => {
                    self.pos = if self.get_value(1, mode1) == 0 {
                        self.get_value(2, mode2).try_into()?
                    } else {
                        self.pos + opcode.len()
                    };
                }
                Opcode::LessThan => {
                    let x = self.get_value(1, mode1);
                    let y = self.get_value(2, mode2);
                    self.set_value(3, mode3, i64::from(x < y));
                    self.pos += opcode.len();
                }
                Opcode::Equals => {
                    let x = self.get_value(1, mode1);
                    let y = self.get_value(2, mode2);
                    self.set_value(3, mode3, i64::from(x == y));
                    self.pos += opcode.len();
                }
                Opcode::AdjustBase => {
                    self.rel_base += self.get_value(1, mode1);
                    self.pos += opcode.len();
                }
                Opcode::Exit => {
                    break;
                }
            }
        }
        Ok(0)
    }

    /// Check whether this program is still executable or reached the end of execution. The end is
    /// reached when either the current position is at the end of commands or the current command is
    /// the **exit** (`99`) code.
    pub fn is_finished(&self) -> bool {
        self.pos >= self.cmds.len() || self.cmds[self.pos] == i64::from(Opcode::Exit)
    }

    /// Return the current state of the commands this program runs. This will consume the program.
    pub fn cmds(self) -> Vec<i64> {
        self.cmds
    }

    fn get_address(&self, offset: usize, mode: Mode) -> usize {
        let pos = self.pos + offset;
        match mode {
            Mode::Immediate => pos,
            Mode::Position => self.cmds[pos] as usize,
            Mode::Relative => (self.cmds[pos] + self.rel_base) as usize,
        }
    }

    fn get_value(&self, offset: usize, mode: Mode) -> i64 {
        let i = self.get_address(offset, mode);
        if self.cmds.len() <= i {
            return 0;
        }

        self.cmds[i]
    }

    fn set_value(&mut self, offset: usize, mode: Mode, value: i64) {
        let pos = self.get_address(offset, mode);
        if self.cmds.len() <= pos {
            self.cmds.resize(pos + 1, 0);
        }
        self.cmds[pos] = value;
    }
}

/// Parse an input string for use by the [`Program`]. The string is expected to be a list of
/// integers separated by comma `,`.
///
/// # Errors
///
/// If the input is invalid in any way, a [`ProgramError::ParseInt`] is returned.
pub fn parse_input(input: &str) -> Result<Vec<i64>> {
    input
        .split(',')
        .map(|v| v.trim().parse::<i64>().map_err(ProgramError::from).map_err(Error::from))
        .collect()
}

fn parse_opcode(opcode: i64) -> Result<(Opcode, Mode, Mode, Mode)> {
    Ok((
        (opcode % 100).try_into()?,
        (opcode / 100 % 10).try_into()?,
        (opcode / 1000 % 10).try_into()?,
        (opcode / 10000 % 10).try_into()?,
    ))
}

#[cfg(test)]
pub(crate) mod tests {
    use itertools::Itertools;

    use super::*;

    /// Turn a slice of integers back into a comma separated string. For testing purposes.
    pub fn input_to_string(input: &[i64]) -> String {
        input.iter().map(|i| i.to_string()).join(",")
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(
            (Opcode::Mul, Mode::Position, Mode::Immediate, Mode::Position),
            parse_opcode(1002).unwrap()
        );
        assert_eq!(
            (Opcode::Add, Mode::Immediate, Mode::Immediate, Mode::Position),
            parse_opcode(1101).unwrap()
        );
    }
}
