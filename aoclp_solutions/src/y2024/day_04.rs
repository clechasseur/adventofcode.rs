use std::collections::HashMap;
use std::iter::successors;

use aoclp::positioning::direction::eight_points::Direction8;
use aoclp::positioning::pt::{matrix_to_map, Pt};
use aoclp::solvers_impl::input::safe_get_input_as_terrain;
use strum::IntoEnumIterator;

pub fn part_1() -> usize {
    WordSearch::default().xmas_count()
}

pub fn part_2() -> usize {
    WordSearch::default().x_mas_count()
}

#[derive(Debug)]
struct WordSearch(HashMap<Pt, char>);

impl WordSearch {
    pub fn xmas_count(&self) -> usize {
        self.0
            .iter()
            .filter(|(_, c)| **c == 'X')
            .map(|(pt, _)| self.xmas_count_from(*pt))
            .sum()
    }

    fn xmas_count_from(&self, pt: Pt) -> usize {
        Direction8::iter()
            .filter(|direction| self.has_xmas(pt, *direction))
            .count()
    }

    fn has_xmas(&self, pt: Pt, direction: Direction8) -> bool {
        successors(Some(pt), |pt| Some(*pt + direction))
            .take(4)
            .map(|pt| self.at(pt))
            .zip("XMAS".chars())
            .all(|(a, b)| a == b)
    }

    fn at(&self, pt: Pt) -> char {
        self.0.get(&pt).copied().unwrap_or('.')
    }

    pub fn x_mas_count(&self) -> usize {
        self.0
            .iter()
            .filter(|(pt, c)| **c == 'A' && self.has_x_mas(**pt))
            .count()
    }

    fn has_x_mas(&self, pt: Pt) -> bool {
        let line = |aft: Direction8, fore: Direction8| -> String {
            [pt + aft, pt, pt + fore]
                .iter()
                .map(|pt| self.at(*pt))
                .collect()
        };
        [
            line(Direction8::TopRight, Direction8::BottomLeft),
            line(Direction8::TopLeft, Direction8::BottomRight),
        ]
        .iter()
        .all(|word| word == "MAS" || word == "SAM")
    }
}

impl From<Vec<Vec<char>>> for WordSearch {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self(matrix_to_map(value))
    }
}

impl Default for WordSearch {
    fn default() -> Self {
        safe_get_input_as_terrain(2024, 4).into()
    }
}
