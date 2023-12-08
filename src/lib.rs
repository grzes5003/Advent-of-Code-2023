#![feature(test)]
#[allow(unused_imports)]
extern crate test;

pub mod days;
mod util;
mod err;

pub use util::solve as solve;