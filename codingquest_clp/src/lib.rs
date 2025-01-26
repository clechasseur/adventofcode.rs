//! Helper library for Coding Quest.

pub mod solvers_impl;

pub type Error = aoclp::Error;
pub type Result<T> = aoclp::Result<T>;

pub use aoclp;
