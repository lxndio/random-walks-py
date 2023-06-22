use crate::dp::DynamicProgram;
use crate::solvers::Direction;
use crate::solvers::Solver;
use num::{BigUint, One, Zero};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;
use std::thread::current;

pub struct CorrelatedRwSolver {
    pub persistence: f32,
}

impl Solver for CorrelatedRwSolver {
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

            let total = dp.at(x, y, t);
            let prev_counts = [
                dp.at(x, y, t - 1),
                dp.at(x - 1, y, t - 1),
                dp.at(x, y - 1, t - 1),
                dp.at(x + 1, y, t - 1),
                dp.at(x, y + 1, t - 1),
            ];

            // Check if persistence is used for this step
            let dir_prob = rng.gen_range(0.0..=1.0);

            if dir_prob <= self.persistence {
                match last_step_direction(&path) {
                    Some(Direction::North) if !prev_counts[2].is_zero() => {
                        // y += 1;
                        // TODO which direction is right?
                        y -= 1;
                        continue;
                    }
                    Some(Direction::East) if !prev_counts[1].is_zero() => {
                        x -= 1;
                        continue;
                    }
                    Some(Direction::South) if !prev_counts[2].is_zero() => {
                        y -= 1;
                        continue;
                    }
                    Some(Direction::West) if !prev_counts[3].is_zero() => {
                        x += 1;
                        continue;
                    }
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
            String::from("crw")
        } else {
            String::from("Correlated RW")
        }
    }
}

fn last_step_direction(path: &Vec<(isize, isize)>) -> Option<Direction> {
    if path.len() < 2 {
        return None;
    }

    let current_pos = path[path.len() - 1];
    let last_pos = path[path.len() - 2];

    if current_pos.1 - 1 == last_pos.1 && current_pos.0 == last_pos.0 {
        Some(Direction::North)
    } else if current_pos.0 + 1 == last_pos.0 && current_pos.1 == last_pos.1 {
        Some(Direction::East)
    } else if current_pos.1 + 1 == last_pos.1 && current_pos.0 == last_pos.0 {
        Some(Direction::South)
    } else if current_pos.0 - 1 == last_pos.0 && current_pos.1 == last_pos.1 {
        Some(Direction::West)
    } else if current_pos.0 == last_pos.0 && current_pos.1 == last_pos.1 {
        Some(Direction::Stay)
    } else {
        panic!("Can't determine last step's direction.")
    }
}
