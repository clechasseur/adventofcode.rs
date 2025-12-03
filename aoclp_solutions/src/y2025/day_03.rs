use std::str::FromStr;

use aoclp::anyhow::anyhow;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> u64 {
    max_joltage(2)
}

pub fn part_2() -> u64 {
    max_joltage(12)
}

fn max_joltage(num_batteries: usize) -> u64 {
    input()
        .into_iter()
        .map(|bank| bank.max_joltage(num_batteries))
        .sum()
}

struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    fn max_joltage(&self, num_batteries: usize) -> u64 {
        let mut max = 0;
        let mut count = 0;
        let mut pos = 0;

        while count < num_batteries {
            let (rel_pos, max_d) = self.batteries
                [pos..=self.batteries.len() - num_batteries + count]
                .iter()
                .enumerate()
                .fold(None::<(usize, u32)>, |acc, (i, &joltage)| match acc {
                    None => Some((i, joltage)),
                    Some((_, acc_joltage)) if joltage > acc_joltage => Some((i, joltage)),
                    acc => acc,
                })
                .unwrap();
            max = max * 10 + (max_d as u64);
            count += 1;
            pos += rel_pos + 1;
        }

        max
    }
}

impl FromStr for Bank {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("wrong digit: {c}")))
                .try_collect()?,
        })
    }
}

fn input() -> Vec<Bank> {
    safe_get_input_as_many(2025, 3)
}
