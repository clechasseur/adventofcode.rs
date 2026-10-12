//! Helpers to interact with the Advent of Code website/API.

use std::env;
use std::env::VarError;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;
use anyhow::Context;
use itertools::Itertools;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use crate::anyhow::anyhow;
use crate::cache::DiskCache;

pub const DEFAULT_INPUT_SEPARATORS: &[char] = &[' ', '\t', '|', ',', ':'];

pub const AOC_BASE_URL: &str = "https://adventofcode.com";
pub const AOC_BASE_URL_ENV_VAR: &str = "AOCLP_AOC_BASE_URL";
pub const AOC_SESSION_ENV_VAR: &str = "AOCLP_AOC_SESSION";

pub const CACHE_PATH_ENV_VAR: &str = "AOCLP_CACHE_PATH";
pub const DEFAULT_CACHE_PATH: &str = ".aoclp";

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

        let mut input = self.get_aoc_input()?;
        if input.ends_with('\n') {
            input.remove(input.len() - 1);
        }

        self.data = Some(input);
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

    fn get_aoc_input(&self) -> crate::Result<String> {
        let cache_key = self.cache_key()?;

        if !self.force && let Some(input) = INPUT_CACHE.get(cache_key.as_str())? {
            return Ok(input);
        }

        let client = Client::builder()
            .user_agent(Self::http_user_agent())
            .build()
            .with_context(|| "failed to build AOC HTTP client")?;

        let url = self.aoc_input_url()?;
        let aoc_session = Self::aoc_session()?;
        let cookie = format!("session={}", aoc_session);
        let input = client.get(url)
            .header(COOKIE, cookie)
            .send()
            .with_context(|| "failed to fetch AOC input data")?
            .error_for_status()
            .with_context(|| "failed to fetch AOC input data?")?
            .text()
            .with_context(|| "failed to decode AOC input's charset")?;

        INPUT_CACHE.set(cache_key.as_str(), input.as_str())?;
        Ok(input)
    }

    fn cache_key(&self) -> crate::Result<String> {
        self.puzzle
            .map(|id| format!("aoc{}_{:0>2}", id.year, id.day))
            .ok_or_else(|| anyhow!("cannot get cache key for example"))
    }

    fn aoc_session() -> crate::Result<String> {
        match env::var(AOC_SESSION_ENV_VAR) {
            Ok(aoc_session) => Ok(aoc_session),
            Err(VarError::NotPresent) => Err(anyhow!("AOC session token needs to be passed via the {AOC_SESSION_ENV_VAR} environment variable")),
            Err(err) => Err(anyhow!("failed to get AOC session token from environment: {err}")),
        }
    }

    fn aoc_input_url(&self) -> crate::Result<String> {
        let base_url = match env::var(AOC_BASE_URL_ENV_VAR) {
            Ok(base_url) => Some(base_url),
            Err(VarError::NotPresent) => None,
            Err(err) => return Err(anyhow!("failed to get AOC base URL from environment: {err}")),
        };
        let base_url = base_url.as_ref().map(<_>::as_ref).unwrap_or(AOC_BASE_URL);

        self.puzzle
            .map(|id| format!("{base_url}/{}/day/{}/input", id.year, id.day))
            .ok_or_else(|| anyhow!("cannot get AOC input URL for example"))
    }

    fn http_user_agent() -> String {
        format!("clechasseur/{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }
}

static INPUT_CACHE: LazyLock<DiskCache> = LazyLock::new(|| {
    let path: PathBuf = match env::var(CACHE_PATH_ENV_VAR) {
        Ok(path) => path.into(),
        Err(VarError::NotPresent) => DEFAULT_CACHE_PATH.into(),
        Err(err) => panic!("failed to get cache path from environment: {err}"),
    };

    DiskCache::new(path).expect("failed to create AOC solutions cache")
});
