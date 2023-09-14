use crate::dp::builder::DynamicProgramBuilder;
use crate::dp::{DynamicProgram, DynamicPrograms};
use crate::kernel;
use crate::kernel::Kernel;
use anyhow::{bail, Context};
use pyo3::pyclass;
use std::time::Instant;
#[cfg(feature = "saving")]
use {
    std::fs::File,
    std::io::{BufReader, Read},
    std::io::{BufWriter, Write},
    zstd::{Decoder, Encoder},
};

#[pyclass]
#[derive(Clone)]
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

        self.table[t][variant][x][y]
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

    fn field_probability_set(&mut self, x: isize, y: isize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.field_probabilities[x][y] = val;
    }

    #[cfg(feature = "saving")]
    pub fn load(filename: String) -> anyhow::Result<DynamicProgram> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut decoder = Decoder::new(reader).context("could not create decoder")?;

        let mut time_limit = [0u8; 8];
        let time_limit = match decoder.read_exact(&mut time_limit) {
            Ok(()) => u64::from_le_bytes(time_limit),
            Err(_) => bail!("could not read time limit from file"),
        };

        let mut variants = [0u8; 8];
        let variants = match decoder.read_exact(&mut variants) {
            Ok(()) => u64::from_le_bytes(variants) as usize,
            Err(_) => bail!("could not read number of variants from file"),
        };

        let DynamicProgram::Multi(mut dp) = DynamicProgramBuilder::new()
            .multi()
            .time_limit(time_limit as usize)
            .kernels(vec![
                kernel!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
                variants
            ])
            .build()?
        else {
            unreachable!();
        };

        let (limit_neg, limit_pos) = dp.limits();
        let mut buf = [0u8; 8];

        for t in 0..=limit_pos as usize {
            for variant in 0..variants {
                for x in limit_neg..=limit_pos {
                    for y in limit_neg..=limit_pos {
                        decoder.read_exact(&mut buf)?;
                        dp.set(x, y, t, variant, f64::from_le_bytes(buf));
                    }
                }
            }
        }

        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                decoder.read_exact(&mut buf)?;
                dp.field_probability_set(x, y, f64::from_le_bytes(buf));
            }
        }

        Ok(DynamicProgram::Multi(dp))
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

    #[cfg(feature = "plotting")]
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

    #[cfg(feature = "saving")]
    fn save(&self, filename: String) -> anyhow::Result<()> {
        let (limit_neg, limit_pos) = self.limits();
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        let mut encoder = Encoder::new(writer, 9).context("could not create encoder")?;

        encoder
            .multithread(4)
            .context("could not enable multithreading")?;

        let mut encoder = encoder.auto_finish();

        encoder.write(&(self.time_limit as u64).to_le_bytes())?;
        encoder.write(&(self.kernels.len() as u64).to_le_bytes())?;

        for t in 0..=limit_pos as usize {
            for variant in 0..self.kernels.len() {
                for x in limit_neg..=limit_pos {
                    for y in limit_neg..=limit_pos {
                        encoder.write(&self.at(x, y, t, variant).to_le_bytes())?;
                    }
                }
            }
        }

        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                encoder.write(&self.field_probability_at(x, y).to_le_bytes())?;
            }
        }

        Ok(())
    }
}

impl PartialEq for MultiDynamicProgram {
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit
            && self.table == other.table
            && self.field_probabilities == other.field_probabilities
    }
}

impl Eq for MultiDynamicProgram {}
