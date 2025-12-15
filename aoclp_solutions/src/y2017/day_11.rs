use std::iter::successors;

use aoclp::num::{Zero, zero};
use aoclp::positioning::pt::{Pt, manhattan};
use aoclp::solvers_impl::input::safe_get_input_as_one_vec;
use itertools::Itertools;
use strum::EnumString;

pub fn part_1() -> usize {
    distance_to(child_position())
}

pub fn part_2() -> usize {
    child_path()
        .sorted_by_key(|pt| -manhattan(Pt::zero(), *pt))
        .map(distance_to)
        .next()
        .unwrap()
}

fn child_path() -> impl Iterator<Item = Pt> {
    input().into_iter().scan(zero(), |pt: &mut Pt, dir| {
        *pt += dir.displacement();
        Some(*pt)
    })
}

fn child_position() -> Pt {
    child_path().last().unwrap()
}

fn distance_to(goal: Pt) -> usize {
    path_to(goal).count() - 1
}

fn path_to(goal: Pt) -> impl Iterator<Item = Pt> {
    successors(Some(zero()), move |pt: &Pt| {
        match ((goal.x - pt.x).signum(), (goal.y - pt.y).signum()) {
            (0, 0) => None,
            (0, y) => Some(*pt + Pt::new(0, y * 2)),
            (x, y) => Some(*pt + Pt::new(x, y)),
        }
    })
}

#[derive(Debug, Copy, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
enum HexDirection {
    NW,
    N,
    NE,
    SE,
    S,
    SW,
}

impl HexDirection {
    fn displacement(&self) -> Pt {
        match self {
            HexDirection::NW => Pt::new(-1, 1),
            HexDirection::N => Pt::new(0, 2),
            HexDirection::NE => Pt::new(1, 1),
            HexDirection::SE => Pt::new(1, -1),
            HexDirection::S => Pt::new(0, -2),
            HexDirection::SW => Pt::new(-1, -1),
        }
    }
}

fn input() -> Vec<HexDirection> {
    safe_get_input_as_one_vec(2017, 11)
}
