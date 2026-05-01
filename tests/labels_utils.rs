use napari_rs::layers::labels::labels_utils::{
    DenseLabels, LabelsUtilsError, SliceRange, expand_slice, first_nonzero_coordinate,
    indices_in_shape, interpolate_coordinates, sphere_indices,
};

#[test]
fn interpolate_coordinates_matches_python_labels_helper() {
    let coords = interpolate_coordinates(Some(&[0.0, 1.0]), Some(&[0.0, 10.0]), 3.0).unwrap();

    assert_eq!(
        coords,
        vec![
            vec![0.0, 1.75],
            vec![0.0, 2.5],
            vec![0.0, 3.25],
            vec![0.0, 4.0],
            vec![0.0, 4.75],
            vec![0.0, 5.5],
            vec![0.0, 6.25],
            vec![0.0, 7.0],
            vec![0.0, 7.75],
            vec![0.0, 8.5],
            vec![0.0, 9.25],
            vec![0.0, 10.0],
        ]
    );
}

#[test]
fn interpolate_coordinates_with_missing_endpoint_returns_single_coordinate() {
    assert_eq!(
        interpolate_coordinates(Some(&[5.0, 5.0]), None, 1.0),
        Ok(vec![vec![5.0, 5.0]])
    );
    assert_eq!(
        interpolate_coordinates(None, Some(&[5.0, 5.0]), 5.0),
        Ok(vec![vec![5.0, 5.0]])
    );
}

#[test]
fn sphere_indices_generates_centered_integer_indices_inside_scaled_radius() {
    assert_eq!(
        sphere_indices(1.0, &[1.0, 1.0]).unwrap(),
        vec![vec![-1, 0], vec![0, -1], vec![0, 0], vec![0, 1], vec![1, 0],]
    );
    assert_eq!(
        sphere_indices(1.0, &[0.0]),
        Err(LabelsUtilsError::ScaleContainsZero)
    );
}

#[test]
fn indices_in_shape_filters_coordinates_like_python_helper() {
    let idxs = vec![vec![5, 6], vec![45, 5], vec![2, -5]];

    assert_eq!(indices_in_shape(&idxs, &[10, 10]), vec![vec![5, 6]]);
}

#[test]
fn first_nonzero_coordinate_matches_python_ray_cases() {
    let mut values = vec![0; 11 * 11 * 11];
    for z in 4..7 {
        for y in 4..7 {
            for x in 4..7 {
                values[z * 11 * 11 + y * 11 + x] = 1;
            }
        }
    }
    let data = DenseLabels::new([11, 11, 11], values).unwrap();

    assert_eq!(
        first_nonzero_coordinate(&data, &[0.0, 0.0, 0.0], &[10.0, 10.0, 10.0]).unwrap(),
        Some(vec![4, 4, 4])
    );
    assert_eq!(
        first_nonzero_coordinate(&data, &[10.0, 10.0, 10.0], &[0.0, 0.0, 0.0]).unwrap(),
        Some(vec![6, 6, 6])
    );
    assert_eq!(
        first_nonzero_coordinate(&data, &[0.0, 0.0, 0.0], &[0.0, 1.0, 1.0]).unwrap(),
        None
    );
    assert_eq!(
        first_nonzero_coordinate(&data, &[0.0, 6.0, 6.0], &[10.0, 5.0, 5.0]).unwrap(),
        Some(vec![4, 6, 6])
    );
}

#[test]
fn dense_labels_validates_flat_data_length() {
    assert_eq!(
        DenseLabels::new([2, 2], vec![0, 1, 2]),
        Err(LabelsUtilsError::DataLengthMismatch {
            expected: 4,
            found: 3,
        })
    );
}

#[test]
fn expand_slice_expands_and_clamps_like_python_helper() {
    let slices = vec![SliceRange::new(2, 5, 1), SliceRange::new(0, 3, 1)];

    assert_eq!(
        expand_slice(&slices, &[6, 4], 2),
        vec![SliceRange::new(0, 6, 1), SliceRange::new(0, 4, 1)]
    );
    assert_eq!(
        expand_slice(&slices, &[6, 4], -1),
        vec![SliceRange::new(3, 4, 1), SliceRange::new(1, 2, 1)]
    );
}
