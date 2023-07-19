use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::DynamicProgramType::Simple;
use crate::dp::{DynamicProgram, DynamicProgramType};
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct StandardWalker;

impl Walker for StandardWalker {
    fn generate_path(
        &self,
        dpt: &DynamicProgramType,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        let Simple(dp) = dpt else {
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
            path.push((x, y));

            let prev_probs = [
                dp.at(x, y, t - 1),
                dp.at(x - 1, y, t - 1),
                dp.at(x, y - 1, t - 1),
                dp.at(x + 1, y, t - 1),
                dp.at(x, y + 1, t - 1),
            ];

            let dist = WeightedIndex::new(&prev_probs).unwrap();
            let direction = dist.sample(&mut rng);

            match direction {
                1 => x -= 1,
                2 => y -= 1,
                3 => x += 1,
                4 => y += 1,
                _ => (),
            }
        }

        path.reverse();
        path.insert(0, (x, y));

        Ok(path)
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("swg")
        } else {
            String::from("Standard Walker")
        }
    }
}
