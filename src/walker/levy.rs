use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::DynamicProgram;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use pyo3::{pyclass, pymethods};
use rand::distributions::{WeightedError, WeightedIndex};
use rand::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct LevyWalker {
    pub jump_probability: f64,
    pub jump_distance: usize,
}

#[pymethods]
impl LevyWalker {
    #[new]
    pub fn new(jump_probability: f64, jump_distance: usize) -> Self {
        Self {
            jump_probability,
            jump_distance,
        }
    }

    // Trait function wrappers for Python

    pub fn generate_path(
        &self,
        dp: SimpleDynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        Walker::generate_path(self, &DynamicProgram::Simple(dp), to_x, to_y, time_steps)
    }

    pub fn generate_paths(
        &self,
        dp: SimpleDynamicProgram,
        qty: usize,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Vec<Walk>, WalkerError> {
        Walker::generate_paths(
            self,
            &DynamicProgram::Simple(dp),
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

impl Walker for LevyWalker {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        let DynamicProgram::Simple(dp) = dp else {
            return Err(WalkerError::WrongDynamicProgramType);
        };

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return Err(WalkerError::NoPathExists);
        }

        let mut t = time_steps;

        while t >= 1 {
            path.push((x as i64, y as i64).into());

            // Check if jump happens here
            let distance = if thread_rng().gen_range(0f64..1f64) <= self.jump_probability
                && t >= self.jump_distance
            {
                self.jump_distance
            } else {
                1
            };

            let prev_probs = [
                dp.at(x, y, t - distance),                     // Stay
                dp.at(x - distance as isize, y, t - distance), // West
                dp.at(x, y - distance as isize, t - distance), // North
                dp.at(x + distance as isize, y, t - distance), // East
                dp.at(x, y + distance as isize, t - distance), // South
            ];

            let direction = match WeightedIndex::new(prev_probs) {
                Ok(dist) => dist.sample(&mut rng),
                Err(WeightedError::AllWeightsZero) => return Err(WalkerError::InconsistentPath),
                _ => return Err(WalkerError::RandomDistributionError),
            };

            match direction {
                0 => (),                     // Stay
                1 => x -= distance as isize, // West
                2 => y -= distance as isize, // North
                3 => x += distance as isize, // East
                4 => y += distance as isize, // South
                _ => unreachable!("Other directions should not be chosen from the distribution"),
            }

            t -= distance;
        }

        path.reverse();
        path.insert(0, (x as i64, y as i64).into());

        Ok(path.into())
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("lw")
        } else {
            String::from("Lévy Walker")
        }
    }
}
