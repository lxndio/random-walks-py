use num::{One, Zero};

use crate::dp::DynamicProgram;

/// Problems that a dynamic program can solve.
pub trait Problems {
    /// For each cell, count the paths starting at `(0,0)` and ending at that cell.
    /// The counts are computed for each time step `0 ≤ t ≤ time_limit`, with
    /// `time_limit` being the time limit set for the dynamic program.
    fn count_paths(&mut self);

    /// Count how many paths starting at `(0,0)` and ending at `(x,y)` pass through each cell.
    /// The counts are computed for each time step `0 ≤ t ≤ time_limit`, with
    /// `time_limit` being the time limit set for the dynamic program.
    fn count_paths_over(&self, x: isize, y: isize, t: usize);
}

impl Problems for DynamicProgram {
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

    fn count_paths_over(&self, x: isize, y: isize, t: usize) {
        todo!()
    }
}
