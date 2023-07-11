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
