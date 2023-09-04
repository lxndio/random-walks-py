use crate::dataset::point::XYPoint;
use crate::dp::multi::MultiDynamicProgram;
use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::{DynamicProgram, DynamicProgramType};
use crate::kernel::Kernel;
use num::Zero;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DynamicProgramBuilderError {
    #[error("a type of dynamic program must be chosen")]
    NoTypeSet,
    #[error("a time limit must be set")]
    NoTimeLimitSet,
    #[error("a kernel must be set")]
    NoKernelSet,
    #[error("kernels must be set")]
    NoKernelsSet,
    #[error("a multi DP takes multiple kernels and not a single one")]
    SingleKernelForMulti,
    #[error("a simple DP takes one kernel and not multiple ones")]
    MultipleKernelsForSimple,
    #[error("field probabilities must be of same size as DP table")]
    WrongSizeOfFieldProbabilities,
    #[error("barriers must be inside the time limit range")]
    BarrierOutOfRange,
}

#[derive(Default)]
pub struct DynamicProgramBuilder {
    time_limit: Option<usize>,
    dp_type: Option<DynamicProgramType>,
    kernel: Option<Kernel>,
    kernels: Option<Vec<Kernel>>,
    field_probabilities: Option<Vec<Vec<f64>>>,
    barriers: Vec<XYPoint>,
}

impl DynamicProgramBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn simple(mut self) -> Self {
        self.dp_type = Some(DynamicProgramType::Simple);

