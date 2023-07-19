pub mod correlated;
pub mod standard;

use crate::dp::DynamicProgramType;
use std::ops::{Index, IndexMut};
use thiserror::Error;

pub type Walk = Vec<(isize, isize)>;

pub trait WalkGenerator {
    fn generate_path(
        &self,
        dpt: &DynamicProgramType,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkGenerationError>;

    fn name(&self, short: bool) -> String;
}

#[derive(Error, Debug)]
pub enum WalkGenerationError {
    #[error("wrong type of dynamic program given")]
    WrongDynamicProgramType,

    #[error("no path exists")]
    NoPathExists,
}
