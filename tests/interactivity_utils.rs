use napari_rs::layers::utils::interactivity_utils::{
    InteractivityError, displayed_plane_from_nd_line_segment, drag_data_to_projected_distance,
    nd_line_segment_to_displayed_data_ray,
};

fn assert_vec3_close(actual: [f64; 3], expected: [f64; 3]) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12);
    }
}

#[test]
fn displayed_plane_from_nd_line_segment_selects_displayed_dims_and_normalizes_direction() {
    let (plane_point, plane_normal) = displayed_plane_from_nd_line_segment(
        &[10.0, 1.0, 2.0, 3.0],
        &[20.0, 1.0, 2.0, 6.0],
        &[1, 2, 3],
    )
    .unwrap();

    assert_eq!(plane_point, [1.0, 2.0, 3.0]);
    assert_eq!(plane_normal, [0.0, 0.0, 1.0]);
}

#[test]
fn drag_data_to_projected_distance_matches_python_cases() {
    assert_eq!(
        drag_data_to_projected_distance(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            &[[1.0, 0.0, 0.0]],
        ),
        vec![0.0]
    );
    assert_eq!(
        drag_data_to_projected_distance(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        ),
        vec![0.0, 0.0]
    );
    assert_eq!(
        drag_data_to_projected_distance(
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            &[[0.0, 1.0, 0.0]],
        ),
        vec![1.0]
    );
}

#[test]
fn nd_line_segment_to_displayed_data_ray_normalizes_and_backs_up_start_position() {
    let (start_position, ray_direction) = nd_line_segment_to_displayed_data_ray(
        &[10.0, 1.0, 2.0, 3.0],
        &[20.0, 1.0, 2.0, 6.0],
        &[1, 2, 3],
    )
    .unwrap();

    assert_vec3_close(start_position, [1.0, 2.0, 2.9]);
    assert_eq!(ray_direction, [0.0, 0.0, 1.0]);
}

#[test]
fn interactivity_helpers_validate_displayed_dimensions() {
    assert_eq!(
        displayed_plane_from_nd_line_segment(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[0, 1]),
        Err(InteractivityError::DisplayedDimensionCount(2))
    );
    assert_eq!(
        nd_line_segment_to_displayed_data_ray(&[0.0, 0.0], &[1.0, 1.0], &[0, 1, 2]),
        Err(InteractivityError::DimensionOutOfBounds { dim: 2, ndim: 2 })
    );
    assert_eq!(
        nd_line_segment_to_displayed_data_ray(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], &[0, 1, 2]),
        Err(InteractivityError::ZeroLengthDirection)
    );
}
