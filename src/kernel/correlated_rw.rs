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
    use crate::kernel::biased_rw::BiasedRwGenerator;
    use crate::kernel::{Direction, Kernel};

    #[test]
    fn test_correlated_rw() {
        let kernel = Kernel::from_generator(BiasedRwGenerator {
            probability: 0.5,
            direction: Direction::North,
        });

        let kernel_correct = Kernel {
            probabilities: vec![
                vec![0.0, 0.125, 0.0],
                vec![0.5, 0.125, 0.125],
                vec![0.0, 0.125, 0.0],
            ],
            name: ("".into(), "".into()),
        };

        assert!(kernel.is_ok());
        assert_eq!(kernel.unwrap(), kernel_correct);
    }
}
