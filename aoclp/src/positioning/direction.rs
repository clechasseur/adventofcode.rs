pub mod eight_points;
pub mod four_points;

use crate::positioning::pt::Pt;

pub trait Direction {
    fn turn_left(&self) -> Self;
    fn turn_right(&self) -> Self;
    fn turn_around(&self) -> Self;
}

pub trait MovementDirection<T>: Direction {
    fn displacement(&self) -> Pt<T>;
}
