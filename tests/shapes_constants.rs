use napari_rs::layers::shapes::constants::{BACKSPACE, Box, ColorMode, Mode, ShapeType};

#[test]
fn shapes_mode_strings_match_python_string_enum_values() {
    assert_eq!(Mode::PanZoom.to_string(), "pan_zoom");
    assert_eq!(Mode::Transform.to_string(), "transform");
    assert_eq!(Mode::Select.to_string(), "select");
    assert_eq!(Mode::Direct.to_string(), "direct");
    assert_eq!(Mode::AddRectangle.to_string(), "add_rectangle");
    assert_eq!(Mode::AddEllipse.to_string(), "add_ellipse");
    assert_eq!(Mode::AddLine.to_string(), "add_line");
    assert_eq!(Mode::AddPolyline.to_string(), "add_polyline");
    assert_eq!(Mode::AddPath.to_string(), "add_path");
    assert_eq!(Mode::AddPolygon.to_string(), "add_polygon");
    assert_eq!(Mode::AddPolygonLasso.to_string(), "add_polygon_lasso");
    assert_eq!(Mode::VertexInsert.to_string(), "vertex_insert");
    assert_eq!(Mode::VertexRemove.to_string(), "vertex_remove");
    assert_eq!("ADD_POLYGON_LASSO".parse(), Ok(Mode::AddPolygonLasso));
}

#[test]
fn shapes_color_mode_strings_match_python_string_enum_values() {
    assert_eq!(ColorMode::Direct.to_string(), "direct");
    assert_eq!(ColorMode::Cycle.to_string(), "cycle");
    assert_eq!(ColorMode::Colormap.to_string(), "colormap");
    assert_eq!("CYCLE".parse(), Ok(ColorMode::Cycle));
}

#[test]
fn shapes_box_indices_match_python_constants() {
    assert_eq!(Box::WITH_HANDLE, [0, 1, 2, 3, 4, 5, 6, 7, 9]);
    assert_eq!(Box::LINE_HANDLE, [7, 6, 4, 2, 0, 7, 8]);
    assert_eq!(Box::LINE, [0, 2, 4, 6, 0]);
    assert_eq!(Box::TOP_LEFT, 0);
    assert_eq!(Box::TOP_CENTER, 7);
    assert_eq!(Box::LEFT_CENTER, 1);
    assert_eq!(Box::BOTTOM_RIGHT, 4);
    assert_eq!(Box::BOTTOM_LEFT, 2);
    assert_eq!(Box::CENTER, 8);
    assert_eq!(Box::HANDLE, 9);
    assert_eq!(Box::LEN, 8);
}

#[test]
fn shapes_backspace_matches_platform_constant() {
    if cfg!(target_os = "macos") {
        assert_eq!(BACKSPACE, "delete");
    } else {
        assert_eq!(BACKSPACE, "backspace");
    }
}

#[test]
fn shape_type_strings_match_python_string_enum_values() {
    assert_eq!(ShapeType::Rectangle.to_string(), "rectangle");
    assert_eq!(ShapeType::Ellipse.to_string(), "ellipse");
    assert_eq!(ShapeType::Line.to_string(), "line");
    assert_eq!(ShapeType::Path.to_string(), "path");
    assert_eq!(ShapeType::Polygon.to_string(), "polygon");
    assert_eq!("POLYGON".parse(), Ok(ShapeType::Polygon));
}
