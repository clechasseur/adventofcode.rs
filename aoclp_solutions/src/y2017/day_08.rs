use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

use aoclp::anyhow::anyhow;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> i64 {
    final_registers().values().max().unwrap()
}

pub fn part_2() -> i64 {
    final_registers().max_ever
}

fn final_registers() -> Registers {
    let mut registers = Registers::default();
    input()
        .into_iter()
        .for_each(|instruction| instruction.apply(&mut registers));
    registers
}

#[derive(Debug, Default)]
struct Registers {
    registers: HashMap<String, i64>,
    max_ever: i64,
}

impl Registers {
    fn get<S>(&self, name: S) -> i64
    where
        S: AsRef<str>,
    {
        self.registers
            .get(name.as_ref())
            .copied()
            .unwrap_or_default()
    }

    fn update<S, F>(&mut self, name: S, f: F)
    where
        S: Into<String>,
        F: FnOnce(i64) -> i64,
    {
        let register = self.registers.entry(name.into()).or_default();
        *register = f(*register);
        self.max_ever = max(self.max_ever, *register);
    }

    fn values(&self) -> impl Iterator<Item = i64> + '_ {
        self.registers.values().copied()
    }
}

#[derive(Debug)]
struct Instruction {
    register: String,
    offset: i64,
    cmp_register: String,
    cmp: Comparison,
    cmp_value: i64,
}

impl Instruction {
    fn apply(&self, registers: &mut Registers) {
        let cmp_register = registers.get(&self.cmp_register);
        if self.cmp.apply(cmp_register, self.cmp_value) {
            registers.update(self.register.clone(), |r| r + self.offset);
        }
    }
}

impl FromStr for Instruction {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [register, op, offset, _, cmp_register, cmp, cmp_value] = s
            .split_whitespace()
            .map(ToString::to_string)
            .collect_vec()
            .try_into()
            .map_err(|_| anyhow!("invalid instruction: {s}"))?;
        let offset = offset
            .parse::<i64>()
            .map(|o| if op == "dec" { -o } else { o })?;

        Ok(Self { register, offset, cmp_register, cmp: cmp.into(), cmp_value: cmp_value.parse()? })
    }
}

#[derive(Debug)]
struct Comparison(Vec<Ordering>);

impl Comparison {
    const MATCHING_ORDERINGS: &'static [(char, Ordering)] =
        &[('<', Ordering::Less), ('>', Ordering::Greater), ('=', Ordering::Equal)];

    fn apply(&self, a: i64, b: i64) -> bool {
        self.0.contains(&a.cmp(&b))
    }
}

impl<S> From<S> for Comparison
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref();

        Self(if value == "!=" {
            vec![Ordering::Less, Ordering::Greater]
        } else {
            Self::MATCHING_ORDERINGS
                .iter()
                .filter_map(|&(c, ord)| value.contains(c).then_some(ord))
                .collect()
        })
    }
}

fn input() -> Vec<Instruction> {
    safe_get_input_as_many(2017, 8)
}
