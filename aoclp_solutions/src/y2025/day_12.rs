use std::str::FromStr;
use std::sync::OnceLock;
use itertools::Itertools;
use aoclp::anyhow::Context;
use aoclp::captures::CapturesHelper;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input;

pub fn part_1() -> usize {
    let input = input();
    println!("Presents: {}, regions: {}", input.0.len(), input.1.len());

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

        let width = captures.ez_get("width");
        let height = captures.ez_get("height");
        let presents = &captures["presents"];
        let presents: Vec<_> = presents
            .split_ascii_whitespace()
            .map(|p| p.parse::<usize>())
            .try_collect()?;

        Ok(Self { width, height, presents })
    }
}

fn input() -> (Vec<Present>, Vec<Region>) {
    parse_input(safe_get_input(2025, 12).lines())
}

fn parse_input<I, S>(input: I) -> (Vec<Present>, Vec<Region>)
where
    I: IntoIterator<Item = S>,
    <I as IntoIterator>::IntoIter: Clone,
    S: AsRef<str> + Clone,
{
    static INDEX_REGEX: OnceLock<Regex> = OnceLock::new();
    let index_re =
        INDEX_REGEX.get_or_init(|| {
            Regex::new(r"^(?<idx>\d+):\s*$").unwrap()
        });

    let mut it = input.into_iter().peekable();
    let mut presents = Vec::new();
    let mut i = 0;
    loop {
        // if let Some(line) = it.peek() && line.as_ref().trim_ascii().is_empty() {
        //
        // }

        let index_s = it.next().expect("end of data before regions!");
        let index_s = index_s.as_ref();
        if index_s.trim_ascii().is_empty() {
            continue;
        }

        match index_re.captures(index_s) {
            None => break,
            Some(index_cap) => {
                let index: usize = index_cap.ez_get("idx");
                if index != i {
                    panic!("expected present #{i}, found present #{index}");
                }

                let present: Present = it.clone().take(3).into();
                presents.push(present);

                it = it.dropping(3);
                i += 1;
            },
        }
    }

    let regions = it
        .skip_while(|l| l.as_ref().trim_ascii().is_empty())
        .map(|l| l.as_ref().parse().unwrap())
        .collect();

    (presents, regions)
}
