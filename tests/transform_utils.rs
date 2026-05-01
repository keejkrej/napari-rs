use napari_rs::utils::transforms::transform_utils::{
    RotationInput, ShearInput, compose_linear_matrix, embed_in_identity_matrix,
    expand_upper_triangular, infer_ndim, is_diagonal, is_matrix_lower_triangular,
    is_matrix_triangular, is_matrix_upper_triangular, rotate_to_matrix, scale_to_vector,
    shear_matrix_from_angle, shear_to_matrix, translate_to_vector,
};

const EPS: f64 = 1e-10;

#[test]
fn vector_converters_pad_leading_dimensions_like_python() {
    assert_vec_close(
        &translate_to_vector(Some(&[4.0, 18.0, 34.0]), 4),
        &[0.0, 4.0, 18.0, 34.0],
    );
    assert_vec_close(&translate_to_vector(None, 3), &[0.0, 0.0, 0.0]);
    assert_vec_close(
        &scale_to_vector(Some(&[4.0, 18.0, 34.0]), 4),
        &[1.0, 4.0, 18.0, 34.0],
    );
    assert_vec_close(&scale_to_vector(None, 3), &[1.0, 1.0, 1.0]);
}

#[test]
fn rotate_to_matrix_handles_2d_angles_and_embedding() {
    let rotation = rotate_to_matrix(Some(RotationInput::Angle2D(90.0)), 3);
    assert_matrix_close(
        &rotation,
        &[
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.0, -1.0],
            vec![0.0, 1.0, 0.0],
        ],
    );
}

#[test]
fn shear_to_matrix_expands_upper_triangular_vectors() {
    let shear = shear_to_matrix(Some(ShearInput::Vector(vec![1.0, 2.0, 3.0])), 3).unwrap();
    assert_matrix_close(
        &shear,
        &[
            vec![1.0, 1.0, 2.0],
            vec![0.0, 1.0, 3.0],
            vec![0.0, 0.0, 1.0],
        ],
    );
    assert!(expand_upper_triangular(&[1.0, 2.0]).is_err());
}

#[test]
fn compose_linear_matrix_preserves_python_order_rotate_shear_scale() {
    let rotate = RotationInput::Matrix(vec![vec![0.0, -1.0], vec![1.0, 0.0]]);
    let shear = ShearInput::Matrix(vec![vec![1.0, 3.0], vec![0.0, 1.0]]);
    let matrix = compose_linear_matrix(rotate, &[2.0, 5.0], shear).unwrap();

    assert_matrix_close(&matrix, &[vec![0.0, -5.0], vec![2.0, 15.0]]);
}

#[test]
fn infer_ndim_uses_largest_component_dimensionality() {
    assert_eq!(
        infer_ndim(
            Some(&[2.0, 3.0]),
            Some(&[1.0, 2.0, 3.0]),
            Some(RotationInput::Angle2D(45.0)),
            None
        ),
        Ok(3)
    );
}

#[test]
fn embedding_and_triangular_checks_match_python_helpers() {
    let embedded = embed_in_identity_matrix(&vec![vec![2.0, 0.0], vec![0.0, 3.0]], 4).unwrap();
    assert_matrix_close(
        &embedded,
        &[
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 2.0, 0.0],
            vec![0.0, 0.0, 0.0, 3.0],
        ],
    );

    let upper = vec![vec![1.0, 1.0], vec![0.0, 1.0]];
    let lower = vec![vec![1.0, 0.0], vec![1.0, 1.0]];
    let full = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
    assert!(is_matrix_upper_triangular(&upper));
    assert!(!is_matrix_upper_triangular(&lower));
    assert!(is_matrix_lower_triangular(&lower));
    assert!(!is_matrix_lower_triangular(&upper));
    assert!(is_matrix_triangular(&upper));
    assert!(is_matrix_triangular(&lower));
    assert!(!is_matrix_triangular(&full));
}

#[test]
fn diagonal_and_shear_angle_helpers_match_python_cases() {
    assert!(is_diagonal(&vec![vec![1.0, 0.0], vec![0.0, 1.0]], 1e-8).unwrap());
    assert!(!is_diagonal(&vec![vec![0.0, 1.0], vec![1.0, 0.0]], 1e-8).unwrap());
    let tiny = vec![
        vec![1.0, 1e-10, 1e-10],
        vec![1e-10, 1.0, 1e-10],
        vec![1e-10, 1e-10, 1.0],
    ];
    assert!(is_diagonal(&tiny, 1e-8).unwrap());
    assert!(!is_diagonal(&tiny, 1e-12).unwrap());

    let shear = shear_matrix_from_angle(35.0, 3, (2, 0));
    assert_vec_close(&[shear[0][0], shear[1][1], shear[2][2]], &[1.0, 1.0, 1.0]);
    assert_close(shear[2][0], 55.0_f64.to_radians().tan());
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= EPS,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_close(actual, expected);
    }
}

fn assert_matrix_close(actual: &[Vec<f64>], expected: &[Vec<f64>]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_vec_close(actual, expected);
    }
}
