use crate::dp::DynamicProgram;
use crate::generators::Generator;
use num::BigUint;

pub struct SimpleGenerator;

impl Generator for SimpleGenerator {
    fn step(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
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

    fn name(&self, short: bool) -> String {
        if short {
            String::from("srw")
        } else {
            String::from("Simple RW")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::problems::Problem;
    use crate::dp::DynamicProgram;
    use crate::generators::simple::SimpleGenerator;

    #[test]
    fn testing() {
        let mut dp = DynamicProgram::new(10, SimpleGenerator);
        dp.count_paths();

        dp.print(3);

        assert_eq!(1, 1);
    }
}
