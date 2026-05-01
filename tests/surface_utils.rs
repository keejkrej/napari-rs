use napari_rs::layers::surface::surface_utils::{
    BarycentricError, calculate_barycentric_coordinates,
};

fn triangle() -> Vec<Vec<f64>> {
    vec![
        vec![5.0, 0.0, 0.0],
        vec![5.0, 0.0, 3.0],
        vec![5.0, 3.0, 0.0],
    ]
}

fn assert_close(actual: [f64; 3], expected: [f64; 3]) {
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert!((actual - expected).abs() < 1e-12);
    }
}

#[test]
fn barycentric_coordinates_match_python_surface_utils_cases() {
    for (point, expected) in [
        ([5.0, 1.0, 1.0], [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]),
        ([5.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        ([5.0, 0.0, 3.0], [0.0, 1.0, 0.0]),
        ([5.0, 3.0, 0.0], [0.0, 0.0, 1.0]),
    ] {
        let barycentric = calculate_barycentric_coordinates(&point, &triangle()).unwrap();

        assert_close(barycentric, expected);
        assert!((barycentric.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }
}

#[test]
fn barycentric_coordinates_reject_invalid_input_shapes() {
    assert_eq!(
        calculate_barycentric_coordinates(&[0.0, 0.0], &[vec![0.0, 0.0]]),
        Err(BarycentricError::TriangleVertexCount(1))
    );
    assert_eq!(
        calculate_barycentric_coordinates(&[0.0, 0.0], &triangle()),
        Err(BarycentricError::DimensionMismatch)
    );
    assert_eq!(
        calculate_barycentric_coordinates(
            &[0.0, 0.0],
            &[vec![0.0, 0.0], vec![1.0, 1.0], vec![2.0, 2.0]],
        ),
        Err(BarycentricError::DegenerateTriangle)
    );
}
