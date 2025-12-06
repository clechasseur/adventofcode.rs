use std::str::FromStr;

use aoclp::anyhow::anyhow;
use aoclp::functional::ByRefPredHelper;
use aoclp::solvers_impl::input::safe_get_input;
use itertools::Itertools;

pub fn part_1() -> usize {
    problems()
        .into_iter()
        .map(Problem::answer.without_ref())
        .sum()
}

pub fn part_2() -> usize {
    cephalopod_problems()
        .into_iter()
        .map(Problem::answer.without_ref())
        .sum()
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn initial(self) -> usize {
        match self {
            Operator::Plus => 0,
            Operator::Times => 1,
        }
    }

    fn apply(self, a: usize, b: usize) -> usize {
        match self {
            Operator::Plus => a + b,
            Operator::Times => a * b,
        }
    }
}

impl FromStr for Operator {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Times),
            op => Err(anyhow!("Unknown operator: {op}")),
        }
    }
}

#[derive(Debug)]
struct Problem {
    operands: Vec<usize>,
    operator: Operator,
}

impl Problem {
    fn answer(&self) -> usize {
        self.operands
            .iter()
            .fold(self.operator.initial(), |acc, i| self.operator.apply(acc, *i))
    }
}

fn problems() -> Vec<Problem> {
    let input = input();
    let operands_list = input
        .lines()
        .dropping_back(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    parse_operators(&input)
        .into_iter()
        .enumerate()
        .map(|(i, operator)| {
            let operands = operands_list
                .iter()
                .map(|operands| operands[i])
                .collect_vec();
            Problem { operands, operator }
        })
        .collect_vec()
}

fn cephalopod_problems() -> Vec<Problem> {
    let input = input();
    let operators = parse_operators(&input);
    let mut operands = Vec::new();

    let operand_lines = input.lines().dropping_back(1).collect_vec();
    let max_len = operand_lines.iter().map(|line| line.len()).max().unwrap();
    let mut add_problem_operands = |first_col, col| {
        let mut problem_operands = Vec::new();
        for problem_col in first_col..col {
            let n: usize = operand_lines
                .iter()
                .map(|line| line.chars().nth(problem_col).unwrap())
                .join("")
                .trim_ascii()
                .parse()
                .unwrap();
            problem_operands.push(n);
        }
        operands.push(problem_operands);
    };

    let mut first_col = 0;
    for col in 0..max_len {
        if operand_lines
            .iter()
            .map(|line| line.chars().nth(col).unwrap())
            .all(|c| c == ' ')
        {
            add_problem_operands(first_col, col);
            first_col = col + 1;
        }
    }
    if first_col != max_len {
        add_problem_operands(first_col, max_len);
    }

    operands
        .into_iter()
        .zip(operators)
        .map(|(operands, operator)| Problem { operands, operator })
        .collect_vec()
}

fn parse_operators(input: &str) -> Vec<Operator> {
    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| op.parse::<Operator>().unwrap())
        .collect_vec()
}

fn input() -> String {
    safe_get_input(2025, 6)
}
