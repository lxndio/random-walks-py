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

pub mod simple;
pub mod store;

use crate::kernel::Kernel;

pub trait DynamicProgram {
    /// Create a new dynamic program with a given `time_limit`
    /// using a given [`kernel`] for generation.
    fn new(time_limit: usize, kernel: Kernel) -> Self;

    fn limits(&self) -> (isize, isize);

    fn at(&self, x: isize, y: isize, t: usize) -> f64;

    fn set(&mut self, x: isize, y: isize, t: usize, val: f64);

    fn apply_kernel_at(&mut self, x: isize, y: isize, t: usize);

    fn compute(&mut self);

    fn print(&self, t: usize);
}
