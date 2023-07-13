use crate::kernel::generator::KernelGenerator;
use std::ops::{Index, IndexMut};
use strum::EnumIter;

pub mod biased_rw;
pub mod generator;
pub mod simple_rw;

#[derive(Clone, Debug)]
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

    /// Rotate kernel matrix counterclockwise by `degrees`. Only multiples of 90° are supported.
    pub fn rotate(&mut self, degrees: usize) -> Result<(), String> {
        if degrees % 90 != 0 {
            Err("degrees must be a multiple of 90.".into())
        } else {
            let n = self.probabilities.len();

            for _ in 0..degrees / 90 {
                // Source: https://www.enjoyalgorithms.com/blog/rotate-a-matrix-by-90-degrees-in-an-anticlockwise-direction
                for i in 0..n / 2 {
                    for j in i..n - i - 1 {
                        let temp = self.probabilities[i][j];

                        self.probabilities[i][j] = self.probabilities[j][n - 1 - i];
                        self.probabilities[j][n - 1 - i] = self.probabilities[n - 1 - i][n - 1 - j];
                        self.probabilities[n - 1 - i][n - 1 - j] = self.probabilities[n - 1 - j][i];
                        self.probabilities[n - 1 - j][i] = temp;
                    }
                }
            }

            Ok(())
        }
    }

    pub fn name(&self, short: bool) -> String {
        if short {
            self.name.0.clone()
        } else {
            self.name.1.clone()
        }
    }
}

impl PartialEq for Kernel {
    fn eq(&self, other: &Self) -> bool {
        self.probabilities == other.probabilities
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Stay,
}

pub struct Directions<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
    pub stay: T,
}

impl TryFrom<(isize, isize)> for Direction {
    type Error = &'static str;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        match value {
            (0, -1) => Ok(Self::North),
            (1, 0) => Ok(Self::East),
            (0, 1) => Ok(Self::South),
            (-1, 0) => Ok(Self::West),
            (0, 0) => Ok(Self::Stay),
            _ => Err("Invalid direction"),
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::Stay => (0, 0),
        }
    }
}

impl<T: Default> Directions<T> {
    pub fn new() -> Self {
        Self {
            north: Default::default(),
            east: Default::default(),
            south: Default::default(),
            west: Default::default(),
            stay: Default::default(),
        }
    }
}

impl<T> Index<Direction> for Directions<T> {
    type Output = T;

    fn index(&self, direction: Direction) -> &Self::Output {
        match direction {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west,
            Direction::Stay => &self.stay,
        }
    }
}

impl<T> IndexMut<Direction> for Directions<T> {
    fn index_mut(&mut self, direction: Direction) -> &mut Self::Output {
        match direction {
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West => &mut self.west,
            Direction::Stay => &mut self.stay,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::Kernel;

    #[test]
    fn test_rotate() {
        let mut kernel = Kernel {
            probabilities: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
            ],
            name: ("".into(), "".into()),
        };

        let correct_rotation = vec![
            vec![3.0, 6.0, 9.0],
            vec![2.0, 5.0, 8.0],
            vec![1.0, 4.0, 7.0],
        ];

        assert!(kernel.rotate(87).is_err());
        assert!(kernel.rotate(90).is_ok());
        assert_eq!(kernel.probabilities, correct_rotation);
    }
}
