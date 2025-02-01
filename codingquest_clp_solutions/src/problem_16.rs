use codingquest_clp::solvers_impl::input::get_input;
use itertools::Itertools;

use crate::helpers::get_problem_input_data;

pub fn solve() -> u64 {
    let message: Message = message_data().into();
    message.fix()
}

fn message_data() -> Vec<Vec<String>> {
    get_input(get_problem_input_data(16).unwrap())
        .unwrap()
        .safe_into_many_vecs()
}

#[derive(Debug)]
struct Message {
    data: Vec<Vec<u8>>,
    row_checksums: Vec<u8>,
    col_checksums: Vec<u8>,
}

impl Message {
    pub fn fix(&self) -> u64 {
        let (invalid_row, invalid_row_checksum_diff) = (0..self.data.len())
            .map(|i| (i, Self::checksum_diff(self.row_checksums[i], self.row(i))))
            .filter(|&(_, diff)| diff != 0)
            .exactly_one()
            .unwrap();
        let (invalid_col, invalid_col_checksum_diff) = (0..self.data[0].len())
            .map(|i| (i, Self::checksum_diff(self.col_checksums[i], self.col(i))))
            .filter(|&(_, diff)| diff != 0)
            .exactly_one()
            .unwrap();

        if invalid_row_checksum_diff != invalid_col_checksum_diff {
            panic!("Invalid row {invalid_row}'s checksum diff ({invalid_row_checksum_diff}) is not equal to col {invalid_col}'s checksum diff ({invalid_col_checksum_diff})!");
        }

        let wrong_byte = self.data[invalid_row][invalid_col];
        let correct_byte = wrong_byte.wrapping_sub(invalid_row_checksum_diff);
        (wrong_byte as u64) * (correct_byte as u64)
    }

    fn row(&self, row: usize) -> impl Iterator<Item = u8> + '_ {
        self.data[row].iter().copied()
    }

    fn col(&self, col: usize) -> impl Iterator<Item = u8> + '_ {
        self.data.iter().map(move |row| row[col])
    }

    fn checksum_diff<D>(expected_checksum: u8, data: D) -> u8
    where
        D: IntoIterator<Item = u8>,
    {
        let actual_checksum = data
            .into_iter()
            .fold(0u8, |acc, byte| acc.wrapping_add(byte));
        actual_checksum.wrapping_sub(expected_checksum)
    }
}

impl From<Vec<Vec<String>>> for Message {
    fn from(value: Vec<Vec<String>>) -> Self {
        let value = value
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|byte| u8::from_str_radix(&byte, 16).unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let data = value
            .iter()
            .take(value.len() - 1)
            .map(|row| row.iter().take(row.len() - 1).copied().collect_vec())
            .collect_vec();
        let row_checksums = value
            .iter()
            .take(value.len() - 1)
            .map(|row| row.last().copied().unwrap())
            .collect_vec();
        let col_checksums = value.last().cloned().unwrap();

        Self { data, row_checksums, col_checksums }
    }
}
