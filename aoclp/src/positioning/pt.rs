use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, RangeBounds, Sub, SubAssign};
use std::str::FromStr;
use std::sync::OnceLock;

use strum::IntoEnumIterator;

use crate::captures::CapturesHelper;
use crate::num::{zero, Signed, Zero};
use crate::positioning::direction::eight_points::Direction8;
use crate::positioning::direction::four_points::Direction4;
use crate::positioning::direction::MovementDirection;
use crate::regex::Regex;

/// A point in 2D space.
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pt<T = i64> {
    pub x: T,
    pub y: T,
}

impl<T> Pt<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pt<T>
where
    Self: Add<Output = Self> + Copy,
    Direction4: MovementDirection<T>,
{
    pub fn four_neighbours(self) -> impl Iterator<Item = Self> {
        Direction4::iter().map(move |dir| self + dir.displacement())
    }
}

impl<T> Pt<T>
where
    Self: Add<Output = Self> + Copy,
    Direction8: MovementDirection<T>,
{
    pub fn eight_neighbours(self) -> impl Iterator<Item = Self> {
        Direction8::iter().map(move |dir| self + dir.displacement())
    }
}

impl<T> Pt<T>
where
    T: PartialOrd,
{
    pub fn within<XR, YR>(&self, x_bounds: XR, y_bounds: YR) -> bool
    where
        XR: RangeBounds<T>,
        YR: RangeBounds<T>,
    {
        x_bounds.contains(&self.x) && y_bounds.contains(&self.y)
    }
}

impl<T, U, V> From<(U, V)> for Pt<T>
where
    U: Into<T>,
    V: Into<T>,
{
    /// Converts from a 2-number tuple to a [`Pt`].
    fn from(value: (U, V)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

impl<T, U, V> From<Pt<T>> for (U, V)
where
    T: Into<U> + Into<V>,
{
    /// Converts from a [`Pt`] to a 2-number tuple.
    fn from(value: Pt<T>) -> Self {
        (value.x.into(), value.y.into())
    }
}

impl<T> FromStr for Pt<T>
where
    T: FromStr,
{
    type Err = Infallible;

    /// Parses a [`Pt`] from a string in the form `(x, y)`.
    /// Parentheses and whitespace are optional.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(r"\(?(?<x>-?\d+(?:\.\d*)?),\s*(?<y>-?\d+(?:\.\d*)?)\)?$").unwrap()
        });

        let captures = re
            .captures(s)
            .unwrap_or_else(|| panic!("invalid Pt value: {s}"));
        Ok(Self::new(captures.ez_get("x"), captures.ez_get("y")))
    }
}

impl<T> Display for Pt<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Pt<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Pt<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Pt<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Zero for Pt<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

/// Returns the [Manhattan distance] between two points in 2D space.
///
/// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan<T>(a: Pt<T>, b: Pt<T>) -> T
where
    T: Signed,
{
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Given a two-dimensional matrix of elements, returns a map of
/// [`Pt`] associated with the element at that position in the matrix.
pub fn matrix_to_map<M, R, T, PT>(matrix: M) -> HashMap<Pt<PT>, T>
where
    M: IntoIterator<Item = R>,
    R: IntoIterator<Item = T>,
    PT: TryFrom<usize>,
    <PT as TryFrom<usize>>::Error: Debug,
    Pt<PT>: Hash + Eq,
{
    matrix
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, t)| (Pt::new(x.try_into().unwrap(), y.try_into().unwrap()), t))
        })
        .collect()
}

/// Given a two-dimensional matrix of elements, returns a map of
/// [`Pt`] associated with the element at that position in the matrix,
/// ignoring any element that are in `skips`.
pub fn filtered_matrix_to_map<M, R, T, PT, S>(matrix: M, skips: S) -> HashMap<Pt<PT>, T>
where
    M: IntoIterator<Item = R>,
    R: IntoIterator<Item = T>,
    PT: TryFrom<usize>,
    <PT as TryFrom<usize>>::Error: Debug,
    Pt<PT>: Hash + Eq,
    S: IntoIterator<Item = T>,
    T: Hash + Eq,
{
    let skips: HashSet<_> = skips.into_iter().collect();

    matrix
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, t)| !skips.contains(t))
                .map(move |(x, t)| (Pt::new(x.try_into().unwrap(), y.try_into().unwrap()), t))
        })
        .collect()
}
