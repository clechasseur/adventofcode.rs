//! Helper library for Advent of Code.

pub mod captures;
pub mod forth;
pub mod looping;
pub mod positioning;
pub mod solvers_impl;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

pub use anyhow;
pub use num;
pub use paste;
pub use regex;
