//! A library for efficient movement interpolation using different random walk stepper.

use pyo3::prelude::PyModule;
use pyo3::{pymodule, PyResult, Python};
use crate::dp::simple::SimpleDynamicProgram;

pub mod dataset;
pub mod dp;
pub mod kernel;
pub mod walk_analyzer;
pub mod walker;

#[pymodule]
pub fn randomwalks_lib(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleDynamicProgram>();

    Ok(())
}
