use std::error::Error;
use std::fmt;

use crate::layers::utils::color_transformations::Rgba;
use crate::layers::utils::layer_utils::{Properties, PropertyValue};

#[derive(Debug, Clone, PartialEq)]
pub enum ColorArgument {
    Name(String),
    Mapped,
    Array(Vec<Rgba>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorManagerUtilsError {
    EmptyPropertyValues,
    InvalidContrastLimits,
}

impl fmt::Display for ColorManagerUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPropertyValues => formatter.write_str("property values cannot be empty"),
            Self::InvalidContrastLimits => {
                formatter.write_str("contrast limits must be a strictly increasing pair of values")
            }
        }
    }
}

impl Error for ColorManagerUtilsError {}

pub fn guess_continuous(values: &[PropertyValue]) -> bool {
    if values
        .iter()
        .all(|value| matches!(value, PropertyValue::Float(_)))
    {
        return !values.is_empty();
    }
    if values
        .iter()
        .all(|value| matches!(value, PropertyValue::Int(_)))
    {
        let mut unique = Vec::new();
        for value in values {
            if !unique.contains(value) {
                unique.push(value.clone());
            }
        }
        return unique.len() >= 16;
    }
    false
}

pub fn is_color_mapped(color: &ColorArgument, properties: &Properties) -> bool {
    match color {
        ColorArgument::Name(name) => properties.contains_key(name),
        ColorArgument::Mapped => true,
        ColorArgument::Array(_) => false,
    }
}

pub fn calculate_contrast_limits(values: &[f64]) -> Option<(f64, f64)> {
    if values.is_empty() || values.iter().any(|value| value.is_nan()) {
        return None;
    }

    let min_value = values.iter().copied().min_by(f64::total_cmp)?;
    let max_value = values.iter().copied().max_by(f64::total_cmp)?;
    (min_value < max_value).then_some((min_value, max_value))
}

pub fn normalize_property_values(
    values: &[f64],
    contrast_limits: Option<(f64, f64)>,
) -> Result<(Vec<f64>, (f64, f64)), ColorManagerUtilsError> {
    let contrast_limits = match contrast_limits {
        Some(limits) => {
            validate_contrast_limits(limits)?;
            limits
        }
        None => inferred_contrast_limits(values)?,
    };
    let (lower, upper) = contrast_limits;
    let scale = upper - lower;
    let normalized = values
        .iter()
        .map(|value| ((value - lower) / scale).clamp(0.0, 1.0))
        .collect();
    Ok((normalized, contrast_limits))
}

fn inferred_contrast_limits(values: &[f64]) -> Result<(f64, f64), ColorManagerUtilsError> {
    if values.is_empty() {
        return Err(ColorManagerUtilsError::EmptyPropertyValues);
    }
    let min_value = values
        .iter()
        .copied()
        .min_by(f64::total_cmp)
        .expect("empty values already checked");
    let max_value = values
        .iter()
        .copied()
        .max_by(f64::total_cmp)
        .expect("empty values already checked");
    validate_contrast_limits((min_value, max_value))?;
    Ok((min_value, max_value))
}

fn validate_contrast_limits(limits: (f64, f64)) -> Result<(), ColorManagerUtilsError> {
    if limits.0 < limits.1 {
        Ok(())
    } else {
        Err(ColorManagerUtilsError::InvalidContrastLimits)
    }
}
