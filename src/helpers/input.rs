use std::str::FromStr;

use anyhow::anyhow;
use aocf::Aoc;
use itertools::Itertools;

#[derive(Debug)]
pub struct Input {
    year: i32,
    day: Option<u32>,
    force: bool,
    data: Option<String>,
}

impl Input {
    pub fn year(year: i32) -> Self {
        Self { year, day: None, force: false, data: None }
    }

    pub fn day(mut self, day: u32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn get(mut self) -> Result<Self, anyhow::Error> {
        let mut aoc = Aoc::new()
            .year(Some(self.year))
            .day(Some(self.day.ok_or(anyhow!("day not set"))?))
            .parse_cli(false)
            .init()
            .map_err(|e| anyhow::anyhow!(e))?;

        self.data = Some(aoc.get_input(self.force).map_err(|e| anyhow::anyhow!(e))?);
        Ok(self)
    }

    pub fn safe_get(self) -> Self {
        self.get().unwrap()
    }

    pub fn into<T>(self) -> Result<T, anyhow::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.data
            .ok_or(anyhow!("data not set"))?
            .parse()
            .map_err(|e| anyhow::anyhow!("failed to parse data: {e:?}"))
    }

    pub fn safe_into<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into().unwrap()
    }

    pub fn into_multi<T>(self) -> Result<Vec<T>, anyhow::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        String::try_from(self)?
            .lines()
            .map(|line| {
                line.parse()
                    .map_err(|e| anyhow::anyhow!("failed to parse \"{line}\": {e:?}"))
            })
            .collect()
    }

    pub fn safe_into_multi<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_multi().unwrap()
    }

    pub fn into_pairs<T>(self) -> Result<Vec<(T, T)>, anyhow::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        String::try_from(self)?
            .lines()
            .map(|line| {
                let (a, b) = line
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .ok_or(anyhow::anyhow!("invalid number of elements in \"{line}\""))?;
                Ok((
                    a.parse()
                        .map_err(|e| anyhow!("failed to parse \"{a}\": {e:?}"))?,
                    b.parse()
                        .map_err(|e| anyhow!("failed to parse \"{b}\": {e:?}"))?,
                ))
            })
            .collect()
    }

    pub fn safe_into_pairs<T>(self) -> Vec<(T, T)>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_pairs().unwrap()
    }

    pub fn into_vecs<T>(self) -> Result<Vec<Vec<T>>, anyhow::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        String::try_from(self)?
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|value| {
                        value
                            .parse()
                            .map_err(|e| anyhow::anyhow!("failed to parse \"{line}\": {e:?}"))
                    })
                    .collect()
            })
            .collect()
    }

    pub fn safe_into_vecs<T>(self) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_vecs().unwrap()
    }
}

impl TryFrom<Input> for String {
    type Error = anyhow::Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        input.data.ok_or(anyhow!("data not set"))
    }
}

pub fn safe_get_input(year: i32, day: u32) -> String {
    Input::year(year).day(day).safe_get().safe_into()
}

pub fn safe_get_input_as<T>(year: i32, day: u32) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into()
}

pub fn safe_get_input_as_multi<T>(year: i32, day: u32) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_multi()
}

pub fn safe_get_input_as_pairs<T>(year: i32, day: u32) -> Vec<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_pairs()
}

pub fn safe_get_input_as_vecs<T>(year: i32, day: u32) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_vecs()
}
