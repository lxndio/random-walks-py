pub mod loader;

use crate::dataset::loader::{CoordinateType, DatasetLoader};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct GCSPoint {
    x: f64,
    y: f64,
}

impl ToString for GCSPoint {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

#[derive(Default, Debug)]
pub struct XYPoint {
    x: i64,
    y: i64,
}

impl ToString for XYPoint {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub enum Point {
    GCS(GCSPoint),
    XY(XYPoint),
}

impl ToString for Point {
    fn to_string(&self) -> String {
        match self {
            Point::GCS(p) => format!("GCS{}", p.to_string()),
            Point::XY(p) => format!("XY{}", p.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Datapoint {
    point: Point,
    metadata: HashMap<String, String>,
}

impl ToString for Datapoint {
    fn to_string(&self) -> String {
        format!("{} | {:?}", self.point.to_string(), self.metadata)
    }
}

pub struct Dataset {
    data: Vec<Datapoint>,
    coordinate_type: CoordinateType,
}

impl Dataset {
    pub fn new(coordinate_type: CoordinateType) -> Self {
        Self {
            data: Vec::new(),
            coordinate_type,
        }
    }

    pub fn from_loader(loader: impl DatasetLoader) -> anyhow::Result<Self> {
        let data = loader.load()?;

        Ok(Self {
            data,
            coordinate_type: loader.coordinate_type(),
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<&Datapoint> {
        self.data.get(index)
    }

    /// Convert all GCS points in the dataset to XY points and normalize them to the range [from, to].
    pub fn convert_gcs_to_xy(&mut self, from: i64, to: i64) -> Result<(), String> {
        if self.coordinate_type != CoordinateType::GCS {
            return Err("Dataset is not in GCS.".into());
        }

        let mut min = (f64::MAX, f64::MAX);
        let mut max = (f64::MIN, f64::MIN);
        let mut temp_points = vec![(0.0, 0.0); self.data.len()];

        for (i, datapoint) in self.data.iter_mut().enumerate() {
            match &mut datapoint.point {
                Point::GCS(point) => {
                    let x = 6371.0 * point.x.to_radians().cos() * point.y.to_radians().cos();
                    let y = 6371.0 * point.x.to_radians().cos() * point.y.to_radians().sin();

                    temp_points[i] = (x, y);

                    if x < min.0 {
                        min.0 = x;
                    }

                    if y < min.1 {
                        min.1 = y;
                    }

                    if x > max.0 {
                        max.0 = x;
                    }

                    if y > max.1 {
                        max.1 = y;
                    }
                }
                Point::XY(_) => (),
            }
        }

        // Normalize data to min-max range
        for (i, datapoint) in self.data.iter_mut().enumerate() {
            match datapoint.point {
                Point::GCS(_) => {
                    datapoint.point = Point::XY(XYPoint {
                        x: ((temp_points[i].0 - min.0) / (max.0 - min.0) * (to - from) as f64
                            + from as f64) as i64,
                        y: ((temp_points[i].1 - min.1) / (max.1 - min.1) * (to - from) as f64
                            + from as f64) as i64,
                    });
                }
                Point::XY(_) => (),
            }
        }

        self.coordinate_type = CoordinateType::XY;

        Ok(())
    }

    pub fn print(&self, from: Option<usize>, to: Option<usize>) {
        let from = from.unwrap_or(0);
        let to = to.unwrap_or(self.data.len());

        for i in from..to {
            println!("{}", self.data[i].to_string());
        }
    }
}
