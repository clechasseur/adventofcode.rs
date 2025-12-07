use std::collections::HashMap;

use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::pt::{matrix_to_map, Pt};
use aoclp::solvers_impl::input::safe_get_input_as_terrain;
use derive_where::derive_where;
use itertools::Itertools;

pub fn part_1() -> usize {
    manifoldize().splits
}

pub fn part_2() -> usize {
    manifoldize().worlds
}

#[derive(Debug, Clone)]
struct Manifold {
    parts: HashMap<Pt, char>,
    starting_point: Pt,
}

impl Default for Manifold {
    fn default() -> Self {
        let parts = matrix_to_map(input());
        let starting_point = *parts.iter().find(|(_, c)| **c == 'S').unwrap().0;
        Self { parts, starting_point }
    }
}

#[derive(Debug, Copy, Clone)]
#[derive_where(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Particle {
    pos: Pt,
    #[derive_where(skip)]
    worlds: usize,
}

impl Particle {
    fn initial(manifold: &Manifold) -> Self {
        Self { pos: manifold.starting_point, worlds: 1 }
    }

    fn travel(self, dir: Direction4) -> Self {
        Self { pos: self.pos + dir, ..self }
    }

    fn onward(self, manifold: &Manifold) -> Vec<Self> {
        let next = self.travel(Direction4::Down);
        if !manifold.parts.contains_key(&next.pos) {
            return vec![self];
        }

        if manifold.parts.get(&next.pos).is_some_and(|c| *c == '^') {
            vec![next.travel(Direction4::Left), next.travel(Direction4::Right)]
        } else {
            vec![next]
        }
    }
}

struct ManifoldizationResult {
    splits: usize,
    worlds: usize,
}

fn manifoldize() -> ManifoldizationResult {
    let manifold = Manifold::default();
    let mut particles = vec![Particle::initial(&manifold)];

    let mut splits = 0;
    loop {
        let new_particles = particles
            .iter()
            .flat_map(|particle| {
                let moved = particle.onward(&manifold);
                if moved.len() > 1 {
                    splits += 1;
                }
                moved
            })
            .sorted()
            .coalesce(|a, b| {
                if a == b {
                    Ok(Particle { pos: a.pos, worlds: a.worlds + b.worlds })
                } else {
                    Err((a, b))
                }
            })
            .collect_vec();

        if new_particles == particles {
            break;
        }
        particles = new_particles;
    }

    let worlds = particles.into_iter().map(|p| p.worlds).sum();
    ManifoldizationResult { splits, worlds }
}

fn input() -> Vec<Vec<char>> {
    safe_get_input_as_terrain(2025, 7)
}
