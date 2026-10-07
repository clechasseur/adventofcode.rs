use std::ops::Mul;

use aoclp::aoc::Input;

use crate::y2017::helpers::knot_hash::KnotHash;

pub fn part_1() -> usize {
    KnotHash::sparse_hash(part_1_lengths(), 1)
        .into_iter()
        .map(|n| n as usize)
        .take(2)
        .reduce(Mul::mul)
        .unwrap()
}

pub fn part_2() -> String {
    KnotHash::new(input()).to_string()
}

fn part_1_lengths() -> Vec<u8> {
    input()
        .split(',')
        .map(|length| length.parse().unwrap())
        .collect()
}

fn input() -> String {
    Input::for_puzzle(2017, 10).into_string()
}
