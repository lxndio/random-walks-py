use crate::steppers::ProbabilityStepper;
use num::{BigInt, BigRational, Zero};
use num::{BigUint, ToPrimitive};
use std::fmt::Debug;
use std::ops::Div;

pub struct ProbabilityDynamicProgram {
    table: Vec<Vec<Vec<BigUint>>>,
    probabilities: Vec<Vec<Vec<f64>>>,
    time_limit: usize,
    stepper: Box<dyn ProbabilityStepper>,
}

impl ProbabilityDynamicProgram {
    pub fn new(time_limit: usize, stepper: impl ProbabilityStepper + 'static) -> Self {
        Self {
            table: vec![vec![vec![Zero::zero(); 2 * time_limit + 2]; 2 * time_limit + 2]; 2],
            probabilities: vec![
                vec![vec![0.0; 2 * time_limit + 2]; 2 * time_limit + 2];
                time_limit + 1
            ],
            time_limit,
            stepper: Box::new(stepper),
        }
    }

    pub fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    pub fn at(&self, x: isize, y: isize, current: bool) -> BigUint {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        if current {
            self.table[1][x][y].clone()
        } else {
            self.table[0][x][y].clone()
        }
    }

    pub fn at_probability(&self, x: isize, y: isize, t: usize) -> f64 {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.probabilities[t][x][y].clone()
    }

    pub fn set(&mut self, x: isize, y: isize, current: bool, val: BigUint) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        if current {
            self.table[1][x][y] = val;
        } else {
            self.table[0][x][y] = val;
        }
    }

    pub fn set_probability(&mut self, x: isize, y: isize, t: usize, val: f64) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.probabilities[t][x][y] = val;
    }

    pub fn update(&mut self, x: isize, y: isize) {
        self.set(x, y, true, self.stepper.step(self, x, y));
    }

    pub fn compute_probabilities(&mut self, t: usize) {
        let sum: BigInt = self.table[1].iter().flatten().sum::<BigUint>().into();
        let (limit_neg, limit_pos) = self.limits();

        for x in limit_neg..=limit_pos {
            for y in limit_neg..=limit_pos {
                let val = BigRational::new(self.at(x, y, true).into(), sum.clone());

                self.set_probability(x, y, t, val.to_f64().expect("Overflow"));
            }
        }

        self.table[0] = self.table[1].clone();
    }

    pub fn print(&self, t: usize) {
        for y in 0..2 * self.time_limit + 2 {
            for x in 0..2 * self.time_limit + 2 {
                print!("{} ", self.probabilities[t][x][y]);
            }

            println!();
        }
    }
}

impl Debug for ProbabilityDynamicProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProbabilityDynamicProgram")
            .field("time_limit", &self.time_limit)
            .finish()
    }
}

impl PartialEq for ProbabilityDynamicProgram {
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit && self.table == other.table
    }
}

impl Eq for ProbabilityDynamicProgram {}
