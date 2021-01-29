//! # Day 18: Operation Order
//!
//! As you look out the window and notice a heavily-forested continent slowly appear over the
//! horizon, you are interrupted by the child sitting next to you. They're curious if you could help
//! them with their math homework.
//!
//! Unfortunately, it seems like this "math" [follows different rules] than you remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that consist of addition
//! (`+`), multiplication (`*`), and parentheses (`(...)`). Just like normal math, parentheses
//! indicate that the expression inside must be evaluated before it can be used by the surrounding
//! expression. Addition still finds the sum of the numbers on both sides of the operator, and
//! multiplication still finds the product.
//!
//! However, the rules of **operator precedence** have changed. Rather than evaluating
//! multiplication before addition, the operators have the **same precedence**, and are evaluated
//! left-to-right regardless of the order in which they appear.
//!
//! For example, the steps to evaluate the expression `1 + 2 * 3 + 4 * 5 + 6` are as follows:
//!
//! ```txt
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! ```
//!
//! Parentheses can override this order; for example, here is what happens if parentheses are added
//! to form `1 + (2 * 3) + (4 * (5 + 6))`:
//!
//! ```txt
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! ```
//!
//! Here are a few more examples:
//!
//! - `2 * 3 + (4 * 5)` becomes **`26`**.
//! - `5 + (8 * 3 + 9 + 3 * 4 * 3)` becomes **`437`**.
//! - `5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))` becomes **`12240`**.
//! - `((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2` becomes **`13632`**.
//!
//! Before you can help with the homework, you need to understand it yourself. **Evaluate the
//! expression on each line of the homework; what is the sum of the resulting values?**
//!
//! [follows different rules]: https://www.youtube.com/watch?v=3QtRK7Y2pPU&t=15
//!
//! ## Part Two
//!
//! You manage to answer the child's questions and they finish part 1 of their homework, but get
//! stuck when they reach the next section: **advanced** math.
//!
//! Now, addition and multiplication have **different** precedence levels, but they're not the ones
//! you're familiar with. Instead, addition is evaluated **before** multiplication.
//!
//! For example, the steps to evaluate the expression `1 + 2 * 3 + 4 * 5 + 6` are now as follows:
//!
//! ```txt
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!   3   *   7   * 5 + 6
//!   3   *   7   *  11
//!      21       *  11
//!          231
//! ```
//!
//! Here are the other examples from above:
//!
//! - `1 + (2 * 3) + (4 * (5 + 6))` still becomes **`51`**.
//! - `2 * 3 + (4 * 5)` becomes **`46`**.
//! - `5 + (8 * 3 + 9 + 3 * 4 * 3)` becomes **`1445`**.
//! - `5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))` becomes **`669060`**.
//! - `((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2` becomes **`23340`**.
//!
//! **What do you get if you add up the results of evaluating the homework problems using these new
//! rules?**

use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use anyhow::{bail, ensure, Result};

pub const INPUT: &str = include_str!("d18.txt");

pub fn solve_part_one(input: &str) -> Result<i64> {
    let instructions = parse_input(input)?;
    let mut sum = 0;

    for inst in instructions {
        let (_, n) = eval(&inst, false)?;
        sum += n;
    }

    Ok(sum)
}

pub fn solve_part_two(input: &str) -> Result<i64> {
    // TODO: This still doesn't work, so the output will be wrong
    let instructions = parse_input(input)?;
    let mut sum = 0;

    for inst in instructions {
        let (_, n) = eval(&inst, true)?;
        sum += n;
    }

    Ok(sum)
}

fn parse_input(input: &str) -> Result<Vec<Vec<V>>> {
    input
        .lines()
        .map(|l| {
            let mut num = String::new();
            let mut vs = Vec::new();

            l.chars().filter(|&c| c != ' ').try_for_each(|c| {
                match c {
                    '0'..='9' => num.push(c),
                    _ => {
                        let v = match c {
                            '+' => V::Add,
                            '*' => V::Mul,
                            '(' => V::Open,
                            ')' => V::Close,
                            _ => bail!("invalid instruction"),
                        };
                        if !num.is_empty() {
                            vs.push(V::Number(num.parse()?));
                        }
                        vs.push(v);
                        num.clear();
                    }
                }

                Ok(())
            })?;

            if !num.is_empty() {
                vs.push(V::Number(num.parse()?));
            }

            Ok(vs)
        })
        .collect()
}

#[derive(Clone, Copy)]
enum V {
    Number(i64),
    Add,
    Mul,
    Open,
    Close,
}

fn eval(inst: &[V], two: bool) -> Result<(usize, i64)> {
    ensure!(!inst.is_empty(), "at least 1 instruction needed");

    let (mut pos, mut current) = match inst[0] {
        V::Number(n) => (1, n),
        V::Open => eval(&inst[1..], two).map(|(p, n)| (p + 1, n))?,
        v => bail!("invalid starting instruction {:?}", v),
    };

    loop {
        match (inst.get(pos), inst.get(pos + 1)) {
            (Some(V::Add), Some(v)) => {
                current = match v {
                    V::Number(n) => current + *n,
                    V::Open => {
                        let (p, v) = eval(&inst[pos + 2..], two)?;
                        pos += p;
                        current + v
                    }
                    v => bail!("invalid instruction {:?}", v),
                }
            }
            (Some(V::Mul), Some(v)) => {
                current = match v {
                    V::Number(n) => {
                        if two && !matches!(inst.get(pos + 2), Some(V::Close)) {
                            let (p, v) = eval(&inst[pos + 1..], two)?;
                            pos += p;
                            current * v
                        } else {
                            current * *n
                        }
                    }
                    V::Open => {
                        let (p, mut v) = eval(&inst[pos + 2..], two)?;
                        pos += p;
                        while two && matches!(inst.get(pos), Some(V::Add)) {
                            let (p, v2) = eval(&inst[pos + 1..], two)?;
                            pos += p - 1;
                            v += v2;
                        }
                        current * v
                    }
                    v => bail!("invalid instruction {:?}", v),
                }
            }
            (Some(V::Close), _) => return Ok((pos + 1, current)),
            _ => return Ok((pos + 1, current)),
        }
        pos += 2;
    }
}

impl Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Add => f.write_str("+"),
            Self::Mul => f.write_str("*"),
            Self::Open => f.write_str("("),
            Self::Close => f.write_str(")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(26, solve_part_one("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(437, solve_part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(12240, solve_part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
        assert_eq!(
            13632,
            solve_part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap()
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(51, solve_part_two("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
        assert_eq!(46, solve_part_two("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(1445, solve_part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(669_060, solve_part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
        assert_eq!(
            23340,
            solve_part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap()
        );
    }
}
