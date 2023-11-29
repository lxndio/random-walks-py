use crate::dp::builder::DynamicProgramBuilder;
use crate::dp::{DynamicProgramPool, DynamicPrograms};
use crate::kernel;
use crate::kernel::Kernel;
use anyhow::{bail, Context};
use num::Zero;
#[cfg(feature = "plotting")]
use plotters::prelude::*;
use pyo3::{pyclass, pymethods, PyCell, PyResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use workerpool::thunk::{Thunk, ThunkWorker};
use workerpool::Pool;
#[cfg(feature = "saving")]
use {
    std::fs::File,
    std::io::{BufReader, Read},
    std::io::{BufWriter, Write},
    zstd::{Decoder, Encoder},
};

#[pyclass]
#[derive(Clone)]
pub struct DynamicProgram {
    pub(crate) table: Vec<Vec<Vec<f64>>>,
    pub(crate) time_limit: usize,
    pub(crate) kernels: Vec<Kernel>,
    pub(crate) field_types: Vec<Vec<usize>>,
}

#[pymethods]
impl DynamicProgram {
    #[new]
    #[pyo3(signature = (time_limit, kernel=None, kernels=Vec::new(), field_types=Vec::new()))]
    pub fn new(
        time_limit: usize,
        kernel: Option<Kernel>,
        kernels: Vec<(usize, Kernel)>,
        mut field_types: Vec<Vec<usize>>,
    ) -> Self {
        if field_types.is_empty() {
            field_types = vec![vec![0; 2 * time_limit + 1]; 2 * time_limit + 1];
        }

        let kernels = if let Some(kernel) = kernel {
            vec![(0, kernel)]
        } else {
            kernels
        };

        // Map field types to contiguous value range

        let mut kernels_mapped = Vec::new();
        let mut field_type_map = HashMap::new();
        let mut i = 0usize;

        for (field_type, kernel) in kernels.iter() {
            kernels_mapped.push(kernel.clone());
            field_type_map.insert(field_type, i);
            i += 1;
        }

        for x in 0..2 * time_limit + 1 {
            for y in 0..2 * time_limit + 1 {
                field_types[x][y] = field_type_map[&field_types[x][y]];
            }
        }

        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 1]; 2 * time_limit + 1];
                time_limit + 1
            ],
            time_limit,
            kernels: kernels_mapped,
            field_types,
        }
    }

    pub fn at(&self, x: isize, y: isize, t: usize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y]
    }

    pub fn at_or(&self, x: isize, y: isize, t: usize, default: f64) -> f64 {
        let (limit_neg, limit_pos) = self.limits();

        if x >= limit_neg && x <= limit_pos && y >= limit_neg && y <= limit_pos {
            let x = (self.time_limit as isize + x) as usize;
            let y = (self.time_limit as isize + y) as usize;

            self.table[t][x][y]
        } else {
            default
        }
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    fn apply_kernel_at(&mut self, x: isize, y: isize, t: usize) {
        let field_type = self.field_type_at(x, y);
        let kernel = self.kernels[field_type].clone();

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

                sum += self.at(i, j, t - 1) * kernel.at(kernel_x, kernel_y);
            }
        }

        self.set(x, y, t, sum);
    }

    fn field_type_at(&self, x: isize, y: isize) -> usize {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.field_types[x][y]
    }

    fn field_type_set(&mut self, x: isize, y: isize, val: usize) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.field_types[x][y] = val;
    }

    #[staticmethod]
    #[pyo3(name = "load")]
    pub fn py_load(filename: String) -> anyhow::Result<DynamicProgram> {
        match DynamicProgram::load(filename) {
            Ok(DynamicProgramPool::Single(dp)) => Ok(dp),
            Err(e) => Err(e),
            _ => unreachable!(),
        }
    }

    // Trait function wrappers for Python

    pub fn limits(&self) -> (isize, isize) {
        DynamicPrograms::limits(self)
    }

    pub fn compute(&mut self) {
        DynamicPrograms::compute(self)
    }

    pub fn field_types(&self) -> Vec<Vec<usize>> {
        DynamicPrograms::field_types(self)
    }

    pub fn heatmap(&self, path: String, t: usize) -> anyhow::Result<()> {
        DynamicPrograms::heatmap(self, path, t)
    }

    pub fn print(&self, t: usize) {
        DynamicPrograms::print(self, t)
    }

    #[cfg(feature = "saving")]
    pub fn save(&self, filename: String) -> anyhow::Result<()> {
        DynamicPrograms::save(self, filename)
    }

    // Python magic methods

    pub fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;

        Ok(format!("{}({})", class_name, slf.borrow().time_limit))
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl DynamicProgram {
    #[cfg(feature = "saving")]
    pub fn load(filename: String) -> anyhow::Result<DynamicProgramPool> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut decoder = Decoder::new(reader).context("could not create decoder")?;

        let mut time_limit = [0u8; 8];
        let time_limit = match decoder.read_exact(&mut time_limit) {
            Ok(()) => u64::from_le_bytes(time_limit),
            Err(_) => bail!("could not read time limit from file"),
        };

        let DynamicProgramPool::Single(mut dp) = DynamicProgramBuilder::new()
            .simple()
            .time_limit(time_limit as usize)
            .kernel(kernel!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
            .build()?
        else {
            unreachable!();
        };

        let (limit_neg, limit_pos) = dp.limits();
        let mut buf = [0u8; 8];

        for t in 0..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    decoder.read_exact(&mut buf)?;
                    dp.set(x, y, t, f64::from_le_bytes(buf));
                }
            }
        }

        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                decoder.read_exact(&mut buf)?;
                dp.field_type_set(x, y, u64::from_le_bytes(buf) as usize);
            }
        }

        Ok(DynamicProgramPool::Single(dp))
    }
}

