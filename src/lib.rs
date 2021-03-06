//! **See what's different in arbitrary data structures**.
//!
//! The main [`diff`] algorithm.
//! we implement here is less than a screen full of lines, yet it enables a vast amount of
//! applications.
//! It can work with all values implementing the `Value` trait.
//!
//! On top of it there is a [`merge`] implementation, which can be used to aggregate changes
//! into a target value. It works with `Value`s which implement the `Mutable` trait.
//!
//! # Usage
//! Please have a look at the tests for [diff][diff-tests] and [merge][merge-tests] tests.
//!
//! Also note that you will have to choose the features to build the library with in order to
//! get trait implementations for `Value` types of common libraries,
//! i.e. `cargo build --features=with-serde-json`.
//! [`diff`]: fn.diff.html
//! [`merge`]: tools/struct.Merger.html
//! [diff-tests]: https://github.com/Byron/treediff-rs/blob/master/tests/diff.rs
//! [merge-tests]: https://github.com/Byron/treediff-rs/blob/master/tests/merge.rs
#![deny(missing_docs)]
#[cfg(feature = "with-rustc-serialize")]
extern crate rustc_serialize;
#[cfg(feature = "with-serde-json")]
extern crate serde_json;
#[cfg(feature = "with-serde-yaml")]
extern crate serde_yaml;
#[cfg(feature = "with-yaml-rust")]
extern crate yaml_rust;

mod traitdef;
mod diff;
pub mod tools;
pub mod value;

pub use traitdef::*;
pub use diff::*;
