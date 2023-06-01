use crate::dp::DynamicProgram;
use num::One;

pub trait Stats {
    fn count_paths(&mut self);
}

impl Stats for DynamicProgram {
    fn count_paths(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        self.set(0, 0, 0, One::one());

        for t in 1..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    self.update(x, y, t, self.walk);
                }
            }
        }
    }
}
