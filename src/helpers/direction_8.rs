use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use num::{one, zero, One, Zero};
use strum::{Display, EnumCount, EnumIter, FromRepr};

use crate::helpers::pt::Pt;

/// ↑ ⌝ → ⌟ ↓ ⌞ ← ⌜
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumCount, Display, EnumIter,
)]
pub enum Direction8 {
    Right,
    BottomRight,
    Down,
    BottomLeft,
    Left,
    TopLeft,
    Up,
    TopRight,
}

impl Direction8 {
    /// Turns 45 degrees to the left.
    pub fn turn_left(&self) -> Self {
        Self::from_repr(((*self as u8) + 7) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns 45 degrees to the right.
    pub fn turn_right(&self) -> Self {
        Self::from_repr(((*self as u8) + 1) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns around (e.g. performs a 180 degrees turn).
    pub fn turn_around(&self) -> Self {
        Self::from_repr(((*self as u8) + 4) % (Self::COUNT as u8)).unwrap()
    }

    /// Returns the displacement to apply to move one step in this direction.
    /// The displacement is returned as a [`Pt`].
    ///
    /// # Notes
    ///
    /// Because this enum is meant to be used to move around a map represented as a series of rows
    /// like on a computer screen, `Up`'s displacement will _subtract_ one from the Y axis, while
    /// `Down`'s will _add_ one to the Y axis.
    pub fn displacement<T>(&self) -> Pt<T>
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            Self::Right => Pt::new(one(), zero()),
            Self::BottomRight => Pt::new(one(), one()),
            Self::Down => Pt::new(zero(), one()),
            Self::BottomLeft => Pt::new(-one::<T>(), one()),
            Self::Left => Pt::new(-one::<T>(), zero()),
            Self::TopLeft => Pt::new(-one::<T>(), -one::<T>()),
            Self::Up => Pt::new(zero(), -one::<T>()),
            Self::TopRight => Pt::new(one(), -one::<T>()),
        }
    }
}

impl<T> Add<Direction8> for Pt<T>
where
    T: Zero + One + Neg<Output = T>,
    Pt<T>: Add<Pt<T>, Output = Pt<T>>,
{
    type Output = Pt<T>;

    fn add(self, rhs: Direction8) -> Self::Output {
        self + rhs.displacement()
    }
}

impl<T> AddAssign<Direction8> for Pt<T>
where
    T: Zero + One + Neg<Output = T>,
    Pt<T>: AddAssign<Pt<T>>,
{
    fn add_assign(&mut self, rhs: Direction8) {
        *self += rhs.displacement();
    }
}

impl<T> Sub<Direction8> for Pt<T>
where
    T: Zero + One + Neg<Output = T>,
    Pt<T>: Sub<Pt<T>, Output = Pt<T>>,
{
    type Output = Pt<T>;

    fn sub(self, rhs: Direction8) -> Self::Output {
        self - rhs.displacement()
    }
}

impl<T> SubAssign<Direction8> for Pt<T>
where
    T: Zero + One + Neg<Output = T>,
    Pt<T>: SubAssign<Pt<T>>,
{
    fn sub_assign(&mut self, rhs: Direction8) {
        *self -= rhs.displacement();
    }
}
