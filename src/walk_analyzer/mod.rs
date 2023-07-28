use crate::kernel::{Direction, Directions};
use crate::walker::Walk;
use num::ToPrimitive;
use strum::IntoEnumIterator;

pub struct AnalysisThresholds {
    pub biased: f32,
    pub correlated: f32,
}

impl Default for AnalysisThresholds {
    fn default() -> Self {
        Self {
            biased: 0.25,
            correlated: 0.5,
        }
    }
}

pub struct WalkAnalyzer {
    walk: Walk,
    thresholds: AnalysisThresholds,

    directional_biases: Directions<f32>,
    persistence: f32,
}

#[derive(Debug)]
pub enum AnalysisResult {
    SimpleRw,
    BiasedRw(Direction, f32),
    CorrelatedRw(f32),
}

#[derive(Debug)]
pub enum AnalysisError {
    WalkTooShort,
    InvalidWalk,
    Overflow,
}

impl WalkAnalyzer {
    pub fn new(walk: Walk) -> Self {
        Self {
            walk,
            thresholds: Default::default(),

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
        let last_direction = Direction::Stay;
        walk.remove(0);

        let mut direction_counts = Directions::<usize>::new();
        let mut persistence_counts = Directions::<usize>::new();

        for (_t, (x, y)) in walk.iter().enumerate() {
            let dx = x - last_x;
            let dy = y - last_y;

            let direction = Direction::try_from((dx, dy)).or(Err(AnalysisError::InvalidWalk))?;
            direction_counts[direction] += 1;

            if *x >= 2 {
                if last_direction == direction {
                    persistence_counts[direction] += 1;
                }
            }

            last_x = *x;
            last_y = *y;
        }

        let walk_len = walk.len().to_f64().ok_or(AnalysisError::Overflow)?;

        for direction in Direction::iter() {
            let direction_count = direction_counts[direction]
                .to_f64()
                .ok_or(AnalysisError::Overflow)?;
            self.directional_biases[direction] = (direction_count / walk_len) as f32;

            let persistence_count = persistence_counts[direction]
                .to_f64()
                .ok_or(AnalysisError::Overflow)?;
            let p = (persistence_count / walk_len) as f32;
            self.persistence = self.persistence.max(p);
        }

        // Check if the analyzed walk is a biased random walk
        for direction in Direction::iter() {
            if self.directional_biases[direction] >= self.thresholds.biased {
                return Ok(AnalysisResult::BiasedRw(
                    direction,
                    self.directional_biases[direction],
                ));
            }
        }

        // Check if the analyzed walk is a correlated random walk
        if self.persistence >= self.thresholds.correlated {
            return Ok(AnalysisResult::CorrelatedRw(self.persistence));
        }

        // Otherwise the analyzed walk could be a simple random walk
        Ok(AnalysisResult::SimpleRw)
    }
}

pub struct WalkAnalyzerBuilder {
    walk: Walk,
    thresholds: AnalysisThresholds,
}

impl WalkAnalyzerBuilder {
    pub fn new(walk: Walk) -> Self {
        Self {
            walk,
            thresholds: Default::default(),
        }
    }

    pub fn with_thresholds(mut self, thresholds: AnalysisThresholds) -> Self {
        self.thresholds = thresholds;

        self
    }

    pub fn build(self) -> WalkAnalyzer {
        WalkAnalyzer {
            walk: self.walk,
            thresholds: self.thresholds,

            directional_biases: Directions::new(),
            persistence: 0.0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::dp::problems::Problems;
//     use crate::dp::DynamicProgram;
//     use crate::stepper::simple::SimpleStepper;
//     use crate::walk_analyzer::{AnalysisResult, WalkAnalyzer};
//     use crate::walker::biased_rw::BiasedRwGenerator;
//     use crate::walker::correlated_rw::CorrelatedRwGenerator;
//     use crate::walker::standard::SimpleRwGenerator;
//     use crate::walker::{Direction, WalkGenerator};
//
//     #[test]
//     fn test_analyze_simple_rw() {
//         let mut dp = DynamicProgram::new(100, SimpleStepper);
//         dp.count_paths();
//
//         let walk = SimpleRwGenerator.generate_path(&dp, 5, -30, 100);
//
//         let mut wa = WalkAnalyzer::new(walk);
//         let res = wa.analyze();
//
//         assert!(res.is_ok());
//         assert!(matches!(res.unwrap(), AnalysisResult::SimpleRw));
//     }
//
//     #[test]
//     fn test_analyze_biased_rw() {
//         let mut dp = DynamicProgram::new(100, SimpleStepper);
//         dp.count_paths();
//
//         let generator = BiasedRwGenerator {
//             direction: Direction::North,
//             probability: 0.5,
//         };
//         let walk = generator.generate_path(&dp, 5, -30, 100);
//
//         let mut wa = WalkAnalyzer::new(walk);
//         let res = wa.analyze();
//
//         println!("{:?}", res);
//
//         assert!(&res.is_ok());
//         let res = res.unwrap();
//
//         assert!(matches!(res, AnalysisResult::BiasedRw(_, _)));
//
//         let AnalysisResult::BiasedRw(direction, probability) = res else {
//             panic!("Expected BiasedRw, got {:?}", res);
//         };
//
//         assert_eq!(direction, Direction::North);
//         assert!((0.5 - 0.1..=0.5 + 0.1).contains(&probability));
//     }
//
//     #[test]
//     fn test_analyze_correlated_rw() {
//         let mut dp = DynamicProgram::new(100, SimpleStepper);
//         dp.count_paths();
//
//         let generator = CorrelatedRwGenerator { persistence: 0.5 };
//         let walk = generator.generate_path(&dp, 5, -30, 100);
//
//         let mut wa = WalkAnalyzer::new(walk);
//         let res = wa.analyze();
//
//         println!("{:?}", res);
//
//         assert!(&res.is_ok());
//         let res = res.unwrap();
//
//         assert!(matches!(res, AnalysisResult::CorrelatedRw(_)));
//
//         let AnalysisResult::CorrelatedRw(persistence) = res else {
//             panic!("Expected CorrelatedRw, got {:?}", res);
//         };
//
//         assert!((0.5 - 0.1..=0.5 + 0.1).contains(&persistence));
//     }
// }
