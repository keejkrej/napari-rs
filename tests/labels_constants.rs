use napari_rs::layers::labels::constants::{
    BACKSPACE, IsoCategoricalGradientMode, LabelColorMode, LabelsRendering, Mode,
};

#[test]
fn labels_mode_strings_match_python_string_enum_values() {
    assert_eq!(Mode::PanZoom.to_string(), "pan_zoom");
    assert_eq!(Mode::Transform.to_string(), "transform");
    assert_eq!(Mode::Pick.to_string(), "pick");
    assert_eq!(Mode::Paint.to_string(), "paint");
    assert_eq!(Mode::Fill.to_string(), "fill");
    assert_eq!(Mode::Erase.to_string(), "erase");
    assert_eq!(Mode::Polygon.to_string(), "polygon");
    assert_eq!("POLYGON".parse(), Ok(Mode::Polygon));
}

#[test]
fn labels_color_mode_strings_match_python_string_enum_values() {
    assert_eq!(LabelColorMode::Auto.to_string(), "auto");
    assert_eq!(LabelColorMode::Direct.to_string(), "direct");
    assert_eq!("DIRECT".parse(), Ok(LabelColorMode::Direct));
}

#[test]
fn labels_rendering_strings_match_python_string_enum_values() {
    assert_eq!(LabelsRendering::Translucent.to_string(), "translucent");
    assert_eq!(
        LabelsRendering::IsoCategorical.to_string(),
        "iso_categorical"
    );
    assert_eq!(
        "ISO_CATEGORICAL".parse(),
        Ok(LabelsRendering::IsoCategorical)
    );
}

#[test]
fn iso_categorical_gradient_strings_match_python_string_enum_values() {
    assert_eq!(IsoCategoricalGradientMode::Fast.to_string(), "fast");
    assert_eq!(IsoCategoricalGradientMode::Smooth.to_string(), "smooth");
    assert_eq!("SMOOTH".parse(), Ok(IsoCategoricalGradientMode::Smooth));
}

#[test]
fn labels_backspace_matches_platform_constant() {
    if cfg!(target_os = "macos") {
        assert_eq!(BACKSPACE, "delete");
    } else {
        assert_eq!(BACKSPACE, "backspace");
    }
}
