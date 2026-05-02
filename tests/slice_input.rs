use napari_rs::layers::utils::slice_input::{
    SliceInput, SliceInputError, ThickNdSlice, apply_units_to_transform,
};
use napari_rs::utils::transforms::affine::Affine;
use napari_rs::utils::transforms::units::Unit;

#[test]
fn thick_nd_slice_make_full_fills_missing_values_and_adjusts_ndim_like_python() {
    assert_eq!(
        ThickNdSlice::make_full(None, None, None, Some(3)).unwrap(),
        ThickNdSlice {
            point: vec![0, 0, 0],
            margin_left: vec![0, 0, 0],
            margin_right: vec![0, 0, 0],
        }
    );

    assert_eq!(
        ThickNdSlice::make_full(Some(&[1, 2]), None, Some(&[3, 4]), Some(4)).unwrap(),
        ThickNdSlice {
            point: vec![0, 0, 1, 2],
            margin_left: vec![0, 0, 0, 0],
            margin_right: vec![0, 0, 3, 4],
        }
    );

    assert_eq!(
        ThickNdSlice::make_full(Some(&[1, 2, 3, 4]), None, None, Some(2)).unwrap(),
        ThickNdSlice {
            point: vec![3, 4],
            margin_left: vec![0, 0],
            margin_right: vec![0, 0],
        }
    );
}

#[test]
fn thick_nd_slice_requires_ndim_when_no_values_are_given() {
    assert_eq!(
        ThickNdSlice::<i32>::make_full(None, None, None, None),
        Err(SliceInputError::MissingNdim)
    );
}

#[test]
fn thick_nd_slice_rows_and_axis_selection_match_python_helpers() {
    let thick_slice = ThickNdSlice {
        point: vec![1, 2, 3],
        margin_left: vec![4, 5, 6],
        margin_right: vec![7, 8, 9],
    };

    assert_eq!(
        thick_slice.as_rows(),
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
    assert_eq!(
        ThickNdSlice::from_rows(&thick_slice.as_rows()).unwrap(),
        thick_slice
    );
    assert_eq!(
        thick_slice.select_axes(&[2, 0]),
        ThickNdSlice {
            point: vec![3, 1],
            margin_left: vec![6, 4],
            margin_right: vec![9, 7],
        }
    );

    let per_dim: Vec<_> = thick_slice.iter_by_dimension().collect();
    assert_eq!(per_dim, vec![(&1, &4, &7), (&2, &5, &8), (&3, &6, &9)]);
}

#[test]
fn thick_nd_slice_from_rows_validates_shape() {
    assert_eq!(
        ThickNdSlice::<i32>::from_rows(&[vec![1], vec![2]]),
        Err(SliceInputError::InvalidArrayRows { rows: 2 })
    );
    assert_eq!(
        ThickNdSlice::from_rows(&[vec![1], vec![2, 3], vec![4]]),
        Err(SliceInputError::MismatchedArrayRowLengths)
    );
}

#[test]
fn slice_input_reports_displayed_and_not_displayed_axes_from_order() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(4)).unwrap(),
        order: vec![2, 0, 3, 1],
    };

    assert_eq!(slice_input.ndim(), 4);
    assert_eq!(slice_input.displayed(), vec![3, 1]);
    assert_eq!(slice_input.not_displayed(), vec![2, 0]);
}

#[test]
fn slice_input_with_ndim_preserves_order_when_dimensions_are_dropped_or_added() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice {
            point: vec![1.0, 2.0, 3.0],
            margin_left: vec![0.1, 0.2, 0.3],
            margin_right: vec![0.4, 0.5, 0.6],
        },
        order: vec![2, 0, 1],
    };

    let reduced = slice_input.with_ndim(2).unwrap();
    assert_eq!(reduced.order, vec![0, 1]);
    assert_eq!(
        reduced.world_slice,
        ThickNdSlice {
            point: vec![2.0, 3.0],
            margin_left: vec![0.2, 0.3],
            margin_right: vec![0.5, 0.6],
        }
    );

    let expanded = slice_input.with_ndim(5).unwrap();
    assert_eq!(expanded.order, vec![0, 1, 4, 2, 3]);
    assert_eq!(
        expanded.world_slice,
        ThickNdSlice {
            point: vec![0.0, 0.0, 1.0, 2.0, 3.0],
            margin_left: vec![0.0, 0.0, 0.1, 0.2, 0.3],
            margin_right: vec![0.0, 0.0, 0.4, 0.5, 0.6],
        }
    );
}

#[test]
fn slice_input_orthogonality_matches_python_non_displayed_subspace_check() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(3)).unwrap(),
        order: vec![0, 1, 2],
    };

    assert!(
        slice_input
            .is_orthogonal_with_linear_matrix(&[
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ])
            .unwrap()
    );
    assert!(
        slice_input
            .is_orthogonal_with_linear_matrix(&[
                vec![1.0, 0.0, 0.0],
                vec![0.5, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ])
            .is_ok_and(|orthogonal| !orthogonal)
    );
}

