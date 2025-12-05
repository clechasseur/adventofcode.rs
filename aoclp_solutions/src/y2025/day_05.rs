use std::cmp::{max, min};
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

use aoclp::functional::ByRefPredHelper;
use aoclp::solvers_impl::input::safe_get_input_as_many_of_two_types;
use itertools::Itertools;

pub fn part_1() -> usize {
    let (fresh_ids, available_ids) = input();
    available_ids
        .into_iter()
        .filter(|&id| fresh_ids.iter().any(|r| r.contains(&id)))
        .count()
}

pub fn part_2() -> usize {
    let (fresh_ids, _) = input();
    fresh_ids
        .into_iter()
        .sorted_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())))
        .fold(Vec::new(), |mut acc, r| {
            match acc.last() {
                None => acc.push(r),
                Some(prev) if (prev.end() + 1) < *r.start() => acc.push(r),
                Some(prev) => {
                    let from = *min(prev.start(), r.start());
                    let to = *max(prev.end(), r.end());
                    acc.pop();
                    acc.push(IdRange(from..=to));
                },
            }

            acc
        })
        .into_iter()
        .map(IdRange::len.without_ref())
        .sum()
}

#[derive(Debug, Clone)]
struct IdRange(RangeInclusive<usize>);

impl IdRange {
    pub fn len(&self) -> usize {
        self.end() - self.start() + 1
    }
}

// noinspection DuplicatedCode
impl FromStr for IdRange {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split('-').collect_tuple().unwrap();
        Ok(Self(from.parse()?..=to.parse()?))
    }
}

impl Deref for IdRange {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn input() -> (Vec<IdRange>, Vec<usize>) {
    safe_get_input_as_many_of_two_types(2025, 5)
}
