use aoclp::solvers_impl::input::safe_get_input;

pub fn part_1() -> u32 {
    sum(&input(), 1)
}

pub fn part_2() -> u32 {
    let input = input();
    sum(&input, input.len() / 2)
}

fn sum(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .enumerate()
        .filter(|(i, c)| *c == nth(input, i + skip))
        .map(|(_, c)| c.to_digit(10).unwrap())
        .sum()
}

fn nth(input: &str, i: usize) -> char {
    input.chars().nth(i % input.len()).unwrap()
}

fn input() -> String {
    safe_get_input(2017, 1)
}
