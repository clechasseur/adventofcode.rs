//! Solutions to [Coding Quest](https://codingquest.io/) puzzles in Rust 🦀

#![allow(dead_code)]

use codingquest_clp::build_solvers;

pub mod helpers;
pub mod problem_13;

pub type Error = codingquest_clp::Error;
pub type Result<T> = codingquest_clp::Result<T>;

build_solvers! {
    [13]
}
