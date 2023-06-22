pub mod biased_rw;
pub mod correlated_rw;
pub mod simple_rw;

use crate::dp::DynamicProgram;

/// A direction for use in different random walk generators.
#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Stay,
}

pub trait Solver {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Vec<(isize, isize)>;

    fn name(&self, short: bool) -> String;
}
