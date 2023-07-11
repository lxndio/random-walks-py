use crate::kernel::Kernel;

pub trait KernelGenerator {
    fn prepare(&self, kernel: &mut Kernel) -> Result<(), String>;
    fn generate(&self, kernel: &mut Kernel) -> Result<(), String>;
    fn name(&self) -> (String, String);
}

pub struct SimpleRwGenerator;

impl KernelGenerator for SimpleRwGenerator {
    fn prepare(&self, kernel: &mut Kernel) -> Result<(), String> {
        kernel.initialize(3).unwrap();

        Ok(())
    }

    fn generate(&self, kernel: &mut Kernel) -> Result<(), String> {
        kernel.set(0, 0, 0.2);
        kernel.set(0, -1, 0.2);
        kernel.set(1, 0, 0.2);
        kernel.set(0, 1, 0.2);
        kernel.set(-1, 0, 0.2);

        Ok(())
    }

    fn name(&self) -> (String, String) {
        ("srw".into(), "Simple RW".into())
    }
}
