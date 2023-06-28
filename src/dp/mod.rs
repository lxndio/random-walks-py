//! The dynamic program used to compute everything.
//!
//! # Examples
//!
//! Create a dynamic program with a `time_limit` of 10 using the [`SimpleStepper`] generator.
//! Then use it to count the number of paths leading to each cell.
//!
//! ```
//! let mut dp = DynamicProgram::new(10, SimpleGenerator);
//! dp.count_paths();
//! ```

pub mod problems;
pub mod store;

use num::BigUint;
use num::{One, Zero};
use std::fmt::Debug;

use crate::steppers::simple::SimpleStepper;
use crate::steppers::Stepper;

pub struct DynamicProgram {
    table: Vec<Vec<Vec<BigUint>>>,
    time_limit: usize,
    stepper: Box<dyn Stepper>,
}

impl DynamicProgram {
    pub fn new(time_limit: usize, stepper: impl Stepper + 'static) -> Self {
        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 2]; 2 * time_limit + 2];
                time_limit + 1
            ],
            time_limit,
            stepper: Box::new(stepper),
        }
    }

    pub fn with_boxed(time_limit: usize, stepper: Box<dyn Stepper>) -> Self {
        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 2]; 2 * time_limit + 2];
                time_limit + 1
            ],
            time_limit,
            stepper,
        }
    }

    pub fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    pub fn at(&self, x: isize, y: isize, t: usize) -> BigUint {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y].clone()
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, val: BigUint) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    pub fn update(&mut self, x: isize, y: isize, t: usize) {
        self.set(x, y, t, self.stepper.step(self, x, y, t - 1));
    }

    pub fn print(&self, t: usize) {
        // Get number of digits of largest number
        let max = self.table[t].iter().flatten().max().unwrap();
        let max_digits = max.to_string().len();

        for y in 0..2 * self.time_limit + 2 {
            for x in 0..2 * self.time_limit + 2 {
                let val = &self.table[t][x][y];
                let digits = val.to_string().len();
                let spaces = " ".repeat(max_digits - digits + 2);

                print!("{}{}", val, spaces);
            }

            println!();
        }
    }
}

impl Debug for DynamicProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicProgram")
            .field("time_limit", &self.time_limit)
            .finish()
    }
}

impl PartialEq for DynamicProgram {
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit && self.table == other.table
    }
}

impl Eq for DynamicProgram {}
