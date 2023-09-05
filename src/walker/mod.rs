//! Provides walkers used to generate random walks by using a dynamic program.

pub mod correlated;
pub mod multi_step;
pub mod standard;

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
