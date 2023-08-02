use crate::kernel::generator::KernelGenerator;
use crate::kernel::{Direction, Kernel};
use strum::IntoEnumIterator;

pub struct CorrelatedRwGenerator {
    pub persistence: f64,
}

impl KernelGenerator for CorrelatedRwGenerator {
    fn prepare(&self, kernels: &mut Vec<Kernel>) -> Result<(), String> {
        if kernels.len() != self.generates_qty() {
            Err("Not enough kernels to prepare.".into())
        } else {
            for kernel in kernels.iter_mut() {
                kernel.initialize(3).unwrap();
            }

            Ok(())
        }
    }

    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), String> {
        if kernels.len() != self.generates_qty() {
            Err("Not enough kernels for generation.".into())
        } else {
            // Generate biased kernel to north which can later be rotated to four directions
            let (direction_x, direction_y) = Direction::North.into();
            let other_prob = (1.0 - self.persistence) / 4.0;

            kernels[0].set(direction_x, direction_y, self.persistence);

            for direction in Direction::iter() {
                if direction != Direction::North {
                    let (direction_x, direction_y) = direction.into();

                    kernels[0].set(direction_x, direction_y, other_prob);
                }
            }

            kernels[1] = kernels[0].clone();
            kernels[2] = kernels[0].clone();
            kernels[3] = kernels[0].clone();

            // Unwraps are safe here because rotation values are correct
            kernels[1].rotate(90).unwrap();
            kernels[2].rotate(180).unwrap();
            kernels[3].rotate(270).unwrap();

            // Generate kernel for staying manually
            let (direction_x, direction_y) = Direction::Stay.into();
            kernels[4].set(direction_x, direction_y, self.persistence);

            for direction in Direction::iter() {
                if direction != Direction::Stay {
                    let (direction_x, direction_y) = direction.into();

                    kernels[4].set(direction_x, direction_y, other_prob);
                }
            }

            Ok(())
        }
    }

    fn generates_qty(&self) -> usize {
        5
    }

    fn name(&self) -> (String, String) {
        ("brw".into(), "Biased RW".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel;
    use crate::kernel::correlated_rw::CorrelatedRwGenerator;
    use crate::kernel::{Direction, Kernel};

    #[test]
    #[rustfmt::skip]
    fn test_correlated_rw() {
        let kernels = Kernel::multiple_from_generator(CorrelatedRwGenerator { persistence: 0.5 });

        let kernel_correct_0 = kernel![
            0.0,   0.5,   0.0,
            0.125, 0.125, 0.125,
            0.0,   0.125, 0.0
        ];
        let kernel_correct_1 = kernel![
            0.0,   0.125, 0.0,
            0.125, 0.125, 0.5,
            0.0,   0.125, 0.0
        ];
        let kernel_correct_2 = kernel![
            0.0,   0.125, 0.0,
            0.125, 0.125, 0.125,
            0.0,   0.5,   0.0
        ];
        let kernel_correct_3 = kernel![
            0.0, 0.125, 0.0,
            0.5, 0.125, 0.125,
            0.0, 0.125, 0.0
        ];
        let kernel_correct_4 = kernel![
            0.0,   0.125, 0.0,
            0.125, 0.5,   0.125,
            0.0,   0.125, 0.0
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
