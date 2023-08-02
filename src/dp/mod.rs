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

use crate::dp::multi::MultiDynamicProgram;
use crate::dp::simple::SimpleDynamicProgram;
use serde::{Deserialize, Serialize};

pub mod builder;
pub mod multi;
pub mod simple;
pub mod store;

pub trait DynamicPrograms {
    fn limits(&self) -> (isize, isize);

    fn compute(&mut self);

    fn field_probabilities(&self) -> Vec<Vec<f64>>;

    fn heatmap(&self, path: String, t: usize) -> anyhow::Result<()>;

    fn print(&self, t: usize);
}

pub enum DynamicProgram {
    Simple(SimpleDynamicProgram),
    Multi(MultiDynamicProgram),
}

impl DynamicProgram {
    fn unwrap(&self) -> &dyn DynamicPrograms {
        match self {
            DynamicProgram::Simple(simple) => simple,
            DynamicProgram::Multi(multi) => multi,
        }
    }

    fn unwrap_mut(&mut self) -> &mut dyn DynamicPrograms {
        match self {
            DynamicProgram::Simple(simple) => simple,
            DynamicProgram::Multi(multi) => multi,
        }
    }
}

impl DynamicPrograms for DynamicProgram {
    fn limits(&self) -> (isize, isize) {
        self.unwrap().limits()
    }

    fn compute(&mut self) {
        self.unwrap_mut().compute()
    }

    fn field_probabilities(&self) -> Vec<Vec<f64>> {
        self.unwrap().field_probabilities()
    }

    fn heatmap(&self, path: String, t: usize) -> anyhow::Result<()> {
        self.unwrap().heatmap(path, t)
    }

    fn print(&self, t: usize) {
        self.unwrap().print(t)
    }
}

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub enum DynamicProgramType {
    #[default]
    Simple,
    Multi,
}
