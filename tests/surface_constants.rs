use napari_rs::layers::surface::constants::Shading;

#[test]
fn surface_shading_strings_match_python_string_enum_values() {
    assert_eq!(Shading::None.to_string(), "none");
    assert_eq!(Shading::Flat.to_string(), "flat");
    assert_eq!(Shading::Smooth.to_string(), "smooth");
    assert_eq!("SMOOTH".parse(), Ok(Shading::Smooth));
}
