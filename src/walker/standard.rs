use crate::dp::simple::DynamicProgram;
use crate::dp::DynamicProgramPool;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use pyo3::{pyclass, pymethods, PyAny};
use rand::distributions::{WeightedError, WeightedIndex};
use rand::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct StandardWalker;

#[pymethods]
impl StandardWalker {
    #[new]
    pub fn new() -> Self {
        Self
    }

    // Trait function wrappers for Python

    pub fn generate_path(
        &self,
        dp: DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        Walker::generate_path(
            self,
            &DynamicProgramPool::Single(dp),
            to_x,
            to_y,
            time_steps,
        )
    }

    pub fn generate_paths(
        &self,
        dp: DynamicProgram,
        qty: usize,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Vec<Walk>, WalkerError> {
        Walker::generate_paths(
            self,
            &DynamicProgramPool::Single(dp),
            qty,
            to_x,
            to_y,
            time_steps,
        )
    }

    pub fn name(&self, short: bool) -> String {
        Walker::name(self, short)
    }
}

impl Walker for StandardWalker {
    fn generate_path(
        &self,
        dp: &DynamicProgramPool,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        let DynamicProgramPool::Single(dp) = dp else {
            return Err(WalkerError::RequiresSingleDynamicProgram);
        };

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return Err(WalkerError::NoPathExists);
        }

        for t in (1..time_steps).rev() {
            path.push((x as i64, y as i64).into());

            let prev_probs = [
                dp.at_or(x, y, t - 1, 0.0),     // Stay
                dp.at_or(x - 1, y, t - 1, 0.0), // West
                dp.at_or(x, y - 1, t - 1, 0.0), // North
                dp.at_or(x + 1, y, t - 1, 0.0), // East
                dp.at_or(x, y + 1, t - 1, 0.0), // South
            ];

            let direction = match WeightedIndex::new(prev_probs) {
                Ok(dist) => dist.sample(&mut rng),
                Err(WeightedError::AllWeightsZero) => return Err(WalkerError::InconsistentPath),
                _ => return Err(WalkerError::RandomDistributionError),
            };

            match direction {
                0 => (),     // Stay
                1 => x -= 1, // West
                2 => y -= 1, // North
                3 => x += 1, // East
                4 => y += 1, // South
                _ => unreachable!("Other directions should not be chosen from the distribution"),
            }
        }

        path.reverse();
        path.insert(0, (x as i64, y as i64).into());

        Ok(path.into())
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("swg")
        } else {
            String::from("Standard Walker")
        }
    }
}
