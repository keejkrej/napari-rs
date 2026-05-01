use std::error::Error;
use std::fmt;

use crate::layers::utils::color_transformations::Rgba;

pub const DEFAULT_COLOR: Rgba = [0.0, 1.0, 1.0, 1.0];

#[derive(Debug, Clone, PartialEq)]
pub enum ColorEncodingSpec {
    Constant {
        constant: Rgba,
    },
    Manual {
        array: Vec<Rgba>,
        default: Rgba,
    },
    Direct {
        feature: String,
        fallback: Rgba,
    },
    Nominal {
        feature: String,
        colormap: String,
        fallback: Rgba,
    },
    Quantitative {
        feature: String,
        colormap: String,
        contrast_limits: Option<(f64, f64)>,
        fallback: Rgba,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorEncodingError {
    InvalidContrastLimits,
}

impl fmt::Display for ColorEncodingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContrastLimits => {
                formatter.write_str("contrast_limits must be a strictly increasing pair of values")
            }
        }
    }
}

impl Error for ColorEncodingError {}

pub fn validate_color_encoding_from_colors(colors: &[Rgba]) -> ColorEncodingSpec {
    if colors.len() == 1 {
        ColorEncodingSpec::Constant {
            constant: colors[0],
        }
    } else {
        ColorEncodingSpec::Manual {
            array: colors.to_vec(),
            default: DEFAULT_COLOR,
        }
    }
}

pub fn direct_color_encoding(feature: impl Into<String>) -> ColorEncodingSpec {
    ColorEncodingSpec::Direct {
        feature: feature.into(),
        fallback: DEFAULT_COLOR,
    }
}

pub fn nominal_color_encoding(
    feature: impl Into<String>,
    colormap: impl Into<String>,
) -> ColorEncodingSpec {
    ColorEncodingSpec::Nominal {
        feature: feature.into(),
        colormap: colormap.into(),
        fallback: DEFAULT_COLOR,
    }
}

pub fn quantitative_color_encoding(
    feature: impl Into<String>,
    colormap: impl Into<String>,
    contrast_limits: Option<(f64, f64)>,
) -> Result<ColorEncodingSpec, ColorEncodingError> {
    validate_contrast_limits(contrast_limits)?;
    Ok(ColorEncodingSpec::Quantitative {
        feature: feature.into(),
        colormap: colormap.into(),
        contrast_limits,
        fallback: DEFAULT_COLOR,
    })
}

pub fn validate_contrast_limits(
    contrast_limits: Option<(f64, f64)>,
) -> Result<Option<(f64, f64)>, ColorEncodingError> {
    if let Some((lower, upper)) = contrast_limits
        && lower >= upper
    {
        return Err(ColorEncodingError::InvalidContrastLimits);
    }
    Ok(contrast_limits)
}
