//! Provides functionality for loading and processing [`Dataset`s](Dataset).
//!
//! # Dataset Creation
//!
//! The easiest way to create a [`Dataset`] is to use the
//! [`DatasetBuilder`](builder::DatasetBuilder).
//!
//! # Shrinking
//!
//! If only some specific entries of the dataset are relevant for later processing,
//! [`keep()`](Dataset::keep) can be used to remove all [`DataPoint`s](Datapoint) that are outside
//! of a specified index range. For example,
//!
//! ```
//! # use randomwalks_lib::dataset::Dataset;
//! # use randomwalks_lib::dataset::loader::CoordinateType;
//! #
//! # let mut dataset = Dataset::new(CoordinateType::XY);
//! #
//! dataset.keep(Some(1000), Some(2001));
//! ```
//!
//! will remove all entries but the ones with indices in the range `[1000, 2001)`. Notice that the
//! lower bound is inclusive, while the upper bound is exclusive. If one side of the range is
//! unspecified (`None`), the range will be open in that side.
//!
//! # Filtering
//!
//! Datasets can be filtered using different [`DatasetFilter`s](DatasetFilter). See the
//! documentation for a list of all filters including descriptions. A filter can be applied as
//! follows:
//!
//! ```
//! # use randomwalks_lib::dataset::{Dataset, DatasetFilter};
//! # use randomwalks_lib::dataset::loader::CoordinateType;
//! # use randomwalks_lib::xy;
//! #
//! # let mut dataset = Dataset::new(CoordinateType::XY);
//! #
//! dataset.filter(vec![
//!     DatasetFilter::ByCoordinates(Point::XY(xy!(100, 100)), Point::XY(xy!(500, 500)));
//! ]).unwrap();
//! ```
//!
//! # Coordinate Conversion
//!
//! When loading a dataset with GCS coordinates, the coordinates have to be converted into XY
//! coordinates, before generating random walks. For that purpose, the following function can be
//! used.
//!
//! ```
//! # use randomwalks_lib::dataset::Dataset;
//! # use randomwalks_lib::dataset::loader::CoordinateType;
//! #
//! # let mut dataset = Dataset::new(CoordinateType::XY);
//! #
//! dataset.convert_gcs_to_xy(-10000, 10000).unwrap();
//! ```
//!
//! When converting the coordinates, a range has to be specified to which the points get normalized.
//! This range depends on the dataset loaded and has to be set correspondingly to allow for large
//! enough distances between the points so that the points are different when represented using
//! integer coordinates.
//!
//! # Generating Random Walks
//!
//! There are two ways to generate random walks from a dataset. The first option generates a single
//! random walk and may be used if no more walks are needed or if implemented into some other
//! processing logic.
//!
//! The following example generates a random walk between the data points with indices 0 and 1 with
//! 400 time steps. A previously computed [`DynamicProgram`](crate::dp::DynamicProgram) and a
//! [`Walker`](crate::walker::Walker) must be specified.
//!
//! ```
//! # use randomwalks_lib::dataset::Dataset;
//! # use randomwalks_lib::dataset::loader::CoordinateType;
//! # use randomwalks_lib::dp::builder::DynamicProgramBuilder;
//! # use randomwalks_lib::dp::DynamicProgram;
//! # use randomwalks_lib::dp::simple::SimpleDynamicProgram;
//! # use randomwalks_lib::kernel::Kernel;
//! # use randomwalks_lib::kernel::simple_rw::SimpleRwGenerator;
//! # use randomwalks_lib::walker::standard::StandardWalker;
//! #
//! # let dataset = Dataset::new(CoordinateType::XY);
//! # let dp = DynamicProgramBuilder::new()
//! #     .simple()
//! #     .time_limit(400)
//! #     .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
//! #     .build()
//! #     .unwrap();
//! # let walker = Box::new(StandardWalker);
//! #
//! let path = dataset.rw_between(&dp, walker, 0, 1, 400).unwrap();
//! ```
//! It is also possible to generate many random walks between different pairs of points at once.
//! To do this, the [`DatasetWalksBuilder`](DatasetWalksBuilder) can be used.
//!
//! The following example generates 10 random walks each between all neighboring pairs of data
//! points between indices 0 and 100, i.e. 10 walks between data points 0 and 1, 10 walks between
//! data points 1 and 2, and so on. All walks have 400 time steps each. A previously computed
//! [`DynamicProgram`](crate::dp::DynamicProgram) and a [`Walker`](crate::walker::Walker) must be
//! specified.
//!
//! ```
//! # use randomwalks_lib::dataset::{Dataset, DatasetWalksBuilder};
//! # use randomwalks_lib::dataset::loader::CoordinateType;
//! # use randomwalks_lib::dp::builder::DynamicProgramBuilder;
//! # use randomwalks_lib::dp::DynamicProgram;
//! # use randomwalks_lib::dp::simple::SimpleDynamicProgram;
//! # use randomwalks_lib::kernel::Kernel;
//! # use randomwalks_lib::kernel::simple_rw::SimpleRwGenerator;
//! # use randomwalks_lib::walker::standard::StandardWalker;
//! #
//! # let dataset = Dataset::new(CoordinateType::XY);
//! # let dp = DynamicProgramBuilder::new()
//! #     .simple()
//! #     .time_limit(400)
//! #     .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
//! #     .build()
//! #     .unwrap();
//! # let walker = Box::new(StandardWalker);
//! #
//! let paths = DatasetWalksBuilder::new()
//!     .dataset(&dataset)
//!     .dp(&dp)
//!     .walker(walker)
//!     .from(0)
//!     .to(100)
//!     .count(10)
//!     .time_steps(400)
//!     .build()
//!     .unwrap();
//! ```
//!
//! Also, the number of time steps can be computed automatically. See the documentation of the
//! [`DatasetWalksBuilder`](DatasetWalksBuilder) for more information.

