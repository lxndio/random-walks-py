//! A collection of different problems that can be solved using the dynamic program.

use crate::dp::DynamicProgram;
use num::bigint::RandBigInt;
use num::{BigUint, One, Zero};
use rand::Rng;

/// A problem that can be solved using the dynamic program.
pub trait Problem {
    fn count_paths(&mut self);
    fn generate_path(&self, x: isize, y: isize, t: usize) -> Vec<(isize, isize)>;
    fn generate_path_bias(&self, x: isize, y: isize, t: usize) -> Vec<(isize, isize)>;
}

impl Problem for DynamicProgram {
    /// For each cell, count the number of paths leading to it in each time step.
    fn count_paths(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        self.set(0, 0, 0, One::one());

        for t in 1..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    self.update(x, y, t);
                }
            }
        }
    }

    /// Generate a path from `(0,0)` to `(x,y)` in `time_steps`.
    fn generate_path(&self, to_x: isize, to_y: isize, time_steps: usize) -> Vec<(isize, isize)> {
        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // If there is no path leading to the given end point, return an empty path
        if self.at(to_x, to_y, time_steps).is_zero() {
            return path;
        }

        for t in (1..=time_steps).rev() {
            path.push((x, y));

            let total = self.at(x, y, t);
            let prev_counts = vec![
                self.at(x, y, t - 1),
                self.at(x - 1, y, t - 1),
                self.at(x, y - 1, t - 1),
                self.at(x + 1, y, t - 1),
                self.at(x, y + 1, t - 1),
            ];

            let mut rchoice = rng.gen_biguint_range(&BigUint::zero(), &total);
            let mut choice = 0;

            // TODO Can crash, probably if there is no path?
            // Can no path occur normally or only if the random walk model is not correct?
            while rchoice >= prev_counts[choice] {
                rchoice -= &prev_counts[choice];
                choice += 1;
            }

            match choice {
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

    /// Experiment
    fn generate_path_bias(&self, to_x: isize, to_y: isize, time_steps: usize) -> Vec<(isize, isize)> {
        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // If there is no path leading to the given end point, return an empty path
        if self.at(to_x, to_y, time_steps).is_zero() {
            return path;
        }

        for t in (1..=time_steps).rev() {
            path.push((x, y));

            let total = self.at(x, y, t);
            let prev_counts = vec![
                self.at(x, y, t - 1),
                self.at(x - 1, y, t - 1),
                self.at(x, y - 1, t - 1),
                self.at(x + 1, y, t - 1),
                self.at(x, y + 1, t - 1),
            ];

            let mut rng = rand::thread_rng();
            let dir_prob = rng.gen_range(0.0..=1.0);

            // Testing with hard-coded 75% north bias
            if dir_prob <= 0.75 && !prev_counts[4].is_zero() {
                y += 1;
                continue;
            }

            // TODO why can it crash here with lower bound > upper bound?
            let mut rchoice = rng.gen_biguint_range(&BigUint::zero(), &total);
            let mut choice = 0;

            // TODO Can crash, probably if there is no path?
            while rchoice >= prev_counts[choice] {
                rchoice -= &prev_counts[choice];
                choice += 1;
            }

            match choice {
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
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problem;
    use crate::dp::DynamicProgram;
    use crate::models::simple_rw::SimpleRw;

    #[test]
    fn testing() {
        let mut dp = DynamicProgram::new(10, SimpleRw);
        dp.count_paths();

        let path = dp.generate_path(2, 5, 10);

        println!("{:?}", path);

        assert_eq!(1, 1);
    }
}
