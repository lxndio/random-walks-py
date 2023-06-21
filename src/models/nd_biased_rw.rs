use crate::dp::DynamicProgram;
use crate::models::{Direction, WalkModel};
use num::BigUint;
use rand::Rng;

/// A non-deterministic approach to computing a DP table for biased random walks.
pub struct NdBiasedRw {
    pub direction: Direction,
    pub probability: f32,
}

impl WalkModel for NdBiasedRw {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
        let mut sum = dp.at(x, y, t);
        let (limit_neg, limit_pos) = dp.limits();

        let mut rng = rand::thread_rng();
        let dir_prob = rng.gen_range(0.0..=1.0);

        // TODO Does this make sense to check if the bias should be applied?
        if dir_prob <= self.probability {
            match self.direction {
                Direction::North if y > limit_neg => sum += dp.at(x, y - 1, t),
                Direction::East if x < limit_pos => sum += dp.at(x + 1, y, t),
                Direction::South if y < limit_pos => sum += dp.at(x, y + 1, t),
                Direction::West if x > limit_neg => sum += dp.at(x + 1, y, t),
                // TODO What should happen if a direction was chosen but it is beyond the border?
                _ => (),
            }
        } else {
            if self.direction != Direction::West && x > limit_neg {
                sum += dp.at(x - 1, y, t);
            }

            if self.direction != Direction::North && y > limit_neg {
                sum += dp.at(x, y - 1, t);
            }

            if self.direction != Direction::East && x < limit_pos {
                sum += dp.at(x + 1, y, t);
            }

            if self.direction != Direction::South && y < limit_pos {
                sum += dp.at(x, y + 1, t);
            }
        }

        sum
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("ndbrw")
        } else {
            String::from("Non-deterministic biased RW")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problem;
    use crate::dp::DynamicProgram;
    use crate::models::nd_biased_rw::NdBiasedRw;
    use crate::models::Direction;

    #[test]
    fn testing() {
        let rw = NdBiasedRw {
            direction: Direction::North,
            probability: 0.5,
        };

        let mut dp = DynamicProgram::new(10, rw);
        dp.count_paths();

        dp.print(5);

        assert_eq!(1, 1);
    }
}
