//! Helpers to interact with the Advent of Code website/API.

use std::str::FromStr;

use aocf::Aoc;
use itertools::Itertools;

use crate::anyhow::anyhow;

pub const DEFAULT_INPUT_SEPARATORS: &[char] = &[' ', '\t', '|', ',', ':'];

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleId {
    year: i32,
    day: u32,
}

#[derive(Debug)]
pub struct Input<'a> {
    puzzle: Option<PuzzleId>,
    force: bool,
    data: Option<String>,
    separators: &'a [char],
}

impl<'a> Input<'a> {
    pub fn for_puzzle(year: i32, day: u32) -> Self {
        Self {
            puzzle: Some(PuzzleId { year, day }),
            force: false,
            data: None,
            separators: DEFAULT_INPUT_SEPARATORS,
        }
    }

    pub fn for_example<S>(example: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            puzzle: None,
            force: false,
            data: Some(example.into()),
            separators: DEFAULT_INPUT_SEPARATORS,
        }
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn separators(mut self, separators: &'a [char]) -> Self {
        self.separators = separators;
        self
    }

    pub fn try_get(mut self) -> crate::Result<Self> {
        if self.data.is_some() {
            return Ok(self);
        }

        let PuzzleId { year, day } = self.puzzle.ok_or(anyhow!("no puzzle ID set"))?;

        let mut aoc = Aoc::new()
            .year(Some(year))
            .day(Some(day))
            .parse_cli(false)
            .init()
            .map_err(|e| anyhow!(e))?;

        self.data = Some(
            aoc.get_input(self.force)
                .map(|mut input| {
                    if input.ends_with('\n') {
                        input.remove(input.len() - 1);
                    }
                    input
                })
                .map_err(|e| anyhow!(e))?,
        );
        Ok(self)
    }

    pub fn get(self) -> Self {
        self.try_get().expect("failed to get AOC puzzle input")
    }

    pub fn try_into<T>(self) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_get()?
            .data
            .unwrap()
            .parse()
            .map_err(|e| anyhow!("failed to parse data: {e:?}"))
    }

    pub fn into<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_into().unwrap()
    }

    pub fn try_into_string(self) -> crate::Result<String> {
        self.try_into()
    }

    pub fn into_string(self) -> String {
        self.into()
    }

    pub fn try_lines_into<T>(self) -> crate::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        Self::parse_lines(self.into_string().lines())
    }

    pub fn lines_into<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_lines_into().unwrap()
    }

    pub fn try_lines_into_pairs<T>(self) -> crate::Result<Vec<(T, T)>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        let vecs = Self::split_and_parse_lines(self.into_string().lines(), separators)?;

        Ok(vecs
            .into_iter()
            .map(|v| {
                let (a, b) = v.into_iter().collect_tuple().unwrap();
                (a, b)
            })
            .collect())
    }

    pub fn lines_into_pairs<T>(self) -> Vec<(T, T)>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_lines_into_pairs().unwrap()
    }

    pub fn try_split_lines_into<T>(self) -> Result<Vec<Vec<T>>, crate::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        Self::split_and_parse_lines(self.into_string().lines(), separators)
    }

    pub fn split_lines_into<T>(self) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_lines_into().unwrap()
    }

    pub fn try_two_types_of_lines_into<T, U>(self) -> crate::Result<(Vec<T>, Vec<U>)>
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        let data = self.try_into_string()?;

        let first = Self::parse_lines(data.lines().take_while(|line| !line.is_empty()))?;
        let second = Self::parse_lines(data.lines().skip_while(|line| !line.is_empty()).skip(1))?;

        Ok((first, second))
    }

    pub fn two_types_of_lines_into<T, U>(self) -> (Vec<T>, Vec<U>)
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        self.try_two_types_of_lines_into().unwrap()
    }

    #[allow(clippy::type_complexity)]
    pub fn try_split_two_types_of_lines_into<T, U>(
        self,
    ) -> crate::Result<(Vec<Vec<T>>, Vec<Vec<U>>)>
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        let data = self.try_into_string()?;

        let first = Self::split_and_parse_lines(
            data.lines().take_while(|line| !line.is_empty()),
            separators,
        )?;
        let second = Self::split_and_parse_lines(
            data.lines().skip_while(|line| !line.is_empty()).skip(1),
            separators,
        )?;
        Ok((first, second))
    }

    pub fn split_two_types_of_lines_into<T, U>(self) -> (Vec<Vec<T>>, Vec<Vec<U>>)
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_two_types_of_lines_into().unwrap()
    }

    pub fn try_split_into<T>(self) -> Result<Vec<T>, crate::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_lines_into()
            .and_then(|vecs| vecs.into_iter().next().ok_or_else(|| anyhow!("empty data")))
    }

    pub fn split_into<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_into().unwrap()
    }

    pub fn try_into_terrain<T>(self) -> crate::Result<Vec<Vec<T>>>
    where
        T: From<char>,
    {
        Ok(self
            .into::<String>()
            .lines()
            .map(|line| line.chars().map(Into::into).collect())
            .collect())
    }

    pub fn into_terrain<T>(self) -> Vec<Vec<T>>
    where
        T: From<char>,
    {
        self.try_into_terrain().unwrap()
    }

    fn parse_lines<T, L, S>(lines: L) -> crate::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        L: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        lines
            .into_iter()
            .map(|line| {
                let line = line.as_ref();
                line.parse()
                    .map_err(|e| anyhow!("failed to parse \"{line}\": {e:?}"))
            })
            .collect()
    }

    fn split_and_parse_lines<T, L, S>(lines: L, separators: &[char]) -> crate::Result<Vec<Vec<T>>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        L: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        lines
            .into_iter()
            .map(|line| {
                let line = line.as_ref();
                line.split(separators)
                    .filter(|value| !value.is_empty())
                    .map(|value| {
                        value.parse().map_err(|e| {
                            anyhow::anyhow!("failed to parse \"{value}\" in \"{line}\": {e:?}")
                        })
                    })
                    .collect()
            })
            .collect()
    }
}
