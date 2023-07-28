pub mod loader;
pub mod point;

use crate::dataset::loader::{CoordinateType, DatasetLoader};
use crate::dp::{DynamicProgram, DynamicProgramType, DynamicPrograms};
use crate::walker::{Walk, Walker};
use anyhow::{anyhow, bail, Context};
use num::Signed;
use plotters::prelude::*;
use point::{Coordinates, GCSPoint, Point, XYPoint};
use rand::Rng;
use std::collections::HashMap;

/// A filter that can be applied to a [`Dataset`] by calling [`Dataset::filter`].
pub enum DatasetFilter {
    /// Filter the dataset by a given metadata key-value pair and only keeps points
    /// which have the corresponding metadata entry.
    ByMetadata(String, String),

    /// Filter the dataset by coordinates and only keeps points where the
    /// coordinates are in the range `[from, to]`.
    ByCoordinates(Point, Point),
}

/// A point in a dataset consisting of a [`Point`] and a set of metadata key-value pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct Datapoint {
    pub point: Point,
    pub metadata: HashMap<String, String>,
}

impl ToString for Datapoint {
    fn to_string(&self) -> String {
        format!("{} | {:?}", self.point.to_string(), self.metadata)
    }
}

/// A dataset storing a set of 2d-points with associated metadata.
pub struct Dataset {
    data: Vec<Datapoint>,
    coordinate_type: CoordinateType,
}

impl Dataset {
    /// Create a new empty dataset.
    ///
    /// The `coordinate_type` parameter specifies the [`CoordinateType`] of the dataset.
    pub fn new(coordinate_type: CoordinateType) -> Self {
        Self {
            data: Vec::new(),
            coordinate_type,
        }
    }

    /// Create a dataset filled with data that is loaded by the given [`DatasetLoader`].
    pub fn from_loader(loader: impl DatasetLoader) -> anyhow::Result<Self> {
        let data = loader.load()?;

        Ok(Self {
            data,
            coordinate_type: loader.coordinate_type(),
        })
    }

    /// Return the number of [`Datapoint`]s in the dataset.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Add a [`Datapoint`] to the dataset.
    pub fn push(&mut self, datapoint: Datapoint) {
        self.data.push(datapoint);
    }

