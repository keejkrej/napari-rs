use napari_rs::layers::intensity::{IntensityError, IntensityVisualization};
use napari_rs::utils::dtype::DType;

#[test]
fn intensity_defaults_match_python_mixin_state() {
    let intensity = IntensityVisualization::default();

    assert_eq!(intensity.gamma, 1.0);
    assert_eq!(intensity.colormap_name, "");
    assert_eq!(intensity.contrast_limits_msg, "");
    assert_eq!(intensity.contrast_limits(), [None, None]);
    assert_eq!(intensity.contrast_limits_range(), [None, None]);
    assert!(!intensity.keep_auto_contrast);
}

#[test]
fn gamma_setter_casts_numeric_values_like_python_property() {
    let mut intensity = IntensityVisualization::default();

    intensity.set_gamma(0.7);

    assert_eq!(intensity.gamma, 0.7);
}

#[test]
fn contrast_limits_validate_increasing_values_and_expand_range() {
    let mut intensity = IntensityVisualization::default();
    intensity
        .set_contrast_limits_range([Some(0.0), Some(100.0)])
        .unwrap();

    intensity.set_contrast_limits([20.0, 40.0]).unwrap();

    assert_eq!(intensity.contrast_limits(), [Some(20.0), Some(40.0)]);
    assert_eq!(intensity.contrast_limits_range(), [Some(0.0), Some(100.0)]);
    assert_eq!(intensity.contrast_limits_msg, "20, 40");

    intensity.set_contrast_limits([0.0, 200.0]).unwrap();

    assert_eq!(intensity.contrast_limits_range(), [Some(0.0), Some(200.0)]);
    assert_eq!(
        intensity.set_contrast_limits([1.0, 1.0]),
        Err(IntensityError::NotIncreasing)
    );
    assert_eq!(
        intensity.set_contrast_limits([2.0, 1.0]),
        Err(IntensityError::NotIncreasing)
    );
}

#[test]
fn contrast_limits_range_clips_or_resets_limits_like_python_mixin() {
    let mut intensity = IntensityVisualization::default();
    intensity
        .set_contrast_limits_range([Some(0.0), Some(100.0)])
        .unwrap();
    intensity.set_contrast_limits([20.0, 40.0]).unwrap();

    intensity
        .set_contrast_limits_range([Some(0.0), Some(30.0)])
        .unwrap();
    assert_eq!(intensity.contrast_limits(), [Some(20.0), Some(30.0)]);

    intensity
        .set_contrast_limits_range([Some(0.0), Some(10.0)])
        .unwrap();
    assert_eq!(intensity.contrast_limits(), [Some(0.0), Some(10.0)]);

    intensity
        .set_contrast_limits_range([Some(0.0), Some(100.0)])
        .unwrap();
    intensity.set_contrast_limits([20.0, 40.0]).unwrap();
    intensity
        .set_contrast_limits_range([Some(60.0), Some(100.0)])
        .unwrap();
    assert_eq!(intensity.contrast_limits(), [Some(60.0), Some(100.0)]);
}

#[test]
fn contrast_limits_range_preserves_boundaries_for_none_values() {
    let mut intensity = IntensityVisualization::default();
    intensity
        .set_contrast_limits_range([Some(0.0), Some(100.0)])
        .unwrap();

    intensity
        .set_contrast_limits_range([None, Some(30.0)])
        .unwrap();

    assert_eq!(intensity.contrast_limits_range(), [Some(0.0), Some(30.0)]);
}

#[test]
fn reset_contrast_limits_range_uses_dtype_limits_for_integer_data() {
    let mut intensity = IntensityVisualization::default();

    intensity
        .reset_contrast_limits_range(DType::UInt8, [0.25, 0.75])
        .unwrap();

    assert_eq!(intensity.contrast_limits_range(), [Some(0.0), Some(255.0)]);

    intensity
        .reset_contrast_limits_range(DType::Float32, [0.25, 0.75])
        .unwrap();

    assert_eq!(intensity.contrast_limits_range(), [Some(0.25), Some(0.75)]);
}
