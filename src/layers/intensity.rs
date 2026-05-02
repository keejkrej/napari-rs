use std::fmt;

use crate::utils::dtype::{DType, get_dtype_limits};
use crate::utils::status_messages::format_float;
use crate::utils::validators::{ValidationError, validate_increasing};

pub type ContrastPair = [f64; 2];
pub type OptionalContrastPair = [Option<f64>; 2];

#[derive(Debug, Clone, PartialEq)]
pub struct IntensityVisualization {
    pub gamma: f64,
    pub colormap_name: String,
    pub contrast_limits_msg: String,
    contrast_limits: OptionalContrastPair,
    contrast_limits_range: OptionalContrastPair,
    pub auto_contrast_source: ContrastSource,
    pub keep_auto_contrast: bool,
}

impl IntensityVisualization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contrast_limits(&self) -> OptionalContrastPair {
        self.contrast_limits
    }

    pub fn contrast_limits_range(&self) -> OptionalContrastPair {
        self.contrast_limits_range
    }

    pub fn set_gamma(&mut self, value: impl Into<f64>) {
        self.gamma = value.into();
    }

    pub fn set_contrast_limits(
        &mut self,
        contrast_limits: ContrastPair,
    ) -> Result<(), IntensityError> {
        validate_pair(contrast_limits)?;
        self.contrast_limits_msg = format!(
            "{}, {}",
            format_float(contrast_limits[0]),
            format_float(contrast_limits[1])
        );
        self.contrast_limits = [Some(contrast_limits[0]), Some(contrast_limits[1])];

        let mut new_range = self.contrast_limits_range;
        new_range[0] = Some(match new_range[0] {
            Some(lower) => lower.min(contrast_limits[0]),
            None => contrast_limits[0],
        });
        new_range[1] = Some(match new_range[1] {
            Some(upper) => upper.max(contrast_limits[1]),
            None => contrast_limits[1],
        });
        self.set_contrast_limits_range(new_range)?;
        Ok(())
    }

    pub fn set_contrast_limits_range(
        &mut self,
        value: OptionalContrastPair,
    ) -> Result<(), IntensityError> {
        validate_optional_pair(value)?;
        if value == self.contrast_limits_range {
            return Ok(());
        }

        let current_range = self.contrast_limits_range;
        let new_range = [value[0].or(current_range[0]), value[1].or(current_range[1])];
        validate_optional_pair(new_range)?;
        self.contrast_limits_range = new_range;

        if let Some(contrast_limits) = self.contrast_limits_pair() {
            let Some(new_range_pair) = optional_pair_to_pair(new_range) else {
                return Ok(());
            };
            let clipped_limits = [
                contrast_limits[0].clamp(new_range_pair[0], new_range_pair[1]),
                contrast_limits[1].clamp(new_range_pair[0], new_range_pair[1]),
            ];
            if clipped_limits[0] < clipped_limits[1] {
                self.set_contrast_limits(clipped_limits)?;
            } else {
                self.set_contrast_limits(new_range_pair)?;
            }
        }
        Ok(())
    }

    pub fn reset_contrast_limits(
        &mut self,
        data_range: ContrastPair,
    ) -> Result<(), IntensityError> {
        self.set_contrast_limits(data_range)
    }

    pub fn reset_contrast_limits_range(
        &mut self,
        dtype: DType,
        data_range: ContrastPair,
    ) -> Result<(), IntensityError> {
        let range = if dtype.is_integer() {
            dtype_limits(dtype)?
        } else {
            data_range
        };
        self.set_contrast_limits_range([Some(range[0]), Some(range[1])])
    }

    fn contrast_limits_pair(&self) -> Option<ContrastPair> {
        optional_pair_to_pair(self.contrast_limits)
    }
}

impl Default for IntensityVisualization {
    fn default() -> Self {
        Self {
            gamma: 1.0,
            colormap_name: String::new(),
            contrast_limits_msg: String::new(),
            contrast_limits: [None, None],
            contrast_limits_range: [None, None],
            auto_contrast_source: ContrastSource::Slice,
            keep_auto_contrast: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastSource {
    Data,
    Slice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntensityError {
    NotIncreasing,
    MissingRangeBoundary,
    NonNumericDType(DType),
}

impl fmt::Display for IntensityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotIncreasing => formatter.write_str("contrast limits must be increasing"),
            Self::MissingRangeBoundary => {
                formatter.write_str("contrast limits range requires concrete boundaries")
            }
            Self::NonNumericDType(dtype) => {
                write!(formatter, "dtype has no numeric limits: {dtype:?}")
            }
        }
    }
}

impl std::error::Error for IntensityError {}

fn validate_pair(value: ContrastPair) -> Result<(), IntensityError> {
    validate_increasing(&value).map_err(|error| match error {
        ValidationError::NotIncreasing => IntensityError::NotIncreasing,
        _ => IntensityError::NotIncreasing,
    })
}

fn validate_optional_pair(value: OptionalContrastPair) -> Result<(), IntensityError> {
    if let Some(pair) = optional_pair_to_pair(value) {
        validate_pair(pair)?;
    }
    Ok(())
}

fn optional_pair_to_pair(value: OptionalContrastPair) -> Option<ContrastPair> {
    Some([value[0]?, value[1]?])
}

fn dtype_limits(dtype: DType) -> Result<ContrastPair, IntensityError> {
    get_dtype_limits(dtype_name(dtype))
        .map(|(lower, upper)| [lower, upper])
        .map_err(|_| IntensityError::NonNumericDType(dtype))
}

fn dtype_name(dtype: DType) -> &'static str {
    match dtype {
        DType::Bool => "bool",
        DType::UInt8 => "uint8",
        DType::UInt16 => "uint16",
        DType::UInt32 => "uint32",
        DType::UInt64 => "uint64",
        DType::Int8 => "int8",
        DType::Int16 => "int16",
        DType::Int32 => "int32",
        DType::Int64 => "int64",
        DType::Float16 => "float16",
        DType::Float32 => "float32",
        DType::Float64 => "float64",
        DType::Complex64 => "complex64",
        DType::Complex128 => "complex128",
    }
}
