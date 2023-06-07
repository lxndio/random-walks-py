use crate::dp::{DynamicProgram, WalkModel};
use num::BigUint;

pub struct SimpleRw;

impl WalkModel for SimpleRw {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
        let mut sum = dp.at(x, y, t);
        let (limit_neg, limit_pos) = dp.limits();

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

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problems;
    use crate::dp::DynamicProgram;
    use crate::models::simple_rw::SimpleRw;

    #[test]
    fn testing() {
        let mut dp = DynamicProgram::new(10, SimpleRw);
        dp.count_paths();

        dp.print(3);

        assert_eq!(1, 1);
    }
}
