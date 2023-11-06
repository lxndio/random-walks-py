use crate::kernel::biased_rw::BiasedRwGenerator;
use crate::kernel::correlated_rw::CorrelatedRwGenerator;
use crate::kernel::generator::{KernelGenerator, KernelGeneratorError};
use crate::kernel::{Direction, Kernel};

pub struct BiasedCorrelatedRwGenerator {
    pub probability: f64,
    pub direction: Direction,
    pub persistence: f64,
}

impl KernelGenerator for BiasedCorrelatedRwGenerator {
    fn prepare(&self, kernels: &mut Vec<Kernel>) -> Result<(), KernelGeneratorError> {
        if kernels.len() != self.generates_qty() {
            Err(KernelGeneratorError::NotEnoughKernels)
        } else {
            for kernel in kernels.iter_mut() {
                kernel.initialize(3).unwrap();
            }

            Ok(())
        }
    }

    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), KernelGeneratorError> {
        if kernels.len() != self.generates_qty() {
            Err(KernelGeneratorError::NotEnoughKernels)
        } else {
            let mut correlated = Kernel::multiple_from_generator(CorrelatedRwGenerator {
                persistence: self.persistence,
            })
            .unwrap();

            let biased = Kernel::from_generator(BiasedRwGenerator {
                probability: self.probability,
                direction: self.direction,
            })
            .unwrap();

            for kernel in correlated.iter_mut() {
                *kernel *= biased.clone();
                // Normalize such that all probabilities still sum to 1
                *kernel /= Kernel::try_from_value(kernel.size(), kernel.sum()).unwrap();
            }

            *kernels = correlated;

            Ok(())
        }
    }

    fn generates_qty(&self) -> usize {
        5
    }

    fn name(&self) -> (String, String) {
        ("bcrw".into(), "Biased and correlated RW".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel;
    use crate::kernel::biased_correlated_rw::BiasedCorrelatedRwGenerator;
    use crate::kernel::{Direction, Kernel};

    #[test]
    #[rustfmt::skip]
    fn test_biased_correlated_rw() {
        let kernels = Kernel::multiple_from_generator(BiasedCorrelatedRwGenerator {
            probability: 0.5,
            direction: Direction::North,
            persistence: 0.5,
        });

        let kernel_correct_0 = kernel![
            0.0,      0.25,     0.0,
            0.015625, 0.015625, 0.015625,
            0.0,      0.015625, 0.0
        ];
        let kernel_correct_1 = kernel![
            0.0,      0.0625,   0.0,
            0.015625, 0.015625, 0.0625,
            0.0,      0.015625, 0.0
        ];
        let kernel_correct_2 = kernel![
            0.0,      0.0625,   0.0,
            0.015625, 0.015625, 0.015625,
            0.0,      0.0625,   0.0
        ];
        let kernel_correct_3 = kernel![
            0.0,    0.0625,   0.0,
            0.0625, 0.015625, 0.015625,
            0.0,    0.015625, 0.0
        ];
        let kernel_correct_4 = kernel![
            0.0,      0.0625,   0.0,
            0.015625, 0.0625,   0.015625,
            0.0,      0.015625, 0.0
        ];

        assert!(kernels.is_ok());

        let kernels = kernels.unwrap();

        assert_eq!(kernels[0], kernel_correct_0);
        assert_eq!(kernels[1], kernel_correct_1);
        assert_eq!(kernels[2], kernel_correct_2);
        assert_eq!(kernels[3], kernel_correct_3);
        assert_eq!(kernels[4], kernel_correct_4);
    }
}