impl DynamicPrograms for DynamicProgram {
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

    fn compute_parallel(&mut self) {
        let (limit_neg, limit_pos) = self.limits();
        let kernels = Arc::new(RwLock::new(self.kernels.clone()));
        let field_types = Arc::new(RwLock::new(self.field_types.clone()));
        let pool = Pool::<ThunkWorker<(Range<isize>, Range<isize>, Vec<Vec<f64>>)>>::new(10);
        let (tx, rx) = channel();

        // Define chunks

        let chunk_size = ((self.time_limit + 1) / 3) as isize;
        let mut ranges = Vec::new();

        for i in 0..3 - 1 {
            ranges.push((limit_neg + i * chunk_size..limit_neg + (i + 1) * chunk_size));
        }

        ranges.push(limit_neg + 2 * chunk_size..limit_pos + 1);
        let mut chunks = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                chunks.push((ranges[x].clone(), ranges[y].clone()));
            }
        }

        self.set(0, 0, 0, 1.0);

        let start = Instant::now();

        for t in 1..=limit_pos as usize {
            let table_old = Arc::new(RwLock::new(self.table[t - 1].clone()));

            for (x_range, y_range) in chunks.clone() {
                let kernels = kernels.clone();
                let field_types = field_types.clone();
                let table_old = table_old.clone();

                pool.execute_to(
                    tx.clone(),
                    Thunk::of(move || {
                        let mut probs = vec![vec![0.0; y_range.len()]; x_range.len()];
                        let (mut i, mut j) = (0, 0);

                        for x in x_range.clone() {
                            for y in y_range.clone() {
                                probs[i][j] = apply_kernel(
                                    &table_old.read().unwrap(),
                                    &kernels.read().unwrap(),
                                    &field_types.read().unwrap(),
                                    (limit_neg, limit_pos),
                                    x,
                                    y,
                                );

                                j += 1;
                            }

                            i += 1;
                            j = 0;
                        }

                        (x_range.clone(), y_range.clone(), probs)
                    }),
                );
            }

            for (x_range, y_range, probs) in rx.iter().take(9) {
                let (mut i, mut j) = (0, 0);

                for x in x_range.clone() {
                    for y in y_range.clone() {
                        self.table[t][(self.time_limit as isize + x) as usize]
                            [(self.time_limit as isize + y) as usize] = probs[i][j];

                        j += 1;
                    }

                    i += 1;
                    j = 0;
                }
            }
        }

        let duration = start.elapsed();

        println!("Computation took {:?}", duration);
    }

    #[cfg(not(tarpaulin_include))]
    fn field_types(&self) -> Vec<Vec<usize>> {
        self.field_types.clone()
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

        let iter = self.table[t].iter().enumerate().flat_map(|(x, l)| {
            l.iter()
                .enumerate()
                .map(move |(y, v)| (x as i32 - limit_pos as i32, y as i32 - limit_pos as i32, v))
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

        for t in 0..=limit_pos as usize {
            for x in limit_neg..=limit_pos {
                for y in limit_neg..=limit_pos {
                    encoder.write(&self.at(x, y, t).to_le_bytes())?;
                }
            }
        }

        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                encoder.write(&(self.field_type_at(x, y) as u64).to_le_bytes())?;
            }
        }

        Ok(())
    }
}

