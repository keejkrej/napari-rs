use std::f64::consts::TAU;
use std::fmt;

use crate::layers::shapes::constants::ShapeType;
use crate::utils::geometry::Vec2;

pub type Triangle2 = [Vec2; 3];

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeData {
    Empty,
    Single(Vec<Vec<f64>>),
    Many(Vec<Vec<Vec<f64>>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeTypeMetadata {
    None,
    Single(ShapeType),
    Many(Vec<ShapeType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeInput {
    Plain(ShapeData),
    Typed(ShapeData, ShapeTypeMetadata),
    ManyTyped(Vec<(Vec<Vec<f64>>, ShapeType)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedShapeType {
    pub data: ShapeData,
    pub shape_type: ShapeTypeMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlanarPoints {
    Points2D(Vec<Vec2>),
    Points3D(Vec<[f64; 3]>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanarAxis {
    pub points: Vec<Vec2>,
    pub axis: Option<usize>,
    pub value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointToLine {
    pub index: usize,
    pub location: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EllipseTriangulation {
    pub vertices: Vec<Vec<f64>>,
    pub triangles: Vec<[usize; 3]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapesUtilsError {
    EmptyData,
    RectangleCornerCount(usize),
    NoLines,
    DimensionMismatch {
        expected: usize,
        found: usize,
    },
    InvalidVertexCount {
        shape_type: ShapeType,
        vertices: usize,
    },
    InvalidCornerShape {
        rows: usize,
        columns: Option<usize>,
    },
    InvalidAxis {
        axis: usize,
        ndim: usize,
    },
    InvalidTriangleIndex {
        index: usize,
        vertices: usize,
    },
}

impl fmt::Display for ShapesUtilsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyData => formatter.write_str("shape data cannot be empty"),
            Self::RectangleCornerCount(count) => {
                write!(formatter, "rectangle requires four corners, got {count}")
            }
            Self::NoLines => formatter.write_str("at least one line segment is required"),
            Self::DimensionMismatch { expected, found } => {
                write!(formatter, "expected dimension {expected}, got {found}")
            }
            Self::InvalidVertexCount {
                shape_type,
                vertices,
            } => write!(
                formatter,
                "{shape_type} has invalid number of vertices: {vertices}"
            ),
            Self::InvalidCornerShape { rows, columns } => {
                write!(
                    formatter,
                    "ellipse corners must have shape [4, 2] or [4, 3], got [{rows}, "
                )?;
                match columns {
                    Some(columns) => write!(formatter, "{columns}]"),
                    None => formatter.write_str("?]"),
                }
            }
            Self::InvalidAxis { axis, ndim } => {
                write!(formatter, "axis {axis} is out of bounds for ndim {ndim}")
            }
            Self::InvalidTriangleIndex { index, vertices } => write!(
                formatter,
                "triangle index {index} is out of bounds for {vertices} vertices"
            ),
        }
    }
}

impl std::error::Error for ShapesUtilsError {}

pub fn find_planar_axis(points: PlanarPoints) -> PlanarAxis {
    match points {
        PlanarPoints::Points2D(points) => PlanarAxis {
            points,
            axis: None,
            value: None,
        },
        PlanarPoints::Points3D(points) => {
            for axis in 0..3 {
                if let Some(first) = points.first()
                    && points.iter().all(|point| point[axis] == first[axis])
                {
                    return PlanarAxis {
                        points: points
                            .iter()
                            .map(|point| match axis {
                                0 => [point[1], point[2]],
                                1 => [point[0], point[2]],
                                _ => [point[0], point[1]],
                            })
                            .collect(),
                        axis: Some(axis),
                        value: Some(first[axis]),
                    };
                }
            }
            PlanarAxis {
                points: Vec::new(),
                axis: None,
                value: None,
            }
        }
    }
}

pub fn fan_triangulation<T: Clone>(poly: &[T]) -> (Vec<T>, Vec<[usize; 3]>) {
    let triangles = if poly.len() < 3 {
        Vec::new()
    } else {
        (1..poly.len() - 1)
            .map(|index| [0, index, index + 1])
            .collect()
    };
    (poly.to_vec(), triangles)
}

pub fn inside_boxes(boxes: &[[Vec2; 8]]) -> Vec<bool> {
    boxes
        .iter()
        .map(|box_| {
            let ab = sub2(box_[0], box_[6]);
            let am = box_[0];
            let bc = sub2(box_[6], box_[4]);
            let bm = box_[6];
            let abam = dot2(ab, am);
            let abab = dot2(ab, ab);
            let bcbm = dot2(bc, bm);
            let bcbc = dot2(bc, bc);
            abam >= 0.0 && abam <= abab && bcbm >= 0.0 && bcbm <= bcbc
        })
        .collect()
}

pub fn triangles_intersect_box(triangles: &[Triangle2], corners: &[Vec2]) -> Vec<bool> {
    let vertices_inside = triangle_vertices_inside_box(triangles, corners);
    let edge_intersects = triangle_edges_intersect_box(triangles, corners);
    vertices_inside
        .iter()
        .zip(edge_intersects)
        .map(|(&vertices_inside, edge_intersects)| vertices_inside || edge_intersects)
        .collect()
}

pub fn triangle_vertices_inside_box(triangles: &[Triangle2], corners: &[Vec2]) -> Vec<bool> {
    let Ok(box_) = create_box(corners) else {
        return vec![false; triangles.len()];
    };
    let min = box_[0];
    let max = box_[4];
    triangles
        .iter()
        .map(|triangle| {
            triangle.iter().any(|vertex| {
                vertex[0] >= min[0]
                    && vertex[0] <= max[0]
                    && vertex[1] >= min[1]
                    && vertex[1] <= max[1]
            })
        })
        .collect()
}

pub fn triangle_edges_intersect_box(triangles: &[Triangle2], corners: &[Vec2]) -> Vec<bool> {
    let Ok(box_) = create_box(corners) else {
        return vec![false; triangles.len()];
    };
    let box_corners = [box_[0], box_[2], box_[4], box_[6]];
    triangles
        .iter()
        .map(|triangle| {
            (0..3).any(|triangle_index| {
                let p1 = triangle[triangle_index];
                let q1 = triangle[(triangle_index + 1) % 3];
                (0..4).any(|box_index| {
                    lines_intersect(
                        p1,
                        q1,
                        box_corners[box_index],
                        box_corners[(box_index + 1) % 4],
                    )
                })
            })
        })
        .collect()
}

pub fn lines_intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> bool {
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    (o1 != o2 && o3 != o4)
        || (o1 == 0 && on_segment(p1, p2, q1))
        || (o2 == 0 && on_segment(p1, q2, q1))
        || (o3 == 0 && on_segment(p2, p1, q2))
        || (o4 == 0 && on_segment(p2, q1, q2))
}

pub fn vectorized_lines_intersect(p1: &[Vec2], q1: &[Vec2], p2: Vec2, q2: Vec2) -> Vec<bool> {
    p1.iter()
        .zip(q1)
        .map(|(&p1, &q1)| lines_intersect(p1, q1, p2, q2))
        .collect()
}

pub fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
    q[0] <= p[0].max(r[0])
        && q[0] >= p[0].min(r[0])
        && q[1] <= p[1].max(r[1])
        && q[1] >= p[1].min(r[1])
}

pub fn is_collinear(points: &[Vec2]) -> bool {
    points.len() < 3
        || points[2..]
            .iter()
            .all(|&point| orientation(points[0], points[1], point) == 0)
}

pub fn point_to_lines(point: Vec2, lines: &[[Vec2; 2]]) -> Result<PointToLine, ShapesUtilsError> {
    if lines.is_empty() {
        return Err(ShapesUtilsError::NoLines);
    }
    let mut closest_index = 0;
    let mut closest_location = 0.0;
    let mut closest_distance = f64::INFINITY;

    for (index, &[start, end]) in lines.iter().enumerate() {
        let line_vector = sub2(end, start);
        let point_vector = sub2(point, start);
        let end_point_vector = sub2(point, end);
        let mut norm_line = norm2(line_vector);
        let reject = norm_line == 0.0;
        if reject {
            norm_line = 1.0;
        }
        let unit_line = [line_vector[0] / norm_line, line_vector[1] / norm_line];
        let mut line_dist = (unit_line[0] * point_vector[1] - unit_line[1] * point_vector[0]).abs();
        let mut line_loc = dot2(unit_line, point_vector) / norm_line;

        if line_loc < 0.0 {
            line_dist = norm2(point_vector);
        } else if line_loc > 1.0 {
            line_dist = norm2(end_point_vector);
        }
        if reject {
            line_dist = norm2(point_vector);
            line_loc = 0.5;
        }

        if line_dist < closest_distance {
            closest_distance = line_dist;
            closest_index = index;
            closest_location = line_loc;
        }
    }

    Ok(PointToLine {
        index: closest_index,
        location: closest_location,
    })
}

pub fn create_box(data: &[Vec2]) -> Result<[Vec2; 9], ShapesUtilsError> {
    let Some(first) = data.first() else {
        return Err(ShapesUtilsError::EmptyData);
    };
    let mut min = *first;
    let mut max = *first;
    for &point in data {
        min[0] = min[0].min(point[0]);
        min[1] = min[1].min(point[1]);
        max[0] = max[0].max(point[0]);
        max[1] = max[1].max(point[1]);
    }
    let tl = [min[0], min[1]];
    let tr = [max[0], min[1]];
    let br = [max[0], max[1]];
    let bl = [min[0], max[1]];
    Ok([
        tl,
        midpoint2(tl, tr),
        tr,
        midpoint2(tr, br),
        br,
        midpoint2(br, bl),
        bl,
        midpoint2(bl, tl),
        [
            (tl[0] + tr[0] + br[0] + bl[0]) / 4.0,
            (tl[1] + tr[1] + br[1] + bl[1]) / 4.0,
        ],
    ])
}

pub fn rectangle_to_box(data: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, ShapesUtilsError> {
    if data.len() != 4 {
        return Err(ShapesUtilsError::RectangleCornerCount(data.len()));
    }
    Ok(vec![
        data[0].clone(),
        midpoint(&data[0], &data[1]),
        data[1].clone(),
        midpoint(&data[1], &data[2]),
        data[2].clone(),
        midpoint(&data[2], &data[3]),
        data[3].clone(),
        midpoint(&data[3], &data[0]),
        mean(data),
    ])
}

pub fn find_corners(data: &[Vec2]) -> Result<[Vec2; 4], ShapesUtilsError> {
    let Some(first) = data.first() else {
        return Err(ShapesUtilsError::EmptyData);
    };
    let mut min = *first;
    let mut max = *first;
    for &point in data {
        min[0] = min[0].min(point[0]);
        min[1] = min[1].min(point[1]);
        max[0] = max[0].max(point[0]);
        max[1] = max[1].max(point[1]);
    }
    Ok([
        [min[0], min[1]],
        [max[0], min[1]],
        [max[0], max[1]],
        [min[0], max[1]],
    ])
}

pub fn center_radii_to_corners(center: Vec2, radii: Vec2) -> [Vec2; 4] {
    find_corners(&[
        [center[0] + radii[0], center[1] + radii[1]],
        [center[0] - radii[0], center[1] - radii[1]],
    ])
    .expect("generated corners are non-empty")
}

pub fn get_default_shape_type(current_type: &[ShapeType]) -> ShapeType {
    let Some(&first) = current_type.first() else {
        return ShapeType::Polygon;
    };
    if current_type.iter().all(|&shape_type| shape_type == first) {
        first
    } else {
        ShapeType::Polygon
    }
}

pub fn extract_shape_type(data: ShapeInput, shape_type: ShapeTypeMetadata) -> ExtractedShapeType {
    match data {
        ShapeInput::Plain(data) => ExtractedShapeType { data, shape_type },
        ShapeInput::Typed(data, shape_type) => ExtractedShapeType { data, shape_type },
        ShapeInput::ManyTyped(items) => {
            let mut data = Vec::with_capacity(items.len());
            let mut shape_type = Vec::with_capacity(items.len());
            for (shape, item_shape_type) in items {
                data.push(shape);
                shape_type.push(item_shape_type);
            }
            ExtractedShapeType {
                data: if data.is_empty() {
                    ShapeData::Empty
                } else {
                    ShapeData::Many(data)
                },
                shape_type: ShapeTypeMetadata::Many(shape_type),
            }
        }
    }
}

pub fn get_shape_ndim(data: &ShapeData) -> Result<usize, ShapesUtilsError> {
    match data {
        ShapeData::Empty => Err(ShapesUtilsError::EmptyData),
        ShapeData::Single(shape) => shape
            .first()
            .map(Vec::len)
            .ok_or(ShapesUtilsError::EmptyData),
        ShapeData::Many(shapes) => shapes
            .first()
            .and_then(|shape| shape.first())
            .map(Vec::len)
            .ok_or(ShapesUtilsError::EmptyData),
    }
}

pub fn number_of_shapes(data: &ShapeData) -> usize {
    match data {
        ShapeData::Empty => 0,
        ShapeData::Single(_) => 1,
        ShapeData::Many(shapes) => shapes.len(),
    }
}

pub fn validate_num_vertices(
    data: &ShapeData,
    shape_type: ShapeType,
    min_vertices: Option<usize>,
    valid_vertices: Option<&[usize]>,
) -> Result<(), ShapesUtilsError> {
    let is_invalid = |shape: &[Vec<f64>]| {
        valid_vertices.is_some_and(|valid_vertices| !valid_vertices.contains(&shape.len()))
            || min_vertices.is_some_and(|min_vertices| shape.len() < min_vertices)
    };

    match data {
        ShapeData::Empty => Ok(()),
        ShapeData::Single(shape) => {
            if is_invalid(shape) {
                Err(ShapesUtilsError::InvalidVertexCount {
                    shape_type,
                    vertices: shape.len(),
                })
            } else {
                Ok(())
            }
        }
        ShapeData::Many(shapes) => {
            for shape in shapes {
                if is_invalid(shape) {
                    return Err(ShapesUtilsError::InvalidVertexCount {
                        shape_type,
                        vertices: shape.len(),
                    });
                }
            }
            Ok(())
        }
    }
}

pub fn perpendicular_distance(
    point: &[f64],
    line_start: &[f64],
    line_end: &[f64],
) -> Result<f64, ShapesUtilsError> {
    let ndim = point.len();
    if line_start.len() != ndim {
        return Err(ShapesUtilsError::DimensionMismatch {
            expected: ndim,
            found: line_start.len(),
        });
    }
    if line_end.len() != ndim {
        return Err(ShapesUtilsError::DimensionMismatch {
            expected: ndim,
            found: line_end.len(),
        });
    }

    if line_start == line_end {
        return Ok(norm(
            &point
                .iter()
                .zip(line_start)
                .map(|(&point, &start)| point - start)
                .collect::<Vec<_>>(),
        ));
    }

    let start_minus_end: Vec<_> = line_start
        .iter()
        .zip(line_end)
        .map(|(&start, &end)| start - end)
        .collect();
    let point_minus_end: Vec<_> = point
        .iter()
        .zip(line_end)
        .map(|(&point, &end)| point - end)
        .collect();
    let t = dot(&point_minus_end, &start_minus_end) / dot(&start_minus_end, &start_minus_end);

    Ok(norm(
        &start_minus_end
            .iter()
            .zip(line_end)
            .zip(point)
            .map(|((&start_minus_end, &end), &point)| t * start_minus_end + end - point)
            .collect::<Vec<_>>(),
    ))
}

pub fn rdp(vertices: &[Vec<f64>], epsilon: f64) -> Result<Vec<Vec<f64>>, ShapesUtilsError> {
    if vertices.len() < 3 || epsilon == 0.0 {
        return Ok(vertices.to_vec());
    }

    let first = &vertices[0];
    let last = &vertices[vertices.len() - 1];
    let mut max_distance = 0.0;
    let mut max_index = 0;

    for (index, vertex) in vertices[1..vertices.len() - 1].iter().enumerate() {
        let distance = perpendicular_distance(vertex, first, last)?;
        if distance > max_distance {
            max_distance = distance;
            max_index = index + 1;
        }
    }

    if max_distance > epsilon {
        let left = rdp(&vertices[..=max_index], epsilon)?;
        let right = rdp(&vertices[max_index..], epsilon)?;
        Ok(left[..left.len() - 1]
            .iter()
            .cloned()
            .chain(right)
            .collect())
    } else {
        Ok(vec![first.clone(), last.clone()])
    }
}

pub fn points_in_poly(points: &[Vec2], vertices: &[Vec2]) -> Vec<bool> {
    let mut inside = vec![false; points.len()];
    if vertices.is_empty() {
        return inside;
    }

    let mut previous = vertices.len() - 1;
    for current in 0..vertices.len() {
        let current_vertex = vertices[current];
        let previous_vertex = vertices[previous];
        let mut delta = sub2(previous_vertex, current_vertex);
        for value in &mut delta {
            if value.abs() < 1e-12 {
                *value = 0.0;
            }
        }

        for (point_index, &point) in points.iter().enumerate() {
            let cond_1 = current_vertex[1] <= point[1] && point[1] < previous_vertex[1];
            let cond_2 = previous_vertex[1] <= point[1] && point[1] < current_vertex[1];
            let cond_3 = cond_1 || cond_2;
            let cond_4 = if delta[1] == 0.0 {
                delta[0] * (point[1] - current_vertex[1]) > 0.0
            } else {
                point[0] < delta[0] * (point[1] - current_vertex[1]) / delta[1] + current_vertex[0]
            };
            if cond_3 && cond_4 {
                inside[point_index] = !inside[point_index];
            }
        }
        previous = current;
    }

    inside
}

pub fn grid_points_in_poly(shape: [usize; 2], vertices: &[Vec2]) -> Vec<Vec<bool>> {
    let points: Vec<_> = (0..shape[0])
        .flat_map(|x| (0..shape[1]).map(move |y| [x as f64, y as f64]))
        .collect();
    points_in_poly(&points, vertices)
        .chunks(shape[1])
        .map(|row| row.to_vec())
        .collect()
}

pub fn path_to_mask(mask_shape: [usize; 2], vertices: &[Vec2]) -> Vec<Vec<bool>> {
    let mut mask = vec![vec![false; mask_shape[1]]; mask_shape[0]];
    if mask_shape[0] == 0 || mask_shape[1] == 0 || vertices.len() < 2 {
        return mask;
    }

    let mut rounded = Vec::with_capacity(vertices.len());
    for &vertex in vertices {
        let clipped = clip_round_vertex(vertex, mask_shape);
        if rounded.last() != Some(&clipped) {
            rounded.push(clipped);
        }
    }

    for window in rounded.windows(2) {
        for [row, col] in bresenham_line(window[0], window[1]) {
            mask[row][col] = true;
        }
    }

    mask
}

pub fn triangulate_ellipse(
    corners: &[Vec<f64>],
    num_segments: usize,
) -> Result<EllipseTriangulation, ShapesUtilsError> {
    let ndim = corners.first().map(Vec::len);
    if corners.len() != 4 || !matches!(ndim, Some(2 | 3)) {
        return Err(ShapesUtilsError::InvalidCornerShape {
            rows: corners.len(),
            columns: ndim,
        });
    }
    let ndim = ndim.expect("validated corner dimensionality");
    for corner in corners {
        if corner.len() != ndim {
            return Err(ShapesUtilsError::InvalidCornerShape {
                rows: corners.len(),
                columns: Some(corner.len()),
            });
        }
    }

    let center = mean(corners);
    let adjusted: Vec<_> = corners
        .iter()
        .map(|corner| {
            corner
                .iter()
                .zip(&center)
                .map(|(&value, &center)| value - center)
                .collect::<Vec<_>>()
        })
        .collect();
    let ax1 = half_difference(&adjusted[1], &adjusted[0]);
    let ax2 = half_difference(&adjusted[2], &adjusted[1]);

    let mut vertices = Vec::with_capacity(num_segments + 1);
    vertices.push(center.clone());
    for index in 0..num_segments {
        let theta = if num_segments <= 1 {
            0.0
        } else {
            index as f64 * TAU / (num_segments - 1) as f64
        };
        vertices.push(
            center
                .iter()
                .zip(&ax1)
                .zip(&ax2)
                .map(|((&center, &ax1), &ax2)| center + theta.cos() * ax1 + theta.sin() * ax2)
                .collect(),
        );
    }

    let mut triangles: Vec<_> = (0..num_segments)
        .map(|index| [0, index + 1, index + 2])
        .collect();
    if let Some(last) = triangles.last_mut() {
        last[2] = 1;
    }

    Ok(EllipseTriangulation {
        vertices,
        triangles,
    })
}

pub fn cull_triangles_not_in_poly(
    vertices: &[Vec2],
    triangles: &[[usize; 3]],
    poly: &[Vec2],
) -> Result<Vec<[usize; 3]>, ShapesUtilsError> {
    let mut centers = Vec::with_capacity(triangles.len());
    for triangle in triangles {
        let mut center = [0.0, 0.0];
        for &index in triangle {
            let Some(vertex) = vertices.get(index) else {
                return Err(ShapesUtilsError::InvalidTriangleIndex {
                    index,
                    vertices: vertices.len(),
                });
            };
            center[0] += vertex[0];
            center[1] += vertex[1];
        }
        centers.push([center[0] / 3.0, center[1] / 3.0]);
    }

    Ok(triangles
        .iter()
        .zip(points_in_poly(&centers, poly))
        .filter_map(|(&triangle, inside)| inside.then_some(triangle))
        .collect())
}

pub fn fix_vertices_if_needed(
    vertices: &[Vec2],
    axis: Option<usize>,
    value: Option<f64>,
) -> Result<Vec<Vec<f64>>, ShapesUtilsError> {
    let Some(axis) = axis else {
        return Ok(vertices
            .iter()
            .map(|vertex| vec![vertex[0], vertex[1]])
            .collect());
    };
    let Some(value) = value else {
        return Ok(vertices
            .iter()
            .map(|vertex| vec![vertex[0], vertex[1]])
            .collect());
    };
    if axis > 2 {
        return Err(ShapesUtilsError::InvalidAxis { axis, ndim: 3 });
    }

    Ok(vertices
        .iter()
        .map(|vertex| match axis {
            0 => vec![value, vertex[0], vertex[1]],
            1 => vec![vertex[0], value, vertex[1]],
            _ => vec![vertex[0], vertex[1], value],
        })
        .collect())
}

fn orientation(p: Vec2, q: Vec2, r: Vec2) -> i8 {
    let value = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1]);
    if value == 0.0 {
        0
    } else if value > 0.0 {
        1
    } else {
        2
    }
}

fn sub2(left: Vec2, right: Vec2) -> Vec2 {
    [left[0] - right[0], left[1] - right[1]]
}

fn dot2(left: Vec2, right: Vec2) -> f64 {
    left[0] * right[0] + left[1] * right[1]
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(&left, &right)| left * right)
        .sum()
}

fn norm2(vector: Vec2) -> f64 {
    dot2(vector, vector).sqrt()
}

fn norm(vector: &[f64]) -> f64 {
    dot(vector, vector).sqrt()
}

fn midpoint2(left: Vec2, right: Vec2) -> Vec2 {
    [(left[0] + right[0]) / 2.0, (left[1] + right[1]) / 2.0]
}

fn midpoint(left: &[f64], right: &[f64]) -> Vec<f64> {
    left.iter()
        .zip(right)
        .map(|(&left, &right)| (left + right) / 2.0)
        .collect()
}

fn half_difference(left: &[f64], right: &[f64]) -> Vec<f64> {
    left.iter()
        .zip(right)
        .map(|(&left, &right)| (left - right) / 2.0)
        .collect()
}

fn mean(data: &[Vec<f64>]) -> Vec<f64> {
    let ndim = data[0].len();
    let mut out = vec![0.0; ndim];
    for point in data {
        for (axis, value) in point.iter().enumerate() {
            out[axis] += value;
        }
    }
    for value in &mut out {
        *value /= data.len() as f64;
    }
    out
}

fn clip_round_vertex(vertex: Vec2, mask_shape: [usize; 2]) -> [usize; 2] {
    [
        clip_round_axis(vertex[0], mask_shape[0]),
        clip_round_axis(vertex[1], mask_shape[1]),
    ]
}

fn clip_round_axis(value: f64, size: usize) -> usize {
    value
        .clamp(0.0, size.saturating_sub(1) as f64)
        .round_ties_even() as usize
}

fn bresenham_line(start: [usize; 2], end: [usize; 2]) -> Vec<[usize; 2]> {
    let [mut row, mut col] = [start[0] as isize, start[1] as isize];
    let [end_row, end_col] = [end[0] as isize, end[1] as isize];
    let delta_row = (end_row - row).abs();
    let delta_col = -(end_col - col).abs();
    let step_row = if row < end_row { 1 } else { -1 };
    let step_col = if col < end_col { 1 } else { -1 };
    let mut error = delta_row + delta_col;
    let mut points = Vec::new();

    loop {
        points.push([row as usize, col as usize]);
        if row == end_row && col == end_col {
            break;
        }
        let doubled_error = 2 * error;
        if doubled_error >= delta_col {
            error += delta_col;
            row += step_row;
        }
        if doubled_error <= delta_row {
            error += delta_row;
            col += step_col;
        }
    }

    points
}
