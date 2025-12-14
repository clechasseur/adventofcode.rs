use std::ops::Range;
use std::str::FromStr;
use std::sync::OnceLock;
use itertools::Itertools;
use aoclp::anyhow::Context;
use aoclp::regex::Regex;

pub fn part_1() -> usize {
    0
}

pub fn part_2() -> usize {
    0
}

#[derive(Debug, Copy, Clone)]
struct Present {
    shape: [[bool; 3]; 3],
}

impl<I, S> From<I> for Present
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn from(value: I) -> Self {
        let shape = value
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(|c| c == '#')
                    .collect_array()
                    .unwrap()
            })
            .collect_array()
            .unwrap();
        Self { shape }
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

impl FromStr for Region {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re =
            REGEX.get_or_init(|| {
                Regex::new(r"^(?<width>\d+)x(?<height>\d+):\s+(?<presents>(?:\d+\s*)+)$").unwrap()
            });

        let captures = re
            .captures(s)
            .with_context(|| format!("invalid region spec: {s}"))?;
        let width = &captures["width"];
        let height = &captures["height"];
        let presents = &captures["presents"];

        let width = width.parse()?;
        let height = height.parse()?;
        let presents: Vec<_> = presents
            .split_ascii_whitespace()
            .map(|p| p.parse::<usize>())
            .try_collect()?;

        Ok(Self { width, height, presents })
    }
}

fn parse_input<I, S>(input: I) -> (Vec<Present>, Vec<Region>)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut it = input.into_iter();

    let mut presents = Vec::new();
    for i in 0.. {
        let index = it.next();
        if 
    }

    todo!()
}
