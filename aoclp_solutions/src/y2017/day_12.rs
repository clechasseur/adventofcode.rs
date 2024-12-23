use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> usize {
    Village::from(input()).group(0).len()
}

pub fn part_2() -> usize {
    Village::from(input()).group_count()
}

#[derive(Debug)]
struct Program {
    id: usize,
    neighbours: Vec<usize>,
}

impl FromStr for Program {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, neighbours) = s.split(" <-> ").collect_tuple().unwrap();
        Ok(Self {
            id: id.parse()?,
            neighbours: neighbours
                .split(", ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Village(HashMap<usize, Program>);

impl Village {
    pub fn group(&self, id: usize) -> HashSet<usize> {
        let mut group = HashSet::new();
        self.fill_group(id, &mut group);
        group
    }

    pub fn group_count(&self) -> usize {
        let mut seen = HashSet::new();
        let mut count = 0;

        self.0.keys().for_each(|&id| {
            if !seen.contains(&id) {
                seen.extend(self.group(id));
                count += 1;
            }
        });

        count
    }

    fn fill_group(&self, id: usize, group: &mut HashSet<usize>) {
        if !group.contains(&id) {
            let program = &self.0[&id];
            group.insert(program.id);
            program
                .neighbours
                .iter()
                .for_each(|&neighbour_id| self.fill_group(neighbour_id, group));
        }
    }
}

impl From<Vec<Program>> for Village {
    fn from(value: Vec<Program>) -> Self {
        Self(value.into_iter().map(|p| (p.id, p)).collect())
    }
}

fn input() -> Vec<Program> {
    safe_get_input_as_many(2017, 12)
}
