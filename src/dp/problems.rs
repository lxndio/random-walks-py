use num::{One, Zero};

use crate::dp::DynamicProgram;

trait Problems {
    fn count_paths(&mut self);
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
