use std::iter::successors;

use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::pt::Pt;
use aoclp::positioning::turtle::Turtle;
use aoclp::solvers_impl::input::safe_get_input_as_many;

pub fn part_1() -> String {
    packet().filter_map(|(_, c)| c.map(char::from)).collect()
}

pub fn part_2() -> usize {
    packet().count()
}

fn blockade(dir: Direction4) -> u8 {
    match dir {
        Direction4::Left | Direction4::Right => b'|',
        Direction4::Up | Direction4::Down => b'-',
    }
}

fn packet() -> impl Iterator<Item = (Turtle, Option<u8>)> {
    let input = input();

    let start_x = input[0].bytes().position(|c| c == b'|').unwrap() as i64;
    let start_pos = Pt::new(start_x, 0);
    let turtle = Turtle::new(start_pos, Direction4::Down);

    let at = move |t: &Turtle| input[t.position.y as usize].as_bytes()[t.position.x as usize];

    successors(Some((turtle, None)), move |(turtle, _)| {
        Some(turtle.advance())
            .filter(|t| at(t) != b' ')
            .or_else(|| {
                Some(turtle.turn_left().advance())
                    .filter(|t| {
                        // Note: if I remove the test for `blockade`, it still works with my data.
                        let new_c = at(t);
                        new_c != b' ' && new_c != blockade(t.direction)
                    })
                    .or_else(|| Some(turtle.turn_right().advance()))
            })
            .map(|t| (t, at(&t)))
            .filter(|(_, c)| *c != b' ')
            .map(|(t, c)| (t, Some(c).filter(u8::is_ascii_uppercase)))
    })
}

fn input() -> Vec<String> {
    safe_get_input_as_many(2017, 19)
}
