use crate::dp::DynamicProgram;
use crate::walker::{Walk, Walker, WalkerError};
use num::Zero;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct LevyWalker;

impl Walker for LevyWalker {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Result<Walk, WalkerError> {
        let DynamicProgram::Multi(dp) = dp else {
            return Err(WalkerError::WrongDynamicProgramType);
        };

        let mut path = Vec::new();
        let (mut x, mut y) = (to_x, to_y);
        let mut rng = rand::thread_rng();

        // Check if any path exists leading to the given end point
        if dp.at(to_x, to_y, time_steps, 0).is_zero() {
            return Err(WalkerError::NoPathExists);
        }

        let mut t = time_steps;

        while t > 1 {
            path.push((x as i64, y as i64).into());

            // Check if jump happens here
            // TODO does it make sense to use t - 1 here instead of t?
            let _jump_prob = dp.at(x, y, t, 1);
            // println!("t: {}, jp: {}", t, jump_prob);
            if thread_rng().gen_range(0f64..1f64) <= 0.008 {
                println!("Jump!");
                let prev_probs = [
                    dp.at(x, y, t - 20, 0),     // Stay
                    dp.at(x - 20, y, t - 1, 0), // West
                    dp.at(x, y - 20, t - 1, 0), // North
                    dp.at(x + 20, y, t - 1, 0), // East
                    dp.at(x, y + 20, t - 1, 0), // South
                ];

                let dist = WeightedIndex::new(prev_probs).unwrap();
                let direction = dist.sample(&mut rng);

                match direction {
                    0 => (),      // Stay
                    1 => x -= 20, // West
                    2 => y -= 20, // North
                    3 => x += 20, // East
                    4 => y += 20, // South
                    _ => {
                        unreachable!("Other directions should not be chosen from the distribution")
                    }
                }

                t -= 20;
                continue;
            }

            let prev_probs = [
                dp.at(x, y, t - 1, 0),     // Stay
                dp.at(x - 1, y, t - 1, 0), // West
                dp.at(x, y - 1, t - 1, 0), // North
                dp.at(x + 1, y, t - 1, 0), // East
                dp.at(x, y + 1, t - 1, 0), // South
            ];

            let dist = WeightedIndex::new(prev_probs).unwrap();
            let direction = dist.sample(&mut rng);

            match direction {
                0 => (),     // Stay
                1 => x -= 1, // West
                2 => y -= 1, // North
                3 => x += 1, // East
                4 => y += 1, // South
                _ => unreachable!("Other directions should not be chosen from the distribution"),
            }

            t -= 1;
        }

        path.reverse();
        path.insert(0, (x as i64, y as i64).into());

        Ok(path.into())
    }

    fn name(&self, short: bool) -> String {
        if short {
            String::from("lw")
        } else {
            String::from("Lévy Walker")
        }
    }
}
