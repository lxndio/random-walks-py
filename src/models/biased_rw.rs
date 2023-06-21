use crate::dp::DynamicProgram;
use crate::models::{Direction, WalkModel};
use num::BigUint;

pub struct BiasedRw {
    pub direction: Direction,
    pub probability: f32,
}

impl WalkModel for BiasedRw {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
        let mut sum = dp.at(x, y, t);
        let (limit_neg, limit_pos) = dp.limits();

        match self.direction {
            Direction::North if y > limit_neg => {
                sum += dp.at(x, y + 1, t) * (self.probability * 10f32) as usize
            }
            Direction::East if x < limit_pos => {
                sum += dp.at(x + 1, y, t) * (self.probability * 10f32) as usize
            }
            Direction::South if y < limit_pos => {
                sum += dp.at(x, y - 1, t) * (self.probability * 10f32) as usize
            }
            Direction::West if x > limit_neg => {
                sum += dp.at(x - 1, y, t) * (self.probability * 10f32) as usize
            }
            // TODO What should happen if a direction was chosen but it is beyond the border?
            _ => (),
        }

        if x > limit_neg {
            sum += dp.at(x - 1, y, t) * ((1f32 - self.probability) * 10f32) as usize;
        }

        if y > limit_neg {
            sum += dp.at(x, y - 1, t) * ((1f32 - self.probability) * 10f32) as usize;
        }

        if x < limit_pos {
            sum += dp.at(x + 1, y, t) * ((1f32 - self.probability) * 10f32) as usize;
        }

        if y < limit_pos {
            sum += dp.at(x, y + 1, t) * ((1f32 - self.probability) * 10f32) as usize;
        }

        sum
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("brw")
        } else {
            String::from("Biased RW")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problem;
    use crate::dp::DynamicProgram;
    use crate::models::biased_rw::BiasedRw;
    use crate::models::Direction;

    #[test]
    fn testing() {
        let rw = BiasedRw {
            direction: Direction::North,
            probability: 0.5,
        };

        let mut dp = DynamicProgram::new(10, rw);
        dp.count_paths();

        dp.print(5);

        assert_eq!(1, 1);
    }
}
