use itertools::Itertools;
use num::abs;

use crate::input::day_01::INPUT;

pub fn part_1() -> i64 {
    list(un).zip(list(deux)).map(|(a, b)| abs(a - b)).sum()
}

pub fn part_2() -> i64 {
    let counts = list(deux).counts();
    list(un)
        .map(|n| n * (counts.get(&n).copied().unwrap_or_default() as i64))
        .sum()
}

fn list<F>(f: F) -> impl Iterator<Item = i64>
where
    F: Fn(&(i64, i64)) -> i64,
{
    INPUT.iter().map(f).sorted_unstable()
}

fn un(value: &(i64, i64)) -> i64 {
    value.0
}

fn deux(value: &(i64, i64)) -> i64 {
    value.1
}
