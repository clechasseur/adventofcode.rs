use std::cmp::Ordering;

use aoclp::solvers_impl::input::safe_get_input_as_many_vecs_of_two_types;
use itertools::Itertools;

pub fn part_1() -> u64 {
    let (rules, updates) = input();
    updates
        .into_iter()
        .filter(|update| rules.iter().all(|rule| update.conforms_to(rule)))
        .map(|update| update.middle_page())
        .sum()
}

pub fn part_2() -> u64 {
    let (rules, updates) = input();
    updates
        .into_iter()
        .filter_map(|update| update.try_fix(&rules))
        .map(|update| update.middle_page())
        .sum()
}

fn input() -> (Vec<Rule>, Vec<Update>) {
    let (rules, updates) = safe_get_input_as_many_vecs_of_two_types(2024, 5);
    (rules.into_iter().map(Into::into).collect(), updates.into_iter().map(Into::into).collect())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Rule(u64, u64);

impl Rule {
    pub fn first(&self) -> u64 {
        self.0
    }

    pub fn positions_in(&self, v: &[u64]) -> (Option<usize>, Option<usize>) {
        (v.iter().position(|&n| n == self.0), v.iter().position(|&n| n == self.1))
    }

    pub fn applies_to(&self, page_1: u64, page_2: u64) -> bool {
        (self.0 == page_1 && self.1 == page_2) || (self.0 == page_2 && self.1 == page_1)
    }
}

impl From<Vec<u64>> for Rule {
    fn from(v: Vec<u64>) -> Self {
        let (first, second) = v.into_iter().collect_tuple().unwrap();
        Self(first, second)
    }
}

#[derive(Debug, Clone)]
struct Update(Vec<u64>);

impl Update {
    pub fn middle_page(&self) -> u64 {
        self.0[self.0.len() / 2]
    }

    pub fn conforms_to(&self, rule: &Rule) -> bool {
        match rule.positions_in(&self.0) {
            (Some(first_pos), Some(second_pos)) => first_pos < second_pos,
            _ => true,
        }
    }

    pub fn try_fix(&self, rules: &[Rule]) -> Option<Self> {
        let fixed = self
            .0
            .iter()
            .copied()
            .sorted_by(|&page_1, &page_2| {
                match rules.iter().find(|rule| rule.applies_to(page_1, page_2)) {
                    Some(rule) if rule.first() == page_1 => Ordering::Less,
                    Some(_) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            })
            .collect_vec();

        (fixed != self.0).then_some(Self(fixed))
    }
}

impl From<Vec<u64>> for Update {
    fn from(v: Vec<u64>) -> Self {
        Self(v)
    }
}
