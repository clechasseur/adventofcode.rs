use std::collections::HashMap;
use std::iter::{repeat_n, successors};
use std::ops::{Index, IndexMut};
use std::vec;

use itertools::Itertools;
use num::ToPrimitive;

use crate::positioning::Point;
use crate::positioning::pt::{Pt, matrix_to_map};

/// A fixed-size rectangular canvas in 2D space that can be rotated and flipped,
/// stored in dynamically-allocated [`Vec`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Canvas<T>(pub Vec<Vec<T>>);

// noinspection DuplicatedCode
impl<T> Canvas<T> {
    /// Creates a new [`Canvas`] from a two-dimensional matrix of pieces.
    ///
    /// # Panics
    ///
    /// Panics if lines are not all the same width.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6]], canvas.0);
    /// ```
    pub fn from_matrix<IR, IT>(matrix: IR) -> Self
    where
        IR: IntoIterator<Item = IT>,
        IT: IntoIterator<Item = T>,
    {
        let canvas = Self(
            matrix
                .into_iter()
                .map(|r| r.into_iter().collect())
                .collect(),
        );
        canvas.validate_width();
        canvas
    }

    /// Creates a new [`Canvas`] from lines of text, using a closure to convert every `char`
    /// into a piece of canvas.
    ///
    /// # Panics
    ///
    /// Panics if lines are not all the same width.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let matrix = "123\n\
    ///               456";
    ///
    /// let canvas = Canvas::from_lines(matrix.lines(), |c| c.to_digit(10).unwrap());
    /// assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6]], canvas.0);
    /// ```
    pub fn from_lines<F, I, S>(lines: I, mut f: F) -> Self
    where
        F: FnMut(char) -> T,
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let canvas = Self(
            lines
                .into_iter()
                .map(move |s| s.as_ref().chars().map(&mut f).collect())
                .collect(),
        );
        canvas.validate_width();
        canvas
    }

    /// Creates a new [`Canvas`] from lines of text, using a closure to convert every byte
    /// into a piece of canvas.
    ///
    /// # Panics
    ///
    /// Panics if lines are not all the same width.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let matrix = "123\n\
    ///               456";
    ///
    /// let canvas = Canvas::from_line_bytes(matrix.lines(), |c| c - b'0');
    /// assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6]], canvas.0);
    /// ```
    pub fn from_line_bytes<F, I, S>(lines: I, mut f: F) -> Self
    where
        F: FnMut(u8) -> T,
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let canvas = Self(
            lines
                .into_iter()
                .map(move |s| s.as_ref().bytes().map(&mut f).collect())
                .collect(),
        );
        canvas.validate_width();
        canvas
    }

    /// Returns the width of the canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(3, canvas.width());
    /// ```
    pub fn width(&self) -> usize {
        self.0.first().map_or(0, Vec::len)
    }

    /// Returns the height of the canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(2, canvas.height());
    /// ```
    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of pieces in the canvas ([width] multiplied by [height]).
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(6, canvas.count());
    /// ```
    ///
    /// [width]: Self::width
    /// [height]: Self::height
    pub fn count(&self) -> usize {
        self.width() * self.height()
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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
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
        self.0.iter().map(|r| r.iter())
    }

    /// Flips the canvas horizontally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![vec![4, 5, 6], vec![1, 2, 3]], canvas.flip_horizontally().0);
    /// ```
    pub fn flip_horizontally(self) -> Self {
        Self(self.0.into_iter().rev().collect())
    }

    /// Flips this canvas vertically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![vec![3, 2, 1], vec![6, 5, 4]], canvas.flip_vertically().0);
    /// ```
    pub fn flip_vertically(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|l| l.into_iter().rev().collect())
                .collect(),
        )
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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
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
    pub fn into_map(self) -> HashMap<Pt<usize>, T> {
        matrix_to_map(self.0)
    }

    fn validate_width(&self) {
        if let Some(row) = self.0.first()
            && self.0.iter().any(|r| r.len() != row.len())
        {
            panic!("all rows need to have the same width");
        }
    }
}

