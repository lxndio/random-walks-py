use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::DynamicProgram;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use pyo3::{pyclass, pymethods, PyAny};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[pyclass]
pub struct StandardWalker;

#[pymethods]
impl StandardWalker {
    #[new]
    pub fn new() -> Self {
        Self
    }

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
}

impl Walker for StandardWalker {
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

        for t in (1..=time_steps).rev() {
            path.push((x as i64, y as i64).into());

            let prev_probs = [
                dp.at(x, y, t - 1),     // Stay
                dp.at(x - 1, y, t - 1), // West
                dp.at(x, y - 1, t - 1), // North
                dp.at(x + 1, y, t - 1), // East
                dp.at(x, y + 1, t - 1), // South
            ];

            let dist = WeightedIndex::new(prev_probs).unwrap();
            let direction = dist.sample(&mut rng);

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
