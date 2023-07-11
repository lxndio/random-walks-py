pub mod standard;

use crate::dp::DynamicProgram;
use std::ops::{Index, IndexMut};
use strum::EnumIter;

pub type Walk = Vec<(isize, isize)>;

pub trait WalkGenerator {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Walk;

    fn name(&self, short: bool) -> String;
}
