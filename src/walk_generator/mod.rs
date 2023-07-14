pub mod correlated;
pub mod standard;

use crate::dp::DynamicProgramType;
use std::ops::{Index, IndexMut};

pub type Walk = Vec<(isize, isize)>;

pub trait WalkGenerator {
    fn generate_path(
        &self,
        dp: &DynamicProgramType,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkGenerationError>;

    fn name(&self, short: bool) -> String;
}

#[derive(Debug)]
pub enum WalkGenerationError {
    WrongDynamicProgramType,
    NoPathExists,
}
