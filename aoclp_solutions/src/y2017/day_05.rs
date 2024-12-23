use std::iter::successors;

use aoclp::solvers_impl::input::safe_get_input_as_many;

pub fn part_1() -> usize {
    steps(false)
}

pub fn part_2() -> usize {
    steps(true)
}

fn steps(strange: bool) -> usize {
    // Skip the initial state, but count the last jump.
    maze(strange).skip(1).count() + 1
}

fn maze(strange: bool) -> impl Iterator<Item = usize> {
    let mut jumps = input();

    successors(Some(0_usize), move |&prev| {
        let jmp = jumps.get_mut(prev).unwrap();
        let next = prev.wrapping_add_signed(*jmp);
        *jmp += if strange && *jmp >= 3 { -1 } else { 1 };
        (next < jumps.len()).then_some(next)
    })
}

fn input() -> Vec<isize> {
    safe_get_input_as_many(2017, 5)
}
