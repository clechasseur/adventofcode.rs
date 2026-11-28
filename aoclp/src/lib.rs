//! Helper library for Advent of Code.

// #![deny(missing_docs)]
// #![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![cfg_attr(aoclp_docsrs, feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod aoc;
pub mod cache;
pub mod captures;
pub mod forth;
pub mod functional;
pub mod looping;
pub mod mapping;
pub mod positioning;
pub mod solvers;
pub mod str;

/// Error type used by this library.
///
/// Because this is meant to be a helper for local code, we use [`anyhow::Error`] for simplicity.
pub type Error = anyhow::Error;

/// Result type using our crate's [`Error`].
pub type Result<T> = anyhow::Result<T>;

pub use anyhow;
pub use num;
pub use paste;
pub use regex;
