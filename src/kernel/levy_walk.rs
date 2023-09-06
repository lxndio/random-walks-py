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
            .initialize(21)
            .unwrap();

        Ok(())
    }

    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), String> {
        let kernel = kernels
            .get_mut(0)
            .ok_or::<String>("No kernel for generation.".into())?;

        // for i in 0..kernel.size() {
        //     kernel.set((i - kernel.size() / 2) as isize, 0, 0.1);
        //     kernel.set(0, (i - kernel.size() / 2) as isize, 0.1);
        // }

        kernel.set(-10, 0, 0.05);
        kernel.set(10, 0, 0.05);
        kernel.set(0, -10, 0.05);
        kernel.set(0, 10, 0.05);

        kernel.set(0, 0, 0.2);
        kernel.set(-1, 0, 0.2);
        kernel.set(1, 0, 0.2);
        kernel.set(0, -1, 0.2);
        kernel.set(0, 1, 0.2);
        // kernel.set(-1, -1, 0.2);
        // kernel.set(1, 1, 0.2);
        // kernel.set(-1, 1, 0.2);
        // kernel.set(1, -1, 0.2);

        Ok(())
    }

    fn generates_qty(&self) -> usize {
        1
    }

    fn name(&self) -> (String, String) {
        ("lw".into(), "Lévy Walk".into())
    }
}
