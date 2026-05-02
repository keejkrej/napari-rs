use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use crate::utils::dtype::DType;
use crate::utils::transforms::affine::Affine;
use crate::utils::transforms::transform_utils::{Matrix, identity};

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

pub type Properties = BTreeMap<String, Vec<PropertyValue>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayerUtilsError {
    PointDimensionMismatch {
        a: usize,
        b: usize,
    },
    UnsupportedPointDimension(usize),
    CurrentPropertyLength {
        key: String,
        len: usize,
    },
    PropertyLengthMismatch {
        key: String,
        expected: usize,
        found: usize,
    },
    InvalidExtentRows {
        rows: usize,
    },
    MismatchedExtentDimensions,
    MultiscaleDimensionMismatch {
        expected: usize,
        found: usize,
    },
    UnrecognizedAffineInput,
    ZeroDownsampleFactor,
}

impl fmt::Display for LayerUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PointDimensionMismatch { a, b } => {
                write!(formatter, "point dimensions must match, got {a} and {b}")
            }
            Self::UnsupportedPointDimension(ndim) => {
                write!(formatter, "point dimension must be 2 or 3, got {ndim}")
            }
            Self::CurrentPropertyLength { key, len } => {
                write!(
                    formatter,
                    "current property {key:?} should have length 1, got {len}"
                )
            }
            Self::PropertyLengthMismatch {
                key,
                expected,
                found,
            } => write!(
                formatter,
                "property {key:?} should have length {expected}, got {found}"
            ),
            Self::InvalidExtentRows { rows } => {
                write!(formatter, "data extent must have two rows, got {rows}")
            }
            Self::MismatchedExtentDimensions => {
                formatter.write_str("data extent rows must have matching dimensionality")
            }
            Self::MultiscaleDimensionMismatch { expected, found } => write!(
                formatter,
                "multiscale inputs must have dimension {expected}, got {found}"
            ),
            Self::UnrecognizedAffineInput => formatter.write_str(
                "affine input not recognized. must be either napari.utils.transforms.Affine or ndarray",
            ),
            Self::ZeroDownsampleFactor => formatter.write_str("downsample factors cannot be zero"),
        }
    }
}

impl std::error::Error for LayerUtilsError {}

pub fn nanmin(values: &[f64]) -> f64 {
    let min_value = values.iter().copied().fold(f64::INFINITY, f64::min);
    if min_value.is_finite() {
        return min_value;
    }

    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .min_by(f64::total_cmp)
        .unwrap_or(0.0)
}

pub fn nanmax(values: &[f64]) -> f64 {
    let max_value = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if max_value.is_finite() {
        return max_value;
    }

    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp)
        .unwrap_or(1.0)
}

pub fn calc_data_range(values: &[f64], dtype: Option<DType>) -> (f64, f64) {
    if dtype == Some(DType::UInt8) {
        return (0.0, 255.0);
    }

    let mut min_value = nanmin(values);
    let mut max_value = nanmax(values);
    if min_value == max_value {
        min_value = min_value.min(0.0);
        max_value = max_value.max(1.0);
    }
    (min_value, max_value)
}

pub fn segment_normal(a: &[f64], b: &[f64], p: [f64; 3]) -> Result<Vec<f64>, LayerUtilsError> {
    if a.len() != b.len() {
        return Err(LayerUtilsError::PointDimensionMismatch {
            a: a.len(),
            b: b.len(),
        });
    }

    let normal = match a.len() {
        2 => vec![b[1] - a[1], -(b[0] - a[0])],
        3 => {
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            vec![
                d[1] * p[2] - d[2] * p[1],
                d[2] * p[0] - d[0] * p[2],
                d[0] * p[1] - d[1] * p[0],
            ]
        }
        ndim => return Err(LayerUtilsError::UnsupportedPointDimension(ndim)),
    };

    let norm = normal.iter().map(|value| value * value).sum::<f64>().sqrt();
    let norm = if norm == 0.0 { 1.0 } else { norm };
    Ok(normal.into_iter().map(|value| value / norm).collect())
}

