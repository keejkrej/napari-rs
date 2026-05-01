use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceRange {
    pub start: isize,
    pub stop: isize,
    pub step: isize,
}

impl SliceRange {
    pub fn new(start: isize, stop: isize, step: isize) -> Self {
        Self { start, stop, step }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseLabels {
    shape: Vec<usize>,
    values: Vec<i64>,
}

impl DenseLabels {
    pub fn new(
        shape: impl Into<Vec<usize>>,
        values: impl Into<Vec<i64>>,
    ) -> Result<Self, LabelsUtilsError> {
        let shape = shape.into();
        let values = values.into();
        let expected_len = shape.iter().product();
        if values.len() != expected_len {
            return Err(LabelsUtilsError::DataLengthMismatch {
                expected: expected_len,
                found: values.len(),
            });
        }
        Ok(Self { shape, values })
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn get(&self, coord: &[usize]) -> Option<i64> {
        if coord.len() != self.shape.len() || coord.iter().zip(&self.shape).any(|(&c, &s)| c >= s) {
            return None;
        }
        let mut flat_index = 0;
        for (&axis_coord, &axis_len) in coord.iter().zip(&self.shape) {
            flat_index = flat_index * axis_len + axis_coord;
        }
        self.values.get(flat_index).copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelsUtilsError {
    CoordinateLengthMismatch { old: usize, new: usize },
    BrushSizeNotPositive,
    ScaleContainsZero,
    DataLengthMismatch { expected: usize, found: usize },
    PointDimensionMismatch { point: usize, ndim: usize },
}

impl fmt::Display for LabelsUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinateLengthMismatch { old, new } => {
                write!(
                    formatter,
                    "coordinate lengths must match, got {old} and {new}"
                )
            }
            Self::BrushSizeNotPositive => formatter.write_str("brush size must be positive"),
            Self::ScaleContainsZero => formatter.write_str("scale values must be non-zero"),
            Self::DataLengthMismatch { expected, found } => {
                write!(
                    formatter,
                    "label data length must be {expected}, got {found}"
                )
            }
            Self::PointDimensionMismatch { point, ndim } => {
                write!(formatter, "point dimension must be {ndim}, got {point}")
            }
        }
    }
}

impl std::error::Error for LabelsUtilsError {}

pub fn interpolate_coordinates(
    old_coord: Option<&[f64]>,
    new_coord: Option<&[f64]>,
    brush_size: f64,
) -> Result<Vec<Vec<f64>>, LabelsUtilsError> {
    if brush_size <= 0.0 {
        return Err(LabelsUtilsError::BrushSizeNotPositive);
    }
    let old_coord = old_coord.or(new_coord).unwrap_or(&[]);
    let new_coord = new_coord.unwrap_or(old_coord);
    if old_coord.len() != new_coord.len() {
        return Err(LabelsUtilsError::CoordinateLengthMismatch {
            old: old_coord.len(),
            new: new_coord.len(),
        });
    }

    let max_distance = old_coord
        .iter()
        .zip(new_coord)
        .map(|(&old, &new)| (new - old).abs())
        .fold(0.0, f64::max);
    let num_step = (max_distance / brush_size * 4.0).round() as usize;
    let mut coords = Vec::with_capacity(num_step + 1);
    for step in 0..=num_step {
        let fraction = if num_step == 0 {
            0.0
        } else {
            step as f64 / num_step as f64
        };
        coords.push(
            old_coord
                .iter()
                .zip(new_coord)
                .map(|(&old, &new)| old + (new - old) * fraction)
                .collect(),
        );
    }
    if coords.len() > 1 {
        coords.remove(0);
    }
    Ok(coords)
}

pub fn sphere_indices(radius: f64, scale: &[f64]) -> Result<Vec<Vec<isize>>, LabelsUtilsError> {
    if scale.contains(&0.0) {
        return Err(LabelsUtilsError::ScaleContainsZero);
    }
    let min_abs_scale = scale
        .iter()
        .map(|value| value.abs())
        .fold(f64::INFINITY, f64::min);
    let scale_normalized: Vec<f64> = scale
        .iter()
        .map(|value| value.abs() / min_abs_scale)
        .collect();
    let ranges: Vec<Vec<isize>> = scale_normalized
        .iter()
        .map(|scale| {
            let r = radius / scale + 0.5;
            let start = -(r.ceil() as isize);
            let stop = r.floor() as isize + 1;
            (start..stop).collect()
        })
        .collect();

    let mut indices = Vec::new();
    build_sphere_indices(
        &ranges,
        &scale_normalized,
        radius * radius,
        &mut Vec::new(),
        &mut indices,
    );
    Ok(indices)
}

pub fn indices_in_shape(idxs: &[Vec<isize>], shape: &[usize]) -> Vec<Vec<isize>> {
    idxs.iter()
        .filter(|coord| {
            coord.len() == shape.len()
                && coord
                    .iter()
                    .zip(shape)
                    .all(|(&index, &axis_len)| index >= 0 && (index as usize) < axis_len)
        })
        .cloned()
        .collect()
}

pub fn first_nonzero_coordinate(
    data: &DenseLabels,
    start_point: &[f64],
    end_point: &[f64],
) -> Result<Option<Vec<usize>>, LabelsUtilsError> {
    let ndim = data.shape.len();
    if start_point.len() != ndim {
        return Err(LabelsUtilsError::PointDimensionMismatch {
            point: start_point.len(),
            ndim,
        });
    }
    if end_point.len() != ndim {
        return Err(LabelsUtilsError::PointDimensionMismatch {
            point: end_point.len(),
            ndim,
        });
    }

    let length = start_point
        .iter()
        .zip(end_point)
        .map(|(&start, &end)| (end - start).powi(2))
        .sum::<f64>()
        .sqrt();
    let length_int = length.round() as usize;
    for step in 0..=length_int {
        let fraction = if length_int == 0 {
            0.0
        } else {
            step as f64 / length_int as f64
        };
        let coord: Vec<usize> = start_point
            .iter()
            .zip(end_point)
            .zip(data.shape())
            .map(|((&start, &end), &axis_len)| {
                let rounded = (start + (end - start) * fraction).round();
                rounded.clamp(0.0, axis_len.saturating_sub(1) as f64) as usize
            })
            .collect();
        if data.get(&coord).unwrap_or(0) != 0 {
            return Ok(Some(coord));
        }
    }
    Ok(None)
}

pub fn expand_slice(axes_slice: &[SliceRange], shape: &[usize], offset: isize) -> Vec<SliceRange> {
    axes_slice
        .iter()
        .zip(shape)
        .map(|(slice, &max_size)| {
            let max_size = max_size as isize;
            SliceRange::new(
                (slice.start - offset).clamp(0, max_size),
                (slice.stop + offset).clamp(0, max_size),
                slice.step,
            )
        })
        .collect()
}

fn build_sphere_indices(
    ranges: &[Vec<isize>],
    scale_normalized: &[f64],
    radius_sq: f64,
    current: &mut Vec<isize>,
    out: &mut Vec<Vec<isize>>,
) {
    if current.len() == ranges.len() {
        let distance_sq = current
            .iter()
            .zip(scale_normalized)
            .map(|(&index, &scale)| (index as f64 * scale).powi(2))
            .sum::<f64>();
        if distance_sq <= radius_sq {
            out.push(current.clone());
        }
        return;
    }

    let axis = current.len();
    for &index in &ranges[axis] {
        current.push(index);
        build_sphere_indices(ranges, scale_normalized, radius_sq, current, out);
        current.pop();
    }
}
