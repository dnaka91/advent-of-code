//! # Day 16: Ticket Translation
//!
//! As you're walking to yet another connecting flight, you realize that one of the legs of your
//! re-routed trip coming up is on a high-speed train. However, the train ticket you were given is
//! in a language you don't understand. You should probably figure out what it says before you get
//! to the train station after the next flight.
//!
//! Unfortunately, you can't actually **read** the words on the ticket. You can, however, read the
//! numbers, and so you figure out **the fields these tickets must have** and **the valid ranges**
//! for values in those fields.
//!
//! You collect the **rules for ticket fields**, the **numbers on your ticket**, and the **numbers
//! on other nearby tickets** for the same train service (via the airport security cameras) together
//! into a single document you can reference (your puzzle input).
//!
//! The **rules for ticket fields** specify a list of fields that exist **somewhere** on the ticket
//! and the **valid ranges of values** for each field. For example, a rule like `class: 1-3 or 5-7`
//! means that one of the fields in every ticket is named `class` and can be any value in the ranges
//! `1-3` or `5-7` (inclusive, such that `3` and `5` are both valid in this field, but `4` is not).
//!
//! Each ticket is represented by a single line of comma-separated values. The values are the
//! numbers on the ticket in the order they appear; every ticket has the same format. For example,
//! consider this ticket:
//!
//! ```txt
//! .--------------------------------------------------------.
//! | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
//! |                                                        |
//! | ??: 301  ??: 302             ???????: 303      ??????? |
//! | ??: 401  ??: 402           ???? ????: 403    ????????? |
//! '--------------------------------------------------------'
//! ```
//!
//! Here, `?` represents text in a language you don't understand. This ticket might be represented
//! as `101,102,103,104,301,302,303,401,402,403`; of course, the actual train tickets you're looking
//! at are **much** more complicated. In any case, you've extracted just the numbers in such a way
//! that the first number is always the same specific field, the second number is always a different
//! specific field, and so on - you just don't know what each position actually means!
//!
//! Start by determining which tickets are **completely invalid**; these are tickets that contain
//! values which **aren't valid for any field**. Ignore **your ticket** for now.
//!
//! For example, suppose you have the following notes:
//!
//! ```txt
//! class: 1-3 or 5-7
//! row: 6-11 or 33-44
//! seat: 13-40 or 45-50
//!
//! your ticket:
//! 7,1,14
//!
//! nearby tickets:
//! 7,3,47
//! 40,4,50
//! 55,2,20
//! 38,6,12
//! ```
//!
//! It doesn't matter which position corresponds to which field; you can identify invalid **nearby
//! tickets** by considering only whether tickets contain **values that are not valid for any
//! field**. In this example, the values on the first **nearby ticket** are all valid for at least
//! one field. This is not true of the other three **nearby tickets**: the values `4`, `55`, and
//! `12` are are not valid for any field. Adding together all of the invalid values produces your
//! **ticket scanning error rate**: `4 + 55 + 12` = **`71`**.
//!
//! Consider the validity of the **nearby tickets** you scanned. **What is your ticket scanning
//! error rate?**
//!
//! ## Part Two
//!
//! Now that you've identified which tickets contain invalid values, **discard those tickets
//! entirely**. Use the remaining valid tickets to determine which field is which.
//!
//! Using the valid ranges for each field, determine what order the fields appear on the tickets.
//! The order is consistent between all tickets: if `seat` is the third field, it is the third field
//! on every ticket, including **your ticket**.
//!
//! For example, suppose you have the following notes:
//!
//! ```txt
//! class: 0-1 or 4-19
//! row: 0-5 or 8-19
//! seat: 0-13 or 16-19
//!
//! your ticket:
//! 11,12,13
//!
//! nearby tickets:
//! 3,9,18
//! 15,1,5
//! 5,14,9
//! ```
//!
//! Based on the **nearby tickets** in the above example, the first position must be `row`, the
//! second position must be `class`, and the third position must be `seat`; you can conclude that in
//! **your ticket**, `class` is `12`, `row` is `11`, and `seat` is `13`.
//!
//! Once you work out which field is which, look for the six fields on **your ticket** that start
//! with the word `departure`. **What do you get if you multiply those six values together?**

use std::ops::RangeInclusive;

use anyhow::{ensure, Context, Result};

