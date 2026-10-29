//! Helpers to interact with the Advent of Code website/API.
//!
//! To fetch puzzle data, see [`Input`].

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

/// Default [`separators`] used by [`Input`] when splitting input data lines into parts.
///
/// [`separators`]: Input::separators
pub const DEFAULT_INPUT_SEPARATORS: &[char] = &[' ', '\t', '|', ',', ':'];

/// Base Advent of Code website URL. Will be used unless overridden via [`AOC_BASE_URL_ENV_VAR`].
pub const AOC_BASE_URL: &str = "https://adventofcode.com";

/// Environment variable that can be used to override the [base Advent of Code website URL].
///
/// [base Advent of Code website URL]: AOC_BASE_URL
pub const AOC_BASE_URL_ENV_VAR: &str = "AOCLP_AOC_BASE_URL";

/// Environment variable that can be used to store the Advent of Code session cookie.
///
/// A valid session cookie is required to fetch puzzle input data from the AoC website.
/// If not provided via this environment variable, must be set via [`session`].
///
/// [`session`]: Input::session
pub const AOC_SESSION_ENV_VAR: &str = "AOCLP_AOC_SESSION";

/// Environment variable that can be used to override the path where the input cache is stored.
///
/// The input cache is used to persist puzzle input data to avoid fetching it multiple times.
/// If not specified via the environment, the [default value] will be used.
///
/// [default value]: DEFAULT_CACHE_PATH
pub const CACHE_PATH_ENV_VAR: &str = "AOCLP_CACHE_PATH";

/// Default path of the directory where input cache data is stored. Relative to the project root.
pub const DEFAULT_CACHE_PATH: &str = ".aoclp";

/// Key used to uniquely identify a puzzle, via its year and day.
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleId {
    /// Puzzle year.
    pub year: i32,

    /// Puzzle day.
    pub day: u32,
}

/// Helper to fetch puzzle input data from the Advent of Code website.
///
/// ### Input data for puzzle
///
/// To fetch input data for a specific puzzle, use [`for_puzzle`]:
///
/// ```no_run
/// # use aoclp::aoc::Input;
///
/// pub fn part_1() -> i64 {
///     let input = Input::for_puzzle(2025, 1).into_string();
///     // ...use input here...
///     # 0
/// }
/// ```
///
/// The call to [`into_string`] fetches the data from AoC and caches it locally on disk before
/// returning it. Subsequent requests for the same input data will fetch the data from the disk
/// cache instead of connecting to AoC directly.
///
/// ### Input data for example
///
/// AoC puzzles usually contain examples of input data and the result you would get if you used them.
/// It's possible to test an algorithm with example data with [`for_example`]:
///
/// ```no_run
/// # use aoclp::aoc::Input;
///
/// const EXAMPLE: &str = "...";
///
/// pub fn part_1() -> i64 {
///     let input = Input::for_example(EXAMPLE).into_string();
///     // ...use example data here...
///     # 0
/// }
/// ```
///
/// ### AoC session cookie
///
/// In order to fetch puzzle data for your account, this helper requires a session cookie for the
/// Advent of Code website. This cookie can be retrieved from the browser's cookies and is named
/// `session`. For more details, follow [these instructions].
///
/// Once you have your session cookie, store it in an environment variable named [`AOCLP_AOC_SESSION`]
/// and this helper will use it automatically. You can also set the session cookie explicitly via
/// [`session`].
///
/// ### Input parsing
///
/// This helper provides methods to convert the data, initially read as a string, into other types.
/// Any type implementing [`FromStr`] can be parsed automatically via [`into`]:
///
/// ```no_run
/// # use std::convert::Infallible;
/// # use std::str::FromStr;
/// # use itertools::Itertools;
/// # use aoclp::aoc::Input;
///
/// fn input_as_i64() -> i64 {
///     Input::for_puzzle(2017, 1).into()
/// }
///
/// struct MyPoint(i64, i64);
///
/// impl FromStr for MyPoint {
///     type Err = Infallible;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         let (x, y) = s
///             .split(',')
///             .map(|n| n.parse().unwrap())
///             .collect_tuple()
///             .unwrap();
///         Ok(Self(x, y))
///     }
/// }
///
/// fn input_as_custom_type() -> MyPoint {
///     Input::for_puzzle(2025, 2).into()
/// }
/// ```
///
/// There are also various helpers to parse the input data in common formats, like [`lines_into`] to
/// parse each line as a different value, or [`split_into`] to split a line into multiple values
/// separated by the [`separators`].
///
/// ### Do _and_ do not: there is `try`
///
/// Every parsing method comes in two flavours: a `try_...` version (ex: [`try_lines_into`]) that
/// returns an error if something goes wrong, and a regular version (ex: [`lines_into`]) that
/// panics instead. Given that this is meant to be used for running AoC solutions locally, the
/// latter might be acceptable to most.
///
/// [`for_puzzle`]: Self::for_puzzle
/// [`into_string`]: Self::into_string
/// [these instructions]: https://github.com/clechasseur/aocf/blob/efe2bdef7c90735f5ffd1f4370f54d6817b9c9ba/cookie.md
/// [`AOCLP_AOC_SESSION`]: AOC_SESSION_ENV_VAR
/// [`session`]: Self::session
/// [`into`]: Self::into
/// [`lines_into`]: Self::lines_into
/// [`split_into`]: Self::split_into
/// [`separators`]: Self::separators
/// [`try_lines_into`]: Self::try_lines_into
#[derive(Debug, Clone)]
pub struct Input<'a> {
    puzzle: Option<PuzzleId>,
    force: bool,
    session: Option<String>,
    data: Option<String>,
    separators: &'a [char],
}

