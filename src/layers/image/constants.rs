use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    Bessel,
    Cubic,
    Linear,
    Blackman,
    Catrom,
    Gaussian,
    Hamming,
    Hanning,
    Hermite,
    Kaiser,
    Lanczos,
    Mitchell,
    Nearest,
    Spline16,
    Spline36,
    Custom,
}

impl Interpolation {
    pub fn view_subset() -> [Self; 5] {
        [
            Self::Cubic,
            Self::Linear,
            Self::Kaiser,
            Self::Nearest,
            Self::Spline36,
        ]
    }
}

impl fmt::Display for Interpolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Bessel => "bessel",
            Self::Cubic => "cubic",
            Self::Linear => "linear",
            Self::Blackman => "blackman",
            Self::Catrom => "catrom",
            Self::Gaussian => "gaussian",
            Self::Hamming => "hamming",
            Self::Hanning => "hanning",
            Self::Hermite => "hermite",
            Self::Kaiser => "kaiser",
            Self::Lanczos => "lanczos",
            Self::Mitchell => "mitchell",
            Self::Nearest => "nearest",
            Self::Spline16 => "spline16",
            Self::Spline36 => "spline36",
            Self::Custom => "custom",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseInterpolationError;

impl fmt::Display for ParseInterpolationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid interpolation mode")
    }
}

impl std::error::Error for ParseInterpolationError {}

impl FromStr for Interpolation {
    type Err = ParseInterpolationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "bessel" => Ok(Self::Bessel),
            "cubic" => Ok(Self::Cubic),
            "linear" => Ok(Self::Linear),
            "blackman" => Ok(Self::Blackman),
            "catrom" => Ok(Self::Catrom),
            "gaussian" => Ok(Self::Gaussian),
            "hamming" => Ok(Self::Hamming),
            "hanning" => Ok(Self::Hanning),
            "hermite" => Ok(Self::Hermite),
            "kaiser" => Ok(Self::Kaiser),
            "lanczos" => Ok(Self::Lanczos),
            "mitchell" => Ok(Self::Mitchell),
            "nearest" => Ok(Self::Nearest),
            "spline16" => Ok(Self::Spline16),
            "spline36" => Ok(Self::Spline36),
            "custom" => Ok(Self::Custom),
            _ => Err(ParseInterpolationError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageRendering {
    Translucent,
    Additive,
    Iso,
    Mip,
    Minip,
    AttenuatedMip,
    Average,
}

impl fmt::Display for ImageRendering {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Translucent => "translucent",
            Self::Additive => "additive",
            Self::Iso => "iso",
            Self::Mip => "mip",
            Self::Minip => "minip",
            Self::AttenuatedMip => "attenuated_mip",
            Self::Average => "average",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseImageRenderingError;

impl fmt::Display for ParseImageRenderingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid image rendering mode")
    }
}

impl std::error::Error for ParseImageRenderingError {}

impl FromStr for ImageRendering {
    type Err = ParseImageRenderingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "translucent" => Ok(Self::Translucent),
            "additive" => Ok(Self::Additive),
            "iso" => Ok(Self::Iso),
            "mip" => Ok(Self::Mip),
            "minip" => Ok(Self::Minip),
            "attenuated_mip" => Ok(Self::AttenuatedMip),
            "average" => Ok(Self::Average),
            _ => Err(ParseImageRenderingError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeDepiction {
    Volume,
    Plane,
}

impl fmt::Display for VolumeDepiction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Volume => "volume",
            Self::Plane => "plane",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseVolumeDepictionError;

impl fmt::Display for ParseVolumeDepictionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid volume depiction")
    }
}

impl std::error::Error for ParseVolumeDepictionError {}

impl FromStr for VolumeDepiction {
    type Err = ParseVolumeDepictionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "volume" => Ok(Self::Volume),
            "plane" => Ok(Self::Plane),
            _ => Err(ParseVolumeDepictionError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageProjectionMode {
    None,
    Sum,
    Mean,
    Max,
    Min,
}

impl fmt::Display for ImageProjectionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::Sum => "sum",
            Self::Mean => "mean",
            Self::Max => "max",
            Self::Min => "min",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseImageProjectionModeError;

impl fmt::Display for ParseImageProjectionModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid image projection mode")
    }
}

impl std::error::Error for ParseImageProjectionModeError {}

impl FromStr for ImageProjectionMode {
    type Err = ParseImageProjectionModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "sum" => Ok(Self::Sum),
            "mean" => Ok(Self::Mean),
            "max" => Ok(Self::Max),
            "min" => Ok(Self::Min),
            _ => Err(ParseImageProjectionModeError),
        }
    }
}
