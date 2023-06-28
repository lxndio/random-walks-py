use num::{BigUint, One};
use strum::IntoEnumIterator;
use crate::walk_generators::{Direction, Directions, Walk};

pub struct WalkAnalyzer {
    walk: Walk,
    directional_biases: Directions<f32>,
    persistence: f32,
}

pub enum AnalysisResult {
    SimpleRw,
    BiasedRw(Direction, f32),
    CorrelatedRw(f32),
}

pub enum AnalysisError {
    WalkTooShort,
    InvalidWalk,
}

impl WalkAnalyzer {
    pub fn new(walk: Walk) -> Self {
        Self {
            walk,
            directional_biases: Directions::new(),
            persistence: 0.0,
        }
    }

    pub fn analyze(&mut self) -> Result<AnalysisResult, AnalysisError> {
        if self.walk.len() < 2 {
            return Err(AnalysisError::WalkTooShort);
        }

        let mut walk = self.walk.clone();
        let (mut last_x, mut last_y) = walk[0];
        let mut last_direction = Direction::Stay;
        walk.remove(0);

        let mut direction_counts = Directions::<BigUint>::new();
        let mut persistence_counts = Directions::<BigUint>::new();

        for (t, (x, y)) in walk.iter().enumerate() {
            let dx = x - last_x;
            let dy = y - last_y;

            let direction = Direction::try_from((dx, dy)).or(Err(AnalysisError::InvalidWalk))?;
            direction_counts[direction] += BigUint::one();

            if *x >= 2 {
                if last_direction == direction {
                    persistence_counts[direction] += BigUint::one();
                }
            }

            last_x = *x;
            last_y = *y;
        }

        for direction in Direction::iter() {
            self.directional_biases[direction] = (direction_counts[direction] / BigUint::from(walk.len())).to_f32().unwrap();
        }

        todo!()
    }
}
