use crate::dp::DynamicPrograms;
use crate::kernel::Kernel;
use anyhow::Context;
use num::Zero;
#[cfg(feature = "plotting")]
use plotters::prelude::*;
use std::fmt::Debug;
use std::time::Instant;

pub struct SimpleDynamicProgram {
    pub(crate) table: Vec<Vec<Vec<f64>>>,
    pub(crate) time_limit: usize,
    pub(crate) kernel: Kernel,
    pub(crate) field_probabilities: Vec<Vec<f64>>,
}

impl SimpleDynamicProgram {
    pub fn at(&self, x: isize, y: isize, t: usize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y]
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    fn apply_kernel_at(&mut self, x: isize, y: isize, t: usize) {
        let ks = (self.kernel.size() / 2) as isize;
        let (limit_neg, limit_pos) = self.limits();
        let mut sum = 0.0;

        for i in x - ks..=x + ks {
            if i < limit_neg || i > limit_pos {
                continue;
            }

            for j in y - ks..=y + ks {
                if j < limit_neg || j > limit_pos {
                    continue;
                }

                // Kernel coordinates are inverted offset, i.e. -(i - x) and -(j - y)
                let kernel_x = x - i;
                let kernel_y = y - j;

                sum += self.at(i, j, t - 1) * self.kernel.at(kernel_x, kernel_y);
            }
        }

        self.set(x, y, t, sum * self.field_probability_at(x, y));
    }

    fn field_probability_at(&self, x: isize, y: isize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.field_probabilities[x][y]
    }
}

impl DynamicPrograms for SimpleDynamicProgram {
    #[cfg(not(tarpaulin_include))]
    fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    fn compute(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        self.set(0, 0, 0, 1.0);

        let start = Instant::now();

        for t in 1..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    self.apply_kernel_at(x, y, t);
                }
            }
        }

        let duration = start.elapsed();

        println!("Computation took {:?}", duration);
    }

    #[cfg(not(tarpaulin_include))]
    fn field_probabilities(&self) -> Vec<Vec<f64>> {
        self.field_probabilities.clone()
    }

    #[cfg(not(tarpaulin_include))]
    #[cfg(feature = "plotting")]
    fn heatmap(&self, path: String, t: usize) -> anyhow::Result<()> {
        let (limit_neg, limit_pos) = self.limits();
        let coordinate_range = limit_neg as i32..(limit_pos + 1) as i32;

        let root = BitMapBackend::new(&path, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Heatmap for t = {}", t), ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(coordinate_range.clone(), coordinate_range.clone())?;

        chart.configure_mesh().draw()?;

        let iter = self.table[t]
            .iter()
            .enumerate()
            .flat_map(|(x, l)| {
                l.iter().enumerate().map(move |(y, v)| {
                    (x as i32 - limit_pos as i32, y as i32 - limit_pos as i32, v)
                })
            });

        let min = iter
            .clone()
            .min_by(|(_, _, v1), (_, _, v2)| v1.total_cmp(v2))
            .context("Could not compute minimum value")?
            .2;
        let max = iter
            .clone()
            .max_by(|(_, _, v1), (_, _, v2)| v1.total_cmp(v2))
            .context("Could not compute minimum value")?
            .2;

        chart.draw_series(PointSeries::of_element(iter, 1, &BLACK, &|c, s, _st| {
            Rectangle::new(
                [(c.0, c.1), (c.0 + s, c.1 + s)],
                HSLColor(
                    (*c.2 - min) / (max - min),
                    0.7,
                    if c.2.is_zero() {
                        0.0
                    } else {
                        ((*c.2 - min).ln_1p() / (max - min).ln_1p()).clamp(0.1, 1.0)
                    },
                )
                .filled(),
            )
        }))?;

        root.present()?;

        Ok(())
    }

    #[cfg(not(tarpaulin_include))]
    fn print(&self, t: usize) {
        for y in 0..2 * self.time_limit + 1 {
            for x in 0..2 * self.time_limit + 1 {
                print!("{} ", self.table[t][x][y]);
            }

            println!();
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl Debug for SimpleDynamicProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicProgram")
            .field("time_limit", &self.time_limit)
            .finish()
    }
}

impl PartialEq for SimpleDynamicProgram {
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit && self.table == other.table
    }
}

impl Eq for SimpleDynamicProgram {}

#[cfg(test)]
mod tests {
    use crate::dp::builder::DynamicProgramBuilder;
    use crate::dp::{DynamicProgram, DynamicPrograms};
    use crate::kernel::biased_rw::BiasedRwGenerator;
    use crate::kernel::simple_rw::SimpleRwGenerator;
    use crate::kernel::{Direction, Kernel};

    #[test]
    fn test_simple_dp_at() {
        let mut dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp.compute();

        let DynamicProgram::Simple(dp) = dp else {
            unreachable!();
        };

        assert_eq!(dp.at(0, 0, 0), 1.0);
    }

    #[test]
    fn test_simple_dp_set() {
        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        let DynamicProgram::Simple(mut dp) = dp else {
            unreachable!();
        };

        dp.set(0, 0, 0, 10.0);

        assert_eq!(dp.at(0, 0, 0,), 10.0);
    }

    #[test]
    #[rustfmt::skip]
    fn test_simple_dp_apply_kernel_at() {
        let mut fps = vec![vec![1.0; 21]; 21];

        fps[10][10] = 0.75;

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .field_probabilities(fps)
            .build()
            .unwrap();

        let DynamicProgram::Simple(mut dp) = dp else {
            unreachable!();
        };

        dp.set(0, 0, 0, 0.5);
        dp.set(-1, 0, 0, 0.5);
        dp.apply_kernel_at(0, 0, 1);

        let rounded_res = format!("{:.2}", dp.at(0, 0, 1)).parse::<f64>().unwrap();

        assert_eq!(rounded_res, 0.15);
    }

    #[test]
    fn test_compute() {
        let mut dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(1)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp.compute();

        let DynamicProgram::Simple(mut dp) = dp else {
            unreachable!();
        };

        assert_eq!(dp.at(0, 0, 1), 0.2);
        assert_eq!(dp.at(-1, 0, 1), 0.2);
        assert_eq!(dp.at(1, 0, 1), 0.2);
        assert_eq!(dp.at(0, -1, 1), 0.2);
        assert_eq!(dp.at(0, 1, 1), 0.2);
    }

    #[test]
    fn test_dp_eq() {
        let mut dp1 = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp1.compute();

        let mut dp2 = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp2.compute();

        let DynamicProgram::Simple(mut dp1) = dp1 else {
            unreachable!();
        };
        let DynamicProgram::Simple(mut dp2) = dp2 else {
            unreachable!();
        };

        assert_eq!(dp1, dp2);
    }

    #[test]
    fn test_dp_not_eq() {
        let mut dp1 = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp1.compute();

        let mut dp2 = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(
                Kernel::from_generator(BiasedRwGenerator {
                    probability: 0.5,
                    direction: Direction::North,
                })
                .unwrap(),
            )
            .build()
            .unwrap();

        dp2.compute();

        let DynamicProgram::Simple(mut dp1) = dp1 else {
            unreachable!();
        };
        let DynamicProgram::Simple(mut dp2) = dp2 else {
            unreachable!();
        };

        assert_ne!(dp1, dp2);
    }
}
