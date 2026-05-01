use napari_rs::utils::geometry::{
    BoxFace, Vec2, Vec3, bounding_box_to_face_vertices, clamp_point_to_bounding_box,
    clamp_points_to_bounding_box, distance_between_point_and_line_3d,
    face_coordinate_from_bounding_box, find_front_back_face, find_nearest_triangle_intersection,
    inside_triangles, intersect_line_with_axis_aligned_bounding_box_3d,
    intersect_line_with_axis_aligned_plane, intersect_line_with_multiple_planes_3d,
    intersect_line_with_plane_3d, line_in_quadrilateral_3d, line_in_triangles_3d,
    point_in_quadrilateral_2d, project_points_onto_plane, rotation_matrix_from_vectors_2d,
    rotation_matrix_from_vectors_3d,
};

const EPS: f64 = 1e-10;

#[test]
fn project_point_to_plane() {
    let points = [[10.0, 10.0, 10.0]];
    let (projected_points, distances) =
        project_points_onto_plane(&points, [20.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

    assert_vec3_close(projected_points[0], [10.0, 0.0, 10.0]);
    assert_close(distances[0], 10.0);
}

#[test]
fn project_multiple_points_to_plane() {
    let points = [
        [10.0, 10.0, 10.0],
        [20.0, 10.0, 30.0],
        [20.0, 40.0, 20.0],
        [10.0, -5.0, 30.0],
    ];
    let (projected_points, distances) =
        project_points_onto_plane(&points, [20.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

    assert_vec3_close(projected_points[0], [10.0, 0.0, 10.0]);
    assert_vec3_close(projected_points[1], [20.0, 0.0, 30.0]);
    assert_vec3_close(projected_points[2], [20.0, 0.0, 20.0]);
    assert_vec3_close(projected_points[3], [10.0, 0.0, 30.0]);
    assert_slice_close(&distances, &[10.0, 10.0, 40.0, -5.0]);
}

#[test]
fn rotation_matrix_from_vectors_2d_aligns_vectors() {
    for (vec_1, vec_2) in [
        ([10.0, 0.0], [0.0, 5.0]),
        ([0.0, 5.0], [0.0, 5.0]),
        ([0.0, 5.0], [0.0, -5.0]),
    ] {
        let rotation_matrix = rotation_matrix_from_vectors_2d(vec_1, vec_2);
        let rotated = mat2_vec_mul(rotation_matrix, vec_1);
        assert_vec2_close(normalize2(rotated), normalize2(vec_2));
    }
}

#[test]
fn rotation_matrix_from_vectors_3d_aligns_vectors() {
    for (vec_1, vec_2) in [
        ([10.0, 0.0, 0.0], [0.0, 5.0, 0.0]),
        ([0.0, 5.0, 0.0], [0.0, 5.0, 0.0]),
        ([0.0, 5.0, 0.0], [0.0, -5.0, 0.0]),
    ] {
        let rotation_matrix = rotation_matrix_from_vectors_3d(vec_1, vec_2);
        let rotated = mat3_vec_mul(rotation_matrix, vec_1);
        assert_vec3_close(normalize3(rotated), normalize3(vec_2));
    }
}

#[test]
fn intersect_line_with_plane_3d_matches_python_cases() {
    for (line_position, line_direction, plane_position, plane_normal, expected) in [
        (
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
        ),
        (
            [1.0, 1.0, 1.0],
            [-1.0, -1.0, -1.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
        ),
        (
            [2.0, 2.0, 2.0],
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
        ),
    ] {
        let intersection = intersect_line_with_plane_3d(
            line_position,
            line_direction,
            plane_position,
            plane_normal,
        );
        assert_vec3_close(intersection, expected);
    }
}

#[test]
fn intersect_line_with_multiple_planes_3d_matches_python_case() {
    let intersections = intersect_line_with_multiple_planes_3d(
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        &[[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        &[[0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    );

    assert_vec3_close(intersections[0], [0.0, 0.0, 0.0]);
    assert_vec3_close(intersections[1], [0.0, 0.0, 1.0]);
}

#[test]
fn clamp_point_to_bounding_box_matches_python_cases() {
    let bbox = [[0.0, 10.0], [0.0, 10.0], [0.0, 10.0]];

    assert_slice_close(
        &clamp_point_to_bounding_box(&[5.0, 5.0, 5.0], &bbox),
        &[5.0, 5.0, 5.0],
    );
    assert_slice_close(
        &clamp_point_to_bounding_box(&[10.0, 10.0, 10.0], &bbox),
        &[9.0, 9.0, 9.0],
    );
    assert_slice_close(
        &clamp_point_to_bounding_box(&[5.0, 5.0, 15.0], &bbox),
        &[5.0, 5.0, 9.0],
    );
}

#[test]
fn clamp_multiple_points_to_bounding_box_matches_python_case() {
    let points = vec![
        vec![10.0, 10.0, 10.0],
        vec![0.0, 5.0, 0.0],
        vec![20.0, 0.0, 20.0],
    ];
    let bbox = [[0.0, 25.0], [0.0, 10.0], [3.0, 25.0]];
    let clamped = clamp_points_to_bounding_box(&points, &bbox);

    assert_slice_close(&clamped[0], &[10.0, 9.0, 10.0]);
    assert_slice_close(&clamped[1], &[0.0, 5.0, 3.0]);
    assert_slice_close(&clamped[2], &[20.0, 0.0, 20.0]);
}

#[test]
fn face_coordinate_from_bounding_box_matches_python_cases() {
    let bbox = [[5.0, 10.0], [10.0, 20.0], [20.0, 30.0]];

    assert_close(
        face_coordinate_from_bounding_box(&bbox, [1.0, 0.0, 0.0]),
        10.0,
    );
    assert_close(
        face_coordinate_from_bounding_box(&bbox, [-1.0, 0.0, 0.0]),
        5.0,
    );
    assert_close(
        face_coordinate_from_bounding_box(&bbox, [0.0, 1.0, 0.0]),
        20.0,
    );
    assert_close(
        face_coordinate_from_bounding_box(&bbox, [0.0, -1.0, 0.0]),
        10.0,
    );
    assert_close(
        face_coordinate_from_bounding_box(&bbox, [0.0, 0.0, 1.0]),
        30.0,
    );
    assert_close(
        face_coordinate_from_bounding_box(&bbox, [0.0, 0.0, -1.0]),
        20.0,
    );
}

#[test]
fn line_with_axis_aligned_plane_matches_python_cases() {
    for (plane_intercept, plane_normal, line_start, line_direction, expected) in [
        (
            0.0,
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
        ),
        (
            10.0,
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 10.0],
        ),
        (
            10.0,
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 10.0, 0.0],
        ),
        (
            10.0,
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [10.0, 0.0, 0.0],
        ),
    ] {
        assert_vec3_close(
            intersect_line_with_axis_aligned_plane(
                plane_intercept,
                plane_normal,
                line_start,
                line_direction,
            ),
            expected,
        );
    }
}

#[test]
fn bounding_box_to_face_vertices_matches_python_3d_case() {
    let face_vertices = bounding_box_to_face_vertices(&[[5.0, 10.0], [15.0, 20.0], [25.0, 30.0]]);

    assert_quad3_close(
        face_vertices[&BoxFace::XPos],
        [
            [5.0, 15.0, 30.0],
            [5.0, 20.0, 30.0],
            [10.0, 20.0, 30.0],
            [10.0, 15.0, 30.0],
        ],
    );
    assert_quad3_close(
        face_vertices[&BoxFace::ZNeg],
        [
            [5.0, 15.0, 25.0],
            [5.0, 15.0, 30.0],
            [5.0, 20.0, 30.0],
            [5.0, 20.0, 25.0],
        ],
    );
}

#[test]
fn bounding_box_to_face_vertices_uses_last_three_dimensions() {
    let bbox = [
        [0.0, 0.0],
        [0.0, 0.0],
        [5.0, 10.0],
        [15.0, 20.0],
        [25.0, 30.0],
    ];
    let face_vertices = bounding_box_to_face_vertices(&bbox);

    assert_quad3_close(
        face_vertices[&BoxFace::XNeg],
        [
            [5.0, 15.0, 25.0],
            [5.0, 20.0, 25.0],
            [10.0, 20.0, 25.0],
            [10.0, 15.0, 25.0],
        ],
    );
}

#[test]
fn inside_triangles_matches_python_cases() {
    assert!(inside_triangles(&[[[-1.0, -1.0], [-1.0, 1.0], [1.0, 0.0]]])[0]);
    assert!(!inside_triangles(&[[[1.0, 1.0], [2.0, 1.0], [1.5, 2.0]]])[0]);
}

#[test]
fn point_in_quadrilateral_2d_matches_python_cases() {
    assert!(point_in_quadrilateral_2d(
        [0.5, 0.5],
        [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 1.0]],
    ));
    assert!(!point_in_quadrilateral_2d(
        [2.0, 2.0],
        [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
    ));
}

#[test]
fn line_in_quadrilateral_3d_matches_python_cases() {
    let quadrilateral = [
        [-1.0, -1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
    ];

    assert!(line_in_quadrilateral_3d(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        quadrilateral,
    ));
    assert!(line_in_quadrilateral_3d(
        [0.0, 0.0, 5.0],
        [0.0, 0.0, 1.0],
        quadrilateral,
    ));
    assert!(!line_in_quadrilateral_3d(
        [0.0, 5.0, 0.0],
        [0.0, 0.0, 1.0],
        quadrilateral,
    ));
}

#[test]
fn find_front_back_face_matches_python_cases() {
    let bbox = [[0.0, 10.0], [0.0, 10.0], [0.0, 10.0]];

    assert_eq!(
        find_front_back_face([5.0, 5.0, 5.0], &bbox, [0.0, 0.0, 1.0]),
        (Some([0.0, 0.0, -1.0]), Some([0.0, 0.0, 1.0]))
    );
    assert_eq!(
        find_front_back_face([-5.0, -5.0, -5.0], &bbox, [0.0, 0.0, 1.0]),
        (None, None)
    );
    assert_eq!(
        find_front_back_face([5.0, 5.0, 5.0], &bbox, [0.0, 1.0, 0.0]),
        (Some([0.0, -1.0, 0.0]), Some([0.0, 1.0, 0.0]))
    );
    assert_eq!(
        find_front_back_face([5.0, 5.0, 5.0], &bbox, [1.0, 0.0, 0.0]),
        (Some([-1.0, 0.0, 0.0]), Some([1.0, 0.0, 0.0]))
    );
}

#[test]
fn intersect_line_with_axis_aligned_bounding_box_3d_matches_python_cases() {
    let bbox = [[0.0, 10.0], [0.0, 10.0], [0.0, 10.0]];

    assert_vec3_close(
        intersect_line_with_axis_aligned_bounding_box_3d(
            [5.0, 5.0, 5.0],
            [0.0, 0.0, 1.0],
            &bbox,
            [0.0, 0.0, 1.0],
        ),
        [5.0, 5.0, 10.0],
    );
    assert_vec3_close(
        intersect_line_with_axis_aligned_bounding_box_3d(
            [5.0, 5.0, 5.0],
            [0.0, 0.0, 1.0],
            &bbox,
            [0.0, 0.0, -1.0],
        ),
        [5.0, 5.0, 0.0],
    );
    assert_vec3_close(
        intersect_line_with_axis_aligned_bounding_box_3d(
            [5.0, 5.0, 5.0],
            [0.0, 1.0, 0.0],
            &bbox,
            [0.0, 1.0, 0.0],
        ),
        [5.0, 10.0, 5.0],
    );
    assert_vec3_close(
        intersect_line_with_axis_aligned_bounding_box_3d(
            [5.0, 5.0, 5.0],
            [1.0, 0.0, 0.0],
            &bbox,
            [1.0, 0.0, 0.0],
        ),
        [10.0, 5.0, 5.0],
    );
}

#[test]
fn distance_between_point_and_line_3d_matches_known_case() {
    let distance =
        distance_between_point_and_line_3d([4.0, 6.5, 8.0], [4.0, 2.0, 1.0], [0.0, 0.0, 1.0]);

    assert_close(distance, 4.5);
}

#[test]
fn line_in_triangles_3d_matches_python_case() {
    let triangles = [
        [[10.0, 0.0, 0.0], [19.0, 10.0, 5.0], [5.0, 5.0, 10.0]],
        [[10.0, 4.0, 4.0], [10.0, 0.0, 0.0], [10.0, 4.0, 0.0]],
    ];
    let in_triangle = line_in_triangles_3d([0.0, 5.0, 5.0], [1.0, 0.0, 0.0], &triangles);

    assert_eq!(in_triangle, vec![true, false]);
}

#[test]
fn find_nearest_triangle_intersection_matches_python_cases() {
    let triangles = [
        [[3.0, 0.0, 0.0], [3.0, 0.0, 10.0], [3.0, 10.0, 0.0]],
        [[5.0, 0.0, 0.0], [5.0, 0.0, 10.0], [5.0, 10.0, 0.0]],
        [[2.0, 50.0, 50.0], [2.0, 50.0, 100.0], [2.0, 100.0, 50.0]],
    ];

    let (index, intersection) =
        find_nearest_triangle_intersection([0.0, 1.0, 1.0], [1.0, 0.0, 0.0], &triangles).unwrap();
    assert_eq!(index, 0);
    assert_vec3_close(intersection, [3.0, 1.0, 1.0]);

    let (index, intersection) =
        find_nearest_triangle_intersection([6.0, 1.0, 1.0], [-1.0, 0.0, 0.0], &triangles).unwrap();
    assert_eq!(index, 1);
    assert_vec3_close(intersection, [5.0, 1.0, 1.0]);

    assert!(
        find_nearest_triangle_intersection([0.0, -10.0, -10.0], [-1.0, 0.0, 0.0], &triangles)
            .is_none()
    );
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= EPS,
        "expected {expected}, got {actual}"
    );
}

fn assert_slice_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_close(actual, expected);
    }
}

fn assert_vec2_close(actual: Vec2, expected: Vec2) {
    assert_slice_close(&actual, &expected);
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert_slice_close(&actual, &expected);
}

fn assert_quad3_close(actual: [Vec3; 4], expected: [Vec3; 4]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert_vec3_close(actual, expected);
    }
}

fn mat2_vec_mul(matrix: [[f64; 2]; 2], vector: Vec2) -> Vec2 {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1],
    ]
}

fn mat3_vec_mul(matrix: [[f64; 3]; 3], vector: Vec3) -> Vec3 {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}

fn normalize2(vector: Vec2) -> Vec2 {
    let norm = (vector[0] * vector[0] + vector[1] * vector[1]).sqrt();
    [vector[0] / norm, vector[1] / norm]
}

fn normalize3(vector: Vec3) -> Vec3 {
    let norm = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    [vector[0] / norm, vector[1] / norm, vector[2] / norm]
}
