use num::BigUint;
use num::Zero;
use crate::dp::pregenerated::PregeneratedSolution;
use crate::models::simple_rw::SimpleRw;

pub mod pregenerated;
pub mod problems;

pub struct DynamicProgram {
    table: Vec<Vec<Vec<BigUint>>>,
    time_limit: usize,
    walk_model: Box<dyn WalkModel>,
}

pub trait WalkModel {
    fn walk(&self, dp: &DynamicProgram, x: isize, y: isize, t: usize) -> BigUint;
}

impl DynamicProgram {
    pub fn new(time_limit: usize, walk_model: impl WalkModel + 'static) -> Self {
        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 2]; 2 * time_limit + 2];
                time_limit + 1
            ],
            time_limit,
            walk_model: Box::new(walk_model),
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
        self.set(x, y, t, self.walk_model.walk(self, x, y, t - 1));
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

impl From<PregeneratedSolution> for DynamicProgram {
    fn from(solution: PregeneratedSolution) -> Self {
        Self {
            table: solution.table(),
            time_limit: solution.time_limit(),
            // TODO Probably make walk_model optional in the future, for now use this
            walk_model: Box::new(SimpleRw),
        }
    }
}
