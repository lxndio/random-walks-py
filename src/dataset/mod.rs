pub mod loader;

use crate::dataset::loader::DatasetLoader;
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub data: Vec<Datapoint>,
}

impl Dataset {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_loader(loader: impl DatasetLoader) -> anyhow::Result<Self> {
        let data = loader.load()?;

        Ok(Self { data })
    }

    pub fn print(&self, first: Option<usize>) {
        if let Some(first) = first {
            for (i, point) in self.data.iter().enumerate() {
                if i >= first {
                    break;
                }

                println!("{}", point.to_string());
            }
        } else {
            for point in self.data.iter() {
                println!("{}", point.to_string());
            }
        }
    }
}
