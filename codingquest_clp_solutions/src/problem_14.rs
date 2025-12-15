use std::collections::{HashMap, HashSet};

use codingquest_clp::solvers_impl::input::get_input;
use itertools::Itertools;

use crate::helpers::get_problem_input_data;

pub fn solve() -> String {
    let word_list = word_list();
    let rules: Rules = GUESSES.into();

    word_list
        .into_iter()
        .filter(|word| rules.matches(word))
        .exactly_one()
        .unwrap()
        .clone()
}

fn word_list() -> Vec<String> {
    get_input(get_problem_input_data(14).unwrap())
        .unwrap()
        .safe_into_many()
}

const GUESSES: &[(&str, &str)] =
    &[("keyless", "YYBBYYG"), ("society", "YGYYYBB"), ("phobias", "BBGBGBG")];

#[derive(Debug, Default)]
struct Rules {
    green: HashMap<usize, char>,
    yellow: HashSet<char>,
    black: HashSet<char>,
}

impl Rules {
    pub fn matches<S>(&self, word: S) -> bool
    where
        S: AsRef<str>,
    {
        let word = word.as_ref();

        word.chars()
            .enumerate()
            .filter(|(i, c)| {
                if let Some(green) = self.green.get(i) {
                    green == c
                } else {
                    self.yellow.contains(c) && !self.black.contains(c)
                }
            })
            .count()
            == word.len()
    }
}

impl From<&[(&str, &str)]> for Rules {
    fn from(value: &[(&str, &str)]) -> Self {
        let rules = value
            .iter()
            .flat_map(|(guess, colored)| {
                guess
                    .chars()
                    .enumerate()
                    .zip_eq(colored.chars())
                    .map(|((i, g), color)| (color, (i, g)))
            })
            .into_group_map();

        let color_rule = |color| rules.get(&color).unwrap().clone();
        let simple_color_rule = |color| color_rule(color).into_iter().map(|(_, g)| g).collect();
        let indexed_color_rule = |color| color_rule(color).into_iter().collect();

        let green = indexed_color_rule('G');
        let yellow = simple_color_rule('Y');
        let black = simple_color_rule('B');
        Self { green, yellow, black }
    }
}