fn apply_kernel(
    table_old: &Vec<Vec<f64>>,
    kernels: &Vec<Kernel>,
    field_types: &Vec<Vec<usize>>,
    (limit_neg, limit_pos): (isize, isize),
    x: isize,
    y: isize,
) -> f64 {
    let field_type = field_types[(limit_pos + x) as usize][(limit_pos + y) as usize];
    let kernel = kernels[field_type].clone();

    let ks = (kernel.size() / 2) as isize;
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

            sum += table_old[(limit_pos + i) as usize][(limit_pos + j) as usize]
                * kernel.at(kernel_x, kernel_y);
        }
    }

    sum
}

#[cfg(not(tarpaulin_include))]
impl Debug for DynamicProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicProgram")
            .field("time_limit", &self.time_limit)
            .finish()
    }
}

impl PartialEq for DynamicProgram {
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit
            && self.table == other.table
            && self.field_types == other.field_types
    }
}

impl Eq for DynamicProgram {}

#[cfg(test)]
mod tests {
    use crate::dp::builder::DynamicProgramBuilder;
    use crate::dp::{DynamicProgram, DynamicProgramPool, DynamicPrograms};
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

        let DynamicProgramPool::Single(dp) = dp else {
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

        let DynamicProgramPool::Single(mut dp) = dp else {
            unreachable!();
        };

        dp.set(0, 0, 0, 10.0);

        assert_eq!(dp.at(0, 0, 0,), 10.0);
    }

    // #[test]
    // #[rustfmt::skip]
    // fn test_simple_dp_apply_kernel_at() {
    //     let mut fps = vec![vec![1.0; 21]; 21];
    //
    //     fps[10][10] = 0.75;
    //
    //     let dp = DynamicProgramBuilder::new()
    //         .simple()
    //         .time_limit(10)
    //         .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
    //         .field_probabilities(fps)
    //         .build()
    //         .unwrap();
    //
    //     let DynamicProgram::Simple(mut dp) = dp else {
    //         unreachable!();
    //     };
    //
    //     dp.set(0, 0, 0, 0.5);
    //     dp.set(-1, 0, 0, 0.5);
    //     dp.apply_kernel_at(0, 0, 1);
    //
    //     let rounded_res = format!("{:.2}", dp.at(0, 0, 1)).parse::<f64>().unwrap();
    //
    //     assert_eq!(rounded_res, 0.15);
    // }

    #[test]
    fn test_compute() {
        let mut dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(1)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build()
            .unwrap();

        dp.compute();

        let DynamicProgramPool::Single(mut dp) = dp else {
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

        let DynamicProgramPool::Single(mut dp1) = dp1 else {
            unreachable!();
        };
        let DynamicProgramPool::Single(mut dp2) = dp2 else {
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

        let DynamicProgramPool::Single(mut dp1) = dp1 else {
            unreachable!();
        };
        let DynamicProgramPool::Single(mut dp2) = dp2 else {
            unreachable!();
        };

        assert_ne!(dp1, dp2);
    }
}
