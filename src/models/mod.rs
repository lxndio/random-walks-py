//! A collection of different random walk models for use in the dynamic program.

use crate::dp::DynamicProgram;
use num::BigUint;

pub mod biased_rw;
pub mod nd_biased_rw;
pub mod simple_rw;

/// A direction for use in different random walk models.
#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Stay,
}

pub trait WalkModel {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint;
    fn name(&self, short: bool) -> String;
}
