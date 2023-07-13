use crate::dp::{DynamicProgram, DynamicProgramOptions};
use crate::kernel::Kernel;
use num::Zero;
use std::fmt::Debug;

pub struct SimpleDynamicProgram {
    table: Vec<Vec<Vec<f64>>>,
    time_limit: usize,
    kernel: Kernel,
}

impl SimpleDynamicProgram {
    pub fn at(&self, x: isize, y: isize, t: usize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y].clone()
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    pub fn apply_kernel_at(&mut self, x: isize, y: isize, t: usize) {
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

        self.set(x, y, t, sum);
    }
}

impl DynamicProgram for SimpleDynamicProgram {
    fn new(options: DynamicProgramOptions) -> Self {
        let time_limit = options.time_limit;
        let kernel = options.kernel.expect("kernel option not set.");

        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 1]; 2 * time_limit + 1];
                time_limit + 1
            ],
            time_limit,
            kernel,
        }
    }

    fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    fn compute(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        self.set(0, 0, 0, 1.0);

        for t in 1..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    self.apply_kernel_at(x, y, t);
                }
            }
        }
    }

    fn print(&self, t: usize) {
        for y in 0..2 * self.time_limit + 1 {
            for x in 0..2 * self.time_limit + 1 {
                print!("{} ", self.table[t][x][y]);
            }

            println!();
        }
    }
}

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
