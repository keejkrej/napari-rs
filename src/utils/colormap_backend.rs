use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseColormapBackendError {
    value: String,
}

impl fmt::Display for ParseColormapBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown colormap backend: {}", self.value)
    }
}

impl std::error::Error for ParseColormapBackendError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColormapBackend {
    FastestAvailable,
    PurePython,
    Numba,
    PartSegCore,
}

impl ColormapBackend {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FastestAvailable => "Fastest available",
            Self::PurePython => "Pure Python",
            Self::Numba => "numba",
            Self::PartSegCore => "PartSegCore",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::FastestAvailable => "fastest_available",
            Self::PurePython => "pure_python",
            Self::Numba => "numba",
            Self::PartSegCore => "partsegcore",
        }
    }
}

impl fmt::Display for ColormapBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ColormapBackend {
    type Err = ParseColormapBackendError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.replace(' ', "_").to_ascii_lowercase().as_str() {
            "fastest_available" => Ok(Self::FastestAvailable),
            "pure_python" => Ok(Self::PurePython),
            "numba" => Ok(Self::Numba),
            "partsegcore" => Ok(Self::PartSegCore),
            _ => Err(ParseColormapBackendError {
                value: value.to_string(),
            }),
        }
    }
}
