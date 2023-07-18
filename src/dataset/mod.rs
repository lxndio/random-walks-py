pub mod loader;

use crate::dataset::loader::{CoordinateType, DatasetLoader};
use anyhow::Context;
use num::Signed;
use plotters::prelude::*;
use rand::Rng;
use std::collections::HashMap;

pub trait Coordinates<T: Signed> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GCSPoint {
    x: f64,
    y: f64,
}

impl Coordinates<f64> for GCSPoint {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct XYPoint {
    x: i64,
    y: i64,
}

impl Coordinates<i64> for XYPoint {
    fn x(&self) -> i64 {
        self.x
    }

    fn y(&self) -> i64 {
        self.y
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Point {
    GCS(GCSPoint),
    XY(XYPoint),
}

impl Coordinates<f64> for Point {
    fn x(&self) -> f64 {
        match self {
            Self::GCS(point) => point.x,
            Self::XY(_) => panic!("XY points use i64 instead of f64."),
        }
    }

    fn y(&self) -> f64 {
        match self {
            Self::GCS(point) => point.y,
            Self::XY(_) => panic!("XY points use i64 instead of f64."),
        }
    }
}

impl Coordinates<i64> for Point {
    fn x(&self) -> i64 {
        match self {
            Self::GCS(_) => panic!("GCS points use f64 instead of i64."),
            Self::XY(point) => point.x,
        }
    }

    fn y(&self) -> i64 {
        match self {
            Self::GCS(_) => panic!("GCS points use f64 instead of i64."),
            Self::XY(point) => point.y,
        }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        match self {
            Point::GCS(p) => format!("GCS{}", p.to_string()),
            Point::XY(p) => format!("XY{}", p.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn push(&mut self, datapoint: Datapoint) {
        self.data.push(datapoint);
    }

    pub fn get(&self, index: usize) -> Option<&Datapoint> {
        self.data.get(index)
    }

    /// Remove all datapoints from the dataset, keeping only the datapoints in the range
    /// `[from, to)`.
    ///
    /// If `from` is `None`, then the range starts at the beginning of the dataset. If `to` is
    /// `None`, then the range ends at the end of the dataset.
    pub fn keep(&mut self, from: Option<usize>, to: Option<usize>) {
        let from = from.unwrap_or(0);
        let to = to.unwrap_or(self.data.len());

        self.data = self.data[from..to].to_vec();
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

        // Set colors for different classes

        let mut colors: HashMap<(i64, i64), RGBColor> = HashMap::new();

        if let Some(color_by) = &color_by {
            let mut class_colors = HashMap::new();

            for datapoint in self.data.iter().skip(from).take(to) {
                class_colors.insert(
                    datapoint
                        .metadata
                        .get(color_by)
                        .context("Found datapoint without color_by metadata key.")?
                        .clone(),
                    RGBColor(0, 0, 0),
                );
            }

            let mut rng = rand::thread_rng();

            for color in class_colors.values_mut() {
                *color = RGBColor(rng.gen(), rng.gen(), rng.gen());
            }

            for datapoint in self.data.iter().skip(from).take(to) {
                colors.insert(
                    (datapoint.point.x(), datapoint.point.y()),
                    class_colors[&datapoint.metadata[color_by]],
                );
            }
        }

        // Draw plot

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

        let iter = self.data.iter().skip(from).take(to).map(|datapoint| {
            if let Point::XY(point) = &datapoint.point {
                (point.x, point.y)
            } else {
                unreachable!()
            }
        });

        if color_by.is_some() {
            chart.draw_series(PointSeries::of_element(iter, 2, &BLACK, &|c, s, st| {
                let style = ShapeStyle {
                    color: RGBAColor::from(colors[&c]),
                    filled: true,
                    stroke_width: st.stroke_width,
                };

                EmptyElement::at(c) + Circle::new((0, 0), s, style)
            }))?;
        } else {
            chart.draw_series(PointSeries::of_element(iter, 2, &BLACK, &|c, s, st| {
                EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
            }))?;
        }

        root.present()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dataset::loader::CoordinateType;
    use crate::dataset::{Datapoint, Dataset, Point, XYPoint};
    use std::collections::HashMap;

    #[test]
    fn test_dataset_keep() {
        let mut dataset = Dataset::new(CoordinateType::XY);
        let mut keep_dataset = Dataset::new(CoordinateType::XY);

        for i in 0..1000 {
            dataset.push(Datapoint {
                point: Point::XY(XYPoint { x: i, y: i }),
                metadata: HashMap::new(),
            });

            if i >= 100 && i < 200 {
                keep_dataset.push(Datapoint {
                    point: Point::XY(XYPoint { x: i, y: i }),
                    metadata: HashMap::new(),
                })
            }
        }

        dataset.keep(Some(100), Some(200));

        assert!(keep_dataset
            .data
            .iter()
            .all(|item| dataset.data.contains(item)));
    }
}
