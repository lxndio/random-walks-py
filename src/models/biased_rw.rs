use crate::dp::{DynamicProgram, WalkModel};
use num::BigUint;
use rand::Rng;

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct BiasedRw {
    pub direction: Direction,
    pub probability: f32,
}

impl WalkModel for BiasedRw {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
        let mut sum = dp.at(x, y, t);
        let (limit_neg, limit_pos) = dp.limits();

        let mut rng = rand::thread_rng();
        let dir_prob = rng.gen_range(0.0..=1.0);

        // TODO Does this make sense to check if the bias should be applied?
        if dir_prob >= self.probability {
            match self.direction {
                Direction::North if y > limit_neg => sum += dp.at(x, y - 1, t),
                Direction::East if x < limit_pos => sum += dp.at(x + 1, y, t),
                Direction::South if y < limit_pos => sum += dp.at(x, y + 1, t),
                Direction::West if x > limit_neg => sum += dp.at(x + 1, y, t),
                // TODO What should happen if a direction was chosen but it is beyond the border?
                _ => (),
            }
        } else {
            // TODO Should all directions be updated here or only the non-bias direction?
            // Doing all directions for now.
            if x > limit_neg {
                sum += dp.at(x - 1, y, t);
            }

            if y > limit_neg {
                sum += dp.at(x, y - 1, t);
            }

            if x < limit_pos {
                sum += dp.at(x + 1, y, t);
            }

            if y < limit_pos {
                sum += dp.at(x, y + 1, t);
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problems;
    use crate::dp::DynamicProgram;
    use crate::models::biased_rw::{BiasedRw, Direction};

    #[test]
    fn testing() {
        let rw = BiasedRw { direction: Direction::North, probability: 0.5 };

        let mut dp = DynamicProgram::new(10, rw);
        dp.count_paths();

        dp.print(1);

        assert_eq!(1, 1);
    }
}
