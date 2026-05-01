use napari_rs::components::cursor::Cursor;
use napari_rs::components::viewer_constants::{CanvasPosition, CursorStyle};

#[test]
fn cursor_defaults_match_python_model_defaults() {
    let cursor = Cursor::default();

    assert_eq!(cursor.position, vec![1.0, 1.0]);
    assert_eq!(cursor.viewbox, None);
    assert!(cursor.scaled);
    assert_eq!(cursor.size, 1.0);
    assert_eq!(cursor.style, CursorStyle::Standard);
    assert_eq!(cursor.view_direction, None);
}

#[test]
fn cursor_style_strings_match_python_values() {
    assert_eq!(CursorStyle::Square.to_string(), "square");
    assert_eq!(CursorStyle::Circle.to_string(), "circle");
    assert_eq!(CursorStyle::CircleFrozen.to_string(), "circle_frozen");
    assert_eq!(CursorStyle::Cross.to_string(), "cross");
    assert_eq!(CursorStyle::Forbidden.to_string(), "forbidden");
    assert_eq!(CursorStyle::Pointing.to_string(), "pointing");
    assert_eq!(CursorStyle::Standard.to_string(), "standard");
    assert_eq!(CursorStyle::Crosshair.to_string(), "crosshair");
}

#[test]
fn canvas_position_strings_match_python_values() {
    assert_eq!(CanvasPosition::TopLeft.to_string(), "top_left");
    assert_eq!(CanvasPosition::TopCenter.to_string(), "top_center");
    assert_eq!(CanvasPosition::TopRight.to_string(), "top_right");
    assert_eq!(CanvasPosition::BottomRight.to_string(), "bottom_right");
    assert_eq!(CanvasPosition::BottomCenter.to_string(), "bottom_center");
    assert_eq!(CanvasPosition::BottomLeft.to_string(), "bottom_left");
}

#[test]
fn viewer_constants_parse_case_insensitively() {
    assert_eq!("SQUARE".parse(), Ok(CursorStyle::Square));
    assert_eq!("circle_frozen".parse(), Ok(CursorStyle::CircleFrozen));
    assert_eq!("TOP_LEFT".parse(), Ok(CanvasPosition::TopLeft));
    assert_eq!("bottom_center".parse(), Ok(CanvasPosition::BottomCenter));
}
