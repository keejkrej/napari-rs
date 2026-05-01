use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Direct,
    Cycle,
    Colormap,
}

impl fmt::Display for ColorMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Direct => "direct",
            Self::Cycle => "cycle",
            Self::Colormap => "colormap",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseColorModeError;

impl fmt::Display for ParseColorModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid color manager mode")
    }
}

impl std::error::Error for ParseColorModeError {}

impl FromStr for ColorMode {
    type Err = ParseColorModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "direct" => Ok(Self::Direct),
            "cycle" => Ok(Self::Cycle),
            "colormap" => Ok(Self::Colormap),
            _ => Err(ParseColorModeError),
        }
    }
}
