use std::collections::HashSet;
use std::iter::successors;
use std::ops::Range;
use std::rc::Rc;

use itertools::Itertools;

use crate::helpers::direction_4::Direction4;
use crate::helpers::input::safe_get_input_as_matrix;
use crate::helpers::looping::LoopingItertools;
use crate::helpers::pt::Pt;
use crate::helpers::turtle::Turtle;

pub fn part_1() -> usize {
    input()
        .guard_path()
        .map(|guard| guard.position)
        .unique()
        .count()
}

pub fn part_2() -> usize {
    let lab = input();
    let max_loop_size = part_1() * 2;
    lab.guard_path()
        .skip(1)
        .filter_map(|guard| {
            lab.with_extra_obstacle(guard.position).and_then(|new_lab| {
                new_lab
                    .guard_path()
                    .looping(max_loop_size)
                    .map(|_| guard.position)
            })
        })
        .unique()
        .count()
}

#[derive(Debug, Clone)]
struct Lab {
    x_bounds: Range<i64>,
    y_bounds: Range<i64>,
    obstacles: Rc<HashSet<Pt>>,
    extra_obstacle: Option<Pt>,
    guard_start_pos: Pt,
}

impl Lab {
    pub fn guard(&self) -> Turtle {
        Turtle::new(self.guard_start_pos, Direction4::Up)
    }

    pub fn in_bounds(&self, pt: Pt) -> bool {
        self.x_bounds.contains(&pt.x) && self.y_bounds.contains(&pt.y)
    }

    pub fn blocked(&self, pt: Pt) -> bool {
        self.obstacles.contains(&pt)
            || self
                .extra_obstacle
                .as_ref()
                .map(|&extra| extra == pt)
                .unwrap_or_default()
    }

    pub fn guard_path(&self) -> impl Iterator<Item = Turtle> + '_ {
        successors(Some(self.guard()), |&guard| self.advance_guard(guard))
    }

    pub fn advance_guard(&self, mut guard: Turtle) -> Option<Turtle> {
        while self.blocked(guard.advance().position) {
            guard = guard.turn_right();
        }
        guard = guard.advance();
        self.in_bounds(guard.position).then_some(guard)
    }

    pub fn with_extra_obstacle(&self, new_obstacle: Pt) -> Option<Self> {
        (!self.blocked(new_obstacle) && new_obstacle != self.guard_start_pos)
            .then(|| Self { extra_obstacle: Some(new_obstacle), ..self.clone() })
    }
}

impl From<Vec<Vec<char>>> for Lab {
    fn from(input: Vec<Vec<char>>) -> Self {
        let obstacles: HashSet<_> = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, &c)| (Pt::new(x as i64, y as i64), c))
            })
            .filter(|&(_, c)| c == '#')
            .map(|(pt, _)| pt)
            .collect();
        let guard_start_pos = input
            .iter()
            .enumerate()
            .filter_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_position(|(_, &c)| c == '^')
                    .map(|(x, _)| Pt::new(x as i64, y as i64))
            })
            .exactly_one()
            .unwrap();

        Self {
            x_bounds: 0..(input[0].len() as i64),
            y_bounds: 0..(input.len() as i64),
            obstacles: Rc::new(obstacles),
            extra_obstacle: None,
            guard_start_pos,
        }
    }
}

fn input() -> Lab {
    safe_get_input_as_matrix(2024, 6).into()
}
