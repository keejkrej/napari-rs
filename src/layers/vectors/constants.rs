use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorStyle {
    Line,
    Triangle,
    Arrow,
}

impl fmt::Display for VectorStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Line => "line",
            Self::Triangle => "triangle",
            Self::Arrow => "arrow",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseVectorStyleError;

impl fmt::Display for ParseVectorStyleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid vector style")
    }
}

impl std::error::Error for ParseVectorStyleError {}

impl FromStr for VectorStyle {
    type Err = ParseVectorStyleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "line" => Ok(Self::Line),
            "triangle" => Ok(Self::Triangle),
            "arrow" => Ok(Self::Arrow),
            _ => Err(ParseVectorStyleError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorsProjectionMode {
    None,
    All,
}

impl fmt::Display for VectorsProjectionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::All => "all",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseVectorsProjectionModeError;

impl fmt::Display for ParseVectorsProjectionModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid vectors projection mode")
    }
}

impl std::error::Error for ParseVectorsProjectionModeError {}

impl FromStr for VectorsProjectionMode {
    type Err = ParseVectorsProjectionModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            _ => Err(ParseVectorsProjectionModeError),
        }
    }
}
