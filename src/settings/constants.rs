use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseSettingsConstantError {
    type_name: &'static str,
    value: String,
}

impl ParseSettingsConstantError {
    fn new(type_name: &'static str, value: &str) -> Self {
        Self {
            type_name,
            value: value.to_string(),
        }
    }
}

impl fmt::Display for ParseSettingsConstantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown {} value: {}",
            self.type_name, self.value
        )
    }
}

impl std::error::Error for ParseSettingsConstantError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LabelDType {
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Uint,
    Int,
}

impl LabelDType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uint8 => "uint8",
            Self::Int8 => "int8",
            Self::Uint16 => "uint16",
            Self::Int16 => "int16",
            Self::Uint32 => "uint32",
            Self::Int32 => "int32",
            Self::Uint64 => "uint64",
            Self::Int64 => "int64",
            Self::Uint => "uint",
            Self::Int => "int",
        }
    }
}

impl fmt::Display for LabelDType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LabelDType {
    type Err = ParseSettingsConstantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "uint8" => Ok(Self::Uint8),
            "int8" => Ok(Self::Int8),
            "uint16" => Ok(Self::Uint16),
            "int16" => Ok(Self::Int16),
            "uint32" => Ok(Self::Uint32),
            "int32" => Ok(Self::Int32),
            "uint64" => Ok(Self::Uint64),
            "int64" => Ok(Self::Int64),
            "uint" => Ok(Self::Uint),
            "int" => Ok(Self::Int),
            _ => Err(ParseSettingsConstantError::new("LabelDType", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoopMode {
    Once,
    Loop,
    BackAndForth,
}

impl LoopMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::Loop => "loop",
            Self::BackAndForth => "back_and_forth",
        }
    }
}

impl fmt::Display for LoopMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LoopMode {
    type Err = ParseSettingsConstantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "once" => Ok(Self::Once),
            "loop" => Ok(Self::Loop),
            "back_and_forth" => Ok(Self::BackAndForth),
            _ => Err(ParseSettingsConstantError::new("LoopMode", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrushSizeOnMouseModifiers {
    Alt,
    Ctrl,
    CtrlAlt,
    CtrlShift,
    Disabled,
}

impl BrushSizeOnMouseModifiers {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Alt => "Alt",
            Self::Ctrl => "Control",
            Self::CtrlAlt => "Control+Alt",
            Self::CtrlShift => "Control+Shift",
            Self::Disabled => "Disabled",
        }
    }
}

impl fmt::Display for BrushSizeOnMouseModifiers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BrushSizeOnMouseModifiers {
    type Err = ParseSettingsConstantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Alt" => Ok(Self::Alt),
            "Control" => Ok(Self::Ctrl),
            "Control+Alt" => Ok(Self::CtrlAlt),
            "Control+Shift" => Ok(Self::CtrlShift),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(ParseSettingsConstantError::new(
                "BrushSizeOnMouseModifiers",
                value,
            )),
        }
    }
}
