//! Provides walkers used to generate random walks by using a dynamic program.

pub mod correlated;
pub mod levy;
pub mod multi_step;
pub mod standard;

use crate::dataset::point::{Point, XYPoint};
use crate::dp::DynamicProgram;
use anyhow::{bail, Context};
use geo::algorithm::frechet_distance::FrechetDistance;
use geo::{line_string, Coord, LineString};
use plotters::coord::types::RangedCoordi64;
#[cfg(feature = "plotting")]
use plotters::prelude::*;
use rand::Rng;
use std::collections::HashSet;
use std::ops::{Index, Range};
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Walk(pub Vec<(isize, isize)>);

impl Walk {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<(isize, isize)> {
        self.0.iter()
    }

    pub fn frechet_distance(&self, other: &Walk) -> f64 {
        let self_line = LineString::from(self);
        let other_line = LineString::from(other);

        self_line.frechet_distance(&other_line)
    }

    pub fn directness_deviation(&self) -> f64 {
        let self_line = LineString::from(self);
        let other_line = line_string![
            (x: self.0.first().unwrap().0 as f64, y: self.0.first().unwrap().1 as f64),
            (x: self.0.last().unwrap().0 as f64, y: self.0.last().unwrap().1 as f64),
        ];

        self_line.frechet_distance(&other_line)
    }

    /// Translates all points of a walk.
    ///
    /// ```
    /// # use randomwalks_lib::walker::Walk;
    /// # use randomwalks_lib::dataset::point::XYPoint;
    /// # use randomwalks_lib::xy;
    ///
    /// let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).translate(xy!(5, 1));
    /// let walk2 = Walk(vec![(5, 1), (7, 4), (12, 6)]);
    ///
    /// assert_eq!(walk1, walk2);
    /// ```
    pub fn translate(&self, by: XYPoint) -> Walk {
        Walk(
            self.0
                .iter()
                .map(|(x, y)| (x + by.x as isize, y + by.y as isize))
                .collect(),
        )
    }

    /// Scales all points of a walk.
    ///
    /// ```
    /// # use randomwalks_lib::walker::Walk;
    /// # use randomwalks_lib::dataset::point::XYPoint;
    /// # use randomwalks_lib::xy;
    ///
    /// let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).scale(xy!(2, 1));
    /// let walk2 = Walk(vec![(0, 0), (4, 3), (14, 5)]);
    ///
    /// assert_eq!(walk1, walk2);
    /// ```
    pub fn scale(&self, by: XYPoint) -> Walk {
        Walk(
            self.0
                .iter()
                .map(|(x, y)| (x * by.x as isize, y * by.y as isize))
                .collect(),
        )
    }

    /// Rotates all points of a walk around the origin.
    ///
    /// ```
    /// # use randomwalks_lib::walker::Walk;
    /// # use randomwalks_lib::dataset::point::XYPoint;
    /// # use randomwalks_lib::xy;
    ///
    /// let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).rotate(90.0);
    /// let walk2 = Walk(vec![(0, 0), (-3, 2), (-5, 7)]);
    ///
    /// assert_eq!(walk1, walk2);
    /// ```
    pub fn rotate(&self, degrees: f64) -> Walk {
        let rad = degrees.to_radians();

        Walk(
            self.0
                .iter()
                .map(|(x, y)| {
                    (
                        (*x as f64 * rad.cos() - *y as f64 * rad.sin()) as isize,
                        (*y as f64 * rad.cos() + *x as f64 * rad.sin()) as isize,
                    )
                })
                .collect(),
        )
    }

    #[cfg(feature = "plotting")]
    pub fn plot<S: Into<String>>(&self, filename: S) -> anyhow::Result<()> {
        if self.0.is_empty() {
            bail!("Cannot plot empty walk");
        }

        let filename = filename.into();

        // Initialize plot

        let (coordinate_range_x, coordinate_range_y) = point_range(&vec![self.clone()]);

        let root = BitMapBackend::new(&filename, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(20)
            .build_cartesian_2d(coordinate_range_x, coordinate_range_y)?;

        chart.configure_mesh().draw()?;

        // Draw walk

        chart.draw_series(LineSeries::new(self.0.clone(), &BLACK))?;

        // Draw start and end point

        chart.draw_series(PointSeries::of_element(
            vec![*self.0.first().unwrap(), *self.0.last().unwrap()],
            5,
            &BLACK,
            &|c, s, st| {
                EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font())
            },
        ))?;

        Ok(())
    }

