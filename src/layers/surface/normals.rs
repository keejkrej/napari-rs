use crate::layers::utils::color_transformations::Rgba;

pub const BLACK: Rgba = [0.0, 0.0, 0.0, 1.0];
pub const ORANGE: Rgba = [1.0, 0.647_058_84, 0.0, 1.0];
pub const BLUE: Rgba = [0.0, 0.0, 1.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalMode {
    Face,
    Vertex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Normals {
    mode: NormalMode,
    pub visible: bool,
    pub color: Rgba,
    pub width: f32,
    pub length: f32,
}

impl Normals {
    pub fn new(mode: NormalMode, color: Rgba) -> Self {
        Self {
            mode,
            color,
            ..Self::default()
        }
    }

    pub fn mode(&self) -> NormalMode {
        self.mode
    }

    pub fn update(&mut self, update: NormalsUpdate) {
        if let Some(visible) = update.visible {
            self.visible = visible;
        }
        if let Some(color) = update.color {
            self.color = color;
        }
        if let Some(width) = update.width {
            self.width = width;
        }
        if let Some(length) = update.length {
            self.length = length;
        }
    }

    pub fn reset(&mut self) {
        let mode = self.mode;
        *self = Self::new(mode, BLACK);
    }
}

impl Default for Normals {
    fn default() -> Self {
        Self {
            mode: NormalMode::Face,
            visible: false,
            color: BLACK,
            width: 1.0,
            length: 5.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NormalsUpdate {
    pub visible: Option<bool>,
    pub color: Option<Rgba>,
    pub width: Option<f32>,
    pub length: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceNormals {
    pub face: Normals,
    pub vertex: Normals,
}

impl SurfaceNormals {
    pub fn update(&mut self, update: SurfaceNormalsUpdate) {
        if let Some(face) = update.face {
            self.face.update(face);
        }
        if let Some(vertex) = update.vertex {
            self.vertex.update(vertex);
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for SurfaceNormals {
    fn default() -> Self {
        Self {
            face: Normals::new(NormalMode::Face, ORANGE),
            // This follows napari's current SurfaceNormals default exactly.
            vertex: Normals::new(NormalMode::Face, BLUE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SurfaceNormalsUpdate {
    pub face: Option<NormalsUpdate>,
    pub vertex: Option<NormalsUpdate>,
}
