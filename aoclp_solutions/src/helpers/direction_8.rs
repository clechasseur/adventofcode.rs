use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use num::{one, zero, One, Zero};
use strum::{Display, EnumCount, EnumIter, EnumProperty, FromRepr};

use crate::helpers::direction::{Direction, MovementDirection};
use crate::helpers::pt::Pt;

/// ↑ ⌝ → ⌟ ↓ ⌞ ← ⌜
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumCount,
    Display,
    EnumIter,
    EnumProperty,
)]
pub enum Direction8 {
    #[strum(props(emoji = "→"))]
    Right,
    #[strum(props(emoji = "⌟"))]
    BottomRight,
    #[strum(props(emoji = "↓"))]
    Down,
    #[strum(props(emoji = "⌞"))]
    BottomLeft,
    #[strum(props(emoji = "←"))]
    Left,
    #[strum(props(emoji = "⌜"))]
    TopLeft,
    #[strum(props(emoji = "↑"))]
    Up,
    #[strum(props(emoji = "⌝"))]
    TopRight,
}

impl Direction8 {
    pub fn emoji(&self) -> char {
        self.get_str("emoji").unwrap().chars().next().unwrap()
    }
}

impl Direction for Direction8 {
    /// Turns 45 degrees to the left.
    fn turn_left(&self) -> Self {
        Self::from_repr(((*self as u8) + 7) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns 45 degrees to the right.
    fn turn_right(&self) -> Self {
        Self::from_repr(((*self as u8) + 1) % (Self::COUNT as u8)).unwrap()
    }

    /// Turns around (e.g. performs a 180 degrees turn).
    fn turn_around(&self) -> Self {
        Self::from_repr(((*self as u8) + 4) % (Self::COUNT as u8)).unwrap()
    }
}

impl<T> MovementDirection<T> for Direction8
where
    T: Zero + One + Neg<Output = T>,
{
    /// Returns the displacement to apply to move one step in this direction.
    /// The displacement is returned as a [`Pt`].
    ///
    /// # Notes
    ///
    /// Because this enum is meant to be used to move around a map represented as a series of rows
    /// like on a computer screen, `Up`'s displacement will _subtract_ one from the Y axis, while
    /// `Down`'s will _add_ one to the Y axis.
    fn displacement(&self) -> Pt<T> {
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
