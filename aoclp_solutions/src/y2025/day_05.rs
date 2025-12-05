use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::str::FromStr;

use aoclp::solvers_impl::input::safe_get_input_as_many_vecs_of_two_types;
use itertools::Itertools;

pub fn part_1() -> usize {
    let (fresh_ids, available_ids) = input();
    available_ids
        .into_iter()
        .filter(|&id| fresh_ids.iter().any(|r| r.0.contains(&id)))
        .count()
}

pub fn part_2() -> usize {
    let (fresh_ids, _) = input();
    fresh_ids
        .into_iter()
        .map(|r| r.0)
        .sorted_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())))
        .fold(Vec::new(), |mut acc, r| {
            match acc.last() {
                None => acc.push(r),
                Some(prev) if prev.end() < r.start() => acc.push(r),
                Some(prev) => {
                    let from = *min(prev.start(), r.start());
                    let to = *max(prev.end(), r.end());
                    acc.pop();
                    acc.push(from..=to);
                },
            }

            acc
        })
        .into_iter()
        .map(|r| r.count())
        .sum()
}

#[derive(Debug, Clone)]
struct IdRange(RangeInclusive<usize>);

impl FromStr for IdRange {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split('-').collect_tuple().unwrap();
        Ok(Self(from.parse()?..=to.parse()?))
    }
}

fn input() -> (Vec<IdRange>, Vec<usize>) {
    let (fresh_ids, available_ids) = safe_get_input_as_many_vecs_of_two_types(2025, 5);
    (
        fresh_ids
            .into_iter()
            .map(|v| v.into_iter().next().unwrap())
            .collect_vec(),
        available_ids
            .into_iter()
            .map(|v| v.into_iter().next().unwrap())
            .collect_vec(),
    )
}
