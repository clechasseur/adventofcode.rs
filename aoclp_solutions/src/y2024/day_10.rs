use std::collections::HashMap;

use aoclp::dij;
use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::pt::{Pt, matrix_to_map};
use aoclp::solvers_impl::input::safe_get_input_as_terrain;
use strum::IntoEnumIterator;

pub fn part_1() -> usize {
    Map::default().trailheads().map(|h| h.score).sum()
}

pub fn part_2() -> usize {
    Map::default().trailheads().map(|h| h.rating).sum()
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    height: usize,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Self { height: c.to_digit(10).unwrap() as usize }
    }
}

#[derive(Debug, Copy, Clone)]
struct Trailhead {
    start: Pt,
    score: usize,
    rating: usize,
}

#[derive(Debug)]
struct Map {
    heightmap: HashMap<Pt, usize>,
}

impl Map {
    fn trailheads(&self) -> impl Iterator<Item = Trailhead> + use<'_> {
        self.heightmap
            .iter()
            .filter(|(_, t)| **t == 0)
            .filter_map(move |(p, _)| {
                let dist = dij::build(self, *p).dist;
                let score = dist.values().filter(|d| **d == 9).count();
                if score == 0 {
                    return None;
                }

                let mut cache = HashMap::new();
                let trailhead = Trailhead {
                    start: *p,
                    score,
                    rating: dist
                        .iter()
                        .filter(|(_, d)| **d == 9)
                        .map(|(p, _)| Self::path_count(*p, &dist, &mut cache))
                        .sum(),
                };
                Some(trailhead)
            })
    }

    fn path_count(p: Pt, dist: &HashMap<Pt, usize>, cache: &mut HashMap<Pt, usize>) -> usize {
        if let Some(count) = cache.get(&p) {
            return *count;
        }

        let p_dist = dist[&p];
        if p_dist == 0 {
            return 1;
        }

        let count = Direction4::iter()
            .map(|d| p + d)
            .filter(|n| dist.get(n).is_some_and(|d| *d == p_dist - 1))
            .map(|n| Self::path_count(n, dist, cache))
            .sum();
        cache.insert(p, count);

        count
    }
}

impl dij::Graph<Pt> for Map {
    fn neighbours(&self, node: &Pt) -> impl Iterator<Item = Pt> {
        let height = self.heightmap[node];
        Direction4::iter()
            .map(|d| *node + d)
            .filter(move |n| self.heightmap.get(n).is_some_and(|h| *h == height + 1))
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            heightmap: matrix_to_map(
                safe_get_input_as_terrain::<Tile>(2024, 10)
                    .into_iter()
                    .map(|y| y.into_iter().map(|t| t.height)),
            ),
        }
    }
}
