use crate::kernel::generator::KernelGenerator;

pub mod generator;

pub struct Kernel {
    probabilities: Vec<Vec<f64>>,
    name: (String, String),
}

impl Kernel {
    pub fn from_generator(generator: impl KernelGenerator) -> Result<Kernel, String> {
        let mut kernel = Kernel {
            probabilities: Vec::new(),
            name: generator.name(),
        };

        generator.prepare(&mut kernel)?;
        generator.generate(&mut kernel)?;

        Ok(kernel)
    }

    pub fn initialize(&mut self, size: usize) -> Result<(), String> {
        if size % 2 == 1 {
            self.probabilities = vec![vec![0.0; size]; size];

            Ok(())
        } else {
            Err("Size must be odd.".into())
        }
    }

    pub fn size(&self) -> usize {
        self.probabilities.len()
    }

    pub fn set(&mut self, x: isize, y: isize, val: f64) {
        let x = ((self.probabilities.len() / 2) as isize + x) as usize;
        let y = ((self.probabilities.len() / 2) as isize + y) as usize;

        self.probabilities[x][y] = val;
    }

    pub fn at(&self, x: isize, y: isize) -> f64 {
        let x = ((self.probabilities.len() / 2) as isize + x) as usize;
        let y = ((self.probabilities.len() / 2) as isize + y) as usize;

        self.probabilities[x][y]
    }

    pub fn name(&self, short: bool) -> String {
        if short {
            self.name.0.clone()
        } else {
            self.name.1.clone()
        }
    }
}
