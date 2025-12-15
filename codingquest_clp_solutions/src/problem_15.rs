use std::collections::{HashMap, HashSet, VecDeque};

use codingquest_clp::aoclp::positioning::pt::{matrix_to_map, Pt};
use codingquest_clp::solvers_impl::input::get_input;
use itertools::Itertools;

use crate::helpers::get_problem_input_data;

pub fn solve() -> u64 {
    let bodies: CelestialBodies = sensor_data().into();
    bodies.avg_mass()
}

fn sensor_data() -> Vec<Vec<u64>> {
    get_input(get_problem_input_data(15).unwrap())
        .unwrap()
        .safe_into_many_vecs()
}

#[derive(Debug)]
struct CelestialBody(HashMap<Pt, u64>);

impl CelestialBody {
    pub fn mass(&self) -> u64 {
        self.0.values().sum()
    }
}

impl From<HashMap<Pt, u64>> for CelestialBody {
    fn from(value: HashMap<Pt, u64>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct CelestialBodies(Vec<CelestialBody>);

impl CelestialBodies {
    pub fn avg_mass(&self) -> u64 {
        self.0.iter().map(CelestialBody::mass).sum::<u64>() / (self.0.len() as u64)
    }

    fn body(start: Pt, data: &HashMap<Pt, u64>, cache: &mut HashSet<Pt>) -> CelestialBody {
        let mut pieces = HashMap::new();
        let mut stack = VecDeque::new();
        pieces.insert(start, data[&start]);
        stack.push_back(start);
        while let Some(pt) = stack.pop_front() {
            let neighbours = pt
                .four_neighbours()
                .filter(|neighbour| !cache.contains(neighbour))
                .filter(|neighbour| !pieces.contains_key(neighbour))
                .filter(|neighbour| data.get(neighbour).is_some_and(|data| *data != 0))
                .collect_vec();
            pieces.extend(neighbours.iter().map(|pt| (*pt, data[pt])));
            stack.extend(neighbours.iter().copied());
            cache.extend(neighbours);
        }

        pieces.into()
    }
}

impl From<Vec<Vec<u64>>> for CelestialBodies {
    fn from(value: Vec<Vec<u64>>) -> Self {
        let data = matrix_to_map(value);
        let (bodies, _) =
            data.iter()
                .fold((Vec::new(), HashSet::new()), |(mut bodies, mut cache), (pt, d)| {
                    if !cache.contains(pt) && *d != 0 {
                        let body = Self::body(*pt, &data, &mut cache);
                        bodies.push(body);
                    }
                    (bodies, cache)
                });

        Self(bodies)
    }
}
