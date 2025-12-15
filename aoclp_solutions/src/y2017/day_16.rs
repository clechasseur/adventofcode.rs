use std::fmt::{Display, Formatter};
use std::str::FromStr;

use aoclp::anyhow::{Context, anyhow};
use aoclp::looping::LoopingItertools;
use aoclp::solvers_impl::input::safe_get_input_as_one_vec;
use itertools::Itertools;

pub fn part_1() -> String {
    Dances::default().next().unwrap().to_string()
}

pub fn part_2() -> String {
    Dances::default()
        .looping(1_000_000_000)
        .unwrap()
        .last()
        .unwrap()
        .to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    pub fn apply(self, mut programs: Vec<char>) -> Vec<char> {
        match self {
            Self::Spin(len) => programs
                .iter()
                .tail(len)
                .chain(programs.iter().dropping_back(len))
                .copied()
                .collect_vec(),
            Self::Exchange(pos_a, pos_b) => {
                programs.swap(pos_a, pos_b);
                programs
            },
            Self::Partner(par_a, par_b) => {
                let pos_a = programs.iter().position(|c| *c == par_a).unwrap();
                let pos_b = programs.iter().position(|c| *c == par_b).unwrap();
                programs.swap(pos_a, pos_b);
                programs
            },
        }
    }
}

impl FromStr for Move {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid_move = || format!("invalid move: {s}");

        match s.chars().next() {
            Some('s') => Ok(Self::Spin(s[1..].parse()?)),
            Some('x') => {
                let (a, b) = s[1..].split_once('/').with_context(invalid_move)?;
                Ok(Self::Exchange(a.parse()?, b.parse()?))
            },
            Some('p') => {
                let (a, b) = s[1..].split_once('/').with_context(invalid_move)?;
                Ok(Self::Partner(
                    a.chars().next().with_context(invalid_move)?,
                    b.chars().next().with_context(invalid_move)?,
                ))
            },
            Some(mv) => Err(anyhow!("invalid move: {mv}")),
            None => Err(anyhow!("empty move string")),
        }
    }
}

fn moves() -> Vec<Move> {
    safe_get_input_as_one_vec(2017, 16)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Programs(Vec<char>);

impl Programs {
    pub fn dance(self, moves: &[Move]) -> Self {
        Self(moves.iter().fold(self.0, |ps, mv| mv.apply(ps)))
    }
}

impl Default for Programs {
    fn default() -> Self {
        Self(('a'..='p').collect_vec())
    }
}

impl Display for Programs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

#[derive(Debug, Clone)]
struct Dances {
    moves: Vec<Move>,
    programs: Option<Programs>,
}

impl Default for Dances {
    fn default() -> Self {
        Self { moves: moves(), programs: Some(Programs::default()) }
    }
}

impl Iterator for Dances {
    type Item = Programs;

    fn next(&mut self) -> Option<Self::Item> {
        self.programs = Some(self.programs.take()?.dance(&self.moves));
        self.programs.clone()
    }
}
