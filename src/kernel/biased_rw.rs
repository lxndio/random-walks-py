use crate::kernel::generator::KernelGenerator;
use crate::kernel::{Direction, Kernel};
use strum::IntoEnumIterator;

pub struct BiasedRwGenerator {
    pub probability: f64,
    pub direction: Direction,
}

impl KernelGenerator for BiasedRwGenerator {
    fn prepare(&self, kernel: &mut Kernel) -> Result<(), String> {
        kernel.initialize(3).unwrap();

        Ok(())
    }

    fn generate(&self, kernel: &mut Kernel) -> Result<(), String> {
        let (direction_x, direction_y) = self.direction.into();
        let other_prob = (1.0 - self.probability) / 4.0;

        kernel.set(direction_x, direction_y, self.probability);

        for direction in Direction::iter() {
            if direction != self.direction {
                let (direction_x, direction_y) = direction.into();

                kernel.set(direction_x, direction_y, other_prob);
            }
        }

        Ok(())
    }

    fn name(&self) -> (String, String) {
        ("brw".into(), "Biased RW".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::DynamicProgram;
    use crate::kernel::biased_rw::BiasedRwGenerator;
    use crate::kernel::{Direction, Kernel};

    #[test]
    fn test_biased_rw() {
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
