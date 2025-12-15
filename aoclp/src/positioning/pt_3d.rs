use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, RangeBounds, Sub, SubAssign};
use std::str::FromStr;
use std::sync::OnceLock;

use num::{NumCast, cast};

use crate::captures::CapturesHelper;
use crate::num::{Signed, Zero, zero};
use crate::regex::Regex;

/// A point in 3D space.
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pt3d<T = i64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pt3d<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Pt3d<T>
where
    T: PartialOrd,
{
    /// Checks if this [`Pt3d`] is within the given bounds.
    pub fn within<XR, YR, ZR>(&self, x_bounds: XR, y_bounds: YR, z_bounds: ZR) -> bool
    where
        XR: RangeBounds<T>,
        YR: RangeBounds<T>,
        ZR: RangeBounds<T>,
    {
        x_bounds.contains(&self.x) && y_bounds.contains(&self.y) && z_bounds.contains(&self.z)
    }
}

impl<T> Pt3d<T>
where
    T: NumCast,
{
    /// Casts this [`Pt3d`] to another numeric type.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoclp::positioning::pt_3d::Pt3d;
    ///
    /// let p: Pt3d<i32> = Pt3d::new(42, 23, 11);
    /// let fp: Pt3d<f64> = p.cast();
    /// assert_eq!(Pt3d::new(42.0, 23.0, 11.0), fp);
    /// ```
    pub fn cast<U>(self) -> Pt3d<U>
    where
        U: NumCast,
    {
        Pt3d::new(cast(self.x).unwrap(), cast(self.y).unwrap(), cast(self.z).unwrap())
    }
}

impl<T, U, V, W> From<(U, V, W)> for Pt3d<T>
where
    U: Into<T>,
    V: Into<T>,
    W: Into<T>,
{
    /// Converts form a 3-number tuple to a [`Pt3d`].
    fn from(value: (U, V, W)) -> Self {
        Self::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl<T, U, V, W> From<Pt3d<T>> for (U, V, W)
where
    T: Into<U> + Into<V> + Into<W>,
{
    /// Converts from a [`Pt3d`] to a 3-number tuple.
    fn from(value: Pt3d<T>) -> Self {
        (value.x.into(), value.y.into(), value.z.into())
    }
}

impl<T> FromStr for Pt3d<T>
where
    T: FromStr,
{
    type Err = Infallible;

    /// Parses a [`Pt3d`] from a string in the form `(x, y, z)`.
    /// Parentheses and whitespace are optional.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(
                r"\(?(?<x>-?\d+(?:\.\d*)?),\s*(?<y>-?\d+(?:\.\d*)?),\s*(?<z>-?\d+(?:\.\d*)?)\)?$",
            )
            .unwrap()
        });

        let captures = re
            .captures(s)
            .unwrap_or_else(|| panic!("invalid Pt3d value: {s}"));
        Ok(Self::new(captures.ez_get("x"), captures.ez_get("y"), captures.ez_get("z")))
    }
}

impl<T> Display for Pt3d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> Add for Pt3d<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> AddAssign for Pt3d<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Pt3d<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> SubAssign for Pt3d<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Zero for Pt3d<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(zero(), zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

/// Returns the [Manhattan distance] between two points in 3D space.
///
/// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan<T>(a: Pt3d<T>, b: Pt3d<T>) -> T
where
    T: Signed,
{
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

/// Returns the square of the [Euclidian distance] between two points in 3D space.
///
/// [Euclidian distance]: https://en.wikipedia.org/wiki/Euclidean_distance
pub fn euclidian_squared<T>(a: Pt3d<T>, b: Pt3d<T>) -> T
where
    T: Signed + Clone,
{
    let (x, y, z) = ((a.x - b.x).abs(), (a.y - b.y).abs(), (a.z - b.z).abs());
    (x.clone() * x) + (y.clone() * y) + (z.clone() * z)
}
