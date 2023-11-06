use crate::kernel::generator::{KernelGenerator, KernelGeneratorError};
use crate::kernel::Kernel;

pub struct SimpleRwGenerator;

impl KernelGenerator for SimpleRwGenerator {
    fn prepare(&self, kernels: &mut Vec<Kernel>) -> Result<(), KernelGeneratorError> {
        kernels
            .get_mut(0)
            .ok_or(KernelGeneratorError::OneKernelRequired)?
            .initialize(3)
            .unwrap();

        Ok(())
    }

    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), KernelGeneratorError> {
        let kernel = kernels
            .get_mut(0)
            .ok_or(KernelGeneratorError::OneKernelRequired)?;

        kernel.set(0, 0, 0.2);
        kernel.set(0, -1, 0.2);
        kernel.set(1, 0, 0.2);
        kernel.set(0, 1, 0.2);
        kernel.set(-1, 0, 0.2);

        Ok(())
    }

    fn generates_qty(&self) -> usize {
        1
    }

    fn name(&self) -> (String, String) {
        ("srw".into(), "Simple RW".into())
    }
}