impl<'a> Input<'a> {
    /// Creates an input helper for a specific AoC puzzle.
    ///
    /// Note: this will not fetch the data right away; data is fetched from the AoC website
    /// only when it is requested in a specific format (via [`into_string`], for example).
    ///
    /// [`into_string`]: Self::into_string
    pub fn for_puzzle(year: i32, day: u32) -> Self {
        Self {
            puzzle: Some(PuzzleId { year, day }),
            force: false,
            session: None,
            data: None,
            separators: DEFAULT_INPUT_SEPARATORS,
        }
    }

    /// Creates an input helper for a puzzle example.
    pub fn for_example<S>(example: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            puzzle: None,
            force: false,
            session: None,
            data: Some(example.into()),
            separators: DEFAULT_INPUT_SEPARATORS,
        }
    }

    /// Sets whether this helper should fetch puzzle data from the AoC website, even if it
    /// finds existing cached data.
    ///
    /// This should not be necessary in general unless the cached data is corrupted, since
    /// puzzle data does not change by account.
    ///
    /// ```no_run
    /// # use aoclp::aoc::Input;
    ///
    /// pub fn part_1() -> i64 {
    ///     let input = Input::for_puzzle(2025, 1).force(true).into_string();
    ///     // ...use input here...
    ///     # 0
    /// }
    /// ```
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Overrides the session cookie used to authenticate to the Aoc website.
    ///
    /// If not specified, the session cookie must be set via [`AOC_SESSION_ENV_VAR`].
    ///
    /// ```no_run
    /// # use aoclp::aoc::Input;
    ///
    /// const SESSION_COOKIE: &str = "...";
    ///
    /// pub fn part_1() -> i64 {
    ///     let input = Input::for_puzzle(2025, 1)
    ///         .session(SESSION_COOKIE)
    ///         .into_string();
    ///     // ...use input here...
    ///     # 0
    /// }
    /// ```
    pub fn session<S>(mut self, session: S) -> Self
    where
        S: Into<String>,
    {
        self.session = Some(session.into());
        self
    }

    /// Overrides the list of separators used to split lines.
    ///
    /// Separators are used by methods like [`split_lines_into`].
    ///
    /// If not set, the [`DEFAULT_INPUT_SEPARATORS`] are used.
    ///
    /// [`split_lines_into`]: Self::split_lines_into
    pub fn separators(mut self, separators: &'a [char]) -> Self {
        self.separators = separators;
        self
    }

    /// Tries fetching the input data from the AoC website. If successful,
    /// the data is cached on disk.
    ///
    /// If the data already exists on disk, it is read from the cache instead
    /// of contacting the AoC website. If the data has already been fetched
    /// via a previous call to [`try_get`] or [`get`], the same data is returned.
    ///
    /// ## Note
    ///
    /// It is not necessary to call this method explicitly - all the parsing
    /// methods like [`into_string`] call this to get the data, which means it
    /// will be fetched automatically.
    ///
    /// [`try_get`]: Self::try_get
    /// [`get`]: Self::get
    /// [`into_string`]: Self::into_string
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

    /// Tries to fetch the data from the AoC website using [`try_get`].
    /// Panics if an error occurs.
    ///
    /// ## Note
    ///
    /// It is not necessary to call this method explicitly - all the parsing
    /// methods like [`into_string`] call this to get the data, which means it
    /// will be fetched automatically.
    ///
    /// [`into_string`]: Self::into_string
    pub fn get(self) -> Self {
        self.try_get().expect("failed to get AOC puzzle input")
    }

    /// Attempts to parse the entire input data into a value that implements
    /// `FromStr`. Returns an error if parsing fails.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// ## Note
    ///
    /// If you want the raw input data for manual parsing, [`into_string`] might be
    /// easier to use.
    ///
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    /// [`into_string`]: Self::into_string
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

    /// Parse the entire input data into a value that implements `FromStr`.
    /// Panics if parsing fails.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use aoclp::aoc::Input;
    ///
    /// fn input() -> usize {
    ///     Input::for_puzzle(2017, 3).into()
    /// }
    /// ```
    ///
    /// ## Note
    ///
    /// If you want the raw input data for manual parsing, [`into_string`] might be
    /// easier to use.
    ///
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    /// [`into_string`]: Self::into_string
    pub fn into<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_into().unwrap()
    }

    /// Returns the entire input data, unparsed.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use aoclp::aoc::Input;
    ///
    /// fn input() -> String {
    ///     Input::for_puzzle(2017, 3).into_string()
    /// }
    /// ```
    ///
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    pub fn into_string(self) -> String {
        self.try_into()
            .expect("puzzle data should always be convertible to a string")
    }

    /// Splits the input data into lines and attempts to parse each line into a
    /// value that implements `FromStr`. Returns an error if parsing fails for any
    /// single line.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    pub fn try_lines_into<T>(self) -> crate::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        Self::parse_lines(self.into_string().lines())
    }

    /// Splits the input data into lines and parses each line into a value that implements
    /// `FromStr`. Returns the list of resulting values. Panics if parsing fails for any
    /// single line.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use std::convert::Infallible;
    /// # use std::str::FromStr;
    ///
    /// # use aoclp::aoc::Input;
    ///
    /// #[derive(Debug)]
    /// pub struct ProgramSpec {
    ///     pub name: String,
    ///     pub weight: usize,
    ///     pub sub_programs: Vec<String>,
    /// }
    ///
    /// impl FromStr for ProgramSpec {
    ///     type Err = Infallible;
    ///
    ///     fn from_str(s: &str) -> Result<Self, Self::Err> {
    ///         // ...parsing implementation...
    ///         # Ok(Self { name: "pbga".into(), weight: 66, sub_programs: vec![] })
    ///     }
    /// }
    ///
    /// fn input() -> Vec<ProgramSpec> {
    ///     Input::for_puzzle(2017, 7).lines_into()
    /// }
    /// ```
    ///
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    pub fn lines_into<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_lines_into().unwrap()
    }

    /// Splits the input data's only line using the [`separators`] and attempts to parse
    /// each element into a value implementing `FromStr`. Returns an error if the input
    /// data is empty, if it contains more than one line or if parsing fails for any
    /// single element.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// [`separators`]: Self::separators
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    pub fn try_split_into<T>(self) -> Result<Vec<T>, crate::Error>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_lines_into().and_then(|vecs| {
            vecs.into_iter()
                .exactly_one()
                .map_err(|_| anyhow!("expected one line"))
        })
    }

    /// Splits the input data's only line using the [`separators`] and parses each element
    /// into a value implementing `FromStr`. Panics if the input data is empty, if it contains
    /// more than one line or if parsing fails for any single element.
    ///
    /// Calls [`try_get`] to get the input data; if [`for_puzzle`] was used, this
    /// will fetch the puzzle data from the AoC website automatically.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use aoclp::aoc::Input;
    ///
    /// #[derive(Debug, Copy, Clone, strum::EnumString)]
    /// #[strum(serialize_all = "snake_case")]
    /// enum HexDirection {
    ///     NW,
    ///     N,
    ///     NE,
    ///     SE,
    ///     S,
    ///     SW,
    /// }
    ///
    /// fn input() -> Vec<HexDirection> {
    ///     Input::for_puzzle(2017, 11).split_into()
    /// }
    /// ```
    ///
    /// [`separators`]: Self::separators
    /// [`try_get`]: Self::try_get
    /// [`for_puzzle`]: Self::for_puzzle
    pub fn split_into<T>(self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.try_split_into().unwrap()
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

    pub fn try_two_types_of_lines_into<T, U>(self) -> crate::Result<(Vec<T>, Vec<U>)>
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        let data = self.into_string();

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
        let data = self.into_string();

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

        if !self.force
            && let Some(input) = INPUT_CACHE.get(cache_key.as_str())?
        {
            return Ok(input);
        }

        let client = Client::builder()
            .user_agent(Self::http_user_agent())
            .build()
            .with_context(|| "failed to build AOC HTTP client")?;

        let url = self.aoc_input_url()?;
        let aoc_session = self.aoc_session()?;
        let cookie = format!("session={}", aoc_session);
        let input = client
            .get(url)
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

    fn aoc_session(&self) -> crate::Result<String> {
        if let Some(session) = self.session.as_ref() {
            return Ok(session.clone());
        }

        match env::var(AOC_SESSION_ENV_VAR) {
            Ok(aoc_session) => Ok(aoc_session),
            Err(VarError::NotPresent) => Err(anyhow!(
                "AOC session cookie needs to be set via `session` or passed via the {AOC_SESSION_ENV_VAR} environment variable"
            )),
            Err(err) => Err(anyhow!("failed to get AOC session cookie from environment: {err}")),
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
