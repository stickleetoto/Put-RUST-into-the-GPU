//! CUDA environment probe; actual GPU kernels are not implemented yet.
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CudaEnvironment {
    pub nvcc_available: bool,
    pub nvidia_smi_available: bool,
}

fn command_works(program: &str, argument: &str) -> bool {
    Command::new(program)
        .arg(argument)
        .output()
        .map(|result| result.status.success())
        .unwrap_or(false)
}

pub fn probe_environment() -> CudaEnvironment {
    CudaEnvironment {
        nvcc_available: command_works("nvcc", "--version"),
        nvidia_smi_available: command_works("nvidia-smi", "--list-gpus"),
    }
}
