pub mod direction;
pub mod pt;
pub mod pt_3d;
pub mod turtle;

/// Trait implemented by [`Pt`](pt::Pt) and [`Pt3d`](pt_3d::Pt3d) with
/// metadata about the point type.
pub trait Point {
    /// Type used to represent a point's coordinates.
    type Coord;
}
