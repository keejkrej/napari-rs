use napari_rs::layers::utils::color_encoding::{
    ColorEncodingError, ColorEncodingSpec, DEFAULT_COLOR, direct_color_encoding,
    nominal_color_encoding, quantitative_color_encoding, validate_color_encoding_from_colors,
    validate_contrast_limits,
};

#[test]
fn default_color_matches_python_cyan_fallback() {
    assert_eq!(DEFAULT_COLOR, [0.0, 1.0, 1.0, 1.0]);
}

#[test]
fn validate_color_encoding_from_colors_selects_constant_for_single_color() {
    assert_eq!(
        validate_color_encoding_from_colors(&[[1.0, 0.0, 0.0, 1.0]]),
        ColorEncodingSpec::Constant {
            constant: [1.0, 0.0, 0.0, 1.0],
        }
    );
}

#[test]
fn validate_color_encoding_from_colors_selects_manual_for_color_arrays() {
    let colors = vec![[1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 0.5]];

    assert_eq!(
        validate_color_encoding_from_colors(&colors),
        ColorEncodingSpec::Manual {
            array: colors,
            default: DEFAULT_COLOR,
        }
    );
    assert_eq!(
        validate_color_encoding_from_colors(&[]),
        ColorEncodingSpec::Manual {
            array: Vec::new(),
            default: DEFAULT_COLOR,
        }
    );
}

#[test]
fn derived_color_encoding_specs_use_python_default_fallback() {
    assert_eq!(
        direct_color_encoding("edge_color"),
        ColorEncodingSpec::Direct {
            feature: "edge_color".to_owned(),
            fallback: DEFAULT_COLOR,
        }
    );
    assert_eq!(
        nominal_color_encoding("label", "categorical"),
        ColorEncodingSpec::Nominal {
            feature: "label".to_owned(),
            colormap: "categorical".to_owned(),
            fallback: DEFAULT_COLOR,
        }
    );
}

#[test]
fn quantitative_color_encoding_validates_contrast_limits_like_python() {
    assert_eq!(
        quantitative_color_encoding("score", "viridis", Some((0.0, 10.0))).unwrap(),
        ColorEncodingSpec::Quantitative {
            feature: "score".to_owned(),
            colormap: "viridis".to_owned(),
            contrast_limits: Some((0.0, 10.0)),
            fallback: DEFAULT_COLOR,
        }
    );
    assert_eq!(
        validate_contrast_limits(Some((1.0, 1.0))),
        Err(ColorEncodingError::InvalidContrastLimits)
    );
    assert_eq!(
        validate_contrast_limits(Some((2.0, 1.0))),
        Err(ColorEncodingError::InvalidContrastLimits)
    );
    assert_eq!(validate_contrast_limits(None), Ok(None));
}
