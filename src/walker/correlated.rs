use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::DynamicProgramType::Multi;
use crate::dp::{DynamicProgram, DynamicProgramType};
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::Rng;

pub struct CorrelatedWalker;

impl Walker for CorrelatedWalker {
    fn generate_path(
        &self,
        dpt: &DynamicProgramType,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        let Multi(dp) = dpt else {
            return Err(WalkerError::WrongDynamicProgramType);
        };

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point for each variant
        for variant in 0..dp.variants() {
            if dp.at(to_x, to_y, time_steps, variant).is_zero() {
                return Err(WalkerError::NoPathExists);
            }
        }

        path.push((x, y));

        // Compute first (= last, because reconstructing backwards) step manually
        let direction: usize = rng.gen_range(0..4);

        match direction {
            1 => x -= 1,
            2 => y -= 1,
            3 => x += 1,
            4 => y += 1,
            _ => (),
        }

        let mut last_direction = direction;

        for t in (1..time_steps).rev() {
            path.push((x, y));

            let variant: usize = match last_direction {
                0 => 4,
                1 => 1,
                2 => 0,
                3 => 3,
                4 => 2,
                _ => panic!("Invalid last direction. This should not happen."),
            };

            let prev_probs = [
                dp.at(x, y, t - 1, variant),
                dp.at(x - 1, y, t - 1, variant),
                dp.at(x, y - 1, t - 1, variant),
                dp.at(x + 1, y, t - 1, variant),
                dp.at(x, y + 1, t - 1, variant),
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
            String::from("cwg")
        } else {
            String::from("Correlated Walker")
        }
    }
}