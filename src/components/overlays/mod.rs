pub mod base;

use std::error::Error;
use std::fmt;

use crate::components::overlays::base::{CanvasOverlay, Color, SceneOverlay};
use crate::components::viewer_constants::CanvasPosition;
use crate::layers::base::constants::InteractionBoxHandle;
use crate::layers::utils::interaction_box::{
    Bounds2, Point2, calculate_bounds_from_contained_points,
};

pub type ZoomBounds = ((f64, f64), (f64, f64));

#[derive(Debug, Clone, PartialEq)]
pub struct AxesOverlay {
    pub base: SceneOverlay,
    pub labels: bool,
    pub colored: bool,
    pub dashed: bool,
    pub arrows: bool,
}

impl Default for AxesOverlay {
    fn default() -> Self {
        Self {
            base: SceneOverlay::default(),
            labels: true,
            colored: true,
            dashed: false,
            arrows: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrushCircleOverlay {
    pub base: CanvasOverlay,
    pub size: usize,
    pub position: (i32, i32),
    pub position_is_frozen: bool,
}

impl Default for BrushCircleOverlay {
    fn default() -> Self {
        Self {
            base: CanvasOverlay::default(),
            size: 10,
            position: (0, 0),
            position_is_frozen: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScaleBarOverlay {
    pub base: CanvasOverlay,
    pub colored: bool,
    pub color: Color,
    pub ticks: bool,
    pub font_size: f32,
    pub unit: Option<String>,
    pub length: Option<f64>,
}

impl Default for ScaleBarOverlay {
    fn default() -> Self {
        Self {
            base: CanvasOverlay::default(),
            colored: false,
            color: [1.0, 0.0, 1.0, 1.0],
            ticks: true,
            font_size: 10.0,
            unit: None,
            length: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextOverlay {
    pub base: CanvasOverlay,
    pub color: Option<Color>,
    pub font_size: f32,
    pub text: String,
}

impl Default for TextOverlay {
    fn default() -> Self {
        Self {
            base: CanvasOverlay::default(),
            color: None,
            font_size: 10.0,
            text: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerNameOverlay {
    pub text: TextOverlay,
}

impl Default for LayerNameOverlay {
    fn default() -> Self {
        let mut text = TextOverlay::default();
        text.base.position = CanvasPosition::TopLeft;
        Self { text }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurrentSliceOverlay {
    pub text: TextOverlay,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZoomOverlay {
    pub base: CanvasOverlay,
    pub position: ZoomBounds,
    pub zoom_area: ZoomBounds,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionBoxOverlay {
    pub base: SceneOverlay,
    pub bounds: Bounds2,
    pub handles: bool,
    pub selected_handle: Option<InteractionBoxHandle>,
}

impl Default for SelectionBoxOverlay {
    fn default() -> Self {
        Self {
            base: SceneOverlay::default(),
            bounds: ([0.0, 0.0], [0.0, 0.0]),
            handles: false,
            selected_handle: None,
        }
    }
}

impl SelectionBoxOverlay {
    pub fn update_from_points(&mut self, points: &[Point2]) {
        if let Some(bounds) = calculate_bounds_from_contained_points(points) {
            self.bounds = bounds;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransformBoxOverlay {
    pub base: SceneOverlay,
    pub selected_handle: Option<InteractionBoxHandle>,
}

impl Default for ZoomOverlay {
    fn default() -> Self {
        Self {
            base: CanvasOverlay::default(),
            position: ((0.0, 0.0), (0.0, 0.0)),
            zoom_area: ((0.0, 0.0), (0.0, 0.0)),
        }
    }
}

impl ZoomOverlay {
    pub fn set_position_from_points(&mut self, points: &[Vec<f64>; 2]) -> Result<(), ZoomError> {
        self.position = validate_bounds(points)?;
        Ok(())
    }

    pub fn set_zoom_area_from_points(&mut self, points: &[Vec<f64>; 2]) -> Result<(), ZoomError> {
        self.zoom_area = validate_bounds(points)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoomError;

impl fmt::Display for ZoomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("zoom bounds must contain two 2D points")
    }
}

impl Error for ZoomError {}

fn validate_bounds(points: &[Vec<f64>; 2]) -> Result<ZoomBounds, ZoomError> {
    let [first, second] = points;
    if first.len() != 2 || second.len() != 2 {
        return Err(ZoomError);
    }
    Ok(((first[0], first[1]), (second[0], second[1])))
}
