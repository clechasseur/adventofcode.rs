use std::fmt;
use std::iter::once;

use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::direction::{Direction, MovementDirection};
use aoclp::positioning::pt::{Pt, min_max, rectangle_corners, rectangular_area};
use aoclp::solvers_impl::input::{Input, safe_get_input_as_many};
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
    let walls = walls(&red_tiles).collect_vec();

    let valid_rectangle = |a: Pt, b: Pt| {
        let corners = rectangle_corners(a, b);
        corners
            .into_iter()
            .chain(once(corners[0]))
            .tuple_windows()
            .map(|(a, b)| GridLine::from_endpoints(a, b))
            .all(|line| !walls.iter().any(|w| w.intersects(line)))
    };

    red_tiles
        .iter()
        .copied()
        .array_combinations()
        .filter(|[a, b]| valid_rectangle(*a, *b))
        .map(|[a, b]| rectangular_area(a, b))
        .max()
        .unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GridLine {
    Horizontal { y: i64, left_x: i64, right_x: i64 },
    Vertical { x: i64, top_y: i64, bottom_y: i64 },
    Point(Pt),
}

impl GridLine {
    fn from_endpoints(a: Pt, b: Pt) -> Self {
        let (a, b) = min_max(a, b);
        match (a.x == b.x, a.y == b.y) {
            (true, true) => Self::Point(a),
            (true, false) => Self::Vertical { x: a.x, top_y: a.y, bottom_y: b.y },
            (false, true) => Self::Horizontal { y: a.y, left_x: a.x, right_x: b.x },
            (false, false) => panic!("{a} and {b} do not form a line snapped to the grid"),
        }
    }

    fn extend(self, direction: Direction4, len: i64) -> Self {
        match (self, direction) {
            (Self::Horizontal { y, left_x, right_x }, Direction4::Left) => {
                Self::Horizontal { y, left_x: left_x - len, right_x }
            },
            (Self::Horizontal { y, left_x, right_x }, Direction4::Right) => {
                Self::Horizontal { y, left_x, right_x: right_x + len }
            },
            (Self::Vertical { x, top_y, bottom_y }, Direction4::Up) => {
                Self::Vertical { x, top_y: top_y - len, bottom_y }
            },
            (Self::Vertical { x, top_y, bottom_y }, Direction4::Down) => {
                Self::Vertical { x, top_y, bottom_y: bottom_y + len }
            },
            (Self::Point(p), direction) => {
                Self::from_endpoints(p, p + (direction.displacement() * len))
            },
            (line, direction) => {
                panic!("line {line} cannot be extended {direction}");
            },
        }
    }

    fn intersects(self, rhs: Self) -> bool {
        match (self, rhs) {
            (Self::Horizontal { y, left_x, right_x }, Self::Vertical { x, top_y, bottom_y })
            | (Self::Vertical { x, top_y, bottom_y }, Self::Horizontal { y, left_x, right_x }) => {
                (top_y..=bottom_y).contains(&y) && (left_x..=right_x).contains(&x)
            },
            (Self::Horizontal { y, left_x, right_x }, Self::Point(p))
            | (Self::Point(p), Self::Horizontal { y, left_x, right_x }) => {
                p.y == y && (left_x..=right_x).contains(&p.x)
            },
            (Self::Vertical { x, top_y, bottom_y }, Self::Point(p))
            | (Self::Point(p), Self::Vertical { x, top_y, bottom_y }) => {
                p.x == x && (top_y..=bottom_y).contains(&p.y)
            },
            _ => false,
        }
    }
}

impl fmt::Display for GridLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Horizontal { y, left_x, right_x } => {
                write!(f, "{} - {}", Pt::new(left_x, y), Pt::new(right_x, y))
            },
            Self::Vertical { x, top_y, bottom_y } => {
                write!(f, "{} - {}", Pt::new(x, top_y), Pt::new(x, bottom_y))
            },
            Self::Point(p) => write!(f, "{p} - {p}"),
        }
    }
}

fn walls(red_tiles: &[Pt]) -> impl Iterator<Item = GridLine> + use<'_> {
    let starting_point = red_tiles
        .iter()
        .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        .copied()
        .next()
        .unwrap();

    let get_direction = |a: Pt, b: Pt| {
        let displacement = Pt::new((b.x - a.x).signum(), (b.y - a.y).signum());
        Direction4::iter()
            .find(|d| d.displacement() == displacement)
            .unwrap()
    };

    red_tiles
        .iter()
        .copied()
        .cycle()
        .skip_while(move |p| *p != starting_point)
        .take(red_tiles.len() + 2)
        .tuple_windows()
        .scan(true, move |turned_right, (a, b, c)| {
            let direction = get_direction(a, b);
            let turning_right = get_direction(b, c) == direction.turn_right();

            let mut line = GridLine::from_endpoints(
                a + direction + direction.turn_left(),
                b + direction.turn_around() + direction.turn_left(),
            );
            if *turned_right {
                line = line.extend(direction.turn_around(), 1);
            }
            if turning_right {
                line = line.extend(direction, 2);
            }

            *turned_right = turning_right;
            Some(line)
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

fn example() -> Vec<Pt> {
    Input::for_example(EXAMPLE).safe_into_many()
}
