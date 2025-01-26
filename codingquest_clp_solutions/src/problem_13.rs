use codingquest_clp::solvers_impl::input::get_input;
use itertools::Itertools;

use crate::helpers::get_problem_input_data;

pub fn solve() -> usize {
    let (board, moves) = input();

    let mut p1 = 0;
    let mut p2 = 0;
    for (count, &(p1_move, p2_move)) in moves.moves.iter().enumerate() {
        p1 += p1_move;
        while (p1 as usize) < board.tiles.len() && board.tiles[p1 as usize] != 0 {
            p1 += board.tiles[p1 as usize];
        }
        if (p1 as usize) >= board.tiles.len() {
            return count + 1;
        }

        p2 += p2_move;
        while (p2 as usize) < board.tiles.len() && board.tiles[p2 as usize] != 0 {
            p2 += board.tiles[p2 as usize];
        }
        if (p2 as usize) >= board.tiles.len() {
            return 2 * (count + 1);
        }
    }

    panic!("No solution found");
}

fn input() -> (Board, Moves) {
    let input_data = get_problem_input_data(13).unwrap();
    (Board::from_input_data(&input_data), Moves::from_input_data(&input_data))
}

struct Board {
    pub tiles: Vec<i32>,
}

const BOARD_SIZE: usize = 20;

impl Board {
    fn from_input_data<I>(input_data: I) -> Self
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

struct Moves {
    pub moves: Vec<(i32, i32)>,
}

impl Moves {
    fn from_input_data<I>(input_data: I) -> Self
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
