use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PanZoom,
    Transform,
    Pick,
    Paint,
    Fill,
    Erase,
    Polygon,
}

impl fmt::Display for Mode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PanZoom => "pan_zoom",
            Self::Transform => "transform",
            Self::Pick => "pick",
            Self::Paint => "paint",
            Self::Fill => "fill",
            Self::Erase => "erase",
            Self::Polygon => "polygon",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseModeError;

impl fmt::Display for ParseModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid labels mode")
    }
}

impl std::error::Error for ParseModeError {}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "pan_zoom" => Ok(Self::PanZoom),
            "transform" => Ok(Self::Transform),
            "pick" => Ok(Self::Pick),
            "paint" => Ok(Self::Paint),
            "fill" => Ok(Self::Fill),
            "erase" => Ok(Self::Erase),
            "polygon" => Ok(Self::Polygon),
            _ => Err(ParseModeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelColorMode {
    Auto,
    Direct,
}

impl fmt::Display for LabelColorMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Auto => "auto",
            Self::Direct => "direct",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLabelColorModeError;

impl fmt::Display for ParseLabelColorModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid label color mode")
    }
}

impl std::error::Error for ParseLabelColorModeError {}

impl FromStr for LabelColorMode {
    type Err = ParseLabelColorModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "direct" => Ok(Self::Direct),
            _ => Err(ParseLabelColorModeError),
        }
    }
}

#[cfg(target_os = "macos")]
pub const BACKSPACE: &str = "delete";
#[cfg(not(target_os = "macos"))]
pub const BACKSPACE: &str = "backspace";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelsRendering {
    Translucent,
    IsoCategorical,
}

impl fmt::Display for LabelsRendering {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Translucent => "translucent",
            Self::IsoCategorical => "iso_categorical",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLabelsRenderingError;

impl fmt::Display for ParseLabelsRenderingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid labels rendering mode")
    }
}

impl std::error::Error for ParseLabelsRenderingError {}

impl FromStr for LabelsRendering {
    type Err = ParseLabelsRenderingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "translucent" => Ok(Self::Translucent),
            "iso_categorical" => Ok(Self::IsoCategorical),
            _ => Err(ParseLabelsRenderingError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoCategoricalGradientMode {
    Fast,
    Smooth,
}

impl fmt::Display for IsoCategoricalGradientMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Fast => "fast",
            Self::Smooth => "smooth",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseIsoCategoricalGradientModeError;

impl fmt::Display for ParseIsoCategoricalGradientModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid iso-categorical gradient mode")
    }
}

impl std::error::Error for ParseIsoCategoricalGradientModeError {}

impl FromStr for IsoCategoricalGradientMode {
    type Err = ParseIsoCategoricalGradientModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "fast" => Ok(Self::Fast),
            "smooth" => Ok(Self::Smooth),
            _ => Err(ParseIsoCategoricalGradientModeError),
        }
    }
}