    /// Return a reference to the [`Datapoint`] at the given index in the dataset.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Datapoint> {
        self.data.get(index)
    }

    /// Return an iterator over the [`Datapoint`]s in the dataset.
    pub fn iter(&self) -> std::slice::Iter<'_, Datapoint> {
        self.data.iter()
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

    /// Remove all datapoints from the dataset, keeping only the datapoints that match
    /// the given [`DatasetFilter`]s.
    ///
    /// Returns an error if a filter is invalid, otherwise returns the number of datapoints
    /// that were kept.
    pub fn filter(&mut self, filters: Vec<DatasetFilter>) -> anyhow::Result<usize> {
        let mut filtered_data = Vec::new();

        for datapoint in self.data.iter() {
            let mut keep = true;

            for filter in filters.iter() {
                match filter {
                    DatasetFilter::ByMetadata(key, value) => {
                        if datapoint.metadata.get(key) != Some(value) {
                            keep = false;
                            break;
                        }
                    }
                    DatasetFilter::ByCoordinates(from, to) => match self.coordinate_type {
                        CoordinateType::GCS => {
                            let Point::GCS(from) = from else {
                                    return Err(anyhow!("Expected GCS coordinates in filter."));
                                };
                            let Point::GCS(to) = to else {
                                    return Err(anyhow!("Expected GCS coordinates in filter."));
                                };

                            let x: f64 = datapoint.point.x();
                            let y: f64 = datapoint.point.y();

                            if x < from.x || x > to.x || y < from.y || y > to.y {
                                keep = false;
                                break;
                            }
                        }
                        CoordinateType::XY => {
                            let Point::XY(from) = from else {
                                    return Err(anyhow!("Expected XY coordinates in filter."));
                                };
                            let Point::XY(to) = to else {
                                    return Err(anyhow!("Expected XY coordinates in filter."));
                                };

                            let x: i64 = datapoint.point.x();
                            let y: i64 = datapoint.point.y();

                            if x < from.x || x > to.x || y < from.y || y > to.y {
                                keep = false;
                                break;
                            }
                        }
                    },
                }
            }

            if keep {
                filtered_data.push(datapoint.clone());
            }
        }

        let filtered = filtered_data.len();

        self.data = filtered_data;

        Ok(filtered)
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

    pub fn rw_between(
        &self,
        dp: &DynamicProgram,
        walker: Box<dyn Walker>,
        from: usize,
        to: usize,
        time_steps: usize,
    ) -> anyhow::Result<Walk> {
        let from = &self.get(from).context("from index out of bounds.")?.point;
        let to = &self.get(to).context("to index out of bounds.")?.point;

        let Point::XY(from) = *from else {
            bail!("Points have to be in XY coordinates.");
        };
        let Point::XY(to) = *to else {
            bail!("Points have to be in XY coordinates.");
        };

        // Translate `to`, s.t. it still has the same relative position from `from`, under the
        // condition that `from` is (0, 0)
        let translated_to = to - from;

        let mut walk = walker
            .generate_path(
                &dp,
                translated_to.x as isize,
                translated_to.y as isize,
                time_steps,
            )
            .context("error while generating random walk path")?;

        // Translate all coordinates in walk back to original coordinates
        Ok(walk
            .iter()
            .map(|(x, y)| (x + from.x() as isize, y + from.y() as isize))
            .collect())
    }

    /// Print all [`Datapoint`]s in the dataset with index in range [from, to).
    pub fn print(&self, from: Option<usize>, to: Option<usize>) {
        let from = from.unwrap_or(0);
        let to = to.unwrap_or(self.data.len());

        for i in from..to {
            println!("{}:\t{}", i, self.data[i].to_string());
        }
    }

    /// Plot all [`Datapoint`]s in the dataset with index in range [from, to).
    ///
    /// Saves the plot to the given `path`.
    ///
    /// If `color_by` is `Some`, the points will be colored differently for each value of the
    /// given metadata key.
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
    use crate::dataset::point::{Point, XYPoint};
    use crate::dataset::{Datapoint, Dataset, DatasetFilter};
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

    #[test]
    fn test_dataset_filter_metadata() {
        let mut dataset = Dataset::new(CoordinateType::XY);
        let mut filtered_dataset = Dataset::new(CoordinateType::XY);

        for i in 0..500 {
            dataset.push(Datapoint {
                point: Point::XY(XYPoint { x: i, y: i }),
                metadata: HashMap::new(),
            });
        }

        let mut metadata = HashMap::new();
        metadata.insert("test".into(), "test".into());

        for i in 0..500 {
            dataset.push(Datapoint {
                point: Point::XY(XYPoint { x: i, y: i }),
                metadata: metadata.clone(),
            });

            filtered_dataset.push(Datapoint {
                point: Point::XY(XYPoint { x: i, y: i }),
                metadata: metadata.clone(),
            });
        }

        let filter = DatasetFilter::ByMetadata("test".into(), "test".into());
        let res = dataset.filter(vec![filter]);

        assert!(res.is_ok());

        assert!(filtered_dataset
            .data
            .iter()
            .all(|item| dataset.data.contains(item)));
    }

    #[test]
    fn test_dataset_filter_coordinates() {
        let mut dataset = Dataset::new(CoordinateType::XY);
        let mut filtered_dataset = Dataset::new(CoordinateType::XY);

        for i in 0..1000 {
            dataset.push(Datapoint {
                point: Point::XY(XYPoint { x: i, y: i }),
                metadata: HashMap::new(),
            });

            if i >= 500 {
                filtered_dataset.push(Datapoint {
                    point: Point::XY(XYPoint { x: i, y: i }),
                    metadata: HashMap::new(),
                });
            }
        }

        let filter = DatasetFilter::ByCoordinates(
            Point::XY(XYPoint { x: 500, y: 500 }),
            Point::XY(XYPoint { x: 1000, y: 1000 }),
        );
        let res = dataset.filter(vec![filter]);

        assert!(res.is_ok());

        assert!(filtered_dataset
            .data
            .iter()
            .all(|item| dataset.data.contains(item)));
    }
}
