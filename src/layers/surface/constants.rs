use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shading {
    None,
    Flat,
    Smooth,
}

impl fmt::Display for Shading {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::Flat => "flat",
            Self::Smooth => "smooth",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseShadingError;

impl fmt::Display for ParseShadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid surface shading mode")
    }
}

impl std::error::Error for ParseShadingError {}

impl FromStr for Shading {
    type Err = ParseShadingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "flat" => Ok(Self::Flat),
            "smooth" => Ok(Self::Smooth),
            _ => Err(ParseShadingError),
        }
    }
}
