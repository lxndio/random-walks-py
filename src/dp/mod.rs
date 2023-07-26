//! The dynamic program used to compute everything.
//!
//! # Examples
//!
//! Create a dynamic program with a `time_limit` of 10 using the [`SimpleStepper`].
//! Then use it to count the number of paths leading to each cell.
//!
//! ```
//! let mut dp = DynamicProgram::new(10, SimpleGenerator);
//! dp.count_paths();
//! ```

pub mod multi;
pub mod simple;
pub mod store;

use crate::dp::multi::MultiDynamicProgram;
use crate::dp::simple::SimpleDynamicProgram;
use crate::kernel::Kernel;

pub trait DynamicProgram {
    fn new(options: DynamicProgramOptions) -> Self;

    fn limits(&self) -> (isize, isize);

    fn compute(&mut self);

    fn heatmap(&self, path: String, t: usize) -> anyhow::Result<()>;

    fn print(&self, t: usize);
}

#[derive(Default, Clone)]
pub struct DynamicProgramOptions {
    pub time_limit: usize,
    pub kernel: Option<Kernel>,
    pub kernels: Option<Vec<Kernel>>,
}

pub enum DynamicProgramType {
    Simple(SimpleDynamicProgram),
    Multi(MultiDynamicProgram),
}
