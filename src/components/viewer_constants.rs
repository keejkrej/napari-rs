use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomRight,
    BottomCenter,
    BottomLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Square,
    Circle,
    CircleFrozen,
    Cross,
    Forbidden,
    Pointing,
    Standard,
    Crosshair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseViewerConstantError {
    kind: &'static str,
}

impl fmt::Display for ParseViewerConstantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid {}", self.kind)
    }
}

impl std::error::Error for ParseViewerConstantError {}

impl fmt::Display for CanvasPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TopLeft => "top_left",
            Self::TopCenter => "top_center",
            Self::TopRight => "top_right",
            Self::BottomRight => "bottom_right",
            Self::BottomCenter => "bottom_center",
            Self::BottomLeft => "bottom_left",
        })
    }
}

impl fmt::Display for CursorStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Square => "square",
            Self::Circle => "circle",
            Self::CircleFrozen => "circle_frozen",
            Self::Cross => "cross",
            Self::Forbidden => "forbidden",
            Self::Pointing => "pointing",
            Self::Standard => "standard",
            Self::Crosshair => "crosshair",
        })
    }
}

impl FromStr for CanvasPosition {
    type Err = ParseViewerConstantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "top_left" => Ok(Self::TopLeft),
            "top_center" => Ok(Self::TopCenter),
            "top_right" => Ok(Self::TopRight),
            "bottom_right" => Ok(Self::BottomRight),
            "bottom_center" => Ok(Self::BottomCenter),
            "bottom_left" => Ok(Self::BottomLeft),
            _ => Err(ParseViewerConstantError {
                kind: "canvas position",
            }),
        }
    }
}

impl FromStr for CursorStyle {
    type Err = ParseViewerConstantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "square" => Ok(Self::Square),
            "circle" => Ok(Self::Circle),
            "circle_frozen" => Ok(Self::CircleFrozen),
            "cross" => Ok(Self::Cross),
            "forbidden" => Ok(Self::Forbidden),
            "pointing" => Ok(Self::Pointing),
            "standard" => Ok(Self::Standard),
            "crosshair" => Ok(Self::Crosshair),
            _ => Err(ParseViewerConstantError {
                kind: "cursor style",
            }),
        }
    }
}
