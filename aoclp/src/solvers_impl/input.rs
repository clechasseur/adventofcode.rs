use std::str::FromStr;

use anyhow::anyhow;
use aocf::Aoc;
use itertools::Itertools;

pub const DEFAULT_DATA_SEPARATORS: &[char] = &[' ', '\t', '|', ',', ':'];

#[derive(Debug)]
pub struct Input<'a> {
    year: i32,
    day: Option<u32>,
    force: bool,
    data: Option<String>,
    separators: &'a [char],
}

impl<'a> Input<'a> {
    pub fn year(year: i32) -> Self {
        Self { year, day: None, force: false, data: None, separators: DEFAULT_DATA_SEPARATORS }
    }

    pub fn day(mut self, day: u32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn for_example<S>(example: S) -> Self
    where
        S: Into<String>,
    {
        let mut input = Self::year(0);
        input.data = Some(example.into());
        input
    }

    pub fn separators(mut self, separators: &'a [char]) -> Self {
        self.separators = separators;
        self
    }

    pub fn get(mut self) -> crate::Result<Self> {
        let mut aoc = Aoc::new()
            .year(Some(self.year))
            .day(Some(self.day.ok_or(anyhow!("day not set"))?))
            .parse_cli(false)
            .init()
            .map_err(|e| anyhow!(e))?;

        self.data = Some(aoc.get_input(self.force).map_err(|e| anyhow!(e))?);
        Ok(self)
    }

    pub fn safe_get(self) -> Self {
        self.get().unwrap()
    }

    pub fn into<T>(self) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.data
            .ok_or(anyhow!("data not set"))?
            .parse()
            .map_err(|e| anyhow!("failed to parse data: {e:?}"))
    }

    pub fn safe_into<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into().unwrap()
    }

    pub fn into_many<T>(self) -> crate::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        String::try_from(self)?
            .lines()
            .map(|line| {
                line.parse()
                    .map_err(|e| anyhow!("failed to parse \"{line}\": {e:?}"))
            })
            .collect()
    }

    pub fn safe_into_many<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_many().unwrap()
    }

    pub fn into_many_pairs<T>(self) -> crate::Result<Vec<(T, T)>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        String::try_from(self)?
            .lines()
            .map(|line| {
                let (a, b) = line
                    .split(separators)
                    .filter(|value| !value.is_empty())
                    .collect_tuple()
                    .ok_or(anyhow!("invalid number of elements in \"{line}\""))?;
                Ok((
                    a.parse()
                        .map_err(|e| anyhow!("failed to parse \"{a}\": {e:?}"))?,
                    b.parse()
                        .map_err(|e| anyhow!("failed to parse \"{b}\": {e:?}"))?,
                ))
            })
            .collect()
    }

    pub fn safe_into_many_pairs<T>(self) -> Vec<(T, T)>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_many_pairs().unwrap()
    }

    pub fn into_many_vecs<T>(self) -> Result<Vec<Vec<T>>, crate::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        Self::parse_many_vecs(String::try_from(self)?.lines(), separators)
    }

    pub fn safe_into_many_vecs<T>(self) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.into_many_vecs().unwrap()
    }

    #[allow(clippy::type_complexity)]
    pub fn into_many_vecs_of_two_types<T, U>(self) -> crate::Result<(Vec<Vec<T>>, Vec<Vec<U>>)>
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        let separators = self.separators;
        let data: String = self.try_into()?;

        let first =
            Self::parse_many_vecs(data.lines().take_while(|line| !line.is_empty()), separators)?;
        let second = Self::parse_many_vecs(
            data.lines().skip_while(|line| !line.is_empty()).skip(1),
            separators,
        )?;
        Ok((first, second))
    }

    pub fn safe_into_many_vecs_of_two_types<T, U>(self) -> (Vec<Vec<T>>, Vec<Vec<U>>)
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        self.into_many_vecs_of_two_types().unwrap()
    }

    pub fn into_matrix<T>(self) -> crate::Result<Vec<Vec<T>>>
    where
        T: From<char>,
    {
        Ok(String::try_from(self)?
            .lines()
            .map(|line| line.chars().map(Into::into).collect())
            .collect())
    }

    pub fn safe_into_matrix<T>(self) -> Vec<Vec<T>>
    where
        T: From<char>,
    {
        self.into_matrix().unwrap()
    }

    fn parse_many_vecs<T, L, S>(lines: L, separators: &[char]) -> crate::Result<Vec<Vec<T>>>
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

impl<'a> TryFrom<Input<'a>> for String {
    type Error = crate::Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
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

pub fn safe_get_input_as_many<T>(year: i32, day: u32) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_many()
}

pub fn safe_get_input_as_many_pairs<T>(year: i32, day: u32) -> Vec<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_many_pairs()
}

pub fn safe_get_input_as_many_vecs<T>(year: i32, day: u32) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year).day(day).safe_get().safe_into_many_vecs()
}

pub fn safe_get_input_as_many_vecs_of_two_types<T, U>(
    year: i32,
    day: u32,
) -> (Vec<Vec<T>>, Vec<Vec<U>>)
where
    T: FromStr,
    U: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
    <U as FromStr>::Err: std::fmt::Debug,
{
    Input::year(year)
        .day(day)
        .safe_get()
        .safe_into_many_vecs_of_two_types()
}

pub fn safe_get_input_as_matrix<T>(year: i32, day: u32) -> Vec<Vec<T>>
where
    T: From<char>,
{
    Input::year(year).day(day).safe_get().safe_into_matrix()
}