#[test]
fn slice_input_orthogonality_uses_ordered_displayed_axes() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(3)).unwrap(),
        order: vec![2, 0, 1],
    };

    assert!(
        slice_input
            .is_orthogonal_with_linear_matrix(&[
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ])
            .unwrap()
    );
    assert!(
        !slice_input
            .is_orthogonal_with_linear_matrix(&[
                vec![1.0, 0.0, 0.25],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ])
            .unwrap()
    );
}

#[test]
fn slice_input_orthogonality_validates_linear_matrix_shape() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(3)).unwrap(),
        order: vec![0, 1, 2],
    };

    assert_eq!(
        slice_input.is_orthogonal_with_linear_matrix(&[vec![1.0], vec![0.0]]),
        Err(SliceInputError::InvalidLinearMatrix {
            rows: 2,
            expected: 3
        })
    );
    assert_eq!(
        slice_input.is_orthogonal_with_linear_matrix(&[
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ]),
        Err(SliceInputError::InvalidLinearMatrixRow {
            row: 1,
            columns: 2,
            expected: 3
        })
    );
}

#[test]
fn slice_input_data_slice_transforms_only_not_displayed_axes() {
    let slice_input = SliceInput {
        ndisplay: 2,
        world_slice: ThickNdSlice {
            point: vec![10.0, 20.0, 30.0],
            margin_left: vec![1.0, 2.0, 3.0],
            margin_right: vec![4.0, 5.0, 6.0],
        },
        order: vec![0, 1, 2],
    };
    let world_to_data = Affine::new(
        vec![2.0, 3.0, 4.0],
        vec![100.0, 200.0, 300.0],
        None,
        None,
        None,
        None,
        Some(3),
        None,
    );

    let data_slice = slice_input.data_slice(&world_to_data);

    assert!(data_slice.point[1].is_nan());
    assert!(data_slice.point[2].is_nan());
    assert!(data_slice.margin_left[1].is_nan());
    assert!(data_slice.margin_right[2].is_nan());
    assert_vec_close(&data_slice.point[..1], &[120.0]);
    assert_vec_close(&data_slice.margin_left[..1], &[2.0]);
    assert_vec_close(&data_slice.margin_right[..1], &[8.0]);
}

#[test]
fn slice_input_data_slice_uses_axis_order_and_preserves_displayed_nans() {
    let slice_input = SliceInput {
        ndisplay: 1,
        world_slice: ThickNdSlice {
            point: vec![10.0, 20.0, 30.0],
            margin_left: vec![1.0, f64::NAN, 3.0],
            margin_right: vec![4.0, 5.0, f64::NAN],
        },
        order: vec![2, 0, 1],
    };
    let world_to_data = Affine::new(
        vec![2.0, 3.0, 4.0],
        vec![100.0, 200.0, 300.0],
        None,
        None,
        None,
        None,
        Some(3),
        None,
    );

    let data_slice = slice_input.data_slice(&world_to_data);

    assert_vec_close(&data_slice.point, &[120.0, f64::NAN, 420.0]);
    assert_vec_close(&data_slice.margin_left, &[2.0, f64::NAN, 12.0]);
    assert_vec_close(&data_slice.margin_right, &[0.0, f64::NAN, 0.0]);
}

#[test]
fn apply_units_to_transform_scales_linear_matrix_by_trailing_world_units() {
    let mut data_to_world = Affine::new(
        vec![2.0, 3.0],
        vec![10.0, 20.0],
        None,
        None,
        None,
        None,
        Some(2),
        Some("data2world".to_owned()),
    );
    data_to_world
        .set_units(vec![Unit::Millimeter, Unit::Centimeter])
        .unwrap();

    let transformed = apply_units_to_transform(
        &data_to_world,
        Some(&[Unit::Pixel, Unit::Meter, Unit::Millimeter]),
    );

    assert_matrix_close(
        &transformed.linear_matrix,
        &[vec![2_000.0, 0.0], vec![0.0, 0.3]],
    );
    assert_eq!(transformed.translate, vec![10.0, 20.0]);
    assert_eq!(transformed.units, vec![Unit::Pixel, Unit::Pixel]);
    assert_eq!(transformed.name, None);
}

#[test]
fn apply_units_to_transform_returns_original_when_units_are_absent_or_too_short() {
    let mut data_to_world = Affine::default();
    data_to_world
        .set_units(vec![Unit::Millimeter, Unit::Millimeter])
        .unwrap();

    assert_eq!(
        apply_units_to_transform(&data_to_world, None),
        data_to_world
    );
    assert_eq!(
        apply_units_to_transform(&data_to_world, Some(&[Unit::Meter])),
        data_to_world
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        if expected.is_nan() {
            assert!(actual.is_nan(), "expected NaN, got {actual}");
        } else {
            assert!(
                (actual - expected).abs() <= 1e-10,
                "expected {expected}, got {actual}"
            );
        }
    }
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
