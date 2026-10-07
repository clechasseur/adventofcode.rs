use aoclp::aoc::Input;
use itertools::Itertools;

pub fn part_1() -> usize {
    valid_count(false)
}

pub fn part_2() -> usize {
    valid_count(true)
}

fn valid_count(hardened: bool) -> usize {
    input()
        .iter()
        .filter(|passphrase| {
            !passphrase
                .split_ascii_whitespace()
                .map(|word| match hardened {
                    true => word.chars().sorted_unstable().collect(),
                    false => word.to_string(),
                })
                .counts()
                .into_values()
                .any(|count| count > 1)
        })
        .count()
}

fn input() -> Vec<String> {
    Input::for_puzzle(2017, 4).lines_into()
}
