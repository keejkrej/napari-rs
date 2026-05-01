use crate::components::viewer_constants::CursorStyle;

#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    pub position: Vec<f64>,
    pub viewbox: Option<(usize, usize)>,
    pub scaled: bool,
    pub size: f64,
    pub style: CursorStyle,
    pub view_direction: Option<Vec<f64>>,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: vec![1.0, 1.0],
            viewbox: None,
            scaled: true,
            size: 1.0,
            style: CursorStyle::Standard,
            view_direction: None,
        }
    }
}
