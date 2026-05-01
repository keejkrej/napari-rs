use crate::layers::base::constants::InteractionBoxHandle;

pub type Point2 = [f64; 2];
pub type Bounds2 = (Point2, Point2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionBoxError {
    InvalidPointDimension,
}

impl std::fmt::Display for InteractionBoxError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPointDimension => formatter.write_str("only 2D coordinates are accepted"),
        }
    }
}

impl std::error::Error for InteractionBoxError {}

pub fn generate_interaction_box_vertices(
    top_left: Point2,
    bottom_right: Point2,
    handles: bool,
    rotation: bool,
) -> Vec<Point2> {
    let [x0, y0] = top_left;
    let [x1, y1] = bottom_right;
    let mut vertices = vec![[x0, y0], [x0, y1], [x1, y0], [x1, y1]];

    if handles {
        let middle_vertices = [
            midpoint(vertices[0], vertices[2]),
            midpoint(vertices[1], vertices[0]),
            midpoint(vertices[2], vertices[3]),
            midpoint(vertices[3], vertices[1]),
        ];
        let box_height = vertices[0][1] - vertices[1][1];
        vertices.extend(middle_vertices);

        if rotation {
            vertices.push([
                middle_vertices[0][0],
                middle_vertices[0][1] + box_height * 0.1,
            ]);
        }
    }

    vertices
}

pub fn calculate_bounds_from_contained_points(points: &[Point2]) -> Option<Bounds2> {
    if points.is_empty() {
        return None;
    }

    let x0 = points
        .iter()
        .map(|point| point[0])
        .fold(f64::INFINITY, f64::min);
    let x1 = points
        .iter()
        .map(|point| point[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let y0 = points
        .iter()
        .map(|point| point[1])
        .fold(f64::INFINITY, f64::min);
    let y1 = points
        .iter()
        .map(|point| point[1])
        .fold(f64::NEG_INFINITY, f64::max);

    Some(([x0, x1], [y0, y1]))
}

pub fn calculate_bounds_from_dynamic_points(
    points: &[Vec<f64>],
) -> Result<Option<Bounds2>, InteractionBoxError> {
    if points.is_empty() {
        return Ok(None);
    }
    let mut typed_points = Vec::with_capacity(points.len());
    for point in points {
        if point.len() != 2 {
            return Err(InteractionBoxError::InvalidPointDimension);
        }
        typed_points.push([point[0], point[1]]);
    }
    Ok(calculate_bounds_from_contained_points(&typed_points))
}

pub fn get_nearby_handle(
    position: Point2,
    handle_coordinates: &[Point2],
) -> Option<InteractionBoxHandle> {
    let top_left = handle_coordinates[InteractionBoxHandle::TopLeft.index()];
    let bottom_right = handle_coordinates[InteractionBoxHandle::BottomRight.index()];
    let distances: Vec<f64> = handle_coordinates
        .iter()
        .map(|&coordinate| distance(position, coordinate))
        .collect();
    let tolerance = distances.iter().copied().fold(0.0_f64, f64::max) / 100.0;

    if let Some(index) = distances.iter().position(|&distance| distance <= tolerance) {
        return InteractionBoxHandle::from_index(index);
    }

    if position[0] >= top_left[0]
        && position[0] <= bottom_right[0]
        && position[1] >= top_left[1]
        && position[1] <= bottom_right[1]
    {
        return Some(InteractionBoxHandle::Inside);
    }

    None
}

fn midpoint(left: Point2, right: Point2) -> Point2 {
    [(left[0] + right[0]) / 2.0, (left[1] + right[1]) / 2.0]
}

fn distance(left: Point2, right: Point2) -> f64 {
    let dx = left[0] - right[0];
    let dy = left[1] - right[1];
    (dx * dx + dy * dy).sqrt()
}
