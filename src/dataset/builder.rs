//! Provides a builder for datasets.
//!
//! This module provides the [`DatasetBuilder`](DatasetBuilder)

use crate::dataset::loader::csv::{CSVLoader, CSVLoaderOptions};
use crate::dataset::loader::polars::{PolarsLoader, PolarsLoaderOptions};
use crate::dataset::loader::{ColumnAction, CoordinateType, DatasetLoader};
use crate::dataset::point::{Coordinates, Point, XYPoint};
use crate::dataset::{Datapoint, Dataset};
use crate::xy;
use anyhow::bail;
use polars::frame::DataFrame;
use rand::Rng;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatasetBuilderError {
    #[error("a data source for the dataset must be set")]
    NoDatasetSourceSet,
    #[error("a coordinate type must be set")]
    NoCoordinateTypeSet,
}

#[derive(Default)]
enum DatasetSource {
    CSV(String),
    Polars(DataFrame),
    Manual,
    #[default]
    None,
}

pub struct DatasetBuilder {
    source: DatasetSource,
    csv_delimiter: u8,
    csv_header: bool,
    column_actions: Vec<ColumnAction<String>>,
    coordinate_type: Option<CoordinateType>,
    points: Vec<Point>,
}

impl DatasetBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_csv<S>(mut self, path: S) -> Self
    where
        S: Into<String>,
    {
        self.source = DatasetSource::CSV(path.into());

        self
    }

    #[cfg(feature = "polars")]
    pub fn from_polars(mut self, df: DataFrame) -> Self {
        self.source = DatasetSource::Polars(df);

        self
    }

    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.csv_delimiter = delimiter;

        self
    }

    pub fn with_header(mut self) -> Self {
        self.csv_header = true;

        self
    }

    pub fn add_column_action(mut self, column_action: ColumnAction<&str>) -> Self {
        self.column_actions.push(column_action.into());

        self
    }

    pub fn add_column_actions(mut self, column_actions: Vec<ColumnAction<&str>>) -> Self {
        let mut column_actions: Vec<_> = column_actions.iter().map(|x| x.clone().into()).collect();

        self.column_actions.append(&mut column_actions);

        self
    }

    pub fn coordinate_type(mut self, coordinate_type: CoordinateType) -> Self {
        self.coordinate_type = Some(coordinate_type);

        self
    }

    pub fn add_point(mut self, point: Point) -> Self {
        self.source = DatasetSource::Manual;
        self.points.push(point);

        self
    }

    pub fn add_points(mut self, points: Vec<Point>) -> Self {
        self.source = DatasetSource::Manual;
        self.points.append(&mut points.clone());

        self
    }

    pub fn line(mut self, qty: usize, from: XYPoint, spacing: XYPoint) -> Self {
        self.source = DatasetSource::Manual;

        for i in 0..qty as i64 {
            self.points.push(Point::XY(xy!(
                from.x + spacing.x * i,
                from.y + spacing.y * i
            )));
        }

        self
    }

    pub fn fill(mut self, from: XYPoint, to: XYPoint, spacing: XYPoint) -> Self {
        self.source = DatasetSource::Manual;

        let (mut x, mut y) = from.into();

        while y < to.y {
            while x < to.x {
                self.points.push(Point::XY(xy!(x, y)));

                x += spacing.x;
            }

            y += spacing.y;
        }

        self
    }

    pub fn random(mut self, qty: usize, from: XYPoint, to: XYPoint) -> Self {
        self.source = DatasetSource::Manual;

        let mut rng = rand::thread_rng();

        for _ in 0..qty {
            let x = rng.gen_range(from.x..to.x);
            let y = rng.gen_range(from.y..to.y);

            self.points.push(Point::XY(xy!(x, y)));
        }

        self
    }

    pub fn build(self) -> anyhow::Result<Dataset> {
        let Some(coordinate_type) = self.coordinate_type else {
            bail!(DatasetBuilderError::NoCoordinateTypeSet);
        };

        match self.source {
            DatasetSource::CSV(path) => {
                let loader = CSVLoader::new(CSVLoaderOptions {
                    path,
                    delimiter: self.csv_delimiter,
                    header: self.csv_header,
                    column_actions: self.column_actions,
                    coordinate_type,
                });

                Dataset::from_loader(loader)
            }
            DatasetSource::Polars(df) => {
                let loader = PolarsLoader::new(PolarsLoaderOptions {
                    df,
                    column_actions: self.column_actions,
                    coordinate_type,
                });

                Dataset::from_loader(loader)
            }
            DatasetSource::Manual => {
                let mut dataset = Dataset::new(coordinate_type);

                dataset.data = self
                    .points
                    .iter()
                    .map(|p| Datapoint {
                        point: p.clone(),
                        metadata: HashMap::new(),
                    })
                    .collect();

                Ok(dataset)
            }
            DatasetSource::None => bail!(DatasetBuilderError::NoDatasetSourceSet),
        }
    }
}

impl Default for DatasetBuilder {
    fn default() -> Self {
        Self {
            source: DatasetSource::default(),
            csv_delimiter: b',',
            csv_header: false,
            column_actions: Vec::new(),
            coordinate_type: None,
            points: Vec::new(),
        }
    }
}
