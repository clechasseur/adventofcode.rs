//! Helpers dealing with positioning: points, directions, etc.

pub mod direction;
pub mod pt;
pub mod pt_3d;
pub mod turtle;

/// Trait implemented by [`Pt`] and [`Pt3d`] with metadata about the point type.
///
/// [`Pt`]: pt::Pt
/// [`Pt3d`]: pt_3d::Pt3d
pub trait Point {
    /// Type used to represent a point's coordinates.
    type Coord;
}
