use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Center,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl fmt::Display for Anchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Center => "center",
            Self::UpperLeft => "upper_left",
            Self::UpperRight => "upper_right",
            Self::LowerLeft => "lower_left",
            Self::LowerRight => "lower_right",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseAnchorError;

impl fmt::Display for ParseAnchorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid text anchor")
    }
}

impl std::error::Error for ParseAnchorError {}

impl FromStr for Anchor {
    type Err = ParseAnchorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "center" => Ok(Self::Center),
            "upper_left" => Ok(Self::UpperLeft),
            "upper_right" => Ok(Self::UpperRight),
            "lower_left" => Ok(Self::LowerLeft),
            "lower_right" => Ok(Self::LowerRight),
            _ => Err(ParseAnchorError),
        }
    }
}
