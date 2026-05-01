use crate::components::viewer_constants::CanvasPosition;
use crate::layers::base::constants::Blending;

pub type Color = [f32; 4];

#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub visible: bool,
    pub opacity: f32,
    pub order: i32,
    pub blending: Blending,
}

impl Overlay {
    pub const DEFAULT_ORDER: i32 = 1_000_000;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasOverlay {
    pub overlay: Overlay,
    pub position: CanvasPosition,
    pub box_visible: bool,
    pub box_color: Option<Color>,
    pub gridded: bool,
}

impl Default for CanvasOverlay {
    fn default() -> Self {
        Self {
            overlay: Overlay {
                visible: false,
                opacity: 1.0,
                order: Overlay::DEFAULT_ORDER,
                blending: Blending::TranslucentNoDepth,
            },
            position: CanvasPosition::BottomRight,
            box_visible: true,
            box_color: None,
            gridded: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneOverlay {
    pub overlay: Overlay,
}

impl Default for SceneOverlay {
    fn default() -> Self {
        Self {
            overlay: Overlay {
                visible: false,
                opacity: 1.0,
                order: Overlay::DEFAULT_ORDER,
                blending: Blending::Translucent,
            },
        }
    }
}