// noinspection DuplicatedCode
impl<T> Canvas<T>
where
    T: Clone,
{
    /// Creates a new [`Canvas`] with every piece set to the same `value`.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::of(3, 2, '#');
    /// assert_eq!(vec![vec!['#', '#', '#'], vec!['#', '#', '#']], canvas.0);
    /// ```
    pub fn of(width: usize, height: usize, value: T) -> Self {
        Self(
            (0..height)
                .map(move |_| repeat_n(value.clone(), width).collect())
                .collect(),
        )
    }

    /// Rotates this canvas 90 degrees to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![vec![3, 6], vec![2, 5], vec![1, 4]], canvas.rotate_left().0);
    /// ```
    pub fn rotate_left(self) -> Self {
        Self(self.into_columns().rev().collect())
    }

    /// Converts the canvas into an [iterator] of its columns, from left to right.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![vec![1, 4], vec![2, 5], vec![3, 6]], canvas.into_columns().collect::<Vec<_>>());
    /// ```
    ///
    /// [iterator]: DoubleEndedIterator
    pub fn into_columns(self) -> impl DoubleEndedIterator<Item = Vec<T>> {
        (0..self.width())
            .map(|col| self.0.iter().map(|r| r[col].clone()).collect_vec())
            .collect_vec()
            .into_iter()
    }

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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2], [3, 4]]);
    ///
    /// let mut rotations = canvas.into_rotations();
    /// assert_eq!(vec![vec![1, 2], vec![3, 4]], rotations.next().unwrap().0);
    /// assert_eq!(vec![vec![2, 4], vec![1, 3]], rotations.next().unwrap().0);
    /// assert_eq!(vec![vec![4, 3], vec![2, 1]], rotations.next().unwrap().0);
    /// assert_eq!(vec![vec![3, 1], vec![4, 2]], rotations.next().unwrap().0);
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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    /// # use itertools::Itertools;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2], [3, 4]]);
    ///
    /// assert_eq!(16, canvas.clone().into_variations().count());
    /// assert_eq!(8, canvas.clone().into_variations().unique().count());
    /// assert!(
    ///     canvas
    ///         .into_variations()
    ///         .find(|c| { *c == Canvas::from_matrix([[1, 3], [2, 4]]) })
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

// noinspection DuplicatedCode
impl<T, PT> Index<PT> for Canvas<T>
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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(5, canvas[Pt::new(1, 1)]);
    /// assert_eq!(3, canvas[(2, 0)]);
    /// ```
    fn index(&self, index: PT) -> &Self::Output {
        let index = index.into();
        &self.0[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

// noinspection DuplicatedCode
impl<T, PT> IndexMut<PT> for Canvas<T>
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
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    /// # use aoclp::positioning::pt::Pt;
    ///
    /// let mut canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// canvas[Pt::new(1, 1)] = 9;
    /// canvas[(2, 0)] = 7;
    /// assert_eq!(vec![vec![1, 2, 7], vec![4, 9, 6]], canvas.0);
    /// ```
    fn index_mut(&mut self, index: PT) -> &mut Self::Output {
        let index = index.into();
        &mut self.0[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

impl<T> IntoIterator for Canvas<T> {
    type Item = vec::IntoIter<T>;
    type IntoIter = vec::IntoIter<Self::Item>;

    /// Converts the [`Canvas`] into an [iterator] of its rows. Each row is itself an
    /// [iterator] that returns canvas pieces.
    ///
    /// [iterator]: Iterator
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::identity;
    /// # use aoclp::mapping::canvas::dynamic::Canvas;
    ///
    /// let canvas = Canvas::from_matrix([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], canvas.into_iter().flat_map(identity).collect::<Vec<_>>());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(<_>::into_iter)
            .collect_vec()
            .into_iter()
    }
}
