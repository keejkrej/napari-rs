use napari_rs::layers::image::constants::{
    ImageProjectionMode, ImageRendering, Interpolation, VolumeDepiction,
};

#[test]
fn interpolation_strings_match_python_string_enum_values() {
    assert_eq!(Interpolation::Bessel.to_string(), "bessel");
    assert_eq!(Interpolation::Cubic.to_string(), "cubic");
    assert_eq!(Interpolation::Linear.to_string(), "linear");
    assert_eq!(Interpolation::Blackman.to_string(), "blackman");
    assert_eq!(Interpolation::Catrom.to_string(), "catrom");
    assert_eq!(Interpolation::Gaussian.to_string(), "gaussian");
    assert_eq!(Interpolation::Hamming.to_string(), "hamming");
    assert_eq!(Interpolation::Hanning.to_string(), "hanning");
    assert_eq!(Interpolation::Hermite.to_string(), "hermite");
    assert_eq!(Interpolation::Kaiser.to_string(), "kaiser");
    assert_eq!(Interpolation::Lanczos.to_string(), "lanczos");
    assert_eq!(Interpolation::Mitchell.to_string(), "mitchell");
    assert_eq!(Interpolation::Nearest.to_string(), "nearest");
    assert_eq!(Interpolation::Spline16.to_string(), "spline16");
    assert_eq!(Interpolation::Spline36.to_string(), "spline36");
    assert_eq!(Interpolation::Custom.to_string(), "custom");
    assert_eq!("SPLINE36".parse(), Ok(Interpolation::Spline36));
}

#[test]
fn interpolation_view_subset_matches_python_helper() {
    assert_eq!(
        Interpolation::view_subset(),
        [
            Interpolation::Cubic,
            Interpolation::Linear,
            Interpolation::Kaiser,
            Interpolation::Nearest,
            Interpolation::Spline36,
        ]
    );
}

#[test]
fn image_rendering_strings_match_python_string_enum_values() {
    assert_eq!(ImageRendering::Translucent.to_string(), "translucent");
    assert_eq!(ImageRendering::Additive.to_string(), "additive");
    assert_eq!(ImageRendering::Iso.to_string(), "iso");
    assert_eq!(ImageRendering::Mip.to_string(), "mip");
    assert_eq!(ImageRendering::Minip.to_string(), "minip");
    assert_eq!(ImageRendering::AttenuatedMip.to_string(), "attenuated_mip");
    assert_eq!(ImageRendering::Average.to_string(), "average");
    assert_eq!("ATTENUATED_MIP".parse(), Ok(ImageRendering::AttenuatedMip));
}

#[test]
fn volume_depiction_strings_match_python_string_enum_values() {
    assert_eq!(VolumeDepiction::Volume.to_string(), "volume");
    assert_eq!(VolumeDepiction::Plane.to_string(), "plane");
    assert_eq!("PLANE".parse(), Ok(VolumeDepiction::Plane));
}

#[test]
fn image_projection_mode_strings_match_python_string_enum_values() {
    assert_eq!(ImageProjectionMode::None.to_string(), "none");
    assert_eq!(ImageProjectionMode::Sum.to_string(), "sum");
    assert_eq!(ImageProjectionMode::Mean.to_string(), "mean");
    assert_eq!(ImageProjectionMode::Max.to_string(), "max");
    assert_eq!(ImageProjectionMode::Min.to_string(), "min");
    assert_eq!("MAX".parse(), Ok(ImageProjectionMode::Max));
}
