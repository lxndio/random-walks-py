use crate::kernel::Kernel;

pub trait KernelGenerator {
    fn prepare(&self, kernels: &mut Vec<Kernel>) -> Result<(), String>;
    fn generate(&self, kernels: &mut Vec<Kernel>) -> Result<(), String>;
    fn generates_qty(&self) -> usize;
    fn name(&self) -> (String, String);
}
