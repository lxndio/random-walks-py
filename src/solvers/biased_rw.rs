use crate::dp::DynamicProgram;
use crate::solvers::Direction;
use crate::solvers::Solver;
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

pub struct BiasedRwSolver {
    pub direction: Direction,
    pub probability: f32,
}

impl Solver for BiasedRwSolver {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Vec<(isize, isize)> {
        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // If there is no path leading to the given end point, return an empty path
        if dp.at(to_x, to_y, time_steps).is_zero() {
            return path;
        }

        for t in (1..=time_steps).rev() {
            path.push((x, y));

            let _total = dp.at(x, y, t);
            let prev_counts = [
                dp.at(x, y, t - 1),
                dp.at(x - 1, y, t - 1),
                dp.at(x, y - 1, t - 1),
                dp.at(x + 1, y, t - 1),
                dp.at(x, y + 1, t - 1),
            ];

            // Check if bias is used for this step
            let dir_prob = rng.gen_range(0.0..=1.0);

            if dir_prob <= self.probability {
                match self.direction {
                    Direction::North if !prev_counts[2].is_zero() => {
                        // y += 1;
                        // TODO which direction is right?
                        y -= 1;
                        continue;
                    }
                    Direction::East if !prev_counts[1].is_zero() => {
                        x -= 1;
                        continue;
                    }
                    Direction::South if !prev_counts[2].is_zero() => {
                        y -= 1;
                        continue;
                    }
                    Direction::West if !prev_counts[3].is_zero() => {
                        x += 1;
                        continue;
                    }
                    Direction::Stay if !prev_counts[0].is_zero() => continue,
                    _ => (),
                }
            }

            let dist = WeightedIndex::new(&prev_counts).unwrap();
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
            String::from("brw")
        } else {
            String::from("Biased RW")
        }
    }
}
