use crate::dataset::{Datapoint, GCSPoint, Point, XYPoint};
use anyhow::Context;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;

pub trait DatasetLoader {
    fn load(&self) -> anyhow::Result<Vec<Datapoint>>;

    fn stream(&self) -> anyhow::Result<()>;
}

pub enum ColumnAction {
    KeepX,
    KeepY,
    KeepMetadata(String),
    Discard,
}

#[derive(Default)]
pub enum CoordinateType {
    #[default]
    GCS,
    XY,
}

pub struct CSVLoaderOptions {
    pub path: String,
    pub delimiter: u8,
    pub header: bool,
    pub column_actions: Vec<ColumnAction>,
    pub coordinate_type: CoordinateType,
}

impl Default for CSVLoaderOptions {
    fn default() -> Self {
        Self {
            path: String::new(),
            delimiter: b',',
            header: false,
            column_actions: Vec::new(),
            coordinate_type: CoordinateType::default(),
        }
    }
}

pub struct CSVLoader {
    options: CSVLoaderOptions,
}

impl CSVLoader {
    pub fn new(options: CSVLoaderOptions) -> Self {
        Self { options }
    }
}

impl DatasetLoader for CSVLoader {
    fn load(&self) -> anyhow::Result<Vec<Datapoint>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(self.options.delimiter)
            .has_headers(self.options.header)
            .from_path(&self.options.path)?;

        let mut data = Vec::new();

        for result in rdr.records() {
            let record = result?;

            if record.len() != self.options.column_actions.len() {
                return Err(io::Error::from(ErrorKind::InvalidData)).context(format!(
                    "Expected {} columns, got {}.",
                    self.options.column_actions.len(),
                    record.len()
                ));
            }

            let mut point = match self.options.coordinate_type {
                CoordinateType::GCS => Point::GCS(GCSPoint::default()),
                CoordinateType::XY => Point::XY(XYPoint::default()),
            };
            let mut metadata = HashMap::new();

            for (i, column) in record.iter().enumerate() {
                match &self.options.column_actions[i] {
                    ColumnAction::KeepX => {
                        if let Point::GCS(point) = &mut point {
                            point.x = column.parse()?;
                        }
                    }
                    ColumnAction::KeepY => {
                        if let Point::GCS(point) = &mut point {
                            point.y = column.parse()?;
                        }
                    }
                    ColumnAction::KeepMetadata(key) => {
                        metadata.insert(key.clone(), column.to_string());
                    }
                    ColumnAction::Discard => (),
                }
            }

            let datapoint = Datapoint { point, metadata };

            data.push(datapoint);
        }

        Ok(data)
    }

    fn stream(&self) -> anyhow::Result<()> {
        todo!()
    }
}
