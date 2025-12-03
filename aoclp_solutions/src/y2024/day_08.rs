use std::collections::HashMap;
use std::iter::successors;
use std::ops::Range;

use aoclp::positioning::pt::{matrix_to_map, Pt};
use aoclp::solvers_impl::input::safe_get_input_as_terrain;
use itertools::Itertools;

pub fn part_1() -> usize {
    Map::default().antinodes(false).count()
}

pub fn part_2() -> usize {
    Map::default().antinodes(true).count()
}

#[derive(Debug)]
struct Map {
    x_bounds: Range<i64>,
    y_bounds: Range<i64>,
    antennas: HashMap<Pt, char>,
}

impl Map {
    fn antinodes(&self, resonating: bool) -> impl Iterator<Item = Pt> {
        let antinodes = self
            .antennas
            .iter()
            .sorted_by_key(|&(_, &freq)| freq)
            .chunk_by(|&(_, &freq)| freq)
            .into_iter()
            .flat_map(|(_, antennas)| {
                let antennas = antennas.map(|(&pt, _)| pt).collect_vec();
                let antinodes = antennas
                    .iter()
                    .cartesian_product(antennas.iter())
                    .filter(|(p1, p2)| p1 != p2)
                    .flat_map(|(&p1, &p2)| {
                        let antinodes = match resonating {
                            true => self.resonating_antinodes_for(p1, p2).collect_vec(),
                            false => self.dull_antinodes_for(p1, p2).collect_vec(),
                        };
                        antinodes.into_iter()
                    })
                    .collect_vec();
                antinodes.into_iter()
            })
            .unique()
            .filter(|pt| pt.within(self.x_bounds.clone(), self.y_bounds.clone()))
            .collect_vec();
        antinodes.into_iter()
    }

    fn dull_antinodes_for(&self, p1: Pt, p2: Pt) -> impl Iterator<Item = Pt> {
        let diff = p1 - p2;
        let anti1 = p1 + diff;
        let anti2 = p2 - diff;
        [anti1, anti2].into_iter()
    }

    fn resonating_antinodes_for(&self, p1: Pt, p2: Pt) -> impl Iterator<Item = Pt> + use<'_> {
        let diff = p1 - p2;
        let from_p1 = successors(Some(p1), move |&pt| {
            let next = pt + diff;
            next.within(self.x_bounds.clone(), self.y_bounds.clone())
                .then_some(next)
        });
        let from_p2 = successors(Some(p2), move |&pt| {
            let next = pt - diff;
            next.within(self.x_bounds.clone(), self.y_bounds.clone())
                .then_some(next)
        });
        from_p1.chain(from_p2)
    }
}

impl Default for Map {
    fn default() -> Self {
        let antennas_matrix = input();

        let y_bounds = 0..antennas_matrix.len() as i64;
        let x_bounds = 0..antennas_matrix[0].len() as i64;
        let antennas = matrix_to_map(antennas_matrix)
            .into_iter()
            .filter(|&(_, c)| c != '.')
            .collect();

        Self { x_bounds, y_bounds, antennas }
    }
}

fn input() -> Vec<Vec<char>> {
    safe_get_input_as_terrain(2024, 8)
}
