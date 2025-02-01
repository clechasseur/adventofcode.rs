use std::ops::Deref;

use codingquest_clp::solvers_impl::input::get_input;
use itertools::Itertools;

use crate::helpers::get_problem_input_data;

pub fn solve() -> usize {
    let (board, moves) = input();

    let move_player = |p: &mut i32, p_move: i32| {
        *p += p_move;
        while (*p as usize) < board.len() && board[*p as usize] != 0 {
            *p += board[*p as usize];
        }
    };

    let mut p1 = 0;
    let mut p2 = 0;
    for (count, &(p1_move, p2_move)) in moves.iter().enumerate() {
        move_player(&mut p1, p1_move);
        if (p1 as usize) >= board.len() {
            return count + 1;
        }

        move_player(&mut p2, p2_move);
        if (p2 as usize) >= board.len() {
            return 2 * (count + 1);
        }
    }

    panic!("No solution found");
}

fn input() -> (Board, Moves) {
    let input_data = get_problem_input_data(13).unwrap();
    (Board::from_input_data(&input_data), Moves::from_input_data(&input_data))
}

#[derive(Debug)]
struct Board {
    tiles: Vec<i32>,
}

const BOARD_SIZE: usize = 20;

impl Board {
    pub fn from_input_data<I>(input_data: I) -> Self
    where
        I: AsRef<str>,
    {
        Self {
            tiles: get_input(
                input_data
                    .as_ref()
                    .lines()
                    .take(BOARD_SIZE)
                    .collect_vec()
                    .join("\n"),
            )
            .unwrap()
            .safe_into_many_vecs()
            .into_iter()
            .rev()
            .enumerate()
            .flat_map(|(i, mut row)| {
                if i % 2 != 0 {
                    row.reverse();
                }
                row
            })
            .collect(),
        }
    }
}

impl Deref for Board {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

#[derive(Debug)]
struct Moves {
    moves: Vec<(i32, i32)>,
}

impl Moves {
    pub fn from_input_data<I>(input_data: I) -> Self
    where
        I: AsRef<str>,
    {
        Self {
            moves: get_input(
                input_data
                    .as_ref()
                    .lines()
                    .skip(BOARD_SIZE)
                    .collect_vec()
                    .join("\n"),
            )
            .unwrap()
            .safe_into_many_pairs(),
        }
    }
}

impl Deref for Moves {
    type Target = [(i32, i32)];

    fn deref(&self) -> &Self::Target {
        &self.moves
    }
}
