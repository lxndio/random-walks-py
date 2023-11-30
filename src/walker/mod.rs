//! Provides walkers used to generate random walks by using a dynamic program.

pub mod correlated;
pub mod levy;
pub mod multi_step;
pub mod standard;

use crate::dp::DynamicProgramPool;
use crate::walk::Walk;
use crate::walker::correlated::CorrelatedWalker;
use crate::walker::levy::LevyWalker;
use crate::walker::multi_step::MultiStepWalker;
use crate::walker::standard::StandardWalker;
use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, FromPyObject, PyErr};
use thiserror::Error;

pub trait Walker {
    fn generate_path(
        &self,
        dp: &DynamicProgramPool,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError>;

    fn generate_paths(
        &self,
        dp: &DynamicProgramPool,
        qty: usize,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Vec<Walk>, WalkerError> {
        let mut paths = Vec::new();

        for _ in 0..qty {
            paths.push(self.generate_path(dp, to_x, to_y, time_steps)?);
        }

        Ok(paths)
    }

    fn name(&self, short: bool) -> String;
}

#[derive(FromPyObject)]
pub enum WalkerType {
    #[pyo3(transparent)]
    Standard(StandardWalker),
    #[pyo3(transparent)]
    Correlated(CorrelatedWalker),
    #[pyo3(transparent)]
    MultiStep(MultiStepWalker),
    #[pyo3(transparent)]
    Levy(LevyWalker),
}

#[pyclass]
#[derive(Error, Debug)]
pub enum WalkerError {
    #[error("the walker requires a single dynamic program but multiple were given")]
    RequiresSingleDynamicProgram,

    #[error("the walker requires multiple dynamic programs but only a single one was given")]
    RequiresMultipleDynamicPrograms,

    #[error("no path exists")]
    NoPathExists,

    #[error("found an inconsistent path, probably due to wrong settings in the dynamic program or walker")]
    InconsistentPath,

    #[error("error while computing random distribution")]
    RandomDistributionError,
}

impl From<WalkerError> for PyErr {
    fn from(value: WalkerError) -> Self {
        PyValueError::new_err(value.to_string())
    }
}
