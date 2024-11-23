use std::ops::Neg;

use num::{one, zero, One, Zero};
use strum::{Display, EnumCount, FromRepr};

use crate::helpers::pt::Pt;

/// ↓ ↑ ← →
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumCount, Display,
)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    /// Turns 90 degrees to the left.
    pub fn turn_left(&self) -> Self {
        Self::from_repr(((*self as u8) + 3) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns 90 degrees to the right.
    pub fn turn_right(&self) -> Self {
        Self::from_repr(((*self as u8) + 1) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns around (e.g. performs a 180 degrees turn).
    pub fn turn_around(&self) -> Self {
        Self::from_repr(((*self as u8) + 2) % (Self::COUNT as u8)).unwrap()
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
            Direction::Right => Pt::new(one(), zero()),
            Direction::Down => Pt::new(zero(), one()),
            Direction::Left => Pt::new(-one::<T>(), zero()),
            Direction::Up => Pt::new(zero(), -one::<T>()),
        }
    }
}
