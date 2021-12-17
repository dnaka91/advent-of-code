//! # Day 12: Passage Pathing
//!
//! With your submarine's subterranean subsystems subsisting suboptimally, the only way you're
//! getting out of this cave anytime soon is by finding a path yourself. Not just **a** path - the
//! only way to know if you've found the **best** path is to find **all** of them.
//!
//! Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining
//! caves (your puzzle input). For example:
//!
//! ```txt
//! start-A
//! start-b
//! A-c
//! A-b
//! b-d
//! A-end
//! b-end
//! ```
//!
//! This is a list of how all of the caves are connected. You start in the cave named `start`, and
//! your destination is the cave named `end`. An entry like `b-d` means that cave `b` is connected
//! to cave `d` - that is, you can move between them.
//!
//! So, the above cave system looks roughly like this:
//!
//! ```txt
//!     start
//!     /   \
//! c--A-----b--d
//!     \   /
//!      end
//! ```
//!
//! Your goal is to find the number of distinct **paths** that start at `start`, end at `end`, and
//! don't visit small caves more than once. There are two types of caves: **big** caves (written in
//! uppercase, like `A`) and **small** caves (written in lowercase, like `b`). It would be a waste
//! of time to visit any small cave more than once, but big caves are large enough that it might be
//! worth visiting them multiple times. So, all paths you find should **visit small caves at most
//! once**, and can **visit big caves any number of times**.
//!
//! Given these rules, there are **`10`** paths through this example cave system:
//!
//! ```txt
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,end
//! ```
//!
//! (Each line in the above list corresponds to a single path; the caves visited by that path are
//! listed in the order they are visited and separated by commas.)
//!
//! Note that in this cave system, cave `d` is never visited by any path: to do so, cave `b` would
//! need to be visited twice (once on the way to cave `d` and a second time when returning from cave
//! `d`), and since cave `b` is small, this is not allowed.
//!
//! Here is a slightly larger example:
//!
//! ```txt
//! dc-end
//! HN-start
//! start-kj
//! dc-start
//! dc-HN
//! LN-dc
//! HN-end
//! kj-sa
//! kj-HN
//! kj-dc
//! ```
//!
//! The `19` paths through it are as follows:
//!
//! ```txt
//! start,HN,dc,HN,end
//! start,HN,dc,HN,kj,HN,end
//! start,HN,dc,end
//! start,HN,dc,kj,HN,end
//! start,HN,end
//! start,HN,kj,HN,dc,HN,end
//! start,HN,kj,HN,dc,end
//! start,HN,kj,HN,end
//! start,HN,kj,dc,HN,end
//! start,HN,kj,dc,end
//! start,dc,HN,end
//! start,dc,HN,kj,HN,end
//! start,dc,end
//! start,dc,kj,HN,end
//! start,kj,HN,dc,HN,end
//! start,kj,HN,dc,end
//! start,kj,HN,end
//! start,kj,dc,HN,end
//! start,kj,dc,end
//! ```
//!
//! Finally, this even larger example has `226` paths through it:
//!
//! ```txt
//! fs-end
//! he-DX
//! fs-he
//! start-DX
//! pj-DX
//! end-zg
//! zg-sl
//! zg-pj
//! pj-he
//! RW-he
//! fs-DX
//! pj-RW
//! zg-RW
//! start-pj
//! he-WI
//! zg-he
//! pj-fs
//! start-RW
//! ```
//!
//! **How many paths through this cave system are there that visit small caves at most once?**
//!
//! ## Part Two
//!
//! After reviewing the available paths, you realize you might have time to visit a single small
//! cave **twice**. Specifically, big caves can be visited any number of times, a single small cave
//! can be visited at most twice, and the remaining small caves can be visited at most once.
//! However, the caves named `start` and `end` can only be visited **exactly once each**: once you
//! leave the `start` cave, you may not return to it, and once you reach the `end` cave, the path
//! must end immediately.
//!
//! Now, the `36` possible paths through the first example above are:
//!
//! ```txt
//! start,A,b,A,b,A,c,A,end
//! start,A,b,A,b,A,end
//! start,A,b,A,b,end
//! start,A,b,A,c,A,b,A,end
//! start,A,b,A,c,A,b,end
//! start,A,b,A,c,A,c,A,end
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,d,b,A,c,A,end
//! start,A,b,d,b,A,end
//! start,A,b,d,b,end
//! start,A,b,end
//! start,A,c,A,b,A,b,A,end
//! start,A,c,A,b,A,b,end
//! start,A,c,A,b,A,c,A,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,d,b,A,end
//! start,A,c,A,b,d,b,end
//! start,A,c,A,b,end
//! start,A,c,A,c,A,b,A,end
//! start,A,c,A,c,A,b,end
//! start,A,c,A,c,A,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,b,A,c,A,end
//! start,b,A,b,A,end
//! start,b,A,b,end
//! start,b,A,c,A,b,A,end
//! start,b,A,c,A,b,end
//! start,b,A,c,A,c,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,d,b,A,c,A,end
//! start,b,d,b,A,end
//! start,b,d,b,end
//! start,b,end
//! ```
//!
//! The slightly larger example above now has `103` paths through it, and the even larger example
//! now has `3509` paths through it.
//!
//! Given these new rules, **how many paths through this cave system are there?**