pub mod builder;
pub mod loader;
pub mod point;

use crate::dataset::loader::{CoordinateType, DatasetLoader};
use crate::dp::DynamicProgram;
use crate::walk::Walk;
use crate::walker::standard::StandardWalker;
use crate::walker::{Walker, WalkerType};
use anyhow::{anyhow, bail, Context};
use line_drawing::Bresenham;
use pathfinding::prelude::{build_path, dijkstra_all};
#[cfg(feature = "plotting")]
use plotters::prelude::*;
use point::{Coordinates, GCSPoint, Point, XYPoint};
use pyo3::{pyclass, pymethods, Py, PyAny, PyCell, PyObject, PyRef, PyRefMut, PyResult};
use rand::distributions::uniform::SampleBorrow;
use rand::Rng;
use std::collections::HashMap;
use thiserror::Error;
use time::macros::format_description;
use time::PrimitiveDateTime;

/// A filter that can be applied to a [`Dataset`] by calling [`Dataset::filter`].
#[derive(Debug)]
pub enum DatasetFilter {
    /// Filters the dataset by a given metadata key-value pair and only keeps points
    /// which have the corresponding metadata entry.
    ByMetadata(String, String),

    /// Filters the dataset by coordinates and only keeps points where the
    /// coordinates are in the range `[from, to]`.
    ByCoordinates(Point, Point),
}

/// A point in a dataset consisting of a [`Point`] and a set of metadata key-value pairs.
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Datapoint {
    pub point: Point,
    pub metadata: HashMap<String, String>,
}

#[pymethods]
impl Datapoint {
    pub fn __str__(&self) -> String {
        self.to_string()
    }
}

impl ToString for Datapoint {
    fn to_string(&self) -> String {
        format!("{} | {:?}", self.point.to_string(), self.metadata)
    }
}

#[pyclass]
pub struct DatasetIterator {
    inner: std::vec::IntoIter<Datapoint>,
}

#[pymethods]
impl DatasetIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<Datapoint> {
        self.inner.next()
    }
}

/// A dataset storing a set of 2d-points with associated metadata.
#[pyclass]
#[derive(Default)]
pub struct Dataset {
    data: Vec<Datapoint>,
    coordinate_type: CoordinateType,
}

#[pymethods]
impl Dataset {
    /// Create a new empty dataset.
    ///
    /// The `coordinate_type` parameter specifies the [`CoordinateType`] of the dataset.
    #[new]
    pub fn new(coordinate_type: CoordinateType) -> Self {
        Self {
            data: Vec::new(),
            coordinate_type,
        }
    }

    // TODO from loader

