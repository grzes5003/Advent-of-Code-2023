#![feature(test)]
#[allow(unused_imports)]
extern crate test;

pub mod days;
mod util;
mod err;
mod commons;

pub use util::solve as solve;