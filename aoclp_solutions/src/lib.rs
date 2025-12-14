//! Solutions to [Advent of Code](https://adventofcode.com/) puzzles in Rust 🦀

#![allow(dead_code)]

use aoclp::build_solvers;

pub mod y2017;
pub mod y2024;
pub mod y2025;

build_solvers! {
    { 2017, [01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25] },
    { 2024, [01, 02, 03, 04, 05, 06, 07, 08, 09, 10] },
    { 2025, [01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11] }
}
