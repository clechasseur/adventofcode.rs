use std::iter::once;

use aoclp::forth::Forth;
use aoclp::solvers_impl::input::safe_get_input_as_many_vecs;
use itertools::{Itertools, repeat_n};

pub fn part_1() -> i64 {
    solve(true)
}

pub fn part_2() -> i64 {
    solve(false)
}

fn solve(elusive_elephants: bool) -> i64 {
    let mut forth = Forth::new();
    input()
        .into_iter()
        .filter(|equation| equation.solvable(elusive_elephants, &mut forth))
        .map(|equation| equation.solution)
        .sum()
}

const OPS: &[&str] = &["+", "*", "||"];

fn possible_ops(num: usize, elusive_elephants: bool) -> impl Iterator<Item = Vec<String>> {
    let num_ops = if elusive_elephants { OPS.len() - 1 } else { OPS.len() };
    repeat_n(OPS.iter().take(num_ops).map(<_>::to_string).collect_vec(), num)
        .multi_cartesian_product()
}

#[derive(Debug, Clone)]
struct Equation {
    pub solution: i64,
    pub terms: Vec<i64>,
}

impl Equation {
    pub fn solvable(&self, elusive_elephants: bool, forth: &mut Forth) -> bool {
        possible_ops(self.terms.len() - 1, elusive_elephants)
            .map(|ops| {
                while !forth.stack().is_empty() {
                    forth.eval("DROP").unwrap();
                }

                let equation = once(self.terms[0].to_string())
                    .chain(
                        self.terms
                            .iter()
                            .skip(1)
                            .map(<_>::to_string)
                            .interleave(ops),
                    )
                    .join(" ");
                forth.eval(&equation).unwrap();
                *forth.stack().iter().exactly_one().unwrap()
            })
            .any(|candidate| candidate == self.solution)
    }
}

impl From<Vec<i64>> for Equation {
    fn from(vecs: Vec<i64>) -> Self {
        Self { solution: vecs[0], terms: vecs[1..].to_vec() }
    }
}

fn input() -> Vec<Equation> {
    safe_get_input_as_many_vecs(2024, 7)
        .into_iter()
        .map_into()
        .collect()
}
