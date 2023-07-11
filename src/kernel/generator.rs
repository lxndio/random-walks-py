use crate::kernel::Kernel;

pub trait KernelGenerator {
    fn prepare(&self, kernel: &mut Kernel) -> Result<(), String>;
    fn generate(&self, kernel: &mut Kernel) -> Result<(), String>;
    fn name(&self) -> (String, String);
}
