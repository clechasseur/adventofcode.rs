use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::iter::successors;

use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::direction::{Direction, MovementDirection};
use aoclp::positioning::pt::{rectangular_area, Pt};
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;
use strum::IntoEnumIterator;

pub fn part_1() -> i64 {
    input()
        .into_iter()
        .array_combinations()
        .map(|[a, b]| rectangular_area(a, b))
        .max()
        .unwrap()
}

pub fn part_2() -> i64 {
    let red_tiles = input();

    let red_zone: BTreeSet<_> = build_red_zone(&red_tiles).collect();
    let safe_rectangle = |a: Pt, b: Pt| {
        let corners = vec![
            Pt::new(min(a.x, b.x), min(a.y, b.y)),
            Pt::new(max(a.x, b.x), min(a.y, b.y)),
            Pt::new(max(a.x, b.x), max(a.y, b.y)),
            Pt::new(min(a.x, b.x), max(a.y, b.y)),
            Pt::new(min(a.x, b.x), min(a.y, b.y)),
        ];
        let edges: BTreeSet<_> = corners
            .into_iter()
            .tuple_windows()
            .flat_map(|(a, b)| {
                let displacement = Pt::new((b.x - a.x).signum(), (b.y - a.y).signum());
                successors(Some(a), move |&p| (p != b).then_some(p + displacement))
            })
            .collect();
        edges.is_disjoint(&red_zone)
    };

    red_tiles
        .into_iter()
        .array_combinations()
        .map(|[a, b]| (a, b, rectangular_area(a, b)))
        .sorted_unstable_by(|(_, _, area_a), (_, _, area_b)| area_b.cmp(area_a))
        .find(|&(a, b, _)| safe_rectangle(a, b))
        .map(|(_, _, area)| area)
        .unwrap()
}

fn build_red_zone(red_tiles: &[Pt]) -> impl Iterator<Item = Pt> + use<'_> {
    let starting_point = red_tiles
        .iter()
        .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        .copied()
        .next()
        .unwrap();

    let get_direction = |a: Pt, b: Pt| {
        let displacement = Pt::new((b.x - a.x).signum(), (b.y - a.y).signum());
        Direction4::iter()
            .find(|&d| d.displacement() == displacement)
            .unwrap()
    };

    red_tiles
        .iter()
        .copied()
        .cycle()
        .skip_while(move |&p| p != starting_point)
        .take(red_tiles.len() + 2)
        .tuple_windows()
        .flat_map(move |(a, b, c)| {
            let direction = get_direction(a, b);
            let turning_left = get_direction(b, c) == direction.turn_left();

            let tail = if turning_left {
                vec![]
            } else {
                vec![
                    b + direction.turn_left(),
                    b + direction + direction.turn_left(),
                    b + direction,
                ]
            };

            successors(Some(a), move |&p| {
                let next = p + direction;
                (next != b).then_some(next)
            })
            .skip(1)
            .map(move |p| p + (direction.turn_left()))
            .chain(tail)
        })
}

const EXAMPLE: &str = "\
    7,1\n\
    11,1\n\
    11,7\n\
    9,7\n\
    9,5\n\
    2,5\n\
    2,3\n\
    7,3";

fn input() -> Vec<Pt> {
    safe_get_input_as_many(2025, 9)
}
