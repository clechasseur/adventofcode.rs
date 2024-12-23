use std::collections::HashMap;
use std::rc::Rc;

use aoclp::solvers_impl::input::safe_get_input;
use bit_vec::BitVec;
use itertools::Itertools;
use strum::EnumString;

pub fn part_1() -> usize {
    let mut machine = TuringMachine::default();
    machine.run_until_checksum();
    machine.checksum()
}

pub fn part_2() -> &'static str {
    ""
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TuringMove {
    Left,
    Right,
}

impl TuringMove {
    pub fn displacement(&self) -> isize {
        match self {
            TuringMove::Left => -1,
            TuringMove::Right => 1,
        }
    }

    pub fn apply(&self, machine: &mut TuringMachine) {
        machine.cursor += self.displacement();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TuringOp {
    pub value: usize,
    pub movement: TuringMove,
    pub next_state: char,
}

impl TuringOp {
    pub fn apply(&self, machine: &mut TuringMachine) {
        machine.tape.set(machine.cursor, self.value);
        self.movement.apply(machine);
        machine.current_state = self.next_state;
    }
}

const WRITE_THE_VALUE_PREFIX: &str = "    - Write the value ";
const MOVE_TO_THE_PREFIX: &str = "    - Move one slot to the ";
const CONTINUE_WITH_STATE_PREFIX: &str = "    - Continue with state ";

impl From<[&str; 3]> for TuringOp {
    fn from(v: [&str; 3]) -> Self {
        let value = v[0][WRITE_THE_VALUE_PREFIX.len()..][0..=0].parse().unwrap();
        let movement = {
            let suffix = &v[1][MOVE_TO_THE_PREFIX.len()..];
            suffix[0..suffix.len() - 1].parse().unwrap()
        };
        let next_state = v[2][CONTINUE_WITH_STATE_PREFIX.len()..]
            .chars()
            .next()
            .unwrap();

        TuringOp { value, movement, next_state }
    }
}

#[derive(Debug, Clone)]
pub struct TuringState([TuringOp; 2]);

impl TuringState {
    pub fn apply(&self, machine: &mut TuringMachine) {
        let current_value = machine.tape.get(machine.cursor);
        self.0[current_value].apply(machine);
    }
}

impl From<[&str; 8]> for TuringState {
    fn from(v: [&str; 8]) -> Self {
        let for_0: [&str; 3] = v[1..=3].try_into().unwrap();
        let for_1: [&str; 3] = v[5..=7].try_into().unwrap();

        Self([for_0.into(), for_1.into()])
    }
}

#[derive(Debug, Clone)]
pub struct TuringStates(HashMap<char, Rc<TuringState>>);

impl TuringStates {
    pub fn get(&self, name: char) -> Rc<TuringState> {
        Rc::clone(self.0.get(&name).unwrap())
    }
}

const IN_STATE_PREFIX: &str = "In state ";

impl From<Vec<&str>> for TuringStates {
    fn from(v: Vec<&str>) -> Self {
        Self(
            v.into_iter()
                .chunk_by(|line| line.is_empty())
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i % 2 == 0)
                .map(|(_, (_, mut chunk))| {
                    let name = chunk
                        .next()
                        .unwrap()
                        .chars()
                        .nth(IN_STATE_PREFIX.len())
                        .unwrap();
                    let state: [&str; 8] = chunk.collect_vec().try_into().unwrap();
                    (name, Rc::new(state.into()))
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct TuringTape(BitVec);

impl TuringTape {
    const CAPACITY: usize = 10_000;
    const OFFSET: isize = 4_000;

    pub fn get(&self, pos: isize) -> usize {
        self.0
            .get((pos + Self::OFFSET) as usize)
            .map(|b| if b { 1 } else { 0 })
            .unwrap_or_default()
    }

    pub fn set(&mut self, pos: isize, value: usize) {
        self.0.set((pos + Self::OFFSET) as usize, value != 0)
    }

    pub fn checksum(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Default for TuringTape {
    fn default() -> Self {
        Self(BitVec::from_elem(Self::CAPACITY, false))
    }
}

#[derive(Debug, Clone)]
pub struct TuringMachine {
    states: TuringStates,
    checksum_after: usize,
    current_state: char,
    tape: TuringTape,
    cursor: isize,
    steps: usize,
}

impl TuringMachine {
    pub fn step(&mut self) {
        let state = self.states.get(self.current_state);
        state.apply(self);
        self.steps += 1;
    }

    pub fn run_until_checksum(&mut self) {
        while self.steps < self.checksum_after {
            self.step();
        }
    }

    pub fn checksum(&self) -> usize {
        self.tape.checksum()
    }
}

const BEGIN_IN_STATE_PREFIX: &str = "Begin in state ";
const DIAGNOSTIC_PREFIX: &str = "Perform a diagnostic checksum after ";
const DIAGNOSTIC_SUFFIX: &str = " steps.";

impl Default for TuringMachine {
    fn default() -> Self {
        let input = input();
        let mut lines = input.lines();
        let current_state = lines
            .next()
            .unwrap()
            .chars()
            .nth(BEGIN_IN_STATE_PREFIX.len())
            .unwrap();
        let checksum_after = {
            let suffix = &lines.next().unwrap()[DIAGNOSTIC_PREFIX.len()..];
            suffix[0..(suffix.len() - DIAGNOSTIC_SUFFIX.len())]
                .parse()
                .unwrap()
        };
        let states = lines.skip(1).collect_vec().into();

        Self {
            states,
            checksum_after,
            current_state,
            tape: TuringTape::default(),
            cursor: 0,
            steps: 0,
        }
    }
}

fn input() -> String {
    safe_get_input(2017, 25)
}
