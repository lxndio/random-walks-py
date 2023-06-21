pub mod biased_rw;
pub mod correlated_rw;
pub mod simple_rw;

use crate::dp::DynamicProgram;
use num::One;

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
