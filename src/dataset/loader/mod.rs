pub mod csv;
#[cfg(feature = "polars")]
pub mod polars;

use crate::dataset::Datapoint;
use pyo3::{pyclass, pymethods, FromPyObject, PyCell, PyResult};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub trait DatasetLoader {
    fn load(&self) -> anyhow::Result<Vec<Datapoint>>;

    fn stream(&self) -> anyhow::Result<()>;

    fn coordinate_type(&self) -> CoordinateType;
}

#[pyclass]
#[derive(Error, Debug)]
pub enum DatasetLoaderError {
    #[error("a column containing X coordinates must be specified")]
    NoXColumnSpecified,
    #[error("a column containing Y coordinates must be specified")]
    NoYColumnSpecified,
    #[error("there are more columns in the dataset than actions have been set")]
    MoreColumnsThanActions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnAction<S: Into<String>> {
    KeepX,
    KeepY,
    KeepMetadata(S),
    #[default]
    Discard,
}

impl From<ColumnAction<&str>> for ColumnAction<String> {
    fn from(value: ColumnAction<&str>) -> Self {
        match value {
            ColumnAction::KeepX => ColumnAction::KeepX,
            ColumnAction::KeepY => ColumnAction::KeepY,
            ColumnAction::KeepMetadata(s) => ColumnAction::KeepMetadata(s.into()),
            ColumnAction::Discard => ColumnAction::Discard,
        }
    }
}

/// The type of coordinates used in a dataset.
#[pyclass]
#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinateType {
    /// Geographic coordinate system (GCS) coordinates.
    #[default]
    GCS,

    /// XY coordinates.
    XY,
}

#[pymethods]
impl CoordinateType {
    pub fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;

        let name = match *slf.borrow() {
            CoordinateType::GCS => "GCS",
            CoordinateType::XY => "XY",
        };

        Ok(format!("{}({})", class_name, name,))
    }
}
