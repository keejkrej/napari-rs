use std::fmt;
use std::str::FromStr;

use crate::layers::points::constants::{ParseSymbolError, Symbol};
use crate::utils::geometry::{Vec2, Vec3, project_points_onto_plane};

#[derive(Debug, Clone, PartialEq)]
pub enum RawPointData {
    Empty,
    Points(Vec<Vec<f64>>),
    Single(Vec<f64>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointsUtilsError {
    EmptyData,
    DimensionMismatch { expected: usize, found: usize },
    InvalidPointDimension(usize),
    SizeLengthMismatch { points: usize, sizes: usize },
    ParseSymbol(ParseSymbolError),
}

impl fmt::Display for PointsUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyData => formatter.write_str("point data cannot be empty"),
            Self::DimensionMismatch { expected, found } => {
                write!(formatter, "expected dimension {expected}, got {found}")
            }
            Self::InvalidPointDimension(ndim) => {
                write!(formatter, "point dimension must be 2 or 3, got {ndim}")
            }
            Self::SizeLengthMismatch { points, sizes } => {
                write!(formatter, "got {points} points but {sizes} point sizes")
            }
            Self::ParseSymbol(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for PointsUtilsError {}

pub fn create_box_from_corners_3d(
    box_corners: [Vec3; 2],
    box_normal: Vec3,
    up_vector: Vec3,
) -> [Vec3; 4] {
    let horizontal_vector = cross3(box_normal, up_vector);
    let diagonal_vector = sub3(box_corners[1], box_corners[0]);
    let up_displacement = scale3(up_vector, dot3(diagonal_vector, up_vector));
    let horizontal_displacement =
        scale3(horizontal_vector, dot3(diagonal_vector, horizontal_vector));
    let corner_1 = add3(box_corners[0], horizontal_displacement);
    let corner_3 = add3(box_corners[0], up_displacement);

    [box_corners[0], corner_1, box_corners[1], corner_3]
}

pub fn create_box(data: &[Vec2]) -> Result<[Vec2; 4], PointsUtilsError> {
    let Some(first) = data.first() else {
        return Err(PointsUtilsError::EmptyData);
    };
    let mut min_val = *first;
    let mut max_val = *first;
    for point in data {
        min_val[0] = min_val[0].min(point[0]);
        min_val[1] = min_val[1].min(point[1]);
        max_val[0] = max_val[0].max(point[0]);
        max_val[1] = max_val[1].max(point[1]);
    }

    Ok([
        [min_val[0], min_val[1]],
        [max_val[0], min_val[1]],
        [max_val[0], max_val[1]],
        [min_val[0], max_val[1]],
    ])
}

pub fn points_to_squares(points: &[Vec2], sizes: &[f64]) -> Result<Vec<Vec2>, PointsUtilsError> {
    if points.len() != sizes.len() {
        return Err(PointsUtilsError::SizeLengthMismatch {
            points: points.len(),
            sizes: sizes.len(),
        });
    }

    let mut squares = Vec::with_capacity(points.len() * 4);
    for signs in [[1.0, 1.0], [1.0, -1.0], [-1.0, 1.0], [-1.0, -1.0]] {
        for (&point, &size) in points.iter().zip(sizes) {
            squares.push([
                point[0] + 0.5 * signs[0] * size,
                point[1] + 0.5 * signs[1] * size,
            ]);
        }
    }
    Ok(squares)
}

pub fn points_in_box(
    corners: &[Vec2],
    points: &[Vec2],
    sizes: &[f64],
) -> Result<Vec<usize>, PointsUtilsError> {
    if points.len() != sizes.len() {
        return Err(PointsUtilsError::SizeLengthMismatch {
            points: points.len(),
            sizes: sizes.len(),
        });
    }

    let box_corners = create_box(corners)?;
    let min = box_corners[0];
    let max = box_corners[2];
    Ok(points
        .iter()
        .zip(sizes)
        .enumerate()
        .filter_map(|(index, (&point, &size))| {
            let half = size / 2.0;
            let square = [
                [point[0] + half, point[1] + half],
                [point[0] + half, point[1] - half],
                [point[0] - half, point[1] + half],
                [point[0] - half, point[1] - half],
            ];
            square
                .iter()
                .any(|corner| {
                    corner[0] >= min[0]
                        && corner[0] <= max[0]
                        && corner[1] >= min[1]
                        && corner[1] <= max[1]
                })
                .then_some(index)
        })
        .collect())
}

pub fn points_in_box_3d(
    box_corners: [Vec3; 2],
    points: &[Vec3],
    sizes: &[f64],
    box_normal: Vec3,
    up_direction: Vec3,
) -> Result<Vec<usize>, PointsUtilsError> {
    let bbox_corners = create_box_from_corners_3d(box_corners, box_normal, up_direction);
    let (projected_points, _) = project_points_onto_plane(points, bbox_corners[0], box_normal);
    let horizontal_direction = cross3(box_normal, up_direction);

    let bbox_corners_axis_aligned: Vec<Vec2> = bbox_corners
        .iter()
        .map(|&point| [dot3(point, up_direction), dot3(point, horizontal_direction)])
        .collect();
    let points_axis_aligned: Vec<Vec2> = projected_points
        .iter()
        .map(|&point| [dot3(point, up_direction), dot3(point, horizontal_direction)])
        .collect();

    points_in_box(&bbox_corners_axis_aligned, &points_axis_aligned, sizes)
}

pub fn fix_data_points(
    points: Option<RawPointData>,
    ndim: Option<usize>,
) -> Result<(Vec<Vec<f64>>, usize), PointsUtilsError> {
    match points {
        None | Some(RawPointData::Empty) => {
            let ndim = ndim.unwrap_or(2);
            Ok((Vec::new(), ndim))
        }
        Some(RawPointData::Single(point)) => fix_non_empty_points(vec![point], ndim),
        Some(RawPointData::Points(points)) => {
            if points.is_empty() {
                let ndim = ndim.unwrap_or(2);
                Ok((Vec::new(), ndim))
            } else {
                fix_non_empty_points(points, ndim)
            }
        }
    }
}

pub fn symbol_conversion(symbol: &str) -> Result<Symbol, PointsUtilsError> {
    Symbol::from_str(symbol).map_err(PointsUtilsError::ParseSymbol)
}

pub fn coerce_symbols(symbols: &[&str]) -> Result<Vec<Symbol>, PointsUtilsError> {
    symbols
        .iter()
        .map(|symbol| symbol_conversion(symbol))
        .collect()
}

fn fix_non_empty_points(
    points: Vec<Vec<f64>>,
    ndim: Option<usize>,
) -> Result<(Vec<Vec<f64>>, usize), PointsUtilsError> {
    let data_ndim = points[0].len();
    if matches!(data_ndim, 0 | 1) {
        return Err(PointsUtilsError::InvalidPointDimension(data_ndim));
    }
    for point in &points {
        if point.len() != data_ndim {
            return Err(PointsUtilsError::DimensionMismatch {
                expected: data_ndim,
                found: point.len(),
            });
        }
    }
    if let Some(ndim) = ndim
        && ndim != data_ndim
    {
        return Err(PointsUtilsError::DimensionMismatch {
            expected: ndim,
            found: data_ndim,
        });
    }
    Ok((points, data_ndim))
}

fn add3(left: Vec3, right: Vec3) -> Vec3 {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
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

fn cross3(left: Vec3, right: Vec3) -> Vec3 {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}
