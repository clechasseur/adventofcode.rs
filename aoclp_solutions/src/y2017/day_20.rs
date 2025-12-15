use std::convert::Infallible;
use std::iter::successors;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::captures::CapturesHelper;
use aoclp::num::zero;
use aoclp::positioning::pt_3d::{manhattan, Pt3d};
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> usize {
    Universe::default()
        .iter()
        .sorted_unstable_by(|p1, p2| {
            cmp_acceleration(p1, p2)
                .then_with(|| cmp_velocity(p1, p2))
                .then_with(|| cmp_position(p1, p2))
        })
        .next()
        .unwrap()
        .id
}

pub fn part_2() -> usize {
    expanding_universe().last().unwrap().len()
}

fn expanding_universe() -> impl Iterator<Item = Universe> {
    successors(Some(Universe::default()), |universe| {
        let expanded_universe = universe.move_one_tick();

        let blueshift = 'blue: {
            let mut distances = universe.distances();
            for (ep1_id, ep2_id, ed) in expanded_universe.distances() {
                let (_, _, d) = distances
                    .find(|(p1_id, p2_id, _)| *p1_id == ep1_id && *p2_id == ep2_id)
                    .unwrap();
                if d >= ed {
                    break 'blue true;
                }
            }
            false
        };

        blueshift.then_some(expanded_universe)
    })
}

type Coords = Pt3d;

fn distance_to_0(c: Coords) -> i64 {
    manhattan(zero(), c)
}

macro_rules! cmp_attribute {
    ($attr:ident) => {
        ::aoclp::paste::paste! {
            fn [<cmp_ $attr>](p1: &Particle, p2: &Particle) -> ::std::cmp::Ordering {
                distance_to_0(p1.$attr).cmp(&distance_to_0(p2.$attr))
            }
        }
    };
}

cmp_attribute!(position);
cmp_attribute!(velocity);
cmp_attribute!(acceleration);

#[derive(Debug, Default, Copy, Clone)]
struct Particle {
    pub id: usize,
    pub position: Coords,
    pub velocity: Coords,
    pub acceleration: Coords,
}

impl Particle {
    pub fn with_id(self, id: usize) -> Self {
        Self { id, ..self }
    }

    pub fn move_one_tick(&self) -> Self {
        let velocity = self.velocity + self.acceleration;
        let position = self.position + velocity;
        Self { position, velocity, ..*self }
    }
}

impl FromStr for Particle {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX
            .get_or_init(|| Regex::new(r"^p=<(?<p>.+)>,\s*v=<(?<v>.+)>,\s*a=<(?<a>.+)>$").unwrap());

        let captures = re
            .captures(s)
            .unwrap_or_else(|| panic!("invalid Particle value: {s}"));
        Ok(Self {
            position: captures.ez_get("p"),
            velocity: captures.ez_get("v"),
            acceleration: captures.ez_get("a"),
            ..Self::default()
        })
    }
}

#[derive(Debug, Clone)]
struct Universe(Vec<Particle>);

impl Universe {
    fn new<I>(particles: I) -> Self
    where
        I: IntoIterator<Item = Particle>,
    {
        Self(
            particles
                .into_iter()
                .sorted_unstable_by_key(|p| p.position)
                .dedup_by_with_count(|p1, p2| p1.position == p2.position)
                .filter(|(count, _)| *count == 1)
                .map(|(_, p)| p)
                .sorted_unstable_by_key(|p| p.id)
                .collect_vec(),
        )
    }

    pub fn move_one_tick(&self) -> Self {
        Self::new(self.0.iter().map(Particle::move_one_tick))
    }

    pub fn distances(&self) -> impl Iterator<Item = (usize, usize, i64)> + '_ {
        self.0
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| (p1.id, p2.id, manhattan(p1.position, p2.position)))
    }
}

impl Deref for Universe {
    type Target = [Particle];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new(
            input()
                .into_iter()
                .enumerate()
                .map(|(id, particle)| particle.with_id(id)),
        )
    }
}

fn input() -> Vec<Particle> {
    safe_get_input_as_many(2017, 20)
}