    pub fn plot_multiple<S: Into<String>>(walks: &Vec<Walk>, filename: S) -> anyhow::Result<()> {
        let filename = filename.into();

        // Initialize plot

        let (coordinate_range_x, coordinate_range_y) = point_range(walks);

        let root = BitMapBackend::new(&filename, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(20)
            .build_cartesian_2d(coordinate_range_x, coordinate_range_y)?;

        chart.configure_mesh().draw()?;

        // Draw walks

        let walks: Vec<Vec<(isize, isize)>> = walks.iter().map(|x| x.0.clone()).collect();
        let mut rng = rand::thread_rng();

        for walk in walks.iter() {
            chart.draw_series(LineSeries::new(
                walk.clone(),
                RGBColor(
                    rng.gen_range(30..220),
                    rng.gen_range(30..220),
                    rng.gen_range(30..220),
                ),
            ))?;
        }

        // Find unique start and end points

        let mut se_points = HashSet::new();

        for walk in walks.iter() {
            se_points.insert((
                walk.first().copied().unwrap(),
                walk.last().copied().unwrap(),
            ));
        }

        // Draw start and end points

        for (start, end) in se_points {
            chart.draw_series(PointSeries::of_element(
                vec![start, end],
                5,
                &BLACK,
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font())
                },
            ))?;
        }

        Ok(())
    }
}

#[cfg(feature = "plotting")]
fn point_range(walks: &Vec<Walk>) -> (Range<isize>, Range<isize>) {
    // Compute size of plotting area

    let points: Vec<_> = walks.iter().map(|x| &x.0).flatten().copied().collect();

    let xs: Vec<isize> = points.iter().map(|(x, _)| x).copied().collect();
    let ys: Vec<isize> = points.iter().map(|(_, y)| y).copied().collect();

    let x_range = (*xs.iter().min().unwrap(), *xs.iter().max().unwrap());
    let y_range = (*ys.iter().min().unwrap(), *ys.iter().max().unwrap());

    let coordinate_range_x = x_range.0 - 5..x_range.1 + 5;
    let coordinate_range_y = y_range.1 + 5..y_range.0 - 5;

    (coordinate_range_x, coordinate_range_y)
}

impl From<Vec<(isize, isize)>> for Walk {
    fn from(value: Vec<(isize, isize)>) -> Self {
        Self(value)
    }
}

impl From<Walk> for Vec<(isize, isize)> {
    fn from(value: Walk) -> Self {
        value.0
    }
}

impl From<&Walk> for LineString<f64> {
    fn from(value: &Walk) -> Self {
        Self(
            value
                .0
                .iter()
                .map(|(x, y)| (*x as f64, *y as f64))
                .map(|p| Coord::from(p))
                .collect(),
        )
    }
}

impl FromIterator<(isize, isize)> for Walk {
    fn from_iter<T: IntoIterator<Item = (isize, isize)>>(iter: T) -> Self {
        let mut c = Vec::new();

        for i in iter {
            c.push(i);
        }

        Self(c)
    }
}

impl Index<usize> for Walk {
    type Output = (isize, isize);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub trait Walker {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError>;

    fn generate_paths(
        &self,
        dp: &DynamicProgram,
        qty: usize,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Vec<Walk>, WalkerError> {
        let mut paths = Vec::new();

        for _ in 0..qty {
            paths.push(self.generate_path(dp, to_x, to_y, time_steps)?);
        }

        Ok(paths)
    }

    fn name(&self, short: bool) -> String;
}

#[derive(Error, Debug)]
pub enum WalkerError {
    #[error("wrong type of dynamic program given")]
    WrongDynamicProgramType,

    #[error("no path exists")]
    NoPathExists,
}

#[cfg(test)]
mod tests {
    use crate::dataset::point::XYPoint;
    use crate::walker::Walk;
    use crate::xy;

    #[test]
    fn test_walk_translate() {
        let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).translate(xy!(5, 1));
        let walk2 = Walk(vec![(5, 1), (7, 4), (12, 6)]);

        assert_eq!(walk1, walk2);
    }

    #[test]
    fn test_walk_scale() {
        let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).scale(xy!(2, 1));
        let walk2 = Walk(vec![(0, 0), (4, 3), (14, 5)]);

        assert_eq!(walk1, walk2);
    }

    #[test]
    fn test_walk_rotate() {
        let walk1 = Walk(vec![(0, 0), (2, 3), (7, 5)]).rotate(90.0);
        let walk2 = Walk(vec![(0, 0), (-3, 2), (-5, 7)]);

        assert_eq!(walk1, walk2);
    }
}
