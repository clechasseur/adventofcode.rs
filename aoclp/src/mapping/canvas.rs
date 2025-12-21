use std::iter::{repeat_n, successors};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Canvas<T, const W: usize, const H: usize = W>(pub [[T; W]; H]);

impl<T, const W: usize, const H: usize> Canvas<T, W, H> {
    pub fn new(data: [[T; W]; H]) -> Self {
        Self(data)
    }

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

    pub const fn height(&self) -> usize {
        H
    }

    pub const fn width(&self) -> usize {
        W
    }

    pub const fn count(&self) -> usize {
        H * W
    }

    pub fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.0.iter().map(|row| row.iter())
    }
}

impl<T, const W: usize, const H: usize> Canvas<T, W, H>
where
    T: Clone,
{
    pub fn of(value: T) -> Self {
        Self(
            (0..H)
                .map(move |_| repeat_n(value.clone(), W).collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    pub fn flipped_horizontally(&self) -> Self {
        Self(self.0.iter().cloned().rev().collect_array().unwrap())
    }

    pub fn flipped_vertically(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|l| l.iter().cloned().rev().collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }

    pub fn rotated_left(&self) -> Canvas<T, H, W> {
        Canvas(self.columns().rev().collect_array().unwrap())
    }

    pub fn columns(&self) -> impl DoubleEndedIterator<Item = [T; H]> {
        (0..W).map(|x| self.0.iter().map(|l| l[x].clone()).collect_array().unwrap())
    }
}

impl<T, const N: usize> Canvas<T, N, N>
where
    T: Clone,
{
    pub fn into_rotations(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |p| Some(p.rotated_left())).take(4)
    }

    pub fn into_combinations(self) -> impl Iterator<Item = Self> {
        vec![self.flipped_horizontally(), self]
            .into_iter()
            .flat_map(|c| vec![c.flipped_vertically(), c])
            .flat_map(Self::into_rotations)
    }
}

impl<T, const W: usize, const H: usize> Default for Canvas<T, W, H>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self::of(T::default())
    }
}

impl<T, const W: usize, const H: usize, IR, IV, V> From<IR> for Canvas<T, W, H>
where
    IR: IntoIterator<Item = IV>,
    IV: IntoIterator<Item = V>,
    V: Into<T>,
{
    fn from(value: IR) -> Self {
        Self(
            value
                .into_iter()
                .map(|r| r.into_iter().map(<_>::into).collect_array().unwrap())
                .collect_array()
                .unwrap(),
        )
    }
}
