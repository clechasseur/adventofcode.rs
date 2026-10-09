use aoclp::aoc::Input;
use itertools::Itertools;

pub fn part_1() -> usize {
    input().into_iter().filter(safe).count()
}

pub fn part_2() -> usize {
    input()
        .into_iter()
        .filter(|v| dampened(v).iter().any(safe))
        .count()
}

fn input() -> Vec<Vec<i64>> {
    Input::for_puzzle(2024, 2).split_lines_into()
}

#[allow(clippy::ptr_arg)]
fn safe(levels: &Vec<i64>) -> bool {
    let signum = (levels[0] - levels[1]).signum();
    levels.iter().tuple_windows().all(|(a, b)| {
        let new_diff = a - b;
        new_diff.signum() == signum && (1_i64..=3).contains(&new_diff.abs())
    })
}

#[allow(clippy::ptr_arg)]
fn dampened(levels: &Vec<i64>) -> Vec<Vec<i64>> {
    (0..levels.len())
        .map(|i| {
            let mut dampened = levels.clone();
            dampened.remove(i);
            dampened
        })
        .collect()
}
