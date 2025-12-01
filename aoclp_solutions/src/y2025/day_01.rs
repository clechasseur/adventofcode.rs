use std::iter::{once, successors};
use std::str::FromStr;

use aoclp::anyhow::anyhow;
use aoclp::solvers_impl::input::safe_get_input_as_many;

pub fn part_1() -> usize {
    moves().filter(|dial| *dial == 0).count()
}

pub fn part_2() -> usize {
    all_moves().filter(|dial| *dial == 0).count()
}

fn input() -> Vec<Rotation> {
    safe_get_input_as_many(2025, 1)
}

fn moves() -> impl Iterator<Item = i64> {
    let mut dial = 50;
    let rotations = input();

    once(dial).chain(rotations.into_iter().map(move |rotation| {
        dial = rotation.apply(dial);
        dial
    }))
}

fn all_moves() -> impl Iterator<Item = i64> {
    let mut dial = 50;
    let rotations = input();

    once(dial).chain(rotations.into_iter().flat_map(move |rotation| {
        let from = dial;
        dial = rotation.apply(dial);

        let tick = rotation.direction.one_tick();
        successors(Some(from), move |int_dial| Some(tick.apply(*int_dial)))
            .skip(1)
            .take(rotation.clicks as usize)
    }))
}

#[repr(i64)]
#[derive(Debug, Copy, Clone)]
enum RotationDirection {
    Left = -1,
    Right = 1,
}

impl RotationDirection {
    fn one_tick(self) -> Rotation {
        Rotation { direction: self, clicks: 1 }
    }
}

impl FromStr for RotationDirection {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Self::Left),
            Some('R') => Ok(Self::Right),
            Some(c) => Err(anyhow!("wrong rotation direction identifier: {c}")),
            None => Err(anyhow!("no rotation direction identifier provided")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    direction: RotationDirection,
    clicks: i64,
}

impl Rotation {
    fn apply(&self, dial: i64) -> i64 {
        (dial + self.clicks * (self.direction as i64)).rem_euclid(100)
    }
}

impl FromStr for Rotation {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: RotationDirection = s[0..1].parse()?;
        let clicks: i64 = s[1..].parse()?;
        Ok(Self { direction, clicks })
    }
}
