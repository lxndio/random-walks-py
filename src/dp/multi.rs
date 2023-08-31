use crate::dp::DynamicPrograms;
use crate::kernel::Kernel;
use std::time::Instant;

pub struct MultiDynamicProgram {
    pub(crate) table: Vec<Vec<Vec<Vec<f64>>>>,
    pub(crate) time_limit: usize,
    pub(crate) kernels: Vec<Kernel>,
    pub(crate) field_probabilities: Vec<Vec<f64>>,
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

    pub fn variants(&self) -> usize {
        self.kernels.len()
    }

    pub fn apply_kernels_at(&mut self, x: isize, y: isize, t: usize) {
        for (variant, kernel) in self.kernels.clone().iter().enumerate() {
            let ks = (kernel.size() / 2) as isize;
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

                    sum += self.at(i, j, t - 1, variant) * kernel.at(kernel_x, kernel_y);
                }
            }

            self.set(x, y, t, variant, sum * self.field_probability_at(x, y));
        }
    }

    fn field_probability_at(&self, x: isize, y: isize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.field_probabilities[x][y]
    }
}

impl DynamicPrograms for MultiDynamicProgram {
    fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    fn compute(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        for variant in 0..self.kernels.len() {
            self.set(0, 0, 0, variant, 1.0);
        }

        let start = Instant::now();

        for t in 1..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    self.apply_kernels_at(x, y, t);
                }
            }
        }

        let duration = start.elapsed();

        println!("Computation took {:?}", duration);
    }

    fn field_probabilities(&self) -> Vec<Vec<f64>> {
        self.field_probabilities.clone()
    }

    fn heatmap(&self, _path: String, _t: usize) -> anyhow::Result<()> {
        todo!()
    }

    fn print(&self, t: usize) {
        for variant in 0..self.kernels.len() {
            for y in 0..2 * self.time_limit + 1 {
                for x in 0..2 * self.time_limit + 1 {
                    print!("{} ", self.table[t][variant][x][y]);
                }

                println!();
            }
        }
    }
}
