//! A collection of different generators for use in the dynamic program.

use crate::dp::DynamicProgram;
use num::BigUint;

pub mod simple;

pub trait Generator {
    fn step(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint;
    fn name(&self, short: bool) -> String;
}
