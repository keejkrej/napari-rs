use napari_rs::utils::transforms::affine::{Affine, CompositeAffine, Transform, TransformChain};
use napari_rs::utils::transforms::scale_translate::ScaleTranslate;
use napari_rs::utils::transforms::transform_utils::{
    RotationInput, ShearInput, compose_linear_matrix, mat_vec_mul,
};
use napari_rs::utils::transforms::units::{Quantity, Unit};

const EPS: f64 = 1e-10;

#[test]
fn affine_transform_and_inverse_round_trip_metadata_vectors() {
    let affine = Affine::new(
        vec![2.0, 3.0],
        vec![1.0, -4.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("affine".to_owned()),
    );
    let point = vec![10.0, 13.0];
    let transformed = affine.transform_point(&point);
    assert_vec_close(&transformed, &[21.0, 35.0]);
    let restored = affine.inverse().transform_point(&transformed);
    assert_vec_close(&restored, &point);
}

#[test]
fn affine_defaults_include_python_axis_labels_units_and_physical_scale() {
    let affine = Affine::new(vec![], vec![], None, None, None, None, Some(3), None);

    assert_eq!(affine.axis_labels, vec!["-3", "-2", "-1"]);
    assert_eq!(affine.units, vec![Unit::Pixel, Unit::Pixel, Unit::Pixel]);
    assert_eq!(
        affine.physical_scale(),
        vec![
            Quantity {
                magnitude: 1.0,
                unit: Unit::Pixel,
            },
            Quantity {
                magnitude: 1.0,
                unit: Unit::Pixel,
            },
            Quantity {
                magnitude: 1.0,
                unit: Unit::Pixel,
            },
        ]
    );
}

#[test]
fn affine_units_and_axis_labels_validate_lengths_and_known_unit_names() {
    let mut affine = Affine::default();

    affine
        .set_axis_labels(vec!["x".to_owned(), "y".to_owned()])
        .unwrap();
    affine.set_units_from_names(&["mm", "cm"]).unwrap();

    assert_eq!(affine.axis_labels, vec!["x", "y"]);
    assert_eq!(affine.units, vec![Unit::Millimeter, Unit::Centimeter]);
    assert!(affine.set_axis_labels(vec!["x".to_owned()]).is_err());
    assert!(affine.set_units(vec![Unit::Meter]).is_err());
    assert!(affine.set_units_from_names(&["ugh", "ugh"]).is_err());
}

#[test]
fn affine_slice_preserves_metadata_and_inverse_preserves_units_like_python() {
    let mut affine = Affine::new(
        vec![2.0, 3.0, 4.0],
        vec![1.0, 2.0, 3.0],
        None,
        None,
        None,
        None,
        Some(3),
        None,
    );
    affine
        .set_axis_labels(vec!["z".to_owned(), "y".to_owned(), "x".to_owned()])
        .unwrap();
    affine.set_units_from_names(&["nm", "um", "mm"]).unwrap();

    let sliced = affine.set_slice(&[0, 2]);
    assert_eq!(sliced.axis_labels, vec!["z", "x"]);
    assert_eq!(sliced.units, vec![Unit::Nanometer, Unit::Millimeter]);
    assert_eq!(affine.inverse().units, affine.units);
}

#[test]
fn affine_composed_chain_matches_manual_application() {
    let scale = ScaleTranslate::new(vec![2.0], vec![4.0], Some("scale".to_owned()));
    let affine = Affine::new(
        vec![1.0, 3.0],
        vec![0.0, -1.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("affine".to_owned()),
    );
    let chain = TransformChain::new(vec![scale.clone().into(), affine.clone().into()]);

    let point = vec![10.0, 13.0];
    let from_chain = chain.transform_point(&point);
    let from_steps = affine.transform_point(&scale.transform_point(&point));

    assert_vec_close(&from_chain, &from_steps);
}

#[test]
fn transform_chain_inverse_restores_input() {
    let scale = ScaleTranslate::new(vec![2.0], vec![4.0], Some("scale".to_owned()));
    let affine_matrix = vec![
        vec![0.0, -1.0, 1.0],
        vec![1.0, 0.0, 2.0],
        vec![0.0, 0.0, 1.0],
    ];
    let rotate = Affine::from_affine_matrix(affine_matrix, Some("rotate".to_owned()));
    let chain = TransformChain::new(vec![scale.clone().into(), rotate.clone().into()]);

    let point = vec![10.0, 13.0];
    let reconstructed = chain
        .inverse()
        .transform_point(&chain.transform_point(&point));
    assert_vec_close(&reconstructed, &point);
}

#[test]
fn transform_chain_simplified_is_composable_affine() {
    let scale = ScaleTranslate::new(vec![2.0], vec![1.0], Some("scale".to_owned()));
    let affine = Affine::new(
        vec![4.0],
        vec![3.0],
        None,
        None,
        None,
        None,
        Some(1),
        Some("affine".to_owned()),
    );
    let chain = TransformChain::new(vec![scale.into(), affine.into()]);
    let simplified = chain.simplified().unwrap();
    let point = vec![10.0];
    assert_vec_close(
        &simplified.transform_point(&point),
        &chain.transform_point(&point),
    );
}

#[test]
fn transform_chain_simplified_preserves_order_for_non_commuting_affines() {
    let translate = Affine::new(
        vec![1.0, 1.0],
        vec![2.0, -1.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("translate".to_owned()),
    );
    let rotate = Affine::new(
        vec![1.0, 1.0],
        vec![0.0, 0.0],
        None,
        None,
        Some(RotationInput::Angle2D(90.0)),
        None,
        Some(2),
        Some("rotate".to_owned()),
    );
    let chain = TransformChain::new(vec![translate.into(), rotate.into()]);
    let simplified = chain.simplified().unwrap();
    let point = vec![3.0, 5.0];

    assert_vec_close(
        &simplified.transform_point(&point),
        &chain.transform_point(&point),
    );
}

#[test]
fn composite_affine_set_slice_and_expand_dims_round_trip_behavior() {
    let composite = CompositeAffine::new(
        vec![2.0, 3.0],
        vec![1.0, -2.0],
        None,
        None,
        Some(2),
        Some("composite".to_owned()),
    );

    let sliced = composite.set_slice(&[0]);
    let expanded = sliced.expand_dims(&[1]);
    assert_vec_close(&composite.set_slice(&[0]).scale, &sliced.scale);
    assert_eq!(expanded.ndim(), 2);
    assert_vec_close(&expanded.scale, &[2.0, 1.0]); // expanded axis insertions keep first/last axes.
    let point = vec![1.0, 2.0];
    assert_vec_close(
        &expanded.transform_point(&point),
        &sliced.expand_dims(&[1]).transform_point(&point),
    );
}

#[test]
fn composite_affine_slice_round_trips_through_set_slice() {
    let composite = CompositeAffine::new(
        vec![2.0, 3.0, 4.0],
        vec![1.0, 2.0, 3.0],
        None,
        None,
        Some(3),
        Some("composite".to_owned()),
    );
    let sliced = composite.set_slice(&[0, 2]);
    let rebuilt = sliced.expand_dims(&[1]);
    assert_eq!(rebuilt.scale, vec![2.0, 1.0, 4.0]);
    assert_eq!(rebuilt.ndim(), 3);
}

#[test]
fn composite_affine_metadata_delegates_to_base_affine() {
    let mut composite = CompositeAffine::new(
        vec![2.0, 3.0, 4.0],
        vec![1.0, 2.0, 3.0],
        None,
        None,
        Some(3),
        None,
    );
    composite
        .set_axis_labels(vec!["z".to_owned(), "y".to_owned(), "x".to_owned()])
        .unwrap();
    composite.set_units_from_names(&["nm", "um", "mm"]).unwrap();

    let sliced = composite.set_slice(&[1, 2]);
    assert_eq!(sliced.base.axis_labels, vec!["y", "x"]);
    assert_eq!(sliced.base.units, vec![Unit::Micrometer, Unit::Millimeter]);
    assert_eq!(
        sliced.physical_scale(),
        vec![
            Quantity {
                magnitude: 3.0,
                unit: Unit::Micrometer,
            },
            Quantity {
                magnitude: 4.0,
                unit: Unit::Millimeter,
            },
        ]
    );
}

#[test]
fn affine_decompose_linear_matrix_rebuilds_similar_matrix() {
    let linear = vec![vec![2.0, 1.0], vec![0.0, 3.0]];
    let affine =
        Affine::from_linear_matrix(linear.clone(), vec![0.0, 0.0], Some("rebuild".to_owned()));
    let (rotate, scale, shear) = affine.decompose_linear_matrix();
    let rebuilt = compose_linear_matrix(RotationInput::Matrix(rotate), &scale, shear).unwrap();
    assert_matrix_close(&rebuilt, &linear, 1e-8);
}

#[test]
fn affine_decompose_linear_matrix_rebuilds_lower_triangular_style_matrix() {
    let linear = vec![vec![2.0, 0.0], vec![0.5, 3.0]];
    let mut affine = Affine::from_linear_matrix(linear.clone(), vec![0.0, 0.0], None);
    affine.upper_triangular = false;
    let (rotate, scale, shear) = affine.decompose_linear_matrix();
    let rebuilt = compose_linear_matrix(RotationInput::Matrix(rotate), &scale, shear).unwrap();
    assert_matrix_close(&rebuilt, &linear, 1e-8);
}

#[test]
fn affine_setters_rebuild_transform_in_a_round_trip() {
    let linear = vec![vec![2.0, 1.0], vec![0.0, 3.0]];
    let mut affine =
        Affine::from_linear_matrix(linear, vec![1.0, -2.0], Some("setters".to_owned()));

    affine.set_scale(vec![4.0]);
    affine.set_rotate(RotationInput::Angle2D(45.0));
    affine.set_shear(ShearInput::Vector(vec![0.25]));

    let point = vec![1.0, 2.0];
    let rotate = affine.rotate();
    let scale = affine.scale();
    let shear = affine.shear();
    let linear = compose_linear_matrix(RotationInput::Matrix(rotate), &scale, shear).unwrap();
    let expected = mat_vec_mul(&linear, &point);
    let expected = expected
        .iter()
        .zip([1.0, -2.0].iter())
        .map(|(value, shift)| value + shift)
        .collect::<Vec<_>>();
    assert_vec_close(&affine.transform_point(&point), &expected);
}

#[test]
fn affine_diagonal_rotation_property_is_identity() {
    let affine = Affine::new(
        vec![2.0, 3.0],
        vec![8.0, -5.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("diagonal".to_owned()),
    );

    assert_matrix_close(&affine.rotate(), &[vec![1.0, 0.0], vec![0.0, 1.0]], 1e-10);
}

#[test]
fn affine_set_shear_after_diagonal_scale_composes_without_double_scaling() {
    let mut affine = Affine::new(
        vec![2.0, 3.0],
        vec![0.0, 0.0],
        None,
        None,
        None,
        None,
        Some(2),
        None,
    );
    affine.set_shear(ShearInput::Vector(vec![1.0]));

    let expected = compose_linear_matrix(
        RotationInput::Matrix(vec![vec![1.0, 0.0], vec![0.0, 1.0]]),
        &[2.0, 3.0],
        ShearInput::Vector(vec![1.0]),
    )
    .unwrap();
    assert_matrix_close(&affine.linear_matrix, &expected, 1e-10);
}

#[test]
fn affine_constructor_preserves_lower_triangular_shear_decomposition() {
    let shear = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.5, 0.0, 1.0],
    ];
    let affine = Affine::new(
        vec![1.0, 1.0, 1.0],
        vec![0.0, 0.0, 0.0],
        None,
        None,
        None,
        Some(ShearInput::Matrix(shear.clone())),
        Some(3),
        None,
    );

    assert_eq!(affine.shear(), ShearInput::Matrix(shear));
}

#[test]
fn affine_scale_setter_updates_diagonal_matrix_directly() {
    let mut affine = Affine::default();
    affine.set_scale(vec![3.0, 4.0]);
    let point = vec![2.0, 5.0];
    let transformed = affine.transform_point(&point);
    assert_vec_close(&transformed, &[6.0, 20.0]);
}

#[test]
fn transform_compose_builds_chain_and_chain_compose_pushes_ordered_transform() {
    let scale = Transform::from(ScaleTranslate::new(vec![2.0], vec![4.0], None));
    let affine = Transform::from(Affine::new(
        vec![1.0, 3.0],
        vec![0.0, -1.0],
        None,
        None,
        None,
        None,
        Some(2),
        None,
    ));
    let chain = scale.compose(&affine);
    assert_eq!(chain.transforms.len(), 2);
    let point = vec![1.0, 2.0];
    let expected = affine.transform_point(&scale.transform_point(&point));
    assert_vec_close(&chain.transform_point(&point), &expected);

    let extended = chain.compose(scale.clone());
    assert_eq!(extended.transforms.len(), 3);
    let direct = chain.compose(scale.clone()).transform_point(&point);
    assert_vec_close(&extended.transform_point(&point), &direct);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= EPS,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&a, &b) in actual.iter().zip(expected.iter()) {
        assert_close(a, b);
    }
}

fn assert_matrix_close(actual: &[Vec<f64>], expected: &[Vec<f64>], tol: f64) {
    assert_eq!(actual.len(), expected.len());
    for (actual_row, expected_row) in actual.iter().zip(expected) {
        assert_eq!(actual_row.len(), expected_row.len());
        for (&a, &b) in actual_row.iter().zip(expected_row) {
            assert!((a - b).abs() <= tol, "expected {b}, got {a}");
        }
    }
}
