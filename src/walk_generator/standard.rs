use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::DynamicProgram;
use crate::walk_generator::{Walk, WalkGenerator};
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct StandardWalkGenerator;

impl WalkGenerator for StandardWalkGenerator {
    fn generate_path(
        &self,
        dp: &SimpleDynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Walk {
        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // If there is no path leading to the given end point, return an empty path
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return path;
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

        path
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("swg")
        } else {
            String::from("Standard Walk Generator")
        }
    }
}
