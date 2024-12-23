use aoclp::positioning::direction::four_points::Direction4;
use aoclp::positioning::pt::Pt;
use aoclp::positioning::turtle::Turtle;
use aoclp::solvers_impl::input::safe_get_input_as_terrain;
use strum::{EnumCount, FromRepr};

pub fn part_1() -> usize {
    infections_after(10_000, false)
}

pub fn part_2() -> usize {
    infections_after(10_000_000, true)
}

fn infections_after(bursts: usize, evolved: bool) -> usize {
    let mut state = State::new(evolved);
    for _ in 0..bursts {
        state.burst();
    }

    state.infections
}

type Node = Pt;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr, EnumCount)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    pub fn next(&self, evolved: bool) -> Self {
        let jumps = if evolved { 1 } else { 2 };

        Self::from_repr(((*self as usize) + jumps) % Self::COUNT).unwrap()
    }
}

#[derive(Debug)]
struct Cluster(Vec<NodeState>);

impl Cluster {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.resize(1_000 * 1_000, NodeState::Clean);
        Self(nodes)
    }

    pub fn get_state(&self, node: &Node) -> NodeState {
        self.0
            .get(Self::index(node))
            .copied()
            .unwrap_or(NodeState::Clean)
    }

    pub fn modify_state(&mut self, node: &Node, evolved: bool) -> NodeState {
        let next_state = self.get_state(node).next(evolved);
        self.set_state(node, next_state);
        next_state
    }

    fn index(node: &Node) -> usize {
        ((node.x + 500) * 1_000 + node.y + 500) as usize
    }

    fn set_state(&mut self, node: &Node, state: NodeState) {
        let state_ref = self
            .0
            .get_mut(Self::index(node))
            .expect("cluster size is not big enough for puzzle");
        *state_ref = state;
    }
}

impl From<Vec<Vec<char>>> for Cluster {
    fn from(value: Vec<Vec<char>>) -> Self {
        let mut cluster = Self::new();

        value.into_iter().enumerate().for_each(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_, node)| node == '#')
                .for_each(|(x, _)| {
                    let node = Node::new(x as i64, y as i64);
                    cluster.set_state(&node, NodeState::Infected);
                });
        });

        cluster
    }
}

impl Default for Cluster {
    fn default() -> Self {
        input().into()
    }
}

fn input() -> Vec<Vec<char>> {
    safe_get_input_as_terrain(2017, 22)
}

type Carrier = Turtle;

fn carrier_start_pos() -> Pt {
    let input = input();
    Pt::new((input[0].len() / 2) as i64, (input.len() / 2) as i64)
}

#[derive(Debug)]
struct State {
    cluster: Cluster,
    carrier: Carrier,
    evolved: bool,
    pub infections: usize,
}

impl State {
    pub fn new(evolved: bool) -> Self {
        Self {
            cluster: Cluster::default(),
            evolved,
            carrier: Carrier::new(carrier_start_pos(), Direction4::Up),
            infections: 0,
        }
    }

    pub fn burst(&mut self) {
        self.turn_carrier();
        if self
            .cluster
            .modify_state(&self.carrier.position, self.evolved)
            == NodeState::Infected
        {
            self.infections += 1;
        }
        self.carrier = self.carrier.advance();
    }

    fn turn_carrier(&mut self) {
        self.carrier = match self.cluster.get_state(&self.carrier.position) {
            NodeState::Clean => self.carrier.turn_left(),
            NodeState::Weakened => self.carrier,
            NodeState::Infected => self.carrier.turn_right(),
            NodeState::Flagged => self.carrier.turn_around(),
        }
    }
}