use ahash::{AHashMap, AHashSet};
use anyhow::{bail, Context, Result};

pub const INPUT: &str = include_str!("d12.txt");

pub fn solve_part_one(input: &str) -> Result<u64> {
    let routes = parse_input(input)?;
    Ok(traverse_one(&routes, Cave::Start, &mut Vec::new()))
}

pub fn solve_part_two(input: &str) -> Result<u64> {
    let routes = parse_input(input)?;
    Ok(traverse_two(&routes, Cave::Start, &mut Vec::new()))
}

fn parse_input(input: &str) -> Result<AHashMap<Cave, Vec<Cave>>> {
    input
        .lines()
        .map(|route| {
            let (from, to) = route.split_once('-').context("missing route delimiter")?;
            Ok((from.try_into()?, to.try_into()?))
        })
        .try_fold(AHashMap::<Cave, Vec<Cave>>::new(), |mut acc, route: Result<(Cave, Cave)>| {
            let (from, to) = route?;
            acc.entry(from).or_default().push(to);
            acc.entry(to).or_default().push(from);
            Ok(acc)
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Big(&'a str),
}

impl<'a> TryFrom<&'a str> for Cave<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ if value.chars().all(|c| c.is_ascii_lowercase()) => Cave::Small(value),
            _ if value.chars().all(|c| c.is_ascii_uppercase()) => Cave::Big(value),
            _ => bail!("invalid cave `{}`", value),
        })
    }
}

fn traverse_one<'a>(
    routes: &AHashMap<Cave<'a>, Vec<Cave<'a>>>,
    cave: Cave<'a>,
    path: &mut Vec<Cave<'a>>,
) -> u64 {
    match cave {
        Cave::End => return 1,
        Cave::Start if !path.is_empty() => return 0,
        Cave::Small(_) if path.contains(&cave) => return 0,
        _ => (),
    }

    path.push(cave);

    let count = routes
        .get(&cave)
        .map(|caves| caves.iter().map(|&cave| traverse_one(routes, cave, path)).sum())
        .unwrap_or_default();

    path.pop();
    count
}

fn traverse_two<'a>(
    routes: &AHashMap<Cave<'a>, Vec<Cave<'a>>>,
    cave: Cave<'a>,
    path: &mut Vec<Cave<'a>>,
) -> u64 {
    let traverser = match cave {
        Cave::End => return 1,
        Cave::Start if !path.is_empty() => return 0,
        Cave::Small(_) if path.contains(&cave) => traverse_one,
        _ => traverse_two,
    };

    path.push(cave);

    let count = routes
        .get(&cave)
        .map(|caves| caves.iter().map(|&cave| traverser(routes, cave, path)).sum())
        .unwrap_or_default();

    path.pop();
    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT_1: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const INPUT_2: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    const INPUT_3: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    #[test]
    fn part_one() {
        assert_eq!(10, solve_part_one(INPUT_1).unwrap());
        assert_eq!(19, solve_part_one(INPUT_2).unwrap());
        assert_eq!(226, solve_part_one(INPUT_3).unwrap());
    }

    #[test]
    fn part_two() {
        assert_eq!(36, solve_part_two(INPUT_1).unwrap());
        assert_eq!(103, solve_part_two(INPUT_2).unwrap());
        assert_eq!(3509, solve_part_two(INPUT_3).unwrap());
    }
}
