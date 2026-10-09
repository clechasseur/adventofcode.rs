use aoclp::aoc::Input;
use itertools::Itertools;

pub fn part_1() -> i32 {
    input()
        .iter()
        .map(|line| {
            let (min, max) = line.iter().minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

pub fn part_2() -> i32 {
    input()
        .iter()
        .map(|line| {
            line.iter()
                .combinations(2)
                .filter_map(|c| {
                    let (&min, &max) = c.iter().minmax().into_option().unwrap();
                    (max % min == 0).then_some(max / min)
                })
                .exactly_one()
                .unwrap()
        })
        .sum()
}

fn input() -> Vec<Vec<i32>> {
    Input::for_puzzle(2017, 2).split_lines_into()
}
