use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::captures::CapturesHelper;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> String {
    let tower = Tower::build();
    let bottom = tower.bottom.borrow();
    bottom.name.clone()
}

pub fn part_2() -> usize {
    let (_imbalanced, balanced) = Tower::build().imbalance();
    balanced
}

#[derive(Debug)]
pub struct ProgramSpec {
    pub name: String,
    pub weight: usize,
    pub sub_programs: Vec<String>,
}

#[derive(Debug, Default)]
struct Program {
    name: String,
    weight: usize,
    sub_programs: Vec<Rc<RefCell<Program>>>,
}

impl Program {
    fn new<S>(name: S) -> Rc<RefCell<Self>>
    where
        S: Into<String>,
    {
        Rc::new(RefCell::new(Self { name: name.into(), ..Self::default() }))
    }

    fn total_weight(&self, cache: &mut HashMap<String, usize>) -> usize {
        match cache.get(&self.name).copied() {
            Some(weight) => weight,
            None => {
                let weight = self.weight + self.sub_weight(cache);
                cache.insert(self.name.clone(), weight);
                weight
            },
        }
    }

    fn sub_weight(&self, cache: &mut HashMap<String, usize>) -> usize {
        self.sub_programs
            .iter()
            .map(|sub_program| sub_program.borrow().total_weight(cache))
            .sum()
    }

    fn imbalance(&self, weights: &HashMap<String, usize>) -> Option<(usize, usize)> {
        let sub_data: Vec<_> = self
            .sub_programs
            .iter()
            .map(|sub_program| (sub_program, sub_program.borrow().imbalance(weights)))
            .collect();

        match sub_data
            .iter()
            .filter(|(_, imbalance)| imbalance.is_some())
            .at_most_one()
        {
            Ok(Some((_, imbalance))) => *imbalance,
            Ok(None) => {
                let programs: Vec<_> = sub_data
                    .into_iter()
                    .map(|(sub_program, _)| sub_program)
                    .sorted_by_key(|sub_program| weights[&sub_program.borrow().name])
                    .dedup_by_with_count(|sp1, sp2| {
                        weights[&sp1.borrow().name] == weights[&sp2.borrow().name]
                    })
                    .sorted_by_key(|(count, _)| *count)
                    .map(|(_, sub_program)| sub_program)
                    .collect();

                (programs.len() == 2).then(|| {
                    let imbalance = weights[&programs[0].borrow().name] as isize
                        - weights[&programs[1].borrow().name] as isize;
                    let imbalanced_weight = programs[0].borrow().weight;
                    let balanced_weight = imbalanced_weight.checked_add_signed(-imbalance).unwrap();
                    (imbalanced_weight, balanced_weight)
                })
            },
            Err(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Tower {
    bottom: Rc<RefCell<Program>>,
}

impl Tower {
    fn build() -> Self {
        let mut programs = HashMap::new();
        let mut parents = HashSet::new();

        for spec in input() {
            let program = Rc::clone(programs.entry(spec.name.clone()).or_insert_with(|| {
                parents.insert(spec.name.clone());
                Program::new(spec.name)
            }));
            program.borrow_mut().weight = spec.weight;
            for sub_prog_name in spec.sub_programs {
                let sub_program = Rc::clone(
                    programs
                        .entry(sub_prog_name.clone())
                        .or_insert_with(|| Program::new(sub_prog_name.clone())),
                );
                program.borrow_mut().sub_programs.push(sub_program);
                parents.remove(&sub_prog_name);
            }
        }

        Tower {
            bottom: programs
                .remove(&parents.into_iter().exactly_one().unwrap())
                .unwrap(),
        }
    }

    fn total_weights(&self) -> HashMap<String, usize> {
        let mut weights = HashMap::new();
        let bottom_weight = self.bottom.borrow().total_weight(&mut weights);
        weights.insert(self.bottom.borrow().name.clone(), bottom_weight);
        weights
    }

    fn imbalance(&self) -> (usize, usize) {
        let weights = self.total_weights();
        self.bottom.borrow().imbalance(&weights).unwrap()
    }
}

impl FromStr for ProgramSpec {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(
                r"^(?<name>\w+)\s+\((?<weight>\d+)\)(?:\s+->\s+(?<subs>(?:\w+(?:,\s*)?)+))?$",
            )
            .unwrap()
        });

        let captures = re.captures(s).unwrap();
        let name = captures["name"].to_string();
        let weight: usize = captures.ez_get("weight");
        let sub_programs = captures
            .name("subs")
            .map(|subs| {
                subs.as_str()
                    .split(", ")
                    .map(ToString::to_string)
                    .collect_vec()
            })
            .unwrap_or_default();

        Ok(ProgramSpec { name, weight, sub_programs })
    }
}

fn input() -> Vec<ProgramSpec> {
    safe_get_input_as_many(2017, 7)
}
