pub mod csv;

use crate::dataset::Datapoint;
use anyhow::Context;
use serde::{Deserialize, Serialize};

pub trait DatasetLoader {
    fn load(&self) -> anyhow::Result<Vec<Datapoint>>;

    fn stream(&self) -> anyhow::Result<()>;

    fn coordinate_type(&self) -> CoordinateType;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnAction {
    KeepX,
    KeepY,
    KeepMetadata(String),
    #[default]
    Discard,
}

/// The type of coordinates used in a dataset.
#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinateType {
    /// Geographic coordinate system (GCS) coordinates.
    #[default]
    GCS,

    /// XY coordinates.
    XY,
}
