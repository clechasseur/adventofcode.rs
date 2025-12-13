use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::anyhow::Context;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
        let mut cache = HashMap::new();
        let presses = self.fewest_presses_for_joltage_from(
            vec![0; self.joltage_reqs.len()],
            0,
            usize::MAX,
            &mut cache,
        );
        println!("{presses}");
        presses
    }

    fn fewest_presses_for_joltage_from(
        &self,
        cur: Vec<usize>,
        steps: usize,
        max_steps: usize,
        _cache: &mut HashMap<Vec<usize>, usize>,
    ) -> usize {
        if *cur == self.joltage_reqs {
            return steps;
        // } else if let Some(presses) = cache.get(&cur) {
        //     return *presses;
        } else if steps == max_steps
            || cur
                .iter()
                .zip(self.joltage_reqs.iter())
                .any(|(cur, req)| *cur > *req)
        {
            return max_steps;
        }

        let presses = self.button_wirings.iter().fold(max_steps, |max, wiring| {
            let mut next = cur.clone();
            for w in wiring {
                next[*w] += 1;
            }
            let next_presses = self.fewest_presses_for_joltage_from(next, steps + 1, max, _cache);
            min(max, next_presses)
        });
        // cache.insert(cur, presses);
        presses
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

fn input() -> Vec<Machine> {
    safe_get_input_as_many(2025, 10)
}
