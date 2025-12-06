use aoclp::forth::Forth;
use aoclp::solvers_impl::input::safe_get_input;
use itertools::Itertools;

pub fn part_1() -> usize {
    problems().into_iter().map(Problem::answer).sum()
}

pub fn part_2() -> usize {
    cephaloproblems().into_iter().map(Problem::answer).sum()
}

#[derive(Debug)]
struct Problem {
    operands: Vec<usize>,
    operator: String,
}

impl Problem {
    fn answer(self) -> usize {
        let mut forth = Forth::new();
        for operand in self.operands {
            forth.eval(&operand.to_string()).unwrap();
        }
        while forth.stack().len() > 1 {
            forth.eval(&self.operator).unwrap();
        }
        forth.stack()[0] as usize
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

fn cephaloproblems() -> Vec<Problem> {
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

fn parse_operators(input: &str) -> Vec<String> {
    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(<_>::to_string)
        .collect_vec()
}

fn input() -> String {
    safe_get_input(2025, 6)
}
