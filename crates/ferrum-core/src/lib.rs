//! Shared device types and numerical errors.
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    Cpu,
    Cuda(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeError {
    LengthMismatch { left: usize, right: usize },
    EmptyInput,
    InvalidParameter(&'static str),
    UnsupportedDevice(String),
}

impl fmt::Display for ComputeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { left, right } => write!(f, "length mismatch: {left} vs {right}"),
            Self::EmptyInput => write!(f, "input must not be empty"),
            Self::InvalidParameter(name) => write!(f, "invalid parameter: {name}"),
            Self::UnsupportedDevice(name) => write!(f, "unsupported device: {name}"),
        }
    }
}
impl std::error::Error for ComputeError {}

impl FromStr for Device {
    type Err = ComputeError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.eq_ignore_ascii_case("cpu") {
            return Ok(Self::Cpu);
        }
        if let Some(index) = value.strip_prefix("cuda:") {
            if let Ok(index) = index.parse::<u32>() {
                return Ok(Self::Cuda(index));
            }
        }
        Err(ComputeError::UnsupportedDevice(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_devices() {
        assert_eq!("cpu".parse::<Device>().unwrap(), Device::Cpu);
        assert_eq!("cuda:0".parse::<Device>().unwrap(), Device::Cuda(0));
        assert!("cuda:x".parse::<Device>().is_err());
    }
}
