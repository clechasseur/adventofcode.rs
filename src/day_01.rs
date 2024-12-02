use itertools::Itertools;

use crate::helpers::input::safe_get_input_as_pairs;

pub fn part_1() -> String {
    list(un)
        .zip(list(deux))
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
        .to_string()
}

pub fn part_2() -> String {
    let counts = list(deux).counts();
    list(un)
        .map(|n| n * (counts.get(&n).copied().unwrap_or_default() as i64))
        .sum::<i64>()
        .to_string()
}

fn list<F>(f: F) -> impl Iterator<Item = i64>
where
    F: Fn((i64, i64)) -> i64,
{
    safe_get_input_as_pairs(2024, 1)
        .into_iter()
        .map(f)
        .sorted_unstable()
}

fn un(value: (i64, i64)) -> i64 {
    value.0
}

fn deux(value: (i64, i64)) -> i64 {
    value.1
}
