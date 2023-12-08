use crate::dp::simple::DynamicProgram;
use crate::dp::DynamicProgramPool;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use pyo3::{pyclass, pymethods};
use rand::distributions::{WeightedError, WeightedIndex};
use rand::prelude::*;
use crate::kernel::Kernel;

#[pyclass]
#[derive(Clone)]
pub struct MultiStepWalker {
    pub max_step_size: usize,
    pub kernel: Kernel,
}

#[pymethods]
impl MultiStepWalker {
    #[new]
    pub fn new(max_step_size: usize, kernel: Kernel) -> Self {
        Self {
            max_step_size,
            kernel,
        }
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

impl Walker for MultiStepWalker {
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
        let max_step_size = self.max_step_size as isize;

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return Err(WalkerError::NoPathExists);
        }

        for t in (1..time_steps).rev() {
            path.push((x as i64, y as i64).into());

            let mut prev_probs = Vec::new();
            let mut movements = Vec::new();

            for i in x - max_step_size..=x + max_step_size {
                for j in y - max_step_size..=y + max_step_size {
                    let p_b = dp.at_or(i, j, t - 1, 0.0);
                    let p_a = dp.at_or(x, y, t, 0.0);
                    let p_a_b = self.kernel.at(i - x, j - y);

                    prev_probs.push((p_a_b * p_b) / p_a);
                    movements.push((i - x, j - y));
                }
            }

            let direction = match WeightedIndex::new(prev_probs) {
                Ok(dist) => dist.sample(&mut rng),
                Err(WeightedError::AllWeightsZero) => {
                    eprintln!("time step: {t}, x: {x}, y: {y}");
                    return Err(WalkerError::InconsistentPath)
                },
                _ => return Err(WalkerError::RandomDistributionError),
            };
            let (dx, dy) = movements[direction];

            x += dx;
            y += dy;
        }

        path.reverse();
        path.insert(0, (x as i64, y as i64).into());

        Ok(path.into())
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("msw")
        } else {
            String::from("Multi Step Walker")
        }
    }
}
