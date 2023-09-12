//! Provides walkers used to generate random walks by using a dynamic program.

pub mod correlated;
pub mod levy;
pub mod multi_step;
pub mod standard;

use crate::dataset::point::XYPoint;
use crate::dp::DynamicProgram;
use crate::walk::Walk;
use anyhow::{bail, Context};
use geo::algorithm::frechet_distance::FrechetDistance;
use geo::{line_string, Coord, LineString};
#[cfg(feature = "plotting")]
use plotters::prelude::*;
use rand::Rng;
use std::collections::HashSet;
use std::ops::{Index, Range};
use thiserror::Error;

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
