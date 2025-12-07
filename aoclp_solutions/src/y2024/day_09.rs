use aoclp::solvers_impl::input::safe_get_input;
use itertools::repeat_n;
use strum::{EnumCount, EnumIs, FromRepr};

pub fn part_1() -> usize {
    let mut blocks = blocks();
    defrag_blocks(&mut blocks);
    checksum(blocks)
}

pub fn part_2() -> usize {
    let mut blocks = blocks();
    defrag_files(&mut blocks);
    checksum(blocks)
}

type Block = Option<usize>;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIs, EnumCount, FromRepr)]
enum BlockType {
    EmptySpace,
    File,
}

impl BlockType {
    fn block(self, id: usize) -> Block {
        self.is_file().then_some(id)
    }

    fn next_id(self, id: usize) -> usize {
        id + self as usize
    }

    fn next_type(self) -> Self {
        Self::from_repr((self as usize + 1) % Self::COUNT).unwrap()
    }
}

fn blocks() -> Vec<Block> {
    input()
        .chars()
        .fold((Vec::new(), 0, BlockType::File), |(mut acc, id, block_type), c| {
            let len = c.to_digit(10).unwrap() as usize;
            acc.extend(repeat_n(block_type.block(id), len));
            (acc, block_type.next_id(id), block_type.next_type())
        })
        .0
}

fn defrag_blocks(blocks: &mut [Block]) {
    let mut i = 0;
    let mut last = blocks.len() - 1;
    while i <= last {
        if blocks[i].is_none() {
            while blocks[last].is_none() {
                last -= 1;
            }
            blocks[i] = blocks[last].take();
            last -= 1;
        }
        i += 1;
    }
}

fn defrag_files(blocks: &mut Vec<Block>) {
    let mut i = blocks.len() - 1;
    while i > 0 {
        if let Some(id) = blocks[i] {
            let mut start = i;
            while start > 0 && blocks[start - 1].is_some_and(|block| block == id) {
                start -= 1;
            }
            if start == 0 {
                break;
            }
            let block_len = i - start + 1;

            let mut target = 0;
            while target < start
                && blocks[target..target + block_len]
                    .iter()
                    .any(Option::is_some)
            {
                target += 1;
            }
            if target < start {
                blocks.splice(start..=i, repeat_n(None, block_len));
                blocks.splice(target..target + block_len, repeat_n(Some(id), block_len));
            }

            i = start - 1;
        } else {
            i -= 1;
        }
    }
}

fn checksum<B>(blocks: B) -> usize
where
    B: IntoIterator<Item = Block>,
{
    blocks
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, block)| acc + block.map_or(0, |id| id * i))
}

const EXAMPLE: &str = "2333133121414131402";

fn input() -> String {
    safe_get_input(2024, 9)
}
