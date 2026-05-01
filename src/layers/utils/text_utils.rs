use std::fmt;

use crate::layers::utils::text_constants::Anchor;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewData {
    Coordinates(Vec<Vec<f64>>),
    Vertices(Vec<Vec<Vec<f64>>>),
    Items(Vec<Vec<Vec<f64>>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextAnchorData {
    pub coordinates: Vec<Vec<f64>>,
    pub anchor_x: &'static str,
    pub anchor_y: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BBoxExtents {
    pub min: Vec<Vec<f64>>,
    pub max: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextAnchorError {
    InvalidCoordinateDimension(usize),
    RaggedCoordinateData,
}

impl fmt::Display for TextAnchorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCoordinateDimension(ndim) => {
                write!(formatter, "coordinate dimension must be 2 or 3, got {ndim}")
            }
            Self::RaggedCoordinateData => formatter.write_str("coordinate data is ragged"),
        }
    }
}

impl std::error::Error for TextAnchorError {}

pub fn get_text_anchors(
    view_data: &ViewData,
    ndisplay: usize,
    anchor: Anchor,
) -> Result<TextAnchorData, TextAnchorError> {
    match anchor {
        Anchor::Center => calculate_anchor_center(view_data),
        Anchor::UpperLeft => calculate_anchor_upper_left(view_data, ndisplay),
        Anchor::UpperRight => calculate_anchor_upper_right(view_data, ndisplay),
        Anchor::LowerLeft => calculate_anchor_lower_left(view_data, ndisplay),
        Anchor::LowerRight => calculate_anchor_lower_right(view_data, ndisplay),
    }
}

pub fn calculate_anchor_center(view_data: &ViewData) -> Result<TextAnchorData, TextAnchorError> {
    Ok(TextAnchorData {
        coordinates: calculate_bbox_centers(view_data)?,
        anchor_x: "center",
        anchor_y: "center",
    })
}

pub fn calculate_anchor_upper_left(
    view_data: &ViewData,
    ndisplay: usize,
) -> Result<TextAnchorData, TextAnchorError> {
    if ndisplay != 2 {
        return calculate_anchor_center(view_data);
    }
    let extents = calculate_bbox_extents(view_data)?;
    Ok(TextAnchorData {
        coordinates: extents
            .min
            .iter()
            .map(|coord| vec![coord[0], coord[1]])
            .collect(),
        anchor_x: "left",
        anchor_y: "top",
    })
}

pub fn calculate_anchor_upper_right(
    view_data: &ViewData,
    ndisplay: usize,
) -> Result<TextAnchorData, TextAnchorError> {
    if ndisplay != 2 {
        return calculate_anchor_center(view_data);
    }
    let extents = calculate_bbox_extents(view_data)?;
    Ok(TextAnchorData {
        coordinates: extents
            .min
            .iter()
            .zip(extents.max.iter())
            .map(|(min, max)| vec![min[0], max[1]])
            .collect(),
        anchor_x: "right",
        anchor_y: "top",
    })
}

pub fn calculate_anchor_lower_left(
    view_data: &ViewData,
    ndisplay: usize,
) -> Result<TextAnchorData, TextAnchorError> {
    if ndisplay != 2 {
        return calculate_anchor_center(view_data);
    }
    let extents = calculate_bbox_extents(view_data)?;
    Ok(TextAnchorData {
        coordinates: extents
            .min
            .iter()
            .zip(extents.max.iter())
            .map(|(min, max)| vec![max[0], min[1]])
            .collect(),
        anchor_x: "left",
        anchor_y: "bottom",
    })
}

pub fn calculate_anchor_lower_right(
    view_data: &ViewData,
    ndisplay: usize,
) -> Result<TextAnchorData, TextAnchorError> {
    if ndisplay != 2 {
        return calculate_anchor_center(view_data);
    }
    let extents = calculate_bbox_extents(view_data)?;
    Ok(TextAnchorData {
        coordinates: extents
            .max
            .iter()
            .map(|coord| vec![coord[0], coord[1]])
            .collect(),
        anchor_x: "right",
        anchor_y: "bottom",
    })
}

pub fn calculate_bbox_centers(view_data: &ViewData) -> Result<Vec<Vec<f64>>, TextAnchorError> {
    match view_data {
        ViewData::Coordinates(coords) => {
            validate_coords(coords)?;
            Ok(coords.clone())
        }
        ViewData::Vertices(vertices) => mean_stacked_vertices(vertices),
        ViewData::Items(items) => items.iter().map(|coords| mean_coords(coords)).collect(),
    }
}

pub fn calculate_bbox_extents(view_data: &ViewData) -> Result<BBoxExtents, TextAnchorError> {
    match view_data {
        ViewData::Coordinates(coords) => {
            validate_coords(coords)?;
            Ok(BBoxExtents {
                min: coords.clone(),
                max: coords.clone(),
            })
        }
        ViewData::Vertices(vertices) => {
            let centers_by_item = transpose_stacked_vertices(vertices)?;
            extents_for_items(&centers_by_item)
        }
        ViewData::Items(items) => extents_for_items(items),
    }
}

fn validate_coords(coords: &[Vec<f64>]) -> Result<(), TextAnchorError> {
    for coord in coords {
        validate_coord(coord)?;
    }
    Ok(())
}

fn validate_coord(coord: &[f64]) -> Result<(), TextAnchorError> {
    if matches!(coord.len(), 2 | 3) {
        Ok(())
    } else {
        Err(TextAnchorError::InvalidCoordinateDimension(coord.len()))
    }
}

fn mean_coords(coords: &[Vec<f64>]) -> Result<Vec<f64>, TextAnchorError> {
    validate_coords(coords)?;
    let ndim = coords.first().map_or(0, Vec::len);
    if ndim == 0 {
        return Ok(Vec::new());
    }
    let mut mean = vec![0.0; ndim];
    for coord in coords {
        if coord.len() != ndim {
            return Err(TextAnchorError::RaggedCoordinateData);
        }
        for (axis, value) in coord.iter().enumerate() {
            mean[axis] += value;
        }
    }
    for value in &mut mean {
        *value /= coords.len() as f64;
    }
    Ok(mean)
}

fn mean_stacked_vertices(vertices: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<f64>>, TextAnchorError> {
    let by_item = transpose_stacked_vertices(vertices)?;
    by_item.iter().map(|coords| mean_coords(coords)).collect()
}

fn transpose_stacked_vertices(
    vertices: &[Vec<Vec<f64>>],
) -> Result<Vec<Vec<Vec<f64>>>, TextAnchorError> {
    let Some(first_vertex) = vertices.first() else {
        return Ok(Vec::new());
    };
    let item_count = first_vertex.len();
    let mut by_item = vec![Vec::with_capacity(vertices.len()); item_count];
    for vertex in vertices {
        if vertex.len() != item_count {
            return Err(TextAnchorError::RaggedCoordinateData);
        }
        for (item_index, coord) in vertex.iter().enumerate() {
            validate_coord(coord)?;
            by_item[item_index].push(coord.clone());
        }
    }
    Ok(by_item)
}

fn extents_for_items(items: &[Vec<Vec<f64>>]) -> Result<BBoxExtents, TextAnchorError> {
    let mut min = Vec::with_capacity(items.len());
    let mut max = Vec::with_capacity(items.len());
    for coords in items {
        validate_coords(coords)?;
        let Some(first_coord) = coords.first() else {
            min.push(Vec::new());
            max.push(Vec::new());
            continue;
        };
        let ndim = first_coord.len();
        let mut item_min = first_coord.clone();
        let mut item_max = first_coord.clone();
        for coord in coords {
            if coord.len() != ndim {
                return Err(TextAnchorError::RaggedCoordinateData);
            }
            for (axis, value) in coord.iter().enumerate() {
                item_min[axis] = item_min[axis].min(*value);
                item_max[axis] = item_max[axis].max(*value);
            }
        }
        min.push(item_min);
        max.push(item_max);
    }
    Ok(BBoxExtents { min, max })
}
