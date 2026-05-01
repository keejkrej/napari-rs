use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAxisOrientation {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAxisOrientation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthAxisOrientation {
    Away,
    Towards,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handedness {
    Right,
    Left,
}

pub const DEFAULT_ORIENTATION: (
    DepthAxisOrientation,
    VerticalAxisOrientation,
    HorizontalAxisOrientation,
) = (
    DepthAxisOrientation::Towards,
    VerticalAxisOrientation::Down,
    HorizontalAxisOrientation::Right,
);

pub const DEFAULT_ORIENTATION_STR: (&str, &str, &str) = ("towards", "down", "right");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseOrientationError {
    kind: &'static str,
}

impl fmt::Display for ParseOrientationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid {} orientation", self.kind)
    }
}

impl std::error::Error for ParseOrientationError {}

impl fmt::Display for VerticalAxisOrientation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Up => "up",
            Self::Down => "down",
        })
    }
}

impl fmt::Display for HorizontalAxisOrientation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Left => "left",
            Self::Right => "right",
        })
    }
}

impl fmt::Display for DepthAxisOrientation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Away => "away",
            Self::Towards => "towards",
        })
    }
}

impl fmt::Display for Handedness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Right => "right",
            Self::Left => "left",
        })
    }
}

impl FromStr for VerticalAxisOrientation {
    type Err = ParseOrientationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(ParseOrientationError { kind: "vertical" }),
        }
    }
}

impl FromStr for HorizontalAxisOrientation {
    type Err = ParseOrientationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(ParseOrientationError { kind: "horizontal" }),
        }
    }
}

impl FromStr for DepthAxisOrientation {
    type Err = ParseOrientationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "away" => Ok(Self::Away),
            "towards" => Ok(Self::Towards),
            _ => Err(ParseOrientationError { kind: "depth" }),
        }
    }
}

impl FromStr for Handedness {
    type Err = ParseOrientationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "right" => Ok(Self::Right),
            "left" => Ok(Self::Left),
            _ => Err(ParseOrientationError { kind: "handedness" }),
        }
    }
}
