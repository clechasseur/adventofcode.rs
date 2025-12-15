use std::collections::HashMap;

use aoclp::num::zero;
use aoclp::positioning::pt_3d::{Pt3d, euclidian_squared};
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> usize {
    circuits(false)
        .1
        .values()
        .sorted_unstable_by(|a, b| a.cmp(b).reverse())
        .take(3)
        .product()
}

pub fn part_2() -> i64 {
    let (_, _, (a, b)) = circuits(true);
    a.x * b.x
}

fn circuits(all: bool) -> (HashMap<Pt3d, usize>, HashMap<usize, usize>, (Pt3d, Pt3d)) {
    let boxes = input();

    let mut circuit_id = 0usize;
    let mut circuits = HashMap::new();
    let mut circuit_sizes = HashMap::new();
    let mut last_pair = (zero(), zero());
    boxes
        .into_iter()
        .array_combinations()
        .map(|[a, b]| (a, b, euclidian_squared(a, b)))
        .sorted_unstable_by_key(|(_, _, d)| *d)
        .take(if all { 1_000_000 } else { 1_000 })
        .for_each(|(a, b, _)| match (circuits.get(&a).copied(), circuits.get(&b).copied()) {
            (Some(a_id), Some(b_id)) if a_id == b_id => (),
            (Some(a_id), Some(b_id)) => {
                circuits.iter_mut().for_each(|(_, id)| {
                    if *id == b_id {
                        *id = a_id;
                    }
                });

                let b_size = circuit_sizes.remove(&b_id).unwrap();
                *circuit_sizes.get_mut(&a_id).unwrap() += b_size;

                last_pair = (a, b);
            },
            (Some(a_id), None) => {
                circuits.insert(b, a_id);
                *circuit_sizes.get_mut(&a_id).unwrap() += 1;
            },
            (None, Some(b_id)) => {
                circuits.insert(a, b_id);
                *circuit_sizes.get_mut(&b_id).unwrap() += 1;
            },
            (None, None) => {
                circuits.insert(a, circuit_id);
                circuits.insert(b, circuit_id);
                circuit_sizes.insert(circuit_id, 2usize);
                circuit_id += 1;
            },
        });

    (circuits, circuit_sizes, last_pair)
}

fn input() -> Vec<Pt3d> {
    safe_get_input_as_many(2025, 8)
}
