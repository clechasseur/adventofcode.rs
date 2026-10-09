use aoclp::aoc::Input;

pub fn part_1() -> usize {
    input()
        .into_iter()
        .filter(|layer| layer.catches(0))
        .map(|layer| layer.severity())
        .sum()
}

pub fn part_2() -> usize {
    let input = input();
    (1_usize..)
        .find(|delay| !input.iter().any(|layer| layer.catches(*delay)))
        .unwrap()
}

#[derive(Debug)]
pub struct Layer {
    pub depth: usize,
    pub range: usize,
}

impl Layer {
    pub fn catches(&self, delay: usize) -> bool {
        (self.depth + delay).is_multiple_of((self.range - 1) * 2)
    }

    pub fn severity(&self) -> usize {
        self.depth * self.range
    }
}

fn input() -> Vec<Layer> {
    Input::for_puzzle(2017, 13)
        .lines_into_pairs()
        .into_iter()
        .map(|(depth, range)| Layer { depth, range })
        .collect()
}
