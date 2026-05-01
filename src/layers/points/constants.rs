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
        formatter.write_str("invalid points color mode")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PanZoom,
    Transform,
    Add,
    Select,
}

impl fmt::Display for Mode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PanZoom => "pan_zoom",
            Self::Transform => "transform",
            Self::Add => "add",
            Self::Select => "select",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseModeError;

impl fmt::Display for ParseModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid points mode")
    }
}

impl std::error::Error for ParseModeError {}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "pan_zoom" => Ok(Self::PanZoom),
            "transform" => Ok(Self::Transform),
            "add" => Ok(Self::Add),
            "select" => Ok(Self::Select),
            _ => Err(ParseModeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Arrow,
    Clobber,
    Cross,
    Diamond,
    Disc,
    Hbar,
    Ring,
    Square,
    Star,
    TailedArrow,
    TriangleDown,
    TriangleUp,
    Vbar,
    X,
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Arrow => "arrow",
            Self::Clobber => "clobber",
            Self::Cross => "cross",
            Self::Diamond => "diamond",
            Self::Disc => "disc",
            Self::Hbar => "hbar",
            Self::Ring => "ring",
            Self::Square => "square",
            Self::Star => "star",
            Self::TailedArrow => "tailed_arrow",
            Self::TriangleDown => "triangle_down",
            Self::TriangleUp => "triangle_up",
            Self::Vbar => "vbar",
            Self::X => "x",
        })
    }
}

impl Symbol {
    pub fn from_alias(value: &str) -> Option<Self> {
        match value {
            ">" => Some(Self::Arrow),
            "+" => Some(Self::Cross),
            "o" => Some(Self::Disc),
            "-" => Some(Self::Hbar),
            "s" => Some(Self::Square),
            "*" => Some(Self::Star),
            "->" => Some(Self::TailedArrow),
            "v" => Some(Self::TriangleDown),
            "^" => Some(Self::TriangleUp),
            "|" => Some(Self::Vbar),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseSymbolError;

impl fmt::Display for ParseSymbolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid points symbol")
    }
}

impl std::error::Error for ParseSymbolError {}

impl FromStr for Symbol {
    type Err = ParseSymbolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(symbol) = Self::from_alias(value) {
            return Ok(symbol);
        }

        match value.to_ascii_lowercase().replace(' ', "_").as_str() {
            "arrow" => Ok(Self::Arrow),
            "clobber" => Ok(Self::Clobber),
            "cross" => Ok(Self::Cross),
            "diamond" => Ok(Self::Diamond),
            "disc" => Ok(Self::Disc),
            "hbar" => Ok(Self::Hbar),
            "ring" => Ok(Self::Ring),
            "square" => Ok(Self::Square),
            "star" => Ok(Self::Star),
            "tailed_arrow" => Ok(Self::TailedArrow),
            "triangle_down" => Ok(Self::TriangleDown),
            "triangle_up" => Ok(Self::TriangleUp),
            "vbar" => Ok(Self::Vbar),
            "x" => Ok(Self::X),
            _ => Err(ParseSymbolError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shading {
    None,
    Spherical,
}

impl fmt::Display for Shading {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::Spherical => "spherical",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseShadingError;

impl fmt::Display for ParseShadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid points shading mode")
    }
}

impl std::error::Error for ParseShadingError {}

impl FromStr for Shading {
    type Err = ParseShadingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "spherical" => Ok(Self::Spherical),
            _ => Err(ParseShadingError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointsProjectionMode {
    None,
    All,
}

impl fmt::Display for PointsProjectionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::All => "all",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsePointsProjectionModeError;

impl fmt::Display for ParsePointsProjectionModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid points projection mode")
    }
}

impl std::error::Error for ParsePointsProjectionModeError {}

impl FromStr for PointsProjectionMode {
    type Err = ParsePointsProjectionModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            _ => Err(ParsePointsProjectionModeError),
        }
    }
}
