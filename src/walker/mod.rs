pub mod correlated;
pub mod standard;

use crate::dp::DynamicProgramType;
use std::ops::{Index, IndexMut};
use thiserror::Error;

pub type Walk = Vec<(isize, isize)>;

pub trait Walker {
    fn generate_path(
        &self,
        dpt: &DynamicProgramType,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError>;

    fn name(&self, short: bool) -> String;
}

#[derive(Error, Debug)]
pub enum WalkerError {
    #[error("wrong type of dynamic program given")]
    WrongDynamicProgramType,

    #[error("no path exists")]
    NoPathExists,
}
