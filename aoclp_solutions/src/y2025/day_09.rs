use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::iter::{once, successors};

use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::direction::{Direction, MovementDirection};
use aoclp::positioning::pt::{rectangular_area, Pt};
use aoclp::solvers_impl::input::{safe_get_input_as_many, Input};
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

    let walls = red_tiles
        .iter()
        .copied()
        .chain(once(red_tiles[0]))
        .tuple_windows()
        .map(|(a, b)| {
            let (min_x, min_y) = (min(a.x, b.x), min(a.y, b.y));
            let (max_x, max_y) = (max(a.x, b.x), max(a.y, b.y));
            (Pt::new(min_x, min_y), Pt::new(max_x, max_y))
        })
        .collect_vec();

    let valid = |a: Pt, b: Pt| {
        let (top, bottom) = (min(a.y, b.y), max(a.y, b.y));
        let (left, right) = (min(a.x, b.x), max(a.x, b.x));
        !walls.iter().any(|(a, b)| {
            // a.within(left + 1..=right - 1, top + 1..=bottom - 1) ||
            //     b.within(left + 1..=right - 1, top + 1..=bottom - 1)
            let res = a.y < bottom && b.y > top && a.x < right && b.x > left;
            // if res {
            //     println!("res");
            // }
            res
        })
    };

    let (a, b, area) = red_tiles
        .iter()
        .copied()
        .array_combinations()
        .filter(|[a, b]| valid(*a, *b))
        .map(|[a, b]| (a, b, rectangular_area(a, b)))
        .max_by_key(|(_, _, area)| *area)
        .unwrap();

    // let red_zone: BTreeSet<_> = build_red_zone(&red_tiles).collect();
    // let safe_rectangle = |a: Pt, b: Pt| {
    //     let corners = vec![
    //         Pt::new(min(a.x, b.x), min(a.y, b.y)),
    //         Pt::new(max(a.x, b.x), min(a.y, b.y)),
    //         Pt::new(max(a.x, b.x), max(a.y, b.y)),
    //         Pt::new(min(a.x, b.x), max(a.y, b.y)),
    //         Pt::new(min(a.x, b.x), min(a.y, b.y)),
    //     ];
    //     let edges: BTreeSet<_> = corners
    //         .into_iter()
    //         .tuple_windows()
    //         .flat_map(|(a, b)| {
    //             let displacement = Pt::new((b.x - a.x).signum(), (b.y - a.y).signum());
    //             successors(Some(a), move |&p| (p != b).then_some(p + displacement))
    //         })
    //         .collect();
    //     edges.is_disjoint(&red_zone)
    // };
    //
    // let (a, b, area) = red_tiles
    //     .iter()
    //     .copied()
    //     .array_combinations()
    //     .map(|[a, b]| (a, b, rectangular_area(a, b)))
    //     .sorted_unstable_by(|(_, _, area_a), (_, _, area_b)| area_b.cmp(area_a))
    //     .find(|&(a, b, _)| safe_rectangle(a, b))
    //     .unwrap();

    println!("Largest rectangle is between ({},{}) and ({},{}) and has area {area}", a.x, a.y, b.x, b.y);

    let max_rects = red_tiles
        .iter()
        .copied()
        .array_combinations()
        .map(|[a, b]| (a, b, rectangular_area(a, b)))
        .filter(|(a, b, ar)| *ar == area && valid(*a, *b))
        .collect_vec();

    println!("{} valid rectangles have that area", max_rects.len());
    for (a, b, _) in max_rects {
        println!("  ({}, {}) - ({}, {})", a.x, a.y, b.x, b.y);
    }

    area
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
    11,8\n\
    9,8\n\
    9,5\n\
    2,5\n\
    2,3\n\
    7,3";

fn input() -> Vec<Pt> {
    Input::for_example(EXAMPLE).safe_into_many()
    // safe_get_input_as_many(2025, 9)
}
