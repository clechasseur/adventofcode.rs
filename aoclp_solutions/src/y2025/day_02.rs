use std::ops::RangeInclusive;
use std::str::FromStr;

use aoclp::functional::ConsumingPredHelper;
use aoclp::num::Integer;
use aoclp::solvers_impl::input::safe_get_input_as_one_vec;
use itertools::Itertools;

pub fn part_1() -> usize {
    let pred = invalid;

    // Replace `pred` with this for the slow, regex-based approach:
    // let re = fancy_regex::Regex::new(r"^(\d+)\1$").unwrap();
    // let pred = |id: usize| re.is_match(&id.to_string()).unwrap();

    sum(pred)
}

pub fn part_2() -> usize {
    let pred = invalid_fancy;

    // Replace `pred` with this for the slow, regex-based approach:
    // let re = fancy_regex::Regex::new(r"^(\d+)\1+$").unwrap();
    // let pred = |id: usize| re.is_match(&id.to_string()).unwrap();

    sum(pred)
}

fn sum<P>(pred: P) -> usize
where
    P: Fn(usize) -> bool,
{
    input()
        .into_iter()
        .flat_map(<_>::into_iter)
        .filter(pred.with_ref())
        .sum()
}

fn num_digits(n: usize) -> usize {
    ((n as f64).log10() + 1.0).floor() as usize // hax
}

fn invalid(id: usize) -> bool {
    let num_digits = num_digits(id);
    if num_digits.is_even() {
        let midpoint = 10_usize.pow(num_digits as u32 / 2);
        return (id / midpoint) == (id % midpoint);
    }

    false
}

fn invalid_fancy(id: usize) -> bool {
    let num_digits = num_digits(id);
    (1..=num_digits / 2).any(|of_size| invalid_of_size(id, num_digits, of_size))
}

fn invalid_of_size(mut id: usize, num_digits: usize, of_size: usize) -> bool {
    if num_digits % of_size == 0 {
        let window = 10_usize.pow(of_size as u32);
        let expected = id % window;
        while id != 0 {
            let actual = id % window;
            id /= window;
            if actual != expected {
                return false;
            }
        }

        return true;
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

// noinspection DuplicatedCode
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
