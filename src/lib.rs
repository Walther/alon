//! Solutions for the course "Algoritmit Ongelmanratkaisussa" i.e. Algorithms in Problem Solving at University of Helsinki.
//!
//! - [Official course material](https://alon.mooc.fi) in Finnish
//! - [Exercise list](https://cses.fi/alon/list/) in English
//!
//! This repository uses a `nightly` Rust in order to utilize `cargo bench` for [benchmarking](https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html).
//!
//! Use `cargo test` to run all tests, use `cargo bench` to run all benchmarks.

#![feature(test)]
extern crate test;

pub mod weird_algorithm;
