use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg};

use crate::num::{One, Zero, zero};
use crate::positioning::direction::four_points::Direction4;
use crate::positioning::direction::{Direction, MovementDirection};
use crate::positioning::pt::Pt;

/// A [turtle] moving around 2D space.
///
/// [turtle]: https://en.wikipedia.org/wiki/Logo_(programming_language)#Turtle_and_graphics
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Turtle<T = i64, Dir = Direction4> {
    pub position: Pt<T>,
    pub direction: Dir,
}

impl<T, Dir> Turtle<T, Dir> {
    /// Returns a new [`Turtle`] starting from the given position, facing the given direction.
    pub fn new(position: Pt<T>, direction: Dir) -> Self {
        Self { position, direction }
    }

    /// Returns a new [`Turtle`] starting at point (0, 0) facing the given direction.
    pub fn from_zero(direction: Dir) -> Self
    where
        Pt<T>: Zero,
    {
        Self::new(zero(), direction)
    }
}

impl<T, Dir> Turtle<T, Dir>
where
    Dir: Direction,
{
    /// Turns the [`Turtle`] 90 degrees to the left.
    pub fn turn_left(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_left(), ..*self }
    }

    /// Turns the [`Turtle`] 90 degrees to the right.
    pub fn turn_right(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_right(), ..*self }
    }

    /// Turns the [`Turtle`] around 180 degrees.
    pub fn turn_around(&self) -> Self
    where
        Pt<T>: Copy,
    {
        Self { direction: self.direction.turn_around(), ..*self }
    }
}

impl<T, Dir> Turtle<T, Dir>
where
    Dir: MovementDirection<T> + Copy,
{
    /// Advances the [`Turtle`] one step in the direction it is currently facing.
    pub fn advance(&self) -> Self
    where
        T: Zero + One + Neg<Output = T> + Add<Output = T>,
        Pt<T>: Copy,
    {
        Self { position: self.position + self.direction.displacement(), ..*self }
    }
}

impl<T, Dir> Display for Turtle<T, Dir>
where
    Pt<T>: Display,
    Dir: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ position: {}, direction: {} }}", self.position, self.direction)
    }
}
