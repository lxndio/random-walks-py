use crate::dp::DynamicProgram;

fn walk(dp: &DynamicProgram, x: isize, y: isize, t: usize) -> usize {
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

pub fn random_trajectories(time_limit: usize) {
    let mut dp = DynamicProgram::new(time_limit);
    let (limit_neg, limit_pos) = dp.limits();

    dp.set(0, 0, 0, 1);

    for t in 1..=time_limit {
        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                dp.update(x, y, t, walk);
            }
        }
    }

    dp.print(10);
}

#[cfg(test)]
mod tests {
    use crate::models::simple_rw::random_trajectories;

    #[test]
    fn works() {
        random_trajectories(10);
    }
}
