use std::cmp::min;
use std::iter::FusedIterator;
use std::vec;

pub trait LoopingItertools: Iterator {
    fn looping(self, size: usize) -> Option<Looping<Self::Item>>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        let mut prefix = Vec::new();

        for e in self {
            match prefix.iter().position(|ve| *ve == e) {
                Some(start) => {
                    let cycle = prefix.split_off(start);
                    return Some(Looping::new(prefix, cycle, size));
                },
                None => prefix.push(e),
            }
            if prefix.len() == size {
                break;
            }
        }

        None
    }
}

impl<I> LoopingItertools for I where I: Iterator + ?Sized {}

#[derive(Debug, Clone)]
pub struct Looping<T> {
    prefix: vec::IntoIter<T>,
    prefix_len: usize,
    cycle: Vec<T>,
    cycle_pos: usize,
    cycle_size: usize,
}

impl<T> Looping<T> {
    fn new(prefix: Vec<T>, cycle: Vec<T>, size: usize) -> Self {
        let prefix_len = prefix.len();
        Self {
            prefix: prefix.into_iter(),
            prefix_len,
            cycle,
            cycle_pos: 0,
            cycle_size: size.saturating_sub(prefix_len),
        }
    }

    pub fn prefix_len(&self) -> usize {
        self.prefix_len
    }

    pub fn cycle_items(&self) -> &[T] {
        &self.cycle
    }

    fn cycle_len(&self) -> usize {
        self.cycle_size - self.cycle_pos
    }
}

impl<T> Iterator for Looping<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prefix.next() {
            Some(e) => Some(e),
            None if self.cycle_len() == 0 => None,
            None => {
                let e = self.cycle[self.cycle_pos % self.cycle.len()].clone();
                self.cycle_pos += 1;
                Some(e)
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.prefix.len() + self.cycle_len();
        (exact, Some(exact))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len() == 0 {
            return None;
        }

        let last_pos_in_cycle = (self.cycle_size - 1) % self.cycle.len();
        self.cycle
            .into_iter()
            .nth(last_pos_in_cycle)
            .or_else(|| self.prefix.next_back())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let prefix_len = self.prefix.len();
        self.prefix.nth(n).or_else(|| {
            self.cycle_pos = min(self.cycle_pos + (n - prefix_len), self.cycle_size);
            if self.cycle_len() != 0 {
                self.next()
            } else {
                None
            }
        })
    }
}

impl<T> ExactSizeIterator for Looping<T> where T: Clone {}
impl<T> FusedIterator for Looping<T> where T: Clone {}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const DATA: &[usize] = &[1, 2, 3, 4, 5, 6, 3];

    #[test]
    fn test_iterator() {
        let v = DATA.iter().looping(11).unwrap().copied().collect_vec();
        assert_eq!([1, 2, 3, 4, 5, 6, 3, 4, 5, 6, 3], *v.as_slice());
    }

    #[test]
    fn test_exact_size() {
        let mut i = DATA.iter().looping(11).unwrap();
        assert_eq!(11, i.len());
        assert_eq!((11, Some(11)), i.size_hint());

        let _ = i.next();
        assert_eq!(10, i.len());
        assert_eq!((10, Some(10)), i.size_hint());

        let _ = i.next();
        let _ = i.next();
        assert_eq!(8, i.len());
        assert_eq!((8, Some(8)), i.size_hint());

        while i.next().is_some() {}
        assert_eq!(0, i.len());
        assert_eq!((0, Some(0)), i.size_hint());
    }

    #[test]
    fn test_count() {
        let i = DATA.iter().looping(11).unwrap();
        assert_eq!(11, i.count());

        let mut i = DATA.iter().looping(11).unwrap();
        let _ = i.next();
        let _ = i.next();
        let _ = i.next();
        assert_eq!(8, i.count());
    }

    #[test]
    fn test_last() {
        let i = DATA.iter().copied().looping(11).unwrap();
        assert_eq!(Some(3), i.last());

        let mut i = DATA.iter().copied().looping(11).unwrap();
        let _ = i.next();
        let _ = i.next();
        let _ = i.next();
        assert_eq!(Some(3), i.last());

        let mut i = DATA.iter().copied().looping(11).unwrap();
        while i.next().is_some() {}
        assert_eq!(None, i.last());
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn test_nth() {
        let expected = [1, 2, 3, 4, 5, 6, 3, 4, 5, 6, 3];

        let mut i = DATA.iter().copied().looping(11).unwrap();
        let mut ei = expected.iter().copied();
        while let Some(e) = i.nth(0) {
            assert_eq!(ei.next(), Some(e));
        }
        assert!(ei.next().is_none());

        let mut i = DATA.iter().copied().looping(11).unwrap();
        assert_eq!(Some(2), i.nth(1));
        assert_eq!(Some(4), i.nth(1));
        assert_eq!(Some(3), i.nth(2));
        assert!(i.nth(7).is_none());
        assert!(i.next().is_none());

        let mut i = DATA.iter().copied().looping(11).unwrap();
        assert_eq!(Some(3), i.nth(10));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_no_loop() {
        let data = [1, 2, 3, 4, 5, 6];
        assert!(data.iter().copied().looping(11).is_none());
    }
}
