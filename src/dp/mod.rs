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

use crate::kernel::Kernel;

pub trait DynamicProgram {
    fn new(options: DynamicProgramOptions) -> Self;

    fn limits(&self) -> (isize, isize);

    fn compute(&mut self);

    fn print(&self, t: usize);
}

#[derive(Default)]
pub struct DynamicProgramOptions {
    pub time_limit: usize,
    pub kernel: Option<Kernel>,
    pub kernels: Option<Vec<Kernel>>,
}
