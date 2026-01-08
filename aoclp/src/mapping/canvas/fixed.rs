use std::array;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{repeat_n, successors};
use std::ops::{Index, IndexMut};

use itertools::Itertools;
use num::ToPrimitive;

use crate::positioning::Point;
use crate::positioning::pt::{Pt, matrix_to_map};

/// A fixed-size rectangular canvas in 2D space that can be rotated and flipped,
/// stored in a matrix of arrays.
///
/// If the type of canvas pieces `T` is `Copy`, operations should not incur any allocation or cloning.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Canvas<T, const W: usize, const H: usize = W>(pub [[T; W]; H]);

// noinspection DuplicatedCode
impl<T, const W: usize, const H: usize> Canvas<T, W, H> {
    /// Creates a new [`Canvas`] from a two-dimensional matrix of fixed-size arrays.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let matrix = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let canvas = Canvas::from_array_matrix(matrix);
    /// assert_eq!([[1, 2, 3], [4, 5, 6]], canvas.0);
    /// ```
    pub fn from_array_matrix(matrix: [[T; W]; H]) -> Self {
        Self(matrix)
    }

    /// Creates a new [`Canvas`] from a two-dimensional matrix of pieces.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    ///
    /// let canvas = Canvas::from_matrix(matrix);
    /// assert_eq!([[1, 2, 3], [4, 5, 6]], canvas.0);
    /// ```
    ///
    /// [iterator]: Iterator
    pub fn from_matrix<IR, IT>(matrix: IR) -> Self
    where
        IR: IntoIterator<Item = IT>,
        IT: IntoIterator<Item = T>,
    {
        Self(
            matrix
                .into_iter()
                .map(|r| r.into_iter().collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    /// Creates a new [`Canvas`] from lines of text, using a closure to convert every `char`
    /// into a piece of canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let matrix = "123\n\
    ///               456";
    ///
    /// let canvas = Canvas::from_lines(matrix.lines(), |c| c.to_digit(10).unwrap());
    /// assert_eq!([[1, 2, 3], [4, 5, 6]], canvas.0);
    /// ```
    pub fn from_lines<F, I, S>(lines: I, mut f: F) -> Self
    where
        F: FnMut(char) -> T,
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self(
            lines
                .into_iter()
                .map(move |s| s.as_ref().chars().map(&mut f).collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    /// Creates a new [`Canvas`] from lines of text, using a closure to convert every byte
    /// into a piece of canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let matrix = "123\n\
    ///               456";
    ///
    /// let canvas = Canvas::from_line_bytes(matrix.lines(), |c| c - b'0');
    /// assert_eq!([[1, 2, 3], [4, 5, 6]], canvas.0);
    /// ```
    pub fn from_line_bytes<F, I, S>(lines: I, mut f: F) -> Self
    where
        F: FnMut(u8) -> T,
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self(
            lines
                .into_iter()
                .map(move |s| s.as_ref().bytes().map(&mut f).collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    /// Returns the width of the canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(3, canvas.width());
    /// ```
    pub const fn width(&self) -> usize {
        W
    }

    /// Returns the height of the canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(2, canvas.height());
    /// ```
    pub const fn height(&self) -> usize {
        H
    }

    /// Returns the number of pieces in the canvas ([width] multiplied by [height]).
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(6, canvas.count());
    /// ```
    ///
    /// [width]: Self::width
    /// [height]: Self::height
    pub const fn count(&self) -> usize {
        H * W
    }

    /// Returns an [iterator] that iterates the canvas' rows. Each row is itself an
    /// [iterator] that returns references to the canvas pieces.
    ///
    /// [iterator]: DoubleEndedIterator
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::identity;
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(
    ///     vec![1, 2, 3, 4, 5, 6],
    ///     canvas
    ///         .iter()
    ///         .flat_map(identity)
    ///         .copied()
    ///         .collect::<Vec<_>>()
    /// );
    /// ```
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        self.0.iter().map(|row| row.iter())
    }

    /// Flips the canvas horizontally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!([[4, 5, 6], [1, 2, 3]], canvas.flip_horizontally().0);
    /// ```
    pub fn flip_horizontally(self) -> Self {
        Self(self.0.into_iter().rev().collect_array().unwrap())
    }

    /// Flips this canvas vertically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!([[3, 2, 1], [6, 5, 4]], canvas.flip_vertically().0);
    /// ```
    pub fn flip_vertically(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|l| l.into_iter().rev().collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    /// Rotates this canvas 90 degrees to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!([[3, 6], [2, 5], [1, 4]], canvas.rotate_left().0);
    /// ```
    pub fn rotate_left(self) -> Canvas<T, H, W> {
        Canvas(self.into_columns().rev().collect_array().unwrap())
    }

    /// Converts the canvas into an [iterator] of its columns, from left to right.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![[1, 4], [2, 5], [3, 6]], canvas.into_columns().collect::<Vec<_>>());
    /// ```
    ///
    /// [iterator]: DoubleEndedIterator
    pub fn into_columns(self) -> impl DoubleEndedIterator<Item = [T; H]> {
        let mut rows: [_; H] = self
            .0
            .into_iter()
            .map(|r| r.into_iter())
            .collect_array()
            .unwrap();
        let cols: [_; W] = (0..W)
            .map(move |_| {
                rows.iter_mut()
                    .map(|r| r.next().unwrap())
                    .collect_array()
                    .unwrap()
            })
            .collect_array()
            .unwrap();
        cols.into_iter()
    }

    /// Converts the canvas into a [map](HashMap) associating each canvas piece
    /// to its corresponding [point](Pt).
    ///
    /// The top-left corner of the canvas is at position `(0, 0)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// let map = canvas.into_map();
    /// assert_eq!(
    ///     HashMap::from([
    ///         (Pt::new(0, 0), 1),
    ///         (Pt::new(1, 0), 2),
    ///         (Pt::new(2, 0), 3),
    ///         (Pt::new(0, 1), 4),
    ///         (Pt::new(1, 1), 5),
    ///         (Pt::new(2, 1), 6),
    ///     ]),
    ///     map,
    /// );
    /// ```
    pub fn into_map<PT>(self) -> HashMap<Pt<PT>, T>
    where
        PT: TryFrom<usize>,
        <PT as TryFrom<usize>>::Error: Debug,
        Pt<PT>: Copy + Hash + Eq,
    {
        matrix_to_map(self.0)
    }
}

impl<T, const W: usize, const H: usize> Canvas<T, W, H>
where
    T: Clone,
{
    /// Creates a new [`Canvas`] with every piece set to the same `value`.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::<_, 3, 2>::of('#');
    /// assert_eq!([['#', '#', '#'], ['#', '#', '#']], canvas.0);
    /// ```
    pub fn of(value: T) -> Self {
        Self(
            (0..H)
                .map(move |_| repeat_n(value.clone(), W).collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }
}

// noinspection DuplicatedCode
impl<T, const N: usize> Canvas<T, N, N>
where
    T: Clone,
{
    /// Consumes the canvas and returns an [iterator](Iterator) of all possible 90-degrees
    /// rotations of this canvas.
    ///
    /// The iterator's first element is this canvas. The other elements are generated by
    /// repeatedly [rotating the canvas 90 degrees to the left](Self::rotate_left), cloning
    /// the canvas pieces. The iterator always contains four (4) elements.
    ///
    /// Because the iterator needs to return the same type for all elements, this method requires
    /// the canvas to be square.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2], [3, 4]]);
    ///
    /// let mut rotations = canvas.into_rotations();
    /// assert_eq!([[1, 2], [3, 4]], rotations.next().unwrap().0);
    /// assert_eq!([[2, 4], [1, 3]], rotations.next().unwrap().0);
    /// assert_eq!([[4, 3], [2, 1]], rotations.next().unwrap().0);
    /// assert_eq!([[3, 1], [4, 2]], rotations.next().unwrap().0);
    /// assert!(rotations.next().is_none());
    /// ```
    pub fn into_rotations(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |p| Some(p.clone().rotate_left())).take(4)
    }

    /// Consumes the canvas and returns an [iterator](Iterator) of all variations of this canvas:
    /// flipped horizontally, vertically and rotated in all possible directions.
    ///
    /// Because the iterator needs to return the same type for all elements, this method requires
    /// the canvas to be square.
    ///
    /// # Notes
    ///
    /// Depending on the pattern formed by the canvas pieces, it's possible that this iterator
    /// will return a particular canvas variation multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    /// # use itertools::Itertools;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2], [3, 4]]);
    ///
    /// assert_eq!(16, canvas.clone().into_variations().count());
    /// assert_eq!(8, canvas.clone().into_variations().unique().count());
    /// assert!(
    ///     canvas
    ///         .into_variations()
    ///         .find(|c| { *c == Canvas::from_array_matrix([[1, 3], [2, 4]]) })
    ///         .is_some()
    /// );
    /// ```
    pub fn into_variations(self) -> impl Iterator<Item = Self> {
        vec![self.clone(), self.flip_horizontally()]
            .into_iter()
            .flat_map(|c| vec![c.clone(), c.flip_vertically()])
            .flat_map(Self::into_rotations)
    }
}

impl<T, const W: usize, const H: usize> Default for Canvas<T, W, H>
where
    T: Default + Clone,
{
    /// Returns a [`Canvas`] filled with the default piece value.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::<i32, 3, 2>::default();
    /// assert_eq!([[0, 0, 0], [0, 0, 0]], canvas.0);
    /// ```
    fn default() -> Self {
        Self::of(T::default())
    }
}

// noinspection DuplicatedCode
impl<T, const W: usize, const H: usize, PT> Index<PT> for Canvas<T, W, H>
where
    PT: Into<Pt>,
    <Pt as Point>::Coord: ToPrimitive,
{
    type Output = T;

    /// Returns a reference to the canvas piece at the given [point](Pt).
    /// Also works with coordinate tuples.
    ///
    /// The top-left canvas piece is at position `(0, 0)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(5, canvas[Pt::new(1, 1)]);
    /// assert_eq!(3, canvas[(2, 0)]);
    /// ```
    fn index(&self, index: PT) -> &Self::Output {
        let index = index.into();
        &self.0[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

// noinspection DuplicatedCode
impl<T, const W: usize, const H: usize, PT> IndexMut<PT> for Canvas<T, W, H>
where
    PT: Into<Pt>,
    <Pt as Point>::Coord: ToPrimitive,
{
    /// Returns a mutable reference the canvas piece at the given [point](Pt).
    /// Also works with coordinate tuples.
    ///
    /// The top-left canvas piece is at position `(0, 0)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let mut canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// canvas[Pt::new(1, 1)] = 9;
    /// canvas[(2, 0)] = 7;
    /// assert_eq!([[1, 2, 7], [4, 9, 6]], canvas.0);
    /// ```
    fn index_mut(&mut self, index: PT) -> &mut Self::Output {
        let index = index.into();
        &mut self.0[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

impl<T, const W: usize, const H: usize> IntoIterator for Canvas<T, W, H> {
    type Item = array::IntoIter<T, W>;
    type IntoIter = array::IntoIter<Self::Item, H>;

    /// Converts the [`Canvas`] into an [iterator] of its rows. Each row is itself an
    /// [iterator] that returns canvas pieces.
    ///
    /// [iterator]: Iterator
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::identity;
    /// # use aoclp::mapping::canvas::fixed::Canvas;
    ///
    /// let canvas = Canvas::from_array_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], canvas.into_iter().flat_map(identity).collect::<Vec<_>>());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(<_>::into_iter)
            .collect_array()
            .unwrap()
            .into_iter()
    }
}
