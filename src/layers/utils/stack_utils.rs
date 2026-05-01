use std::error::Error;
use std::fmt;

use crate::layers::multiscale_data::ArrayMetadata;

pub const MAGENTA_GREEN: [&str; 2] = ["magenta", "green"];
pub const CMYBGR: [&str; 6] = ["cyan", "magenta", "yellow", "blue", "green", "red"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackUtilsError {
    EmptyShape,
    AxisOutOfBounds {
        axis: isize,
        ndim: usize,
    },
    ElementOutOfBounds {
        element: usize,
        axis_len: usize,
    },
    MultiscaleAxisOutOfBounds {
        level: usize,
        axis: isize,
        ndim: usize,
    },
    MultiscaleChannelLengthMismatch {
        level: usize,
        expected: usize,
        actual: usize,
    },
    NotEnoughDimensionsForSplit {
        ndim: usize,
    },
    MetadataLengthMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for StackUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShape => formatter.write_str("array shape cannot be empty"),
            Self::AxisOutOfBounds { axis, ndim } => {
                write!(
                    formatter,
                    "axis {axis} is out of bounds for {ndim} dimensions"
                )
            }
            Self::ElementOutOfBounds { element, axis_len } => {
                write!(
                    formatter,
                    "element {element} is out of bounds for axis length {axis_len}"
                )
            }
            Self::MultiscaleAxisOutOfBounds { level, axis, ndim } => write!(
                formatter,
                "axis {axis} is out of bounds for level {level} with {ndim} dimensions"
            ),
            Self::MultiscaleChannelLengthMismatch {
                level,
                expected,
                actual,
            } => write!(
                formatter,
                "level {level} has {actual} channels, expected {expected}"
            ),
            Self::NotEnoughDimensionsForSplit { ndim } => {
                write!(formatter, "image needs more than 2 dimensions for splitting, got {ndim}")
            }
            Self::MetadataLengthMismatch {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "{field} metadata has length {actual}, expected {expected}"
            ),
        }
    }
}

impl Error for StackUtilsError {}

pub fn slice_shape_from_axis(
    shape: &[usize],
    axis: isize,
    element: usize,
) -> Result<Vec<usize>, StackUtilsError> {
    let axis = normalize_axis(axis, shape.len())?;
    let axis_len = shape[axis];
    if element >= axis_len {
        return Err(StackUtilsError::ElementOutOfBounds { element, axis_len });
    }
    let mut sliced = shape.to_vec();
    sliced.remove(axis);
    Ok(sliced)
}

pub fn split_channel_shapes(
    shape: &[usize],
    channel_axis: isize,
) -> Result<Vec<Vec<usize>>, StackUtilsError> {
    let axis = normalize_axis(channel_axis, shape.len())?;
    let n_channels = shape[axis];
    (0..n_channels)
        .map(|channel| slice_shape_from_axis(shape, channel_axis, channel))
        .collect()
}

pub fn split_multiscale_channel_shapes(
    levels: &[ArrayMetadata],
    channel_axis: isize,
) -> Result<Vec<Vec<ArrayMetadata>>, StackUtilsError> {
    if levels.is_empty() {
        return Ok(Vec::new());
    }

    let axis = normalize_axis(channel_axis, levels[0].shape.len())?;
    let n_channels = levels[0].shape[axis];
    let mut channels = vec![Vec::with_capacity(levels.len()); n_channels];

    for (level_index, level) in levels.iter().enumerate() {
        let axis = normalize_axis(channel_axis, level.shape.len()).map_err(|_| {
            StackUtilsError::MultiscaleAxisOutOfBounds {
                level: level_index,
                axis: channel_axis,
                ndim: level.shape.len(),
            }
        })?;
        let level_channels = level.shape[axis];
        if level_channels != n_channels {
            return Err(StackUtilsError::MultiscaleChannelLengthMismatch {
                level: level_index,
                expected: n_channels,
                actual: level_channels,
            });
        }
        for (channel, channel_levels) in channels.iter_mut().enumerate() {
            let shape = slice_shape_from_axis(&level.shape, channel_axis, channel)?;
            channel_levels.push(ArrayMetadata::new(shape, level.dtype));
        }
    }

    Ok(channels)
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackToImagesMetadata {
    pub output_shape: Vec<usize>,
    pub rgb: bool,
    pub scale: Vec<f64>,
    pub translate: Vec<f64>,
}

pub fn stack_to_images_metadata(
    shape: &[usize],
    rgb: bool,
    axis: isize,
    scale: &[f64],
    translate: &[f64],
) -> Result<StackToImagesMetadata, StackUtilsError> {
    let num_dim = if rgb { 3 } else { shape.len() };
    if num_dim < 3 {
        return Err(StackUtilsError::NotEnoughDimensionsForSplit { ndim: num_dim });
    }
    let axis = normalize_axis(axis, num_dim)?;
    validate_metadata_length("scale", scale.len(), num_dim)?;
    validate_metadata_length("translate", translate.len(), num_dim)?;

    let output_shape = slice_shape_from_axis(shape, axis as isize, 0)?;
    let splitting_rgb_channel_axis = rgb && axis == num_dim - 1;
    let output_rgb = rgb && !splitting_rgb_channel_axis;
    let (scale, translate) = if output_rgb || !rgb {
        (remove_axis(scale, axis), remove_axis(translate, axis))
    } else {
        (scale.to_vec(), translate.to_vec())
    };

    Ok(StackToImagesMetadata {
        output_shape,
        rgb: output_rgb,
        scale,
        translate,
    })
}

pub fn default_channel_colormaps(n_channels: usize) -> Vec<&'static str> {
    match n_channels {
        0 => Vec::new(),
        1 => vec!["gray"],
        2 => MAGENTA_GREEN.to_vec(),
        _ => (0..n_channels)
            .map(|index| CMYBGR[index % CMYBGR.len()])
            .collect(),
    }
}

pub fn channel_blending_values(n_channels: usize, blending: Option<&str>) -> Vec<String> {
    if let Some(blending) = blending {
        return vec![blending.to_owned(); n_channels];
    }

    match n_channels {
        0 => Vec::new(),
        1 => vec!["translucent_no_depth".to_owned()],
        _ => std::iter::once("translucent_no_depth".to_owned())
            .chain(std::iter::repeat_n(
                "additive".to_owned(),
                n_channels - 1,
            ))
            .collect(),
    }
}

fn normalize_axis(axis: isize, ndim: usize) -> Result<usize, StackUtilsError> {
    if ndim == 0 {
        return Err(StackUtilsError::EmptyShape);
    }
    let normalized = if axis < 0 { ndim as isize + axis } else { axis };
    if normalized < 0 || normalized >= ndim as isize {
        return Err(StackUtilsError::AxisOutOfBounds { axis, ndim });
    }
    Ok(normalized as usize)
}

fn remove_axis(values: &[f64], axis: usize) -> Vec<f64> {
    let mut output = values.to_vec();
    output.remove(axis);
    output
}

fn validate_metadata_length(
    field: &'static str,
    actual: usize,
    expected: usize,
) -> Result<(), StackUtilsError> {
    if actual == expected {
        Ok(())
    } else {
        Err(StackUtilsError::MetadataLengthMismatch {
            field,
            expected,
            actual,
        })
    }
}
