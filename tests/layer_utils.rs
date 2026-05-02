use std::collections::BTreeMap;

use napari_rs::layers::utils::layer_utils::{
    AffineInput, LayerUtilsError, Properties, PropertyValue, calc_data_range, coerce_affine,
    coerce_current_properties, compute_multiscale_level, compute_multiscale_level_and_corners,
    dims_displayed_world_to_layer, get_current_properties, get_extent_world, nanmax, nanmin,
    segment_normal, unique_element, validate_properties, validate_property_choices,
};
use napari_rs::utils::dtype::DType;
use napari_rs::utils::transforms::affine::Affine;
use napari_rs::utils::transforms::transform_utils::RotationInput;

fn properties(entries: impl IntoIterator<Item = (&'static str, Vec<PropertyValue>)>) -> Properties {
    entries
        .into_iter()
        .map(|(key, values)| (key.to_owned(), values))
        .collect::<BTreeMap<_, _>>()
}

fn assert_matrix_close(actual: &[Vec<f64>], expected: &[Vec<f64>]) {
    assert_eq!(actual.len(), expected.len());
    for (actual_row, expected_row) in actual.iter().zip(expected) {
        assert_eq!(actual_row.len(), expected_row.len());
        for (&actual, &expected) in actual_row.iter().zip(expected_row) {
            assert!(
                (actual - expected).abs() <= 1e-10,
                "expected {expected}, got {actual}"
            );
        }
    }
}

#[test]
fn nanmin_and_nanmax_fall_back_to_finite_values_like_python_helpers() {
    assert_eq!(nanmin(&[f64::NAN, f64::INFINITY]), 0.0);
    assert_eq!(nanmax(&[f64::NAN, f64::NEG_INFINITY]), 1.0);
    assert_eq!(nanmin(&[f64::NAN, -2.0, 3.0]), -2.0);
    assert_eq!(nanmax(&[f64::NAN, -2.0, 3.0]), 3.0);
}

#[test]
fn calc_data_range_matches_python_constant_and_min_max_cases() {
    assert_eq!(calc_data_range(&[0.0, 0.0, 0.0], None), (0.0, 1.0));
    assert_eq!(calc_data_range(&[1.0, 1.0, 1.0], None), (0.0, 1.0));
    assert_eq!(calc_data_range(&[0.0, 0.5, 2.0], None), (0.0, 2.0));
    assert_eq!(calc_data_range(&[-1.0, 0.0, 10.0], None), (-1.0, 10.0));
    assert_eq!(calc_data_range(&[0.0], Some(DType::UInt8)), (0.0, 255.0));
}

#[test]
fn segment_normal_matches_python_2d_and_3d_cases() {
    assert_eq!(
        segment_normal(&[1.0, 1.0], &[1.0, 10.0], [0.0, 0.0, 1.0]).unwrap(),
        vec![1.0, -0.0]
    );
    assert_eq!(
        segment_normal(&[1.0, 1.0, 0.0], &[1.0, 10.0, 0.0], [1.0, 0.0, 0.0],).unwrap(),
        vec![0.0, 0.0, -1.0]
    );
}

#[test]
fn segment_normal_returns_zero_vector_for_degenerate_segments_like_python() {
    assert_eq!(
        segment_normal(&[1.0, 1.0], &[1.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        vec![0.0, -0.0]
    );
}

#[test]
fn segment_normal_validates_shapes() {
    assert_eq!(
        segment_normal(&[1.0, 1.0], &[1.0], [0.0, 0.0, 1.0]),
        Err(LayerUtilsError::PointDimensionMismatch { a: 2, b: 1 })
    );
    assert_eq!(
        segment_normal(&[1.0], &[2.0], [0.0, 0.0, 1.0]),
        Err(LayerUtilsError::UnsupportedPointDimension(1))
    );
}

#[test]
fn get_current_properties_uses_last_property_values_when_data_exists() {
    let props = properties([
        (
            "face_color",
            vec![
                PropertyValue::String("cyan".to_owned()),
                PropertyValue::String("red".to_owned()),
                PropertyValue::String("red".to_owned()),
            ],
        ),
        (
            "angle",
            vec![PropertyValue::Float(0.5), PropertyValue::Float(1.5)],
        ),
    ]);

    let current = get_current_properties(&props, &Properties::new(), 3);

    assert_eq!(
        current,
        properties([
            ("angle", vec![PropertyValue::Float(1.5)]),
            ("face_color", vec![PropertyValue::String("red".to_owned())]),
        ])
    );
}

#[test]
fn get_current_properties_uses_first_choices_when_no_data_exists() {
    let choices = properties([
        (
            "face_color",
            vec![
                PropertyValue::String("cyan".to_owned()),
                PropertyValue::String("red".to_owned()),
            ],
        ),
        (
            "angle",
            vec![PropertyValue::Float(0.5), PropertyValue::Float(1.5)],
        ),
    ]);

    let current = get_current_properties(&Properties::new(), &choices, 0);

    assert_eq!(
        current,
        properties([
            ("angle", vec![PropertyValue::Float(0.5)]),
            ("face_color", vec![PropertyValue::String("cyan".to_owned())]),
        ])
    );
}

#[test]
fn coerce_current_properties_requires_single_value_properties() {
    let current = properties([
        ("annotation", vec![PropertyValue::String("leg".to_owned())]),
        ("confidence", vec![PropertyValue::Int(1)]),
    ]);
    assert_eq!(coerce_current_properties(&current), Ok(current));

    let invalid = properties([(
        "model",
        vec![
            PropertyValue::String("best".to_owned()),
            PropertyValue::String("best_v2_final".to_owned()),
        ],
    )]);
    assert_eq!(
        coerce_current_properties(&invalid),
        Err(LayerUtilsError::CurrentPropertyLength {
            key: "model".to_owned(),
            len: 2,
        })
    );
}

#[test]
fn validate_properties_matches_python_length_checks() {
    assert_eq!(validate_properties(None, None), Ok(Properties::new()));
    assert_eq!(
        validate_properties(Some(&Properties::new()), Some(3)),
        Ok(Properties::new())
    );

    let props = properties([
        (
            "label",
            vec![
                PropertyValue::String("a".to_owned()),
                PropertyValue::String("b".to_owned()),
            ],
        ),
        (
            "score",
            vec![PropertyValue::Float(0.25), PropertyValue::Float(0.75)],
        ),
    ]);
    assert_eq!(validate_properties(Some(&props), None), Ok(props.clone()));
    assert_eq!(
        validate_properties(Some(&props), Some(2)),
        Ok(props.clone())
    );

    let invalid = properties([
        ("label", vec![PropertyValue::String("a".to_owned())]),
        (
            "score",
            vec![PropertyValue::Float(0.25), PropertyValue::Float(0.75)],
        ),
    ]);
    assert_eq!(
        validate_properties(Some(&invalid), None),
        Err(LayerUtilsError::PropertyLengthMismatch {
            key: "score".to_owned(),
            expected: 1,
            found: 2,
        })
    );
    assert_eq!(
        validate_properties(Some(&invalid), Some(2)),
        Err(LayerUtilsError::PropertyLengthMismatch {
            key: "label".to_owned(),
            expected: 2,
            found: 1,
        })
    );
}

#[test]
fn validate_property_choices_returns_sorted_unique_values_like_python_helper() {
    assert_eq!(validate_property_choices(None), Properties::new());

    let choices = properties([
        (
            "class",
            vec![
                PropertyValue::String("b".to_owned()),
                PropertyValue::String("a".to_owned()),
                PropertyValue::String("b".to_owned()),
            ],
        ),
        (
            "visible",
            vec![
                PropertyValue::Bool(true),
                PropertyValue::Bool(false),
                PropertyValue::Bool(true),
            ],
        ),
    ]);

    assert_eq!(
        validate_property_choices(Some(&choices)),
        properties([
            (
                "class",
                vec![
                    PropertyValue::String("a".to_owned()),
                    PropertyValue::String("b".to_owned()),
                ],
            ),
            (
                "visible",
                vec![PropertyValue::Bool(false), PropertyValue::Bool(true)],
            ),
        ])
    );
}

#[test]
fn unique_element_matches_python_helper_for_scalar_and_nested_values() {
    assert_eq!(unique_element::<i64>(&[]), None);
    assert_eq!(unique_element(&[3, 3, 3]), Some(3));
    assert_eq!(unique_element(&[3, 4, 3]), None);
    assert_eq!(unique_element(&["sky", "sky"]), Some("sky"));
    assert_eq!(unique_element(&[vec![1, 2], vec![1, 2]]), Some(vec![1, 2]));
    assert_eq!(unique_element(&[vec![1, 2], vec![2, 1]]), None);
}

#[test]
fn coerce_affine_defaults_none_to_identity_with_requested_dimensionality() {
    let affine = coerce_affine(AffineInput::None, 3, Some("data2world".to_owned())).unwrap();

    assert_eq!(affine.name.as_deref(), Some("data2world"));
    assert_eq!(affine.ndim(), 3);
    assert_eq!(
        affine.affine_matrix(),
        vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    );
}

#[test]
fn coerce_affine_embeds_array_matrices_and_renames_existing_affines() {
    let matrix = vec![
        vec![2.0, 0.0, 10.0],
        vec![0.0, 3.0, 20.0],
        vec![0.0, 0.0, 1.0],
    ];
    let affine = coerce_affine(AffineInput::Matrix(matrix), 3, None).unwrap();

    assert_eq!(affine.ndim(), 3);
    assert_eq!(
        affine.affine_matrix(),
        vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 10.0],
            vec![0.0, 0.0, 3.0, 20.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    );

    let existing = Affine::new(
        vec![2.0, 3.0],
        vec![1.0, -1.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("old".to_owned()),
    );
    let renamed = coerce_affine(AffineInput::Affine(existing), 7, Some("new".to_owned())).unwrap();
    assert_eq!(renamed.ndim(), 2);
    assert_eq!(renamed.name.as_deref(), Some("new"));
}

#[test]
fn coerce_affine_rejects_unrecognized_inputs_like_python_helper() {
    assert_eq!(
        coerce_affine(AffineInput::Invalid, 2, None),
        Err(LayerUtilsError::UnrecognizedAffineInput)
    );
}

#[test]
fn get_extent_world_transforms_all_data_extent_corners_like_python_helper() {
    let affine = Affine::new(
        vec![2.0, 3.0],
        vec![10.0, -5.0],
        None,
        None,
        None,
        None,
        Some(2),
        None,
    );
    let extent = get_extent_world(&[vec![1.0, 2.0], vec![4.0, 6.0]], &affine).unwrap();

    assert_matrix_close(&extent, &[vec![12.0, 1.0], vec![18.0, 13.0]]);
}

#[test]
fn get_extent_world_uses_full_corner_mesh_for_rotated_or_sheared_extents() {
    let affine = Affine::new(
        vec![1.0, 1.0],
        vec![0.0, 0.0],
        None,
        None,
        Some(RotationInput::Angle2D(90.0)),
        None,
        Some(2),
        None,
    );
    let rotated = get_extent_world(&[vec![1.0, 2.0], vec![4.0, 6.0]], &affine).unwrap();
    assert_matrix_close(&rotated, &[vec![-6.0, 1.0], vec![-2.0, 4.0]]);

    let shear =
        Affine::from_linear_matrix(vec![vec![1.0, 2.0], vec![0.0, 1.0]], vec![0.0, 0.0], None);
    let sheared = get_extent_world(&[vec![1.0, 2.0], vec![4.0, 6.0]], &shear).unwrap();
    assert_matrix_close(&sheared, &[vec![5.0, 2.0], vec![16.0, 6.0]]);
}

#[test]
fn get_extent_world_validates_data_extent_shape() {
    let affine = Affine::default();
    assert_eq!(
        get_extent_world(&[vec![0.0, 0.0]], &affine),
        Err(LayerUtilsError::InvalidExtentRows { rows: 1 })
    );
    assert_eq!(
        get_extent_world(&[vec![0.0], vec![1.0, 2.0]], &affine),
        Err(LayerUtilsError::MismatchedExtentDimensions)
    );
}

#[test]
fn dims_displayed_world_to_layer_matches_python_cases() {
    for (dims_displayed, ndim_world, ndim_layer, expected) in [
        (vec![1, 2, 3], 4, 4, vec![1, 2, 3]),
        (vec![0, 1, 2], 4, 4, vec![0, 1, 2]),
        (vec![1, 2, 3], 4, 3, vec![0, 1, 2]),
        (vec![0, 1, 2], 4, 3, vec![2, 0, 1]),
        (vec![1, 2, 3], 4, 2, vec![0, 1]),
        (vec![0, 1, 2], 3, 3, vec![0, 1, 2]),
        (vec![0, 1], 2, 2, vec![0, 1]),
        (vec![1, 0], 2, 2, vec![1, 0]),
    ] {
        assert_eq!(
            dims_displayed_world_to_layer(&dims_displayed, ndim_world, ndim_layer),
            expected
        );
    }
}

#[test]
fn compute_multiscale_level_uses_highest_level_above_threshold_like_python() {
    let downsample_factors = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![4.0, 4.0]];

    assert_eq!(
        compute_multiscale_level(&[100.0, 100.0], &[30.0, 30.0], &downsample_factors),
        Ok(1)
    );
    assert_eq!(
        compute_multiscale_level(&[200.0, 200.0], &[30.0, 30.0], &downsample_factors),
        Ok(2)
    );
    assert_eq!(
        compute_multiscale_level(&[20.0, 20.0], &[30.0, 30.0], &downsample_factors),
        Ok(0)
    );
}

#[test]
fn compute_multiscale_level_and_corners_scales_and_rounds_python_style() {
    let downsample_factors = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![4.0, 4.0]];

    let result = compute_multiscale_level_and_corners(
        [[1.5, 2.5], [101.0, 103.0]],
        &[30.0, 30.0],
        &downsample_factors,
    );

    assert_eq!(result, Ok((1, [[0, 1], [51, 52]])));
}

#[test]
fn compute_multiscale_level_validates_input_dimensions() {
    assert_eq!(
        compute_multiscale_level(&[100.0, 100.0], &[30.0], &[vec![1.0, 1.0]]),
        Err(LayerUtilsError::MultiscaleDimensionMismatch {
            expected: 2,
            found: 1,
        })
    );
    assert_eq!(
        compute_multiscale_level(&[100.0, 100.0], &[30.0, 30.0], &[vec![1.0, 0.0]]),
        Err(LayerUtilsError::ZeroDownsampleFactor)
    );
}
