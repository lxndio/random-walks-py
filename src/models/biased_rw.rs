use num::BigUint;
use crate::dp::DynamicProgram;

fn biased_rw(dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint {
    let mut sum = dp.get(x, y, t);
    let (limit_neg, limit_pos) = dp.limits();

    if x > limit_neg {
        sum += dp.get(x - 1, y, t);
    }

    if y > limit_neg {
        sum += dp.get(x, y - 1, t);
    }

    if x < limit_pos {
        sum += dp.get(x + 1, y, t);
    }

    if y < limit_pos {
        sum += dp.get(x, y + 1, t);
    }

    sum
}

pub struct BiasedRwBuilder {}

impl BiasedRwBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::stats::Stats;
    use crate::dp::DynamicProgram;
    use crate::models::biased_rw::biased_rw;

    #[test]
    fn testing() {
        let mut dp = DynamicProgram::new(10, biased_rw);
        dp.count_paths();

        dp.print(1);

        assert_eq!(1, 1);
    }
}
