use crate::layers::utils::color_transformations::Rgba;

pub const BLACK: Rgba = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceWireframe {
    pub visible: bool,
    pub color: Rgba,
    pub width: f32,
}

impl SurfaceWireframe {
    pub fn update(&mut self, update: SurfaceWireframeUpdate) {
        if let Some(visible) = update.visible {
            self.visible = visible;
        }
        if let Some(color) = update.color {
            self.color = color;
        }
        if let Some(width) = update.width {
            self.width = width;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for SurfaceWireframe {
    fn default() -> Self {
        Self {
            visible: false,
            color: BLACK,
            width: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SurfaceWireframeUpdate {
    pub visible: Option<bool>,
    pub color: Option<Rgba>,
    pub width: Option<f32>,
}
