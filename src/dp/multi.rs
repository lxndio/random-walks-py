use crate::dp::{DynamicProgram, DynamicProgramOptions};
use crate::kernel::Kernel;
use num::Zero;

pub struct MultiDynamicProgram {
    table: Vec<Vec<Vec<Vec<f64>>>>,
    time_limit: usize,
    kernels: Vec<Kernel>,
}

impl MultiDynamicProgram {
    pub fn at(&self, x: isize, y: isize, t: usize, variant: usize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][variant][x][y].clone()
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, variant: usize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][variant][x][y] = val;
    }
}

impl DynamicProgram for MultiDynamicProgram {
    fn new(options: DynamicProgramOptions) -> Self {
        let time_limit = options.time_limit;
        let kernels = options.kernels.expect("kernels option not set.");

        Self {
            table: vec![
                vec![
                    vec![vec![Zero::zero(); 2 * time_limit + 1]; 2 * time_limit + 1];
                    kernels.len()
                ];
                time_limit + 1
            ],
            time_limit,
            kernels,
        }
    }

    fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    fn apply_kernel_at(&mut self, x: isize, y: isize, t: usize) {
        todo!()
        // let ks = (self.kernel.size() / 2) as isize;
        // let (limit_neg, limit_pos) = self.limits();
        // let mut sum = 0.0;
        //
        // for i in x - ks..=x + ks {
        //     if i < limit_neg || i > limit_pos {
        //         continue;
        //     }
        //
        //     for j in y - ks..=y + ks {
        //         if j < limit_neg || j > limit_pos {
        //             continue;
        //         }
        //
        //         // Kernel coordinates are inverted offset, i.e. -(i - x) and -(j - y)
        //         let kernel_x = x - i;
        //         let kernel_y = y - j;
        //
        //         sum += self.at(i, j, t - 1) * self.kernel.at(kernel_x, kernel_y);
        //     }
        // }
        //
        // self.set(x, y, t, sum);
    }

    fn compute(&mut self) {
        todo!()
    }

    fn print(&self, t: usize) {
        todo!()
    }
}