        self
    }

    pub fn multi(mut self) -> Self {
        self.dp_type = Some(DynamicProgramType::Multi);

        self
    }

    pub fn with_type(mut self, dp_type: DynamicProgramType) -> Self {
        self.dp_type = Some(dp_type);

        self
    }

    pub fn time_limit(mut self, time_limit: usize) -> Self {
        self.time_limit = Some(time_limit);

        self
    }

    pub fn kernel(mut self, kernel: Kernel) -> Self {
        self.kernel = Some(kernel);

        self
    }

    pub fn kernels(mut self, kernels: Vec<Kernel>) -> Self {
        self.kernels = Some(kernels);

        self
    }

    pub fn field_probabilities(mut self, probabilities: Vec<Vec<f64>>) -> Self {
        self.field_probabilities = Some(probabilities);

        self
    }

    pub fn add_single_barrier(mut self, at: XYPoint) -> Self {
        self.barriers.push(at);

        self
    }

    pub fn add_rect_barrier(mut self, from: XYPoint, to: XYPoint) -> Self {
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.barriers.push(XYPoint { x, y })
            }
        }

        self
    }

    pub fn build(self) -> Result<DynamicProgram, DynamicProgramBuilderError> {
        let Some(time_limit) = self.time_limit else {
            return Err(DynamicProgramBuilderError::NoTimeLimitSet);
        };
        let Some(dp_type) = self.dp_type else {
            return Err(DynamicProgramBuilderError::NoTypeSet);
        };

        let mut field_probabilities = match self.field_probabilities {
            Some(fp) => {
                if fp.len() != 2 * time_limit + 1 {
                    return Err(DynamicProgramBuilderError::WrongSizeOfFieldProbabilities);
                }

                for fpp in fp.iter() {
                    if fpp.len() != 2 * time_limit + 1 {
                        return Err(DynamicProgramBuilderError::WrongSizeOfFieldProbabilities);
                    }
                }

                fp
            }
            None => vec![vec![1.0; 2 * time_limit + 1]; 2 * time_limit + 1],
        };

        for (x, y) in self.barriers.iter().map(|p| <(i64, i64)>::from(*p)) {
            if x < -(time_limit as i64)
                || x > time_limit as i64
                || y < -(time_limit as i64)
                || y > time_limit as i64
            {
                return Err(DynamicProgramBuilderError::BarrierOutOfRange);
            }

            let x = (time_limit as i64 + x) as usize;
            let y = (time_limit as i64 + y) as usize;

            field_probabilities[x][y] = 0.0;
        }

        match dp_type {
            DynamicProgramType::Simple => {
                if self.kernels.is_some() {
                    return Err(DynamicProgramBuilderError::MultipleKernelsForSimple);
                }

                let Some(kernel) = self.kernel else {
                    return Err(DynamicProgramBuilderError::NoKernelSet);
                };

                Ok(DynamicProgram::Simple(SimpleDynamicProgram {
                    table: vec![
                        vec![vec![Zero::zero(); 2 * time_limit + 1]; 2 * time_limit + 1];
                        time_limit + 1
                    ],
                    time_limit,
                    kernel,
                    field_probabilities,
                }))
            }
            DynamicProgramType::Multi => {
                if self.kernel.is_some() {
                    return Err(DynamicProgramBuilderError::SingleKernelForMulti);
                }

                let Some(kernels) = self.kernels else {
                    return Err(DynamicProgramBuilderError::NoKernelsSet);
                };

                Ok(DynamicProgram::Multi(MultiDynamicProgram {
                    table: vec![
                        vec![
                            vec![vec![Zero::zero(); 2 * time_limit + 1]; 2 * time_limit + 1];
                            kernels.len()
                        ];
                        time_limit + 1
                    ],
                    time_limit,
                    kernels,
                    field_probabilities,
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dataset::point::XYPoint;
    use crate::dp::builder::{DynamicProgramBuilder, DynamicProgramBuilderError};
    use crate::dp::DynamicProgramType;
    use crate::kernel::correlated_rw::CorrelatedRwGenerator;
    use crate::kernel::simple_rw::SimpleRwGenerator;
    use crate::kernel::Kernel;
    use crate::xy;

    #[test]
    fn test_builder_missing_time_limit() {
        let dp = DynamicProgramBuilder::new().simple().build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::NoTimeLimitSet)
        ));
    }

    #[test]
    fn test_builder_missing_type() {
        let dp = DynamicProgramBuilder::new().time_limit(10).build();

        assert!(matches!(dp, Err(DynamicProgramBuilderError::NoTypeSet)));
    }

    #[test]
    fn test_wrong_size_of_field_probabilities() {
        let fps = vec![vec![1.0; 21]; 12];

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .field_probabilities(fps)
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::WrongSizeOfFieldProbabilities)
        ));

        let fps = vec![vec![1.0; 8]; 21];

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .field_probabilities(fps)
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::WrongSizeOfFieldProbabilities)
        ));
    }

    #[test]
    fn test_barrier_out_of_range() {
        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .add_single_barrier(xy!(25, 5))
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::BarrierOutOfRange)
        ));

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .add_single_barrier(xy!(5, 25))
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::BarrierOutOfRange)
        ));

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .add_rect_barrier(xy!(15, 5), xy!(25, 5))
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::BarrierOutOfRange)
        ));

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .add_rect_barrier(xy!(5, 15), xy!(5, 25))
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::BarrierOutOfRange)
        ));
    }

    #[test]
    fn test_multiple_kernels_for_single() {
        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernels(vec![Kernel::from_generator(SimpleRwGenerator).unwrap(); 10])
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::MultipleKernelsForSimple)
        ));

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .kernels(vec![Kernel::from_generator(SimpleRwGenerator).unwrap(); 10])
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::MultipleKernelsForSimple)
        ));

        let dp = DynamicProgramBuilder::new()
            .simple()
            .time_limit(10)
            .kernels(vec![Kernel::from_generator(SimpleRwGenerator).unwrap(); 10])
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::MultipleKernelsForSimple)
        ));
    }

    #[test]
    fn test_single_kernel_for_multi() {
        let dp = DynamicProgramBuilder::new()
            .multi()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::SingleKernelForMulti)
        ));

        let dp = DynamicProgramBuilder::new()
            .multi()
            .time_limit(10)
            .kernels(vec![Kernel::from_generator(SimpleRwGenerator).unwrap(); 10])
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::SingleKernelForMulti)
        ));

        let dp = DynamicProgramBuilder::new()
            .multi()
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .kernels(vec![Kernel::from_generator(SimpleRwGenerator).unwrap(); 10])
            .build();

        assert!(matches!(
            dp,
            Err(DynamicProgramBuilderError::SingleKernelForMulti)
        ));
    }

    #[test]
    fn test_no_kernels_set() {
        let dp = DynamicProgramBuilder::new().simple().time_limit(10).build();

        assert!(matches!(dp, Err(DynamicProgramBuilderError::NoKernelSet)));

        let dp = DynamicProgramBuilder::new().multi().time_limit(10).build();

        assert!(matches!(dp, Err(DynamicProgramBuilderError::NoKernelsSet)));
    }

    #[test]
    fn test_correct() {
        let dp = DynamicProgramBuilder::new()
            .with_type(DynamicProgramType::Simple)
            .time_limit(10)
            .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
            .field_probabilities(vec![vec![1.0; 21]; 21])
            .add_rect_barrier(xy!(5, -5), xy!(5, 5))
            .build();

        assert!(matches!(dp, Ok(_)));

        let dp = DynamicProgramBuilder::new()
            .with_type(DynamicProgramType::Multi)
            .time_limit(10)
            .kernels(
                Kernel::multiple_from_generator(CorrelatedRwGenerator { persistence: 0.5 })
                    .unwrap(),
            )
            .field_probabilities(vec![vec![1.0; 21]; 21])
            .add_rect_barrier(xy!(5, -5), xy!(5, 5))
            .build();

        assert!(matches!(dp, Ok(_)));
    }
}