pub const INPUT: &str = include_str!("d16.txt");

pub fn solve_part_one(input: &str) -> Result<u32> {
    let Input { rules, nearby_tickets, .. } = parse_input(input)?;

    Ok(nearby_tickets
        .into_iter()
        .flat_map(|ticket| {
            ticket.into_iter().filter(|value| {
                !rules.iter().any(|rule| rule.range1.contains(value) || rule.range2.contains(value))
            })
        })
        .sum())
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let Input { rules, my_ticket, mut nearby_tickets } = parse_input(input)?;
    let mut available_pos = (0..my_ticket.len()).fold(0usize, |set, pos| set | 1 << pos);

    nearby_tickets.retain(|ticket| {
        ticket.iter().all(|value| {
            rules.iter().any(|rule| rule.range1.contains(value) || rule.range2.contains(value))
        })
    });

    let mut rules = rules
        .into_iter()
        .map(|rule| {
            let pos = nearby_tickets.iter().fold(available_pos, |pos, ticket| {
                let valid_pos = ticket
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| rule.range1.contains(value) || rule.range2.contains(value))
                    .fold(0, |set, pos| set | 1 << pos.0);

                pos & valid_pos
            });

            Ok((pos, rule))
        })
        .collect::<Result<Vec<_>>>()?;

    rules.sort_unstable_by(|a, b| a.0.count_ones().cmp(&b.0.count_ones()));

    for (pos, _) in &mut rules {
        *pos &= available_pos;
        available_pos &= !*pos;
        ensure!(pos.count_ones() == 1, "found none or more than one positions");
    }

    Ok(rules
        .into_iter()
        .filter(|(_, rule)| rule.field.starts_with("departure"))
        .map(|(pos, _)| my_ticket[pos.trailing_zeros() as usize] as u64)
        .product())
}

fn parse_input(input: &str) -> Result<Input> {
    let mut parts = input.splitn(3, "\n\n");
    let rules = parts.next().context("rules missing")?;
    let my_ticket = parts.next().context("my ticket missing")?;
    let nearby_tickets = parts.next().context("nearby tickets missing")?;

    let rules = rules
        .lines()
        .map(|l| {
            let mut parts = l.splitn(2, ": ");
            let field = parts.next().context("rule name missing")?;

            let mut parts = parts.next().context("value ranges missing")?.splitn(2, " or ");
            let range1 = parts.next().context("first range missing")?;
            let range2 = parts.next().context("second range missing")?;

            let parse_range = |range: &str| -> Result<RangeInclusive<u32>> {
                let mut parts = range.splitn(2, '-');
                let start = parts.next().context("range start missing")?.parse()?;
                let end = parts.next().context("range end missing")?.parse()?;
                ensure!(start < end, "range start is bigger or equal to range end");

                Ok(start..=end)
            };

            let range1 = parse_range(range1)?;
            let range2 = parse_range(range2)?;

            Ok(Rule { field, range1, range2 })
        })
        .collect::<Result<_>>()?;

    ensure!(my_ticket.starts_with("your ticket:\n"), "invalid 'your ticket' section");

    let my_ticket = my_ticket
        .lines()
        .nth(1)
        .context("my ticket values missing")?
        .split(',')
        .map(|v| v.parse::<u32>().map_err(Into::into))
        .collect::<Result<_>>()?;

    ensure!(nearby_tickets.starts_with("nearby tickets:\n"), "invalid 'nearby tickets' section");

    let nearby_tickets = nearby_tickets
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|v| v.parse::<u32>().map_err(Into::into)).collect())
        .collect::<Result<_>>()?;

    Ok(Input { rules, my_ticket, nearby_tickets })
}

struct Input<'a> {
    rules: Vec<Rule<'a>>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct Rule<'a> {
    field: &'a str,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part_one() {
        let input = indoc! {"
            class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12
        "};

        assert_eq!(71, solve_part_one(input).unwrap());
    }

    #[test]
    fn part_two() {
        let input = indoc! {"
            class: 0-1 or 4-19
            row: 0-5 or 8-19
            seat: 0-13 or 16-19

            your ticket:
            11,12,13

            nearby tickets:
            3,9,18
            15,1,5
            5,14,9
        "};

        assert_eq!(1, solve_part_two(input).unwrap());
    }
}
