pub mod base;

use std::error::Error;
use std::fmt;

use crate::components::overlays::base::{CanvasOverlay, Color, Overlay, SceneOverlay};
use crate::components::viewer_constants::CanvasPosition;
use crate::layers::base::constants::InteractionBoxHandle;
use crate::layers::utils::interaction_box::{
    Bounds2, Point2, calculate_bounds_from_contained_points,
};

pub type ZoomBounds = ((f64, f64), (f64, f64));

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBoxOverlay {
    pub base: SceneOverlay,
    pub lines: bool,
    pub line_thickness: f32,
    pub line_color: Color,
    pub points: bool,
    pub point_size: f32,
    pub point_color: Color,
}

impl Default for BoundingBoxOverlay {
    fn default() -> Self {
        Self {
            base: SceneOverlay::default(),
            lines: true,
            line_thickness: 1.0,
            line_color: [1.0, 0.0, 0.0, 1.0],
            points: true,
            point_size: 5.0,
            point_color: [0.0, 0.0, 1.0, 1.0],
        }
    }
}

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
pub struct ColorBarOverlay {
    pub base: CanvasOverlay,
    pub color: Option<Color>,
    pub size: (f32, f32),
    pub tick_length: f32,
    pub font_size: f32,
}

impl Default for ColorBarOverlay {
    fn default() -> Self {
        Self {
            base: CanvasOverlay {
                position: CanvasPosition::TopRight,
                ..CanvasOverlay::default()
            },
            color: None,
            size: (25.0, 150.0),
            tick_length: 5.0,
            font_size: 10.0,
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
pub struct WelcomeOverlay {
    pub overlay: Overlay,
    pub position: Option<CanvasPosition>,
    pub box_visible: bool,
    pub box_color: Option<Color>,
    pub gridded: bool,
    pub version: String,
    pub shortcuts: Vec<String>,
    pub tips: Vec<String>,
}

impl Default for WelcomeOverlay {
    fn default() -> Self {
        Self {
            overlay: Overlay {
                visible: false,
                opacity: 1.0,
                order: Overlay::DEFAULT_ORDER + 10,
                blending: crate::layers::base::constants::Blending::TranslucentNoDepth,
            },
            position: None,
            box_visible: true,
            box_color: None,
            gridded: false,
            version: "not-installed".to_owned(),
            shortcuts: [
                "napari.window.file._image_from_clipboard",
                "napari.window.file.open_files_dialog",
                "napari.window.view.toggle_command_palette",
                "napari:show_shortcuts",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            tips: [
                "You can take a screenshot of the canvas and copy it to your clipboard by pressing {napari.window.file.copy_canvas_screenshot}.",
                "You can change most shortcuts from the File -> Preferences -> Shortcuts menu.",
                "You can right click many components of the graphical interface to access advanced controls.",
                "If you select multiple layers in the layer list, then right click and select \"Link Layers\", their parameters will be synced.",
                "You can press {Ctrl} and scroll the mouse wheel to move the dimension sliders.",
                "To zoom in on a specific area, hold {Alt} and draw a rectangle around it.",
                "Hold {napari:hold_for_pan_zoom} to pan/zoom in any mode (e.g. while painting).",
                "While painting labels, hold {Alt} and move the cursor left/right to quickly decrease/increase the brush size.",
                "If you have questions, you can reach out on our community chat at napari.zulipchat.com!",
                "The community at forum.image.sc is full of imaging experts sharing knowledge and tools for napari and much, much more!",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct LabelsPolygonOverlay {
    pub base: SceneOverlay,
    pub enabled: bool,
    pub points: Vec<Vec<f64>>,
    pub use_double_click_completion_radius: bool,
    pub completion_radius: f64,
}

impl Default for LabelsPolygonOverlay {
    fn default() -> Self {
        Self {
            base: SceneOverlay::default(),
            enabled: false,
            points: Vec::new(),
            use_double_click_completion_radius: false,
            completion_radius: 20.0,
        }
    }
}

impl LabelsPolygonOverlay {
    pub fn take_polygon_for_paint(&mut self) -> Option<Vec<Vec<f64>>> {
        let polygon = (self.points.len() > 2).then(|| self.points.clone());
        self.points.clear();
        polygon
    }
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
