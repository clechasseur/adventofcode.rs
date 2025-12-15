use std::iter::successors;
use std::str::FromStr;

use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> usize {
    matching_pairs(generator_a(), generator_b(), 40_000_000)
}

pub fn part_2() -> usize {
    let a = generator_a().picky(4);
    let b = generator_b().picky(8);

    matching_pairs(a, b, 5_000_000)
}

const THRESHOLD: u64 = i32::MAX as u64;

fn generator(initial_value: u64, factor: u64) -> impl Iterator<Item = u64> {
    successors(Some(initial_value), move |prev| Some(prev * factor % THRESHOLD)).skip(1)
}

trait Picky {
    fn picky(self, multiple_of: u64) -> impl Iterator<Item = u64>;
}

impl<T> Picky for T
where
    T: Iterator<Item = u64>,
{
    fn picky(self, multiple_of: u64) -> impl Iterator<Item = u64> {
        self.filter(move |value| value % multiple_of == 0)
    }
}

fn generator_a() -> impl Iterator<Item = u64> {
    generator(initial_values().generator_a.initial_value(), 16_807)
}

fn generator_b() -> impl Iterator<Item = u64> {
    generator(initial_values().generator_b.initial_value(), 48_271)
}

fn matching_pairs<A, B>(a: A, b: B, rounds: usize) -> usize
where
    A: Iterator<Item = u64>,
    B: Iterator<Item = u64>,
{
    a.zip(b)
        .take(rounds)
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count()
}

#[derive(Debug, Copy, Clone)]
struct Generator(u64);

impl Generator {
    pub fn initial_value(&self) -> u64 {
        self.0
    }
}

const GENERATOR_PREFIX: &str = "Generator X starts with ";

impl FromStr for Generator {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s[GENERATOR_PREFIX.len()..].trim().parse()?))
    }
}

#[derive(Debug, Copy, Clone)]
struct Generators {
    pub generator_a: Generator,
    pub generator_b: Generator,
}

impl From<Vec<Generator>> for Generators {
    fn from(generators: Vec<Generator>) -> Self {
        let (generator_a, generator_b) = generators.into_iter().collect_tuple().unwrap();
        Self { generator_a, generator_b }
    }
}

fn initial_values() -> Generators {
    safe_get_input_as_many(2017, 15).into()
}
