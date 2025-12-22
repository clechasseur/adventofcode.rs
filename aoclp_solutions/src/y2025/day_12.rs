use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::anyhow::Context;
use aoclp::captures::CapturesHelper;
use aoclp::mapping::canvas::fixed::Canvas;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input;
use itertools::Itertools;

pub fn part_1() -> usize {
    let input = input();
    println!("Presents: {}, regions: {}", input.0.len(), input.1.len());

    0
}

pub fn part_2() -> usize {
    0
}

type Present = Canvas<bool, 3>;

#[derive(Debug, Clone)]
struct Region {
    pub width: usize,
    pub height: usize,
    pub presents: Vec<usize>,
}

impl FromStr for Region {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
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
    S: AsRef<str>,
{
    static INDEX_REGEX: OnceLock<Regex> = OnceLock::new();
    let index_re = INDEX_REGEX.get_or_init(|| Regex::new(r"^(?<idx>\d+):\s*$").unwrap());

    let it = input.into_iter();

    let mut present_it = it
        .clone()
        .take_while(|line| line.as_ref().parse::<Region>().is_err());
    let mut presents = Vec::new();
    let mut i = 0;
    loop {
        match present_it.next() {
            None => break,
            Some(s) if s.as_ref().trim_ascii().is_empty() => continue,
            Some(s) => {
                let index_cap = index_re.captures(s.as_ref()).unwrap();
                let index: usize = index_cap.ez_get("idx");
                if index != i {
                    panic!("expected present #{i}, found present #{index}");
                }

                let present = Present::from_lines(present_it.by_ref().take(3), |c| c == '#');
                presents.push(present);
                i += 1;
            },
        }
    }

    let regions = it.filter_map(|l| l.as_ref().parse().ok()).collect();

    (presents, regions)
}
