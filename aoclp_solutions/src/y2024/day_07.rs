use std::iter::once;

use aoclp::aoc::Input;
use aoclp::forth;
use aoclp::forth::stack::Stack;
use aoclp::forth::word::Words;
use aoclp::forth::{Forth, Value};
use itertools::{Itertools, repeat_n};

pub fn part_1() -> i64 {
    solve(true)
}

pub fn part_2() -> i64 {
    solve(false)
}

fn solve(elusive_elephants: bool) -> i64 {
    let mut forth = Forth::new();
    forth.define_word("||", concatenate);
    input()
        .into_iter()
        .filter(|equation| equation.solvable(elusive_elephants, &mut forth))
        .map(|equation| equation.solution)
        .sum()
}

const OPS: &[&str] = &["+", "*", "||"];

fn possible_ops(num: usize, elusive_elephants: bool) -> impl Iterator<Item = Vec<&'static str>> {
    let num_ops = if elusive_elephants { OPS.len() - 1 } else { OPS.len() };
    repeat_n(OPS.iter().take(num_ops).copied().collect_vec(), num).multi_cartesian_product()
}

fn concatenate(stack: &mut Stack, _dictionary: &Words) -> forth::Result<()> {
    let right = stack.pop()?;
    let left = stack.pop()?;
    let merged: Value = format!("{left}{right}")
        .parse()
        .map_err(|_| forth::Error::InvalidWord)?;
    stack.push(merged);
    Ok(())
}

#[derive(Debug, Clone)]
struct Equation {
    pub solution: i64,
    pub terms: Vec<String>,
}

impl Equation {
    pub fn solvable(&self, elusive_elephants: bool, forth: &mut Forth) -> bool {
        possible_ops(self.terms.len() - 1, elusive_elephants)
            .map(|ops| {
                while !forth.stack().is_empty() {
                    forth.eval("DROP").unwrap();
                }

                let equation = once(self.terms[0].as_ref())
                    .chain(self.terms.iter().skip(1).map(<_>::as_ref).interleave(ops))
                    .join(" ");
                forth.eval(&equation).unwrap();
                *forth.stack().iter().exactly_one().unwrap()
            })
            .any(|candidate| candidate == self.solution)
    }
}

impl From<Vec<i64>> for Equation {
    fn from(vecs: Vec<i64>) -> Self {
        Self { solution: vecs[0], terms: vecs.iter().skip(1).map(<_>::to_string).collect_vec() }
    }
}

fn input() -> Vec<Equation> {
    Input::for_puzzle(2024, 7)
        .split_lines_into()
        .into_iter()
        .map_into()
        .collect()
}
