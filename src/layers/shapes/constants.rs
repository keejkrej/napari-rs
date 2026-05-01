use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    PanZoom,
    Transform,
    Select,
    Direct,
    AddRectangle,
    AddEllipse,
    AddLine,
    AddPolyline,
    AddPath,
    AddPolygon,
    AddPolygonLasso,
    VertexInsert,
    VertexRemove,
}

impl fmt::Display for Mode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PanZoom => "pan_zoom",
            Self::Transform => "transform",
            Self::Select => "select",
            Self::Direct => "direct",
            Self::AddRectangle => "add_rectangle",
            Self::AddEllipse => "add_ellipse",
            Self::AddLine => "add_line",
            Self::AddPolyline => "add_polyline",
            Self::AddPath => "add_path",
            Self::AddPolygon => "add_polygon",
            Self::AddPolygonLasso => "add_polygon_lasso",
            Self::VertexInsert => "vertex_insert",
            Self::VertexRemove => "vertex_remove",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseModeError;

impl fmt::Display for ParseModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid shapes mode")
    }
}

impl std::error::Error for ParseModeError {}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "pan_zoom" => Ok(Self::PanZoom),
            "transform" => Ok(Self::Transform),
            "select" => Ok(Self::Select),
            "direct" => Ok(Self::Direct),
            "add_rectangle" => Ok(Self::AddRectangle),
            "add_ellipse" => Ok(Self::AddEllipse),
            "add_line" => Ok(Self::AddLine),
            "add_polyline" => Ok(Self::AddPolyline),
            "add_path" => Ok(Self::AddPath),
            "add_polygon" => Ok(Self::AddPolygon),
            "add_polygon_lasso" => Ok(Self::AddPolygonLasso),
            "vertex_insert" => Ok(Self::VertexInsert),
            "vertex_remove" => Ok(Self::VertexRemove),
            _ => Err(ParseModeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Direct,
    Cycle,
    Colormap,
}

impl fmt::Display for ColorMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Direct => "direct",
            Self::Cycle => "cycle",
            Self::Colormap => "colormap",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseColorModeError;

impl fmt::Display for ParseColorModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid shapes color mode")
    }
}

impl std::error::Error for ParseColorModeError {}

impl FromStr for ColorMode {
    type Err = ParseColorModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "direct" => Ok(Self::Direct),
            "cycle" => Ok(Self::Cycle),
            "colormap" => Ok(Self::Colormap),
            _ => Err(ParseColorModeError),
        }
    }
}

pub struct Box;

impl Box {
    pub const WITH_HANDLE: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 9];
    pub const LINE_HANDLE: [usize; 7] = [7, 6, 4, 2, 0, 7, 8];
    pub const LINE: [usize; 5] = [0, 2, 4, 6, 0];
    pub const TOP_LEFT: usize = 0;
    pub const TOP_CENTER: usize = 7;
    pub const LEFT_CENTER: usize = 1;
    pub const BOTTOM_RIGHT: usize = 4;
    pub const BOTTOM_LEFT: usize = 2;
    pub const CENTER: usize = 8;
    pub const HANDLE: usize = 9;
    pub const LEN: usize = 8;
}

#[cfg(target_os = "macos")]
pub const BACKSPACE: &str = "delete";
#[cfg(not(target_os = "macos"))]
pub const BACKSPACE: &str = "backspace";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Rectangle,
    Ellipse,
    Line,
    Path,
    Polygon,
}

impl fmt::Display for ShapeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Rectangle => "rectangle",
            Self::Ellipse => "ellipse",
            Self::Line => "line",
            Self::Path => "path",
            Self::Polygon => "polygon",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseShapeTypeError;

impl fmt::Display for ParseShapeTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid shape type")
    }
}

impl std::error::Error for ParseShapeTypeError {}

impl FromStr for ShapeType {
    type Err = ParseShapeTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "rectangle" => Ok(Self::Rectangle),
            "ellipse" => Ok(Self::Ellipse),
            "line" => Ok(Self::Line),
            "path" => Ok(Self::Path),
            "polygon" => Ok(Self::Polygon),
            _ => Err(ParseShapeTypeError),
        }
    }
}
