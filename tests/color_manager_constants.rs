use napari_rs::layers::utils::color_manager_constants::ColorMode;

#[test]
fn color_manager_color_mode_strings_match_python_string_enum_values() {
    assert_eq!(ColorMode::Direct.to_string(), "direct");
    assert_eq!(ColorMode::Cycle.to_string(), "cycle");
    assert_eq!(ColorMode::Colormap.to_string(), "colormap");
    assert_eq!("DIRECT".parse(), Ok(ColorMode::Direct));
    assert_eq!("cycle".parse(), Ok(ColorMode::Cycle));
    assert!("not-a-mode".parse::<ColorMode>().is_err());
}
