use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blending {
    Translucent,
    TranslucentNoDepth,
    Additive,
    Minimum,
    Opaque,
    Multiplicative,
}

impl fmt::Display for Blending {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Translucent => "translucent",
            Self::TranslucentNoDepth => "translucent_no_depth",
            Self::Additive => "additive",
            Self::Minimum => "minimum",
            Self::Opaque => "opaque",
            Self::Multiplicative => "multiplicative",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseBlendingError;

impl fmt::Display for ParseBlendingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid blending mode")
    }
}

impl std::error::Error for ParseBlendingError {}

impl FromStr for Blending {
    type Err = ParseBlendingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "translucent" => Ok(Self::Translucent),
            "translucent_no_depth" => Ok(Self::TranslucentNoDepth),
            "additive" => Ok(Self::Additive),
            "minimum" => Ok(Self::Minimum),
            "opaque" => Ok(Self::Opaque),
            "multiplicative" => Ok(Self::Multiplicative),
            _ => Err(ParseBlendingError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PanZoom,
    Transform,
}

impl fmt::Display for Mode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PanZoom => "pan_zoom",
            Self::Transform => "transform",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseModeError;

impl fmt::Display for ParseModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid mode")
    }
}

impl std::error::Error for ParseModeError {}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "pan_zoom" => Ok(Self::PanZoom),
            "transform" => Ok(Self::Transform),
            _ => Err(ParseModeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum InteractionBoxHandle {
    TopLeft = 0,
    TopCenter = 4,
    TopRight = 2,
    CenterLeft = 5,
    CenterRight = 6,
    BottomLeft = 1,
    BottomCenter = 7,
    BottomRight = 3,
    Rotation = 8,
    Inside = 9,
}

impl InteractionBoxHandle {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::TopLeft),
            1 => Some(Self::BottomLeft),
            2 => Some(Self::TopRight),
            3 => Some(Self::BottomRight),
            4 => Some(Self::TopCenter),
            5 => Some(Self::CenterLeft),
            6 => Some(Self::CenterRight),
            7 => Some(Self::BottomCenter),
            8 => Some(Self::Rotation),
            9 => Some(Self::Inside),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn opposite(self) -> Result<Self, InteractionBoxHandleError> {
        match self {
            Self::TopLeft => Ok(Self::BottomRight),
            Self::TopCenter => Ok(Self::BottomCenter),
            Self::TopRight => Ok(Self::BottomLeft),
            Self::CenterLeft => Ok(Self::CenterRight),
            Self::CenterRight => Ok(Self::CenterLeft),
            Self::BottomLeft => Ok(Self::TopRight),
            Self::BottomCenter => Ok(Self::TopCenter),
            Self::BottomRight => Ok(Self::TopLeft),
            Self::Rotation | Self::Inside => Err(InteractionBoxHandleError::NoOpposite(self)),
        }
    }

    pub fn corners() -> [Self; 4] {
        [
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionBoxHandleError {
    NoOpposite(InteractionBoxHandle),
}

impl fmt::Display for InteractionBoxHandleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoOpposite(handle) => write!(formatter, "{handle:?} has no opposite handle"),
        }
    }
}

impl std::error::Error for InteractionBoxHandleError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Adding,
    Removing,
    Changing,
    Added,
    Removed,
    Changed,
}

impl fmt::Display for ActionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Adding => "adding",
            Self::Removing => "removing",
            Self::Changing => "changing",
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseActionTypeError;

impl fmt::Display for ParseActionTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid action type")
    }
}

impl std::error::Error for ParseActionTypeError {}

impl FromStr for ActionType {
    type Err = ParseActionTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "adding" => Ok(Self::Adding),
            "removing" => Ok(Self::Removing),
            "changing" => Ok(Self::Changing),
            "added" => Ok(Self::Added),
            "removed" => Ok(Self::Removed),
            "changed" => Ok(Self::Changed),
            _ => Err(ParseActionTypeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseProjectionMode {
    None,
}

impl fmt::Display for BaseProjectionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("none")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseBaseProjectionModeError;

impl fmt::Display for ParseBaseProjectionModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid base projection mode")
    }
}

impl std::error::Error for ParseBaseProjectionModeError {}

impl FromStr for BaseProjectionMode {
    type Err = ParseBaseProjectionModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            _ => Err(ParseBaseProjectionModeError),
        }
    }
}