pub fn get_current_properties(
    properties: &Properties,
    choices: &Properties,
    num_data: usize,
) -> Properties {
    if num_data > 0 {
        properties
            .iter()
            .filter_map(|(key, values)| {
                values
                    .last()
                    .cloned()
                    .map(|value| (key.clone(), vec![value]))
            })
            .collect()
    } else if !choices.is_empty() {
        choices
            .iter()
            .filter_map(|(key, values)| {
                values
                    .first()
                    .cloned()
                    .map(|value| (key.clone(), vec![value]))
            })
            .collect()
    } else {
        Properties::new()
    }
}

pub fn coerce_current_properties(
    current_properties: &Properties,
) -> Result<Properties, LayerUtilsError> {
    for (key, values) in current_properties {
        if values.len() != 1 {
            return Err(LayerUtilsError::CurrentPropertyLength {
                key: key.clone(),
                len: values.len(),
            });
        }
    }
    Ok(current_properties.clone())
}

pub fn validate_properties(
    properties: Option<&Properties>,
    expected_len: Option<usize>,
) -> Result<Properties, LayerUtilsError> {
    let Some(properties) = properties else {
        return Ok(Properties::new());
    };
    if properties.is_empty() {
        return Ok(Properties::new());
    }

    let expected_len = expected_len.unwrap_or_else(|| {
        properties
            .values()
            .next()
            .map(Vec::len)
            .expect("properties is non-empty")
    });
    for (key, values) in properties {
        if values.len() != expected_len {
            return Err(LayerUtilsError::PropertyLengthMismatch {
                key: key.clone(),
                expected: expected_len,
                found: values.len(),
            });
        }
    }

    Ok(properties.clone())
}

pub fn validate_property_choices(property_choices: Option<&Properties>) -> Properties {
    let Some(property_choices) = property_choices else {
        return Properties::new();
    };

    property_choices
        .iter()
        .map(|(key, values)| {
            let mut unique = Vec::new();
            for value in values {
                if !unique.contains(value) {
                    unique.push(value.clone());
                }
            }
            unique.sort_by(compare_property_values);
            (key.clone(), unique)
        })
        .collect()
}

