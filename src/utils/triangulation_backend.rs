use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseTriangulationBackendError {
    value: String,
}

impl fmt::Display for ParseTriangulationBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown triangulation backend: {}", self.value)
    }
}

impl std::error::Error for ParseTriangulationBackendError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TriangulationBackend {
    FastestAvailable,
    Bermuda,
    PartSegCore,
    Triangle,
    Numba,
    PurePython,
}

impl TriangulationBackend {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FastestAvailable => "Fastest available",
            Self::Bermuda => "bermuda",
            Self::PartSegCore => "PartSegCore",
            Self::Triangle => "triangle",
            Self::Numba => "Numba",
            Self::PurePython => "Pure python",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::FastestAvailable => "fastest_available",
            Self::Bermuda => "bermuda",
            Self::PartSegCore => "partsegcore",
            Self::Triangle => "triangle",
            Self::Numba => "numba",
            Self::PurePython => "pure_python",
        }
    }
}

impl fmt::Display for TriangulationBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TriangulationBackend {
    type Err = ParseTriangulationBackendError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.replace(' ', "_").to_ascii_lowercase().as_str() {
            "fastest_available" => Ok(Self::FastestAvailable),
            "bermuda" => Ok(Self::Bermuda),
            "partsegcore" => Ok(Self::PartSegCore),
            "triangle" => Ok(Self::Triangle),
            "numba" => Ok(Self::Numba),
            "pure_python" => Ok(Self::PurePython),
            _ => Err(ParseTriangulationBackendError {
                value: value.to_string(),
            }),
        }
    }
}
