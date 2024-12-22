//! Helper library for Advent of Code.

pub mod forth;
pub mod looping;
pub mod positioning;
pub mod regex;
pub mod solvers_impl;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;
