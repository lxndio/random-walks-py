//! Provides walkers used to generate random walks by using a dynamic program.

pub mod correlated;
pub mod levy;
pub mod multi_step;
pub mod standard;

use crate::dataset::point::{Point, XYPoint};
use crate::dp::DynamicProgram;
use geo::algorithm::frechet_distance::FrechetDistance;
use geo::{line_string, Coord, LineString};
use std::ops::Index;
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
