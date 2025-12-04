use std::collections::{BTreeSet, HashSet};

use aoclp::positioning::pt::{filtered_matrix_to_map, Pt};
use aoclp::solvers_impl::input::safe_get_input_as_terrain;

pub fn part_1() -> usize {
    let rolls: HashSet<Pt> = filtered_matrix_to_map(input(), ['.']).into_keys().collect();

    rolls
        .iter()
        .filter(|p| p.eight_neighbours().filter(|n| rolls.contains(n)).count() < 4)
        .count()
}

pub fn part_2() -> usize {
    let mut rolls: BTreeSet<Pt> = filtered_matrix_to_map(input(), ['.']).into_keys().collect();

    let mut removed = 0;
    loop {
        let removables: BTreeSet<_> = rolls
            .iter()
            .filter(|p| p.eight_neighbours().filter(|n| rolls.contains(n)).count() < 4)
            .copied()
            .collect();
        if removables.is_empty() {
            break;
        }

        removed += removables.len();
        rolls = rolls.difference(&removables).copied().collect();
    }

    removed
}

fn input() -> Vec<Vec<char>> {
    safe_get_input_as_terrain(2025, 4)
}
