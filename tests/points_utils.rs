use napari_rs::layers::points::constants::Symbol;
use napari_rs::layers::points::points_utils::{
    PointsUtilsError, RawPointData, coerce_symbols, create_box, create_box_from_corners_3d,
    fix_data_points, points_in_box, points_in_box_3d, points_to_squares, symbol_conversion,
};

#[test]
fn create_box_from_corners_3d_matches_python_helper() {
    let box_corners = [[5.0, 0.0, 0.0], [5.0, 10.0, 10.0]];
    let normal = [1.0, 0.0, 0.0];
    let up_direction = [0.0, 1.0, 0.0];

    assert_eq!(
        create_box_from_corners_3d(box_corners, normal, up_direction),
        [
            [5.0, 0.0, 0.0],
            [5.0, 0.0, 10.0],
            [5.0, 10.0, 10.0],
            [5.0, 10.0, 0.0],
        ]
    );
}

#[test]
fn create_box_returns_axis_aligned_2d_box() {
    assert_eq!(
        create_box(&[[3.0, 4.0], [1.0, 8.0], [5.0, 2.0]]),
        Ok([[1.0, 2.0], [5.0, 2.0], [5.0, 8.0], [1.0, 8.0]])
    );
    assert_eq!(create_box(&[]), Err(PointsUtilsError::EmptyData));
}

#[test]
fn points_to_squares_matches_python_concatenation_order() {
    assert_eq!(
        points_to_squares(&[[1.0, 1.0], [4.0, 4.0]], &[2.0, 4.0]).unwrap(),
        vec![
            [2.0, 2.0],
            [6.0, 6.0],
            [2.0, 0.0],
            [6.0, 2.0],
            [0.0, 2.0],
            [2.0, 6.0],
            [0.0, 0.0],
            [2.0, 2.0],
        ]
    );
}

#[test]
fn points_in_box_selects_points_with_any_square_corner_inside_box() {
    let corners = [[0.0, 0.0], [10.0, 10.0]];
    let points = [[5.0, 5.0], [12.0, 12.0], [10.5, 5.0], [-2.0, 5.0]];
    let sizes = [1.0, 1.0, 2.0, 2.0];

    assert_eq!(points_in_box(&corners, &points, &sizes), Ok(vec![0, 2]));
}

#[test]
fn points_in_box_3d_matches_python_helper_case() {
    let normal = [1.0, 0.0, 0.0];
    let up_direction = [0.0, 1.0, 0.0];
    let corners = [[10.0, 10.0, 10.0], [10.0, 20.0, 20.0]];
    let points = [
        [0.0, 15.0, 15.0],
        [10.0, 30.0, 25.0],
        [10.0, 12.0, 18.0],
        [20.0, 15.0, 30.0],
    ];
    let sizes = [1.0; 4];

    assert_eq!(
        points_in_box_3d(corners, &points, &sizes, normal, up_direction),
        Ok(vec![0, 2])
    );
}

#[test]
fn fix_data_points_defaults_empty_data_and_preserves_non_empty_ndim() {
    assert_eq!(fix_data_points(None, None), Ok((Vec::new(), 2)));
    assert_eq!(
        fix_data_points(Some(RawPointData::Empty), Some(3)),
        Ok((Vec::new(), 3))
    );
    assert_eq!(
        fix_data_points(
            Some(RawPointData::Points(vec![vec![1.0, 2.0], vec![3.0, 4.0]])),
            None,
        ),
        Ok((vec![vec![1.0, 2.0], vec![3.0, 4.0]], 2))
    );
}

#[test]
fn fix_data_points_handles_single_point_like_numpy_atleast_2d() {
    assert_eq!(
        fix_data_points(Some(RawPointData::Single(vec![1.0, 2.0, 3.0])), None),
        Ok((vec![vec![1.0, 2.0, 3.0]], 3))
    );
}

#[test]
fn fix_data_points_rejects_incompatible_ndim_and_ragged_data() {
    assert_eq!(
        fix_data_points(Some(RawPointData::Single(vec![1.0, 2.0])), Some(3)),
        Err(PointsUtilsError::DimensionMismatch {
            expected: 3,
            found: 2,
        })
    );
    assert_eq!(
        fix_data_points(
            Some(RawPointData::Points(vec![vec![1.0, 2.0], vec![3.0]])),
            None,
        ),
        Err(PointsUtilsError::DimensionMismatch {
            expected: 2,
            found: 1,
        })
    );
}

#[test]
fn symbol_helpers_convert_aliases_and_sequences() {
    assert_eq!(symbol_conversion(">"), Ok(Symbol::Arrow));
    assert_eq!(symbol_conversion("triangle up"), Ok(Symbol::TriangleUp));
    assert_eq!(
        coerce_symbols(&[">", "o", "tailed_arrow"]),
        Ok(vec![Symbol::Arrow, Symbol::Disc, Symbol::TailedArrow])
    );
}
