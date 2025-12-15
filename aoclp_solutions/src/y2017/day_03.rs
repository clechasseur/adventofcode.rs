use std::collections::HashMap;
use std::iter;

use aoclp::num::{Zero, zero};
use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::direction::{Direction, MovementDirection};
use aoclp::positioning::pt::{Pt, manhattan};
use aoclp::solvers_impl::input::safe_get_input;
use itertools::Itertools;

pub fn part_1() -> i64 {
    manhattan(zero(), spiral().nth(input() - 1).unwrap())
}

pub fn part_2() -> i64 {
    let input = input();
    spiral_stress_test().find(|v| *v > (input as i64)).unwrap()
}

fn spiral() -> impl Iterator<Item = Pt> {
    let mut pt = zero();
    let mut max_moves = 1;
    let mut moves = 1;
    let mut times = 2;
    let mut direction = Direction4::Right;

    iter::repeat_with(move || {
        let this_pt = pt;

        pt += direction.displacement();
        moves -= 1;
        if moves == 0 {
            direction = direction.turn_left();
            times -= 1;
            if times == 0 {
                max_moves += 1;
                times = 2;
            }
            moves = max_moves;
        }

        this_pt
    })
}

fn spiral_stress_test() -> impl Iterator<Item = i64> {
    let mut values = HashMap::new();
    let around: Vec<_> = (-1_i64..=1)
        .cartesian_product(-1_i64..=1)
        .map_into::<Pt>()
        .filter(|pt| !pt.is_zero())
        .collect();

    spiral().map(move |pt| {
        let value = around
            .iter()
            .filter_map(|pt_mod| {
                let neighbour = pt + *pt_mod;
                values.get(&neighbour).copied()
            })
            .sum1()
            .unwrap_or(1_i64);
        values.insert(pt, value);
        value
    })
}

fn input() -> usize {
    safe_get_input(2017, 3).parse().unwrap()
}
