use crate::kernel;
use crate::kernel::generator::KernelGenerator;
use crate::kernel::Kernel;
use std::ops::DerefMut;

pub struct LevyWalkGenerator {}

impl KernelGenerator for LevyWalkGenerator {
    fn prepare(&self, kernels: &mut Vec<Kernel>) -> Result<(), String> {
        kernels
            .get_mut(0)
            .ok_or::<String>("No kernel to prepare.".into())?
            .initialize(9)
            .unwrap();

        Ok(())
    }

    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), String> {
        let kernel = kernels
            .get_mut(0)
            .ok_or::<String>("No kernel for generation.".into())?;

        *kernel = kernel![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.2, 0.2, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ];

        Ok(())
    }

    fn generates_qty(&self) -> usize {
        1
    }

    fn name(&self) -> (String, String) {
        ("lw".into(), "Lévy Walk".into())
    }
}
