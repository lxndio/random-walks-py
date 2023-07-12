pub mod standard;

use crate::dp::simple::SimpleDynamicProgram;
use std::ops::{Index, IndexMut};
use strum::EnumIter;

pub type Walk = Vec<(isize, isize)>;

pub trait WalkGenerator {
    fn generate_path(
        &self,
        dp: &SimpleDynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Walk;

    fn name(&self, short: bool) -> String;
}
