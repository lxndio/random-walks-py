pub mod loader;

use crate::dataset::loader::{CoordinateType, DatasetLoader};
use plotters::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct GCSPoint {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for GCSPoint {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl ToString for GCSPoint {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

#[derive(Default, Debug, Clone)]
pub struct XYPoint {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for XYPoint {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
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

    /// Find the minimum and maximum coordinates of the dataset.
    ///
    /// Returns None if the dataset is empty. Otherwise, returns the minimum and maximum coordinates
    /// of the dataset as a tuple of two [`Point`]s. The first [`Point`] contains the minimum
    /// coordinates, and the second [`Point`] contains the maximum coordinates.
    pub fn min_max(&self, from: Option<usize>, to: Option<usize>) -> Option<(Point, Point)> {
        if self.data.is_empty() {
            return None;
        }

        let from = from.unwrap_or(0);
        let to = to.unwrap_or(self.data.len());

        match self.coordinate_type {
            CoordinateType::GCS => {
                let mut min = (f64::MAX, f64::MAX);
                let mut max = (f64::MIN, f64::MIN);

                for datapoint in self.data.iter().skip(from).take(to) {
                    match &datapoint.point {
                        Point::GCS(point) => {
                            if point.x < min.0 {
                                min.0 = point.x;
                            }

                            if point.y < min.1 {
                                min.1 = point.y;
                            }

                            if point.x > max.0 {
                                max.0 = point.x;
                            }

                            if point.y > max.1 {
                                max.1 = point.y;
                            }
                        }
                        Point::XY(_) => (),
                    }
                }

                Some((
                    Point::GCS(GCSPoint { x: min.0, y: min.1 }),
                    Point::GCS(GCSPoint { x: max.0, y: max.1 }),
                ))
            }
            CoordinateType::XY => {
                let mut min = (i64::MAX, i64::MAX);
                let mut max = (i64::MIN, i64::MIN);

                for datapoint in self.data.iter().skip(from).take(to) {
                    match &datapoint.point {
                        Point::GCS(_) => (),
                        Point::XY(point) => {
                            if point.x < min.0 {
                                min.0 = point.x;
                            }

                            if point.y < min.1 {
                                min.1 = point.y;
                            }

                            if point.x > max.0 {
                                max.0 = point.x;
                            }

                            if point.y > max.1 {
                                max.1 = point.y;
                            }
                        }
                    }
                }

                Some((
                    Point::XY(XYPoint { x: min.0, y: min.1 }),
                    Point::XY(XYPoint { x: max.0, y: max.1 }),
                ))
            }
        }
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

    // TODO implement color_by
    pub fn plot(
        &self,
        path: String,
        from: Option<usize>,
        to: Option<usize>,
        color_by: Option<String>,
    ) -> anyhow::Result<()> {
        if self.coordinate_type == CoordinateType::GCS {
            unimplemented!("Plotting GCS points is not implemented.");
        }

        let (min, max) = match self.min_max(from, to).unwrap() {
            (Point::XY(min), Point::XY(max)) => (min, max),
            _ => unreachable!(),
        };

        let from = from.unwrap_or(0);
        let to = to.unwrap_or(self.data.len());

        let coordinate_range_x = min.x..max.x;
        let coordinate_range_y = min.y..max.y;

        let root = BitMapBackend::new(&path, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("Dataset plot (points {} to {})", from, to),
                ("sans-serif", 20).into_font(),
            )
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(coordinate_range_x, coordinate_range_y)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(PointSeries::of_element(
            self.data.iter().skip(from).take(to).map(|datapoint| {
                if let Point::XY(point) = &datapoint.point {
                    (point.x, point.y)
                } else {
                    unreachable!()
                }
            }),
            2,
            &BLACK,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        root.present()?;

        Ok(())
    }
}
