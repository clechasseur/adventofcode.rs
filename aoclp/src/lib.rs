//! Helper library for Advent of Code.

pub mod captures;
pub mod dij;
pub mod forth;
pub mod functional;
pub mod looping;
pub mod positioning;
pub mod solvers_impl;
pub mod str;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

pub use anyhow;
pub use num;
pub use paste;
pub use regex;
