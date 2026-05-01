use std::fmt;

use crate::utils::geometry::{Vec3, project_points_onto_plane};

#[derive(Debug, Clone, PartialEq)]
pub enum InteractivityError {
    DimensionOutOfBounds { dim: usize, ndim: usize },
    CoordinateLengthMismatch { start: usize, end: usize },
    DisplayedDimensionCount(usize),
    VectorDimensionMismatch { expected: usize, found: usize },
    ZeroLengthDirection,
}

impl fmt::Display for InteractivityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionOutOfBounds { dim, ndim } => {
                write!(
                    formatter,
                    "displayed dimension {dim} is out of bounds for ndim {ndim}"
                )
            }
            Self::CoordinateLengthMismatch { start, end } => {
                write!(
                    formatter,
                    "start and end coordinates must have the same length, got {start} and {end}"
                )
            }
            Self::DisplayedDimensionCount(count) => {
                write!(
                    formatter,
                    "expected exactly 3 displayed dimensions, got {count}"
                )
            }
            Self::VectorDimensionMismatch { expected, found } => {
                write!(
                    formatter,
                    "expected vector dimension {expected}, got {found}"
                )
            }
            Self::ZeroLengthDirection => formatter.write_str("direction vector cannot be zero"),
        }
    }
}

impl std::error::Error for InteractivityError {}

pub fn displayed_plane_from_nd_line_segment(
    start_point: &[f64],
    end_point: &[f64],
    dims_displayed: &[usize],
) -> Result<(Vec3, Vec3), InteractivityError> {
    let plane_point = select_displayed_vec3(start_point, dims_displayed)?;
    let end_position_view = select_displayed_vec3(end_point, dims_displayed)?;
    let ray_direction = sub3(end_position_view, plane_point);
    Ok((plane_point, normalize3(ray_direction)?))
}

pub fn drag_data_to_projected_distance(
    start_position: Vec3,
    end_position: Vec3,
    view_direction: Vec3,
    vectors: &[Vec3],
) -> Vec<f64> {
    let (projected, _) = project_points_onto_plane(&[end_position], start_position, view_direction);
    let drag_vector_canvas = sub3(projected[0], start_position);
    vectors
        .iter()
        .map(|vector| dot3(drag_vector_canvas, *vector))
        .collect()
}

pub fn nd_line_segment_to_displayed_data_ray(
    start_point: &[f64],
    end_point: &[f64],
    dims_displayed: &[usize],
) -> Result<(Vec3, Vec3), InteractivityError> {
    let mut start_position = select_displayed_vec3(start_point, dims_displayed)?;
    let end_position = select_displayed_vec3(end_point, dims_displayed)?;
    let ray_direction = normalize3(sub3(end_position, start_position))?;
    start_position = sub3(start_position, scale3(ray_direction, 0.1));
    Ok((start_position, ray_direction))
}

fn select_displayed_vec3(
    point: &[f64],
    dims_displayed: &[usize],
) -> Result<Vec3, InteractivityError> {
    if dims_displayed.len() != 3 {
        return Err(InteractivityError::DisplayedDimensionCount(
            dims_displayed.len(),
        ));
    }

    let mut selected = [0.0; 3];
    for (index, &dim) in dims_displayed.iter().enumerate() {
        selected[index] = *point
            .get(dim)
            .ok_or(InteractivityError::DimensionOutOfBounds {
                dim,
                ndim: point.len(),
            })?;
    }
    Ok(selected)
}

fn normalize3(vector: Vec3) -> Result<Vec3, InteractivityError> {
    let norm = dot3(vector, vector).sqrt();
    if norm == 0.0 {
        return Err(InteractivityError::ZeroLengthDirection);
    }
    Ok([vector[0] / norm, vector[1] / norm, vector[2] / norm])
}

fn sub3(left: Vec3, right: Vec3) -> Vec3 {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale3(vector: Vec3, scale: f64) -> Vec3 {
    [vector[0] * scale, vector[1] * scale, vector[2] * scale]
}

fn dot3(left: Vec3, right: Vec3) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}
