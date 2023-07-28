use crate::dp::multi::MultiDynamicProgram;
use crate::dp::simple::SimpleDynamicProgram;
use crate::dp::{DynamicProgram, DynamicProgramType, DynamicPrograms};
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
    #[error("a single DP takes one kernel and not multiple ones")]
    MultipleKernelsForSingle,
    #[error("field probabilities must be of same size as DP table")]
    WrongSizeOfFieldProbabilities,
}

#[derive(Default)]
pub struct DynamicProgramBuilder {
    time_limit: Option<usize>,
    dp_type: Option<DynamicProgramType>,
    kernel: Option<Kernel>,
    kernels: Option<Vec<Kernel>>,
    field_probabilities: Option<Vec<Vec<f64>>>,
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

    pub fn build(self) -> Result<DynamicProgram, DynamicProgramBuilderError> {
        let Some(time_limit) = self.time_limit else {
            return Err(DynamicProgramBuilderError::NoTimeLimitSet);
        };
        let Some(dp_type) = self.dp_type else {
            return Err(DynamicProgramBuilderError::NoTypeSet);
        };

        match dp_type {
            DynamicProgramType::Simple => {
                if self.kernels.is_some() {
                    return Err(DynamicProgramBuilderError::MultipleKernelsForSingle);
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
                }))
            }
        }
    }
}
