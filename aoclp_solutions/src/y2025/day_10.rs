use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::anyhow::Context;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::{Input, safe_get_input_as_many};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use z3::ast::Int;
use z3::{Optimize, SatResult};

pub fn part_1() -> usize {
    input().iter().map(Machine::fewest_presses_for_lights).sum()
}

pub fn part_2() -> usize {
    input()
        .par_iter()
        .map(Machine::fewest_presses_for_joltage)
        .sum()
}

#[derive(Debug, Clone)]
struct Machine {
    target_lights: Vec<bool>,
    button_wirings: Vec<Vec<usize>>,
    joltage_reqs: Vec<usize>,
}

impl Machine {
    fn fewest_presses_for_lights(&self) -> usize {
        let mut presses = 0;
        let mut states = vec![vec![false; self.target_lights.len()]];
        loop {
            if states.iter().contains(&self.target_lights) {
                break presses;
            }

            states = states
                .into_iter()
                .flat_map(|l| {
                    self.button_wirings.iter().map(move |wiring| {
                        let mut next = l.clone();
                        for w in wiring {
                            next[*w] = !next[*w]
                        }
                        next
                    })
                })
                .unique()
                .collect();
            presses += 1;
        }
    }

    fn fewest_presses_for_joltage(&self) -> usize {
        let opt = Optimize::new();

        let buttons = (0..self.button_wirings.len())
            .map(|i| Int::new_const(format!("button{i}")))
            .collect_vec();
        for b in &buttons {
            opt.assert(&b.ge(0));
        }

        let presses = buttons
            .iter()
            .skip(1)
            .fold(&buttons[0] + 0, |acc, b| acc + b);

        for (i, j_req) in self.joltage_reqs.iter().copied().enumerate() {
            let matching_buttons = self
                .button_wirings
                .iter()
                .enumerate()
                .filter(|(_, w)| w.contains(&i))
                .map(|(wi, _)| &buttons[wi])
                .collect_vec();
            let jolt = matching_buttons
                .iter()
                .skip(1)
                .fold(matching_buttons[0] + 0, |acc, b| acc + *b);
            opt.assert(&jolt.eq(j_req as u64));
        }

        opt.minimize(&presses);
        let check = opt.check(&[]);
        if check != SatResult::Sat {
            panic!("No combination of presses can reach {:?}", self.joltage_reqs);
        }

        let model = opt.get_model().unwrap();
        let presses = model.eval(&presses, true).unwrap();
        presses.as_u64().unwrap() as usize
    }
}

impl FromStr for Machine {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re = REGEX.get_or_init(|| {
            Regex::new(r"^\s*\[(?<lights>[.#]+)]\s+(?<buttons>(?:\((?:\d+,?)+\)\s+)+)\s*\{(?<joltage>(?:\d+,?)+)}\s*$").unwrap()
        });

        let captures = re
            .captures(s)
            .with_context(|| format!("invalid machine spec: {s}"))?;
        let target_lights = &captures["lights"];
        let button_wirings = &captures["buttons"];
        let joltage_reqs = &captures["joltage"];

        let target_lights = target_lights.chars().map(|c| c == '#').collect();
        let button_wirings = button_wirings
            .split_ascii_whitespace()
            .map(|bw| {
                bw[1..bw.len() - 1]
                    .split(',')
                    .map(|l| l.parse::<usize>())
                    .try_collect()
            })
            .try_collect()?;
        let joltage_reqs = joltage_reqs
            .split(',')
            .map(|j| j.parse::<usize>())
            .try_collect()?;

        Ok(Self { target_lights, button_wirings, joltage_reqs })
    }
}

const EXAMPLE: &str = "\
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn input() -> Vec<Machine> {
    safe_get_input_as_many(2025, 10)
}

fn example() -> Vec<Machine> {
    Input::for_example(EXAMPLE).safe_into_many()
}
