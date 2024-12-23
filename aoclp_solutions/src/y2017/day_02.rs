use aoclp::solvers_impl::input::safe_get_input_as_many_vecs;
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
    safe_get_input_as_many_vecs(2017, 2)
}
