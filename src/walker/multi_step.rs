use crate::dp::DynamicProgram;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct MultiStepWalker {
    pub max_step_size: usize,
}

impl Walker for MultiStepWalker {
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
        let max_step_size = self.max_step_size as isize;

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return Err(WalkerError::NoPathExists);
        }

        for t in (1..=time_steps).rev() {
            path.push((x, y));

            let mut prev_probs = Vec::new();
            let mut movements = Vec::new();

            for i in x - max_step_size..=x + max_step_size {
                for j in y - max_step_size..=y + max_step_size {
                    if i == x || j == y {
                        prev_probs.push(dp.at(i, j, t - 1));
                        movements.push((i - x, j - y));
                    }
                }
            }

            let dist = WeightedIndex::new(prev_probs).unwrap();
            let direction = dist.sample(&mut rng);
            let (dx, dy) = movements[direction];

            x += dx;
            y += dy;
        }

        path.reverse();
        path.insert(0, (x, y));

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
