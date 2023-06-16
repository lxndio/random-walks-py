//! A collection of different random walk models for use in the dynamic program.

use num::BigUint;
use crate::dp::DynamicProgram;

pub mod nd_biased_rw;
pub mod simple_rw;
pub mod biased_rw;

/// A direction for use in different random walk models.
#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub trait WalkModel {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint;
    fn name(&self, short: bool) -> String;
}
