use std::ops::RangeInclusive;
use std::str::FromStr;

use aoclp::num::Integer;
use aoclp::solvers_impl::input::safe_get_input_as_one_vec;
use fancy_regex::Regex;
use itertools::Itertools;

pub fn part_1() -> usize {
    let re = Regex::new(r"^(\d+)\1$").unwrap();
    sum_of_invalid(&re)
}

pub fn part_2() -> usize {
    let re = Regex::new(r"^(\d+)(\1)+$").unwrap();
    sum_of_invalid(&re)
}

fn sum_of_invalid(re: &Regex) -> usize {
    input()
        .into_iter()
        .flat_map(|range| range.into_iter())
        .filter(|id| re.is_match(&id.to_string()).unwrap())
        .sum()
}

fn num_digits(n: usize) -> usize {
    ((n as f64).log10() + 1.0).floor() as usize // hax
}

fn invalid(id: usize) -> bool {
    let num_digits = num_digits(id);
    if num_digits.is_even() {
        let midpoint = 10usize.pow(num_digits as u32 / 2);
        return (id / midpoint) == (id % midpoint);
    }

    false
}

struct IdRange(RangeInclusive<usize>);

impl IntoIterator for IdRange {
    type Item = <RangeInclusive<usize> as IntoIterator>::Item;
    type IntoIter = <RangeInclusive<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

impl FromStr for IdRange {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split('-').collect_tuple().unwrap();
        Ok(Self(from.parse()?..=to.parse()?))
    }
}

fn input() -> Vec<IdRange> {
    safe_get_input_as_one_vec(2025, 2)
}
