use napari_rs::layers::vectors::constants::{VectorStyle, VectorsProjectionMode};

#[test]
fn vector_style_strings_match_python_string_enum_values() {
    assert_eq!(VectorStyle::Line.to_string(), "line");
    assert_eq!(VectorStyle::Triangle.to_string(), "triangle");
    assert_eq!(VectorStyle::Arrow.to_string(), "arrow");
    assert_eq!("ARROW".parse(), Ok(VectorStyle::Arrow));
}

#[test]
fn vectors_projection_mode_strings_match_python_string_enum_values() {
    assert_eq!(VectorsProjectionMode::None.to_string(), "none");
    assert_eq!(VectorsProjectionMode::All.to_string(), "all");
    assert_eq!("ALL".parse(), Ok(VectorsProjectionMode::All));
}