pub fn unique_element<T: PartialEq + Clone>(values: &[T]) -> Option<T> {
    let first = values.first()?;
    if values[1..].iter().any(|value| value != first) {
        None
    } else {
        Some(first.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AffineInput {
    None,
    Matrix(Matrix),
    Affine(Affine),
    Invalid,
}

pub fn coerce_affine(
    affine: AffineInput,
    ndim: usize,
    name: Option<String>,
) -> Result<Affine, LayerUtilsError> {
    let mut affine = match affine {
        AffineInput::None => Affine::from_affine_matrix(identity(ndim + 1), None),
        AffineInput::Matrix(matrix) => Affine::from_affine_matrix_with_ndim(matrix, ndim, None),
        AffineInput::Affine(affine) => affine,
        AffineInput::Invalid => return Err(LayerUtilsError::UnrecognizedAffineInput),
    };
    if name.is_some() {
        affine.name = name;
    }
    Ok(affine)
}

pub fn get_extent_world(
    data_extent: &[Vec<f64>],
    data_to_world: &Affine,
) -> Result<Vec<Vec<f64>>, LayerUtilsError> {
    if data_extent.len() != 2 {
        return Err(LayerUtilsError::InvalidExtentRows {
            rows: data_extent.len(),
        });
    }
    if data_extent[0].len() != data_extent[1].len() {
        return Err(LayerUtilsError::MismatchedExtentDimensions);
    }

    let ndim = data_extent[0].len();
    let mut corners = Vec::with_capacity(usize::pow(2, ndim as u32));
    for mask in 0..usize::pow(2, ndim as u32) {
        let mut corner = Vec::with_capacity(ndim);
        for (axis, (&low, &high)) in data_extent[0].iter().zip(&data_extent[1]).enumerate() {
            let value = if ((mask >> axis) & 1) == 0 { low } else { high };
            corner.push(value);
        }
        corners.push(corner);
    }

    let world_corners = data_to_world.transform_points(&corners);
    let mut min_corner = vec![f64::INFINITY; ndim];
    let mut max_corner = vec![f64::NEG_INFINITY; ndim];
    for corner in world_corners {
        for (axis, value) in corner.into_iter().enumerate() {
            min_corner[axis] = min_corner[axis].min(value);
            max_corner[axis] = max_corner[axis].max(value);
        }
    }

    Ok(vec![min_corner, max_corner])
}

pub fn dims_displayed_world_to_layer(
    dims_displayed_world: &[usize],
    ndim_world: usize,
    ndim_layer: usize,
) -> Vec<usize> {
    let order = if ndim_world > dims_displayed_world.len() {
        let mut order: Vec<usize> = (0..ndim_world)
            .filter(|dim| !dims_displayed_world.contains(dim))
            .collect();
        order.extend_from_slice(dims_displayed_world);
        order
    } else {
        dims_displayed_world.to_vec()
    };

    let offset = ndim_world as isize - ndim_layer as isize;
    let order = if offset <= 0 {
        let mut adjusted: Vec<usize> = (0..(-offset as usize)).collect();
        adjusted.extend(order.iter().map(|dim| (*dim as isize - offset) as usize));
        adjusted
    } else {
        order
            .iter()
            .filter_map(|dim| {
                if *dim as isize >= offset {
                    Some((*dim as isize - offset) as usize)
                } else {
                    None
                }
            })
            .collect()
    };

    let n_display_layer = dims_displayed_world.len().min(ndim_layer);
    order[order.len() - n_display_layer..].to_vec()
}

pub fn compute_multiscale_level(
    requested_shape: &[f64],
    shape_threshold: &[f64],
    downsample_factors: &[Vec<f64>],
) -> Result<usize, LayerUtilsError> {
    validate_multiscale_inputs(requested_shape, shape_threshold, downsample_factors)?;
    let mut level = 0;
    for (index, factors) in downsample_factors.iter().enumerate() {
        if requested_shape
            .iter()
            .zip(shape_threshold)
            .zip(factors)
            .all(|((&requested, &threshold), &factor)| requested / factor > threshold)
        {
            level = index;
        }
    }
    Ok(level)
}

pub fn compute_multiscale_level_and_corners(
    corner_pixels: [[f64; 2]; 2],
    shape_threshold: &[f64],
    downsample_factors: &[Vec<f64>],
) -> Result<(usize, [[isize; 2]; 2]), LayerUtilsError> {
    let requested_shape = [
        corner_pixels[1][0] - corner_pixels[0][0],
        corner_pixels[1][1] - corner_pixels[0][1],
    ];
    let level = compute_multiscale_level(&requested_shape, shape_threshold, downsample_factors)?;
    let factors = &downsample_factors[level];
    Ok((
        level,
        [
            [
                (corner_pixels[0][0] / factors[0]).floor() as isize,
                (corner_pixels[0][1] / factors[1]).floor() as isize,
            ],
            [
                (corner_pixels[1][0] / factors[0]).ceil() as isize,
                (corner_pixels[1][1] / factors[1]).ceil() as isize,
            ],
        ],
    ))
}

fn validate_multiscale_inputs(
    requested_shape: &[f64],
    shape_threshold: &[f64],
    downsample_factors: &[Vec<f64>],
) -> Result<(), LayerUtilsError> {
    let ndim = requested_shape.len();
    if shape_threshold.len() != ndim {
        return Err(LayerUtilsError::MultiscaleDimensionMismatch {
            expected: ndim,
            found: shape_threshold.len(),
        });
    }
    for factors in downsample_factors {
        if factors.len() != ndim {
            return Err(LayerUtilsError::MultiscaleDimensionMismatch {
                expected: ndim,
                found: factors.len(),
            });
        }
        if factors.contains(&0.0) {
            return Err(LayerUtilsError::ZeroDownsampleFactor);
        }
    }
    Ok(())
}

fn compare_property_values(left: &PropertyValue, right: &PropertyValue) -> Ordering {
    match (left, right) {
        (PropertyValue::Bool(left), PropertyValue::Bool(right)) => left.cmp(right),
        (PropertyValue::Int(left), PropertyValue::Int(right)) => left.cmp(right),
        (PropertyValue::Float(left), PropertyValue::Float(right)) => left.total_cmp(right),
        (PropertyValue::String(left), PropertyValue::String(right)) => left.cmp(right),
        _ => property_value_rank(left).cmp(&property_value_rank(right)),
    }
}

fn property_value_rank(value: &PropertyValue) -> u8 {
    match value {
        PropertyValue::Bool(_) => 0,
        PropertyValue::Int(_) => 1,
        PropertyValue::Float(_) => 2,
        PropertyValue::String(_) => 3,
    }
}
