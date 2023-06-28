pub mod biased_rw;
pub mod correlated_rw;
pub mod simple_rw;

use std::ops::Index;
use strum::EnumIter;
use crate::dp::DynamicProgram;

pub type Walk = Vec<(isize, isize)>;

pub trait WalkGenerator {
    fn generate_path(
        &self,
        dp: &DynamicProgram,
        to_x: isize,
        to_y: isize,
        time_steps: usize,
    ) -> Walk;

    fn name(&self, short: bool) -> String;
}

/// A direction for use in different random walk steppers.
#[derive(PartialEq, Copy, Clone, EnumIter)]
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
            Direction::North => &self[0],
            Direction::East => &self[1],
            Direction::South => &self[2],
            Direction::West => &self[3],
            Direction::Stay => &self[4],
        }
    }
}