    /// Return the number of [`Datapoint`]s in the dataset.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the dataset is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the coordinate type of the dataset.
    pub fn coordinate_type(&self) -> CoordinateType {
        self.coordinate_type
    }

    /// Add a [`Datapoint`] to the dataset.
    pub fn push(&mut self, datapoint: Datapoint) {
        self.data.push(datapoint);
    }

    #[pyo3(name = "get")]
    pub fn py_get(&self, index: usize) -> Option<Datapoint> {
        self.data.get(index).cloned()
    }

    pub fn __iter__(&self) -> DatasetIterator {
        DatasetIterator {
            inner: self.data.clone().into_iter(),
        }
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
    // TODO make filter work
    // pub fn filter(&mut self, filters: Vec<DatasetFilter>) -> anyhow::Result<usize> {
    //     let mut filtered_data = Vec::new();
    //
    //     for datapoint in self.data.iter() {
    //         let mut keep = true;
    //
    //         for filter in filters.iter() {
    //             match filter {
    //                 DatasetFilter::ByMetadata(key, value) => {
    //                     if datapoint.metadata.get(key) != Some(value) {
    //                         keep = false;
    //                         break;
    //                     }
    //                 }
    //                 DatasetFilter::ByCoordinates(from, to) => match self.coordinate_type {
    //                     CoordinateType::GCS => {
    //                         let Point::GCS(from) = from else {
    //                             return Err(anyhow!("Expected GCS coordinates in filter."));
    //                         };
    //                         let Point::GCS(to) = to else {
    //                             return Err(anyhow!("Expected GCS coordinates in filter."));
    //                         };
    //
    //                         let x: f64 = datapoint.point.x();
    //                         let y: f64 = datapoint.point.y();
    //
    //                         if x < from.x || x > to.x || y < from.y || y > to.y {
    //                             keep = false;
    //                             break;
    //                         }
    //                     }
    //                     CoordinateType::XY => {
    //                         let Point::XY(from) = from else {
    //                             return Err(anyhow!("Expected XY coordinates in filter."));
    //                         };
    //                         let Point::XY(to) = to else {
    //                             return Err(anyhow!("Expected XY coordinates in filter."));
    //                         };
    //
    //                         let x: i64 = datapoint.point.x();
    //                         let y: i64 = datapoint.point.y();
    //
    //                         if x < from.x || x > to.x || y < from.y || y > to.y {
    //                             keep = false;
    //                             break;
    //                         }
    //                     }
    //                 },
    //             }
    //         }
    //
    //         if keep {
    //             filtered_data.push(datapoint.clone());
    //         }
    //     }
    //
    //     let filtered = filtered_data.len();
    //
    //     self.data = filtered_data;
    //
    //     Ok(filtered)
    // }

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
    pub fn convert_gcs_to_xy(&mut self, from: i64, to: i64) -> anyhow::Result<()> {
        if self.coordinate_type != CoordinateType::GCS {
            bail!("dataset is not in GCS coordinates");
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

    #[pyo3(name = "rw_between")]
    pub fn py_rw_between(
        slf: &PyCell<Self>,
        dp: PyObject,
        walker: PyObject,
        from: usize,
        to: usize,
        time_steps: usize,
    ) -> anyhow::Result<Walk> {
        let dp: DynamicProgram = dp.extract(slf.py())?;
        let walker: WalkerType = walker.extract(slf.py())?;

        let walker: Box<dyn Walker> = match walker {
            WalkerType::Standard(walker) => Box::new(walker),
            WalkerType::Correlated(walker) => Box::new(walker),
            WalkerType::MultiStep(walker) => Box::new(walker),
            WalkerType::Levy(walker) => Box::new(walker),
        };

        slf.borrow().rw_between(&dp, walker, from, to, time_steps)
    }

    pub fn direct_between(&self, from: usize, to: usize) -> anyhow::Result<Walk> {
        let from = &self.get(from).context("from index out of bounds.")?.point;
        let to = &self.get(to).context("to index out of bounds.")?.point;

        let Point::XY(from) = *from else {
            bail!("Points have to be in XY coordinates.");
        };
        let Point::XY(to) = *to else {
            bail!("Points have to be in XY coordinates.");
        };

        // Create graph from space between from and to

        let (min_x, max_x) = (from.x.min(to.x), from.x.max(to.x));
        let (min_y, max_y) = (from.y.min(to.y), from.y.max(to.y));

        let mut vertices = Vec::new();
        let mut edges = HashMap::new();

        let important_vs: Vec<XYPoint> = Bresenham::new(from.into(), to.into())
            .map(XYPoint::from)
            .collect();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut adj = Vec::new();

                if x > min_x {
                    let p = XYPoint::from((x - 1, y));

                    if important_vs.contains(&p) {
                        adj.push((p, 0usize));
                    } else {
                        adj.push((p, 10usize));
                    }
                }
                if x < max_x {
                    let p = XYPoint::from((x + 1, y));

                    if important_vs.contains(&p) {
                        adj.push((p, 0usize));
                    } else {
                        adj.push((p, 10usize));
                    }
                }
                if y > min_y {
                    let p = XYPoint::from((x, y - 1));

                    if important_vs.contains(&p) {
                        adj.push((p, 0usize));
                    } else {
                        adj.push((p, 10usize));
                    }
                }
                if y < max_y {
                    let p = XYPoint::from((x, y + 1));

                    if important_vs.contains(&p) {
                        adj.push((p, 0usize));
                    } else {
                        adj.push((p, 10usize));
                    }
                }

                vertices.push(XYPoint::from((x, y)));
                edges.insert(XYPoint::from((x, y)), adj);
            }
        }

        // Run Dijkstra on graph

        let successors = |i: &u32| {
            let v = vertices[*i as usize];
            let adj = edges[&v].clone();

            adj.iter()
                .map(|(v, weight)| {
                    (
                        vertices.iter().position(|x| x == v).unwrap() as u32,
                        *weight,
                    )
                })
                .collect::<Vec<(u32, usize)>>()
        };

        let from = vertices.iter().position(|x| x == &from).unwrap() as u32;
        let to = vertices.iter().position(|x| x == &to).unwrap() as u32;

        let reachables = dijkstra_all(&from, successors);
        let walk = build_path(&to, &reachables);

        let walk = walk.iter().map(|i| vertices[*i as usize]).collect();

        Ok(walk)
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
    #[cfg(feature = "plotting")]
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

impl Dataset {
    /// Create a dataset filled with data that is loaded by the given [`DatasetLoader`].
    pub fn from_loader(loader: impl DatasetLoader) -> anyhow::Result<Self> {
        let data = loader.load()?;

        Ok(Self {
            data,
            coordinate_type: loader.coordinate_type(),
        })
    }

    /// Return an iterator over the [`Datapoint`]s in the dataset.
    pub fn iter(&self) -> std::slice::Iter<'_, Datapoint> {
        self.data.iter()
    }

    /// Return a reference to the [`Datapoint`] at the given index in the dataset.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Datapoint> {
        self.data.get(index)
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

        let walk = walker
            .generate_path(
                dp,
                translated_to.x as isize,
                translated_to.y as isize,
                time_steps,
            )
            .context("error while generating random walk path")?;

        // Translate all coordinates in walk back to original coordinates
        Ok(walk
            .iter()
            .map(|p| (p.x + from.x(), p.y + from.y()).into())
            .collect())
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

#[derive(Error, Debug)]
pub enum DatasetWalksBuilderError {
    #[error("a dataset must be provided")]
    NoDatasetSet,
    #[error("a dynamic program must be provided")]
    NoDynamicProgramSet,
    #[error("a walker must be provided")]
    NoWalkerSet,
    #[error("the number of time steps for the walks must be set or auto time steps must be used")]
    NoTimeStepsSet,
    #[error("the dataset must contain XY points for walk computation")]
    DatasetNotXY,
}

#[derive(Default, Debug, PartialEq)]
pub enum TimeStepsBy {
    Fixed(usize),
    TimeDifference(f64, &'static str),
    Distance(f64),
    #[default]
    None,
}

pub struct DatasetWalksBuilder<'a> {
    dataset: Option<&'a Dataset>,
    dp: Option<&'a DynamicProgram>,
    walker: Option<Box<dyn Walker>>,
    from: usize,
    to: Option<usize>,
    count: usize,
    time_steps: TimeStepsBy,
}

impl<'a> Default for DatasetWalksBuilder<'a> {
    fn default() -> Self {
        Self {
            dataset: None,
            dp: None,
            walker: None,
            from: 0,
            to: None,
            count: 1,
            time_steps: TimeStepsBy::None,
        }
    }
}

impl<'a> DatasetWalksBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn dataset(mut self, dataset: &'a Dataset) -> Self {
        self.dataset = Some(dataset);

        self
    }

    pub fn dp(mut self, dp: &'a DynamicProgram) -> Self {
        self.dp = Some(dp);

        self
    }

    pub fn walker(mut self, walker: Box<dyn Walker>) -> Self {
        self.walker = Some(walker);

        self
    }

    pub fn from(mut self, from: usize) -> Self {
        self.from = from;

        self
    }

    pub fn to(mut self, to: usize) -> Self {
        self.to = Some(to);

        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = count;

        self
    }

    pub fn time_steps(mut self, time_steps: usize) -> Self {
        self.time_steps = TimeStepsBy::Fixed(time_steps);

        self
    }

    /// Automatically compute the number of time steps allowed for each walk.
    ///
    /// When set, the number of time steps allowed for each walk is automatically computed
    /// by comparing the timestamps of two points and calculating the time difference between
    /// two points. The time difference is then mapped to time steps using `time_step_len` which
    /// specifies the length of a time step in seconds. `metadata_key` specifies where
    /// timestamps are stored for each point.
    pub fn time_steps_by_time(mut self, time_step_len: f64, metadata_key: &'static str) -> Self {
        self.time_steps = TimeStepsBy::TimeDifference(time_step_len, metadata_key);

        self
    }

    pub fn time_steps_by_dist(mut self, multiplier: f64) -> Self {
        self.time_steps = TimeStepsBy::Distance(multiplier);

        self
    }

    pub fn build(self) -> anyhow::Result<Vec<Walk>> {
        let Some(dataset) = self.dataset else {
            return Err(DatasetWalksBuilderError::NoDatasetSet)?;
        };
        let Some(dp) = self.dp else {
            return Err(DatasetWalksBuilderError::NoDynamicProgramSet)?;
        };
        // let Some(walker) = self.walker else {
        //     return Err(DatasetWalksBuilderError::NoWalkerSet)?;
        // };

        if dataset.coordinate_type() != CoordinateType::XY {
            return Err(DatasetWalksBuilderError::DatasetNotXY)?;
        }

        if self.time_steps == TimeStepsBy::None {
            return Err(DatasetWalksBuilderError::NoTimeStepsSet)?;
        }

        let to = match self.to {
            Some(to) => to,
            None => dataset.len() - 1,
        };

        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

        let mut walks = Vec::new();

        for i in self.from..to {
            let time_steps = match self.time_steps {
                TimeStepsBy::Fixed(time_steps) => time_steps,
                TimeStepsBy::TimeDifference(time_step_len, metadata_key) => {
                    let datetime1 = PrimitiveDateTime::parse(
                        dataset.get(i).unwrap().metadata.get(metadata_key).unwrap(),
                        &format,
                    )?;
                    let datetime2 = PrimitiveDateTime::parse(
                        dataset
                            .get(i + 1)
                            .unwrap()
                            .metadata
                            .get(metadata_key)
                            .unwrap(),
                        &format,
                    )?;

                    let diff = (datetime2 - datetime1).as_seconds_f64();

                    println!(
                        "Time difference: {}, time steps: {}",
                        diff,
                        diff / time_step_len
                    );

                    (diff / time_step_len) as usize
                }
                TimeStepsBy::Distance(multiplier) => {
                    let point1 = dataset.get(i).unwrap().clone().point;
                    let point2 = dataset.get(i + 1).unwrap().clone().point;

                    let (x1, y1): (i64, i64) = (point1.x(), point1.y());
                    let (x2, y2): (i64, i64) = (point2.x(), point2.y());

                    let dist = (x1 - x2).abs() + (y1 - y2).abs();

                    println!("Time steps: {}", (dist as f64 * multiplier) as usize);

                    (dist as f64 * multiplier) as usize
                }
                TimeStepsBy::None => {
                    unimplemented!("this should not happen because of the check above")
                }
            };

            for _ in 0..self.count {
                let walker = StandardWalker;
                walks.push(
                    dataset
                        .rw_between(dp, Box::new(walker), i, i + 1, time_steps)
                        .context("could not generate walk")?,
                );
            }
        }

        Ok(walks)
    }
}
