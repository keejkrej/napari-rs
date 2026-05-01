use std::collections::BTreeMap;

pub type Vec2 = [f64; 2];
pub type Vec3 = [f64; 3];
pub type Mat2 = [[f64; 2]; 2];
pub type Mat3 = [[f64; 3]; 3];
pub type Triangle2 = [Vec2; 3];
pub type Triangle3 = [Vec3; 3];
pub type Quad3 = [Vec3; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoxFace {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

impl BoxFace {
    pub fn normal(self) -> Vec3 {
        match self {
            Self::XPos => [0.0, 0.0, 1.0],
            Self::XNeg => [0.0, 0.0, -1.0],
            Self::YPos => [0.0, 1.0, 0.0],
            Self::YNeg => [0.0, -1.0, 0.0],
            Self::ZPos => [1.0, 0.0, 0.0],
            Self::ZNeg => [-1.0, 0.0, 0.0],
        }
    }
}

const BOX_FACES: [BoxFace; 6] = [
    BoxFace::XPos,
    BoxFace::XNeg,
    BoxFace::YPos,
    BoxFace::YNeg,
    BoxFace::ZPos,
    BoxFace::ZNeg,
];

pub fn project_points_onto_plane(
    points: &[Vec3],
    plane_point: Vec3,
    plane_normal: Vec3,
) -> (Vec<Vec3>, Vec<f64>) {
    let mut projected_points = Vec::with_capacity(points.len());
    let mut signed_distances = Vec::with_capacity(points.len());

    for &point in points {
        let point_vector = sub3(point, plane_point);
        let signed_distance = dot3(point_vector, plane_normal);
        signed_distances.push(signed_distance);
        projected_points.push(sub3(point, scale3(plane_normal, signed_distance)));
    }

    (projected_points, signed_distances)
}

pub fn rotation_matrix_from_vectors_2d(vec_1: Vec2, vec_2: Vec2) -> Mat2 {
    let vec_1 = normalize2(vec_1);
    let vec_2 = normalize2(vec_2);
    let diagonal_1 = dot2(vec_1, vec_2);
    let diagonal_2 = vec_1[0] * vec_2[1] - vec_1[1] * vec_2[0];

    [[diagonal_1, -diagonal_2], [diagonal_2, diagonal_1]]
}

pub fn rotation_matrix_from_vectors_3d(vec_1: Vec3, vec_2: Vec3) -> Mat3 {
    let vec_1 = normalize3(vec_1);
    let vec_2 = normalize3(vec_2);
    let cross_prod = cross3(vec_1, vec_2);
    let dot_prod = dot3(vec_1, vec_2);

    if norm3(cross_prod) > 0.0 {
        let s = norm3(cross_prod);
        let kmat = [
            [0.0, -cross_prod[2], cross_prod[1]],
            [cross_prod[2], 0.0, -cross_prod[0]],
            [-cross_prod[1], cross_prod[0], 0.0],
        ];
        let kmat_sq = mat3_mul(kmat, kmat);
        let scale = (1.0 - dot_prod) / (s * s);
        mat3_add(mat3_add(identity3(), kmat), mat3_scale(kmat_sq, scale))
    } else if (dot_prod - 1.0).abs() <= 1e-12 {
        identity3()
    } else {
        [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]]
    }
}

pub fn rotate_points(
    points: &[Vec3],
    current_plane_normal: Vec3,
    new_plane_normal: Vec3,
) -> (Vec<Vec3>, Mat3) {
    let rotation_matrix = rotation_matrix_from_vectors_3d(current_plane_normal, new_plane_normal);
    let rotated_points = points
        .iter()
        .map(|&point| mat3_vec_mul(rotation_matrix, point))
        .collect();

    (rotated_points, rotation_matrix)
}

pub fn point_in_bounding_box(point: &[f64], bounding_box: &[[f64; 2]]) -> bool {
    point
        .iter()
        .zip(bounding_box)
        .all(|(&coord, bounds)| coord >= bounds[0] && coord <= bounds[1])
}

pub fn clamp_point_to_bounding_box(point: &[f64], bounding_box: &[[f64; 2]]) -> Vec<f64> {
    point
        .iter()
        .zip(bounding_box)
        .map(|(&coord, bounds)| coord.clamp(bounds[0], bounds[1] - 1.0))
        .collect()
}

pub fn clamp_points_to_bounding_box(
    points: &[Vec<f64>],
    bounding_box: &[[f64; 2]],
) -> Vec<Vec<f64>> {
    points
        .iter()
        .map(|point| clamp_point_to_bounding_box(point, bounding_box))
        .collect()
}

pub fn face_coordinate_from_bounding_box(bounding_box: &[[f64; 2]], face_normal: Vec3) -> f64 {
    let axis = face_normal
        .iter()
        .position(|&value| value != 0.0)
        .expect("face normal must contain a non-zero axis");

    if face_normal[axis] > 0.0 {
        bounding_box[axis][1]
    } else {
        bounding_box[axis][0]
    }
}

pub fn intersect_line_with_axis_aligned_plane(
    plane_intercept: f64,
    plane_normal: Vec3,
    line_start: Vec3,
    line_direction: Vec3,
) -> Vec3 {
    let plane_axis = plane_normal
        .iter()
        .position(|&value| value != 0.0)
        .expect("plane normal must contain a non-zero axis");
    let t = (plane_intercept - line_start[plane_axis]) / line_direction[plane_axis];
    add3(line_start, scale3(line_direction, t))
}

pub fn bounding_box_to_face_vertices(bounding_box: &[[f64; 2]]) -> BTreeMap<BoxFace, Quad3> {
    let [x_min, x_max] = bounding_box[bounding_box.len() - 1];
    let [y_min, y_max] = bounding_box[bounding_box.len() - 2];
    let [z_min, z_max] = bounding_box[bounding_box.len() - 3];

    BTreeMap::from([
        (
            BoxFace::XPos,
            [
                [z_min, y_min, x_max],
                [z_min, y_max, x_max],
                [z_max, y_max, x_max],
                [z_max, y_min, x_max],
            ],
        ),
        (
            BoxFace::XNeg,
            [
                [z_min, y_min, x_min],
                [z_min, y_max, x_min],
                [z_max, y_max, x_min],
                [z_max, y_min, x_min],
            ],
        ),
        (
            BoxFace::YPos,
            [
                [z_min, y_max, x_min],
                [z_min, y_max, x_max],
                [z_max, y_max, x_max],
                [z_max, y_max, x_min],
            ],
        ),
        (
            BoxFace::YNeg,
            [
                [z_min, y_min, x_min],
                [z_min, y_min, x_max],
                [z_max, y_min, x_max],
                [z_max, y_min, x_min],
            ],
        ),
        (
            BoxFace::ZPos,
            [
                [z_max, y_min, x_min],
                [z_max, y_min, x_max],
                [z_max, y_max, x_max],
                [z_max, y_max, x_min],
            ],
        ),
        (
            BoxFace::ZNeg,
            [
                [z_min, y_min, x_min],
                [z_min, y_min, x_max],
                [z_min, y_max, x_max],
                [z_min, y_max, x_min],
            ],
        ),
    ])
}

pub fn inside_triangles(triangles: &[Triangle2]) -> Vec<bool> {
    triangles
        .iter()
        .map(|triangle| {
            let ab = sub2(triangle[1], triangle[0]);
            let ac = sub2(triangle[2], triangle[0]);
            let bc = sub2(triangle[2], triangle[1]);

            let s_ab = -ab[0] * triangle[0][1] + ab[1] * triangle[0][0] >= 0.0;
            let s_ac = -ac[0] * triangle[0][1] + ac[1] * triangle[0][0] >= 0.0;
            let s_bc = -bc[0] * triangle[1][1] + bc[1] * triangle[1][0] >= 0.0;

            s_ab != s_ac && s_ab == s_bc
        })
        .collect()
}

pub fn intersect_line_with_plane_3d(
    line_position: Vec3,
    line_direction: Vec3,
    plane_position: Vec3,
    plane_normal: Vec3,
) -> Vec3 {
    let line_plane_direction = sub3(plane_position, line_position);
    let line_plane_on_plane_normal = dot3(line_plane_direction, plane_normal);
    let line_direction_on_plane_normal = dot3(line_direction, plane_normal);
    let scale_factor = line_plane_on_plane_normal / line_direction_on_plane_normal;

    add3(line_position, scale3(line_direction, scale_factor))
}

pub fn intersect_line_with_multiple_planes_3d(
    line_position: Vec3,
    line_direction: Vec3,
    plane_positions: &[Vec3],
    plane_normals: &[Vec3],
) -> Vec<Vec3> {
    plane_positions
        .iter()
        .zip(plane_normals)
        .map(|(&plane_position, &plane_normal)| {
            intersect_line_with_plane_3d(
                line_position,
                line_direction,
                plane_position,
                plane_normal,
            )
        })
        .collect()
}

pub fn intersect_line_with_triangles(
    line_point: Vec3,
    line_direction: Vec3,
    triangles: &[Triangle3],
) -> Vec<Vec3> {
    let plane_positions: Vec<Vec3> = triangles.iter().map(|triangle| triangle[0]).collect();
    let plane_normals: Vec<Vec3> = triangles
        .iter()
        .map(|triangle| {
            let edge_1 = sub3(triangle[1], triangle[0]);
            let edge_2 = sub3(triangle[2], triangle[0]);
            normalize3(cross3(edge_1, edge_2))
        })
        .collect();

    intersect_line_with_multiple_planes_3d(
        line_point,
        line_direction,
        &plane_positions,
        &plane_normals,
    )
}

pub fn point_in_quadrilateral_2d(point: Vec2, quadrilateral: [Vec2; 4]) -> bool {
    let triangles = [
        [
            sub2(quadrilateral[0], point),
            sub2(quadrilateral[1], point),
            sub2(quadrilateral[2], point),
        ],
        [
            sub2(quadrilateral[0], point),
            sub2(quadrilateral[2], point),
            sub2(quadrilateral[3], point),
        ],
    ];

    inside_triangles(&triangles)
        .into_iter()
        .any(|inside| inside)
}

pub fn line_in_quadrilateral_3d(
    line_point: Vec3,
    line_direction: Vec3,
    quadrilateral: Quad3,
) -> bool {
    let (vertices_plane, _) = project_points_onto_plane(&quadrilateral, line_point, line_direction);
    let (rotated_vertices, rotation_matrix) =
        rotate_points(&vertices_plane, line_direction, [0.0, 0.0, 1.0]);
    let quadrilateral_2d = [
        [rotated_vertices[0][0], rotated_vertices[0][1]],
        [rotated_vertices[1][0], rotated_vertices[1][1]],
        [rotated_vertices[2][0], rotated_vertices[2][1]],
        [rotated_vertices[3][0], rotated_vertices[3][1]],
    ];
    let click_pos = mat3_vec_mul(rotation_matrix, line_point);

    point_in_quadrilateral_2d([click_pos[0], click_pos[1]], quadrilateral_2d)
}

pub fn line_in_triangles_3d(
    line_point: Vec3,
    line_direction: Vec3,
    triangles: &[Triangle3],
) -> Vec<bool> {
    let vertices: Vec<Vec3> = triangles
        .iter()
        .flat_map(|triangle| triangle.iter().copied())
        .collect();
    let (vertices_plane, _) = project_points_onto_plane(&vertices, line_point, line_direction);
    let rotation_matrix = rotation_matrix_from_vectors_3d(line_direction, [0.0, 0.0, 1.0]);
    let rotated_vertices: Vec<Vec3> = vertices_plane
        .iter()
        .map(|&vertex| mat3_vec_mul(rotation_matrix, vertex))
        .collect();
    let line_pos = mat3_vec_mul(rotation_matrix, line_point);

    let rotated_triangles: Vec<Triangle2> = rotated_vertices
        .chunks_exact(3)
        .map(|chunk| {
            [
                [chunk[0][0] - line_pos[0], chunk[0][1] - line_pos[1]],
                [chunk[1][0] - line_pos[0], chunk[1][1] - line_pos[1]],
                [chunk[2][0] - line_pos[0], chunk[2][1] - line_pos[1]],
            ]
        })
        .collect();

    inside_triangles(&rotated_triangles)
}

pub fn find_front_back_face(
    click_pos: Vec3,
    bounding_box: &[[f64; 2]],
    view_dir: Vec3,
) -> (Option<Vec3>, Option<Vec3>) {
    let mut front_face_normal = None;
    let mut back_face_normal = None;
    let bbox_face_coords = bounding_box_to_face_vertices(bounding_box);

    for face in BOX_FACES {
        let normal = face.normal();
        let face_coords = bbox_face_coords[&face];
        if dot3(view_dir, normal) < -0.001 {
            if line_in_quadrilateral_3d(click_pos, view_dir, face_coords) {
                front_face_normal = Some(normal);
            }
        } else if line_in_quadrilateral_3d(click_pos, view_dir, face_coords) {
            back_face_normal = Some(normal);
        }

        if front_face_normal.is_some() && back_face_normal.is_some() {
            break;
        }
    }

    (front_face_normal, back_face_normal)
}

pub fn intersect_line_with_axis_aligned_bounding_box_3d(
    line_point: Vec3,
    line_direction: Vec3,
    bounding_box: &[[f64; 2]],
    face_normal: Vec3,
) -> Vec3 {
    let front_face_coordinate = face_coordinate_from_bounding_box(bounding_box, face_normal);
    intersect_line_with_axis_aligned_plane(
        front_face_coordinate,
        face_normal,
        line_point,
        scale3(line_direction, -1.0),
    )
}

pub fn distance_between_point_and_line_3d(
    point: Vec3,
    line_position: Vec3,
    line_direction: Vec3,
) -> f64 {
    let line_direction_normalized = normalize3(line_direction);
    let projection_on_line_direction = dot3(sub3(point, line_position), line_direction);
    let closest_point_on_line = add3(
        line_position,
        scale3(line_direction_normalized, projection_on_line_direction),
    );

    norm3(sub3(point, closest_point_on_line))
}

pub fn find_nearest_triangle_intersection(
    ray_position: Vec3,
    ray_direction: Vec3,
    triangles: &[Triangle3],
) -> Option<(usize, Vec3)> {
    let inside = line_in_triangles_3d(ray_position, ray_direction, triangles);
    if !inside.iter().any(|&is_inside| is_inside) {
        return None;
    }

    let intersected_triangles: Vec<Triangle3> = triangles
        .iter()
        .zip(&inside)
        .filter_map(|(&triangle, &is_inside)| is_inside.then_some(triangle))
        .collect();
    let intersection_points =
        intersect_line_with_triangles(ray_position, ray_direction, &intersected_triangles);

    let (closest_intersection_offset, intersection) = intersection_points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            norm3(sub3(**a, ray_position))
                .partial_cmp(&norm3(sub3(**b, ray_position)))
                .unwrap()
        })
        .map(|(index, &intersection)| (index, intersection))
        .unwrap();

    let original_index = inside
        .iter()
        .enumerate()
        .filter_map(|(index, &is_inside)| is_inside.then_some(index))
        .nth(closest_intersection_offset)
        .unwrap();

    Some((original_index, intersection))
}

pub fn get_center_bbox(roi: [Vec2; 4]) -> (Vec2, f64, f64) {
    let min_y = roi
        .iter()
        .map(|point| point[0])
        .fold(f64::INFINITY, f64::min);
    let min_x = roi
        .iter()
        .map(|point| point[1])
        .fold(f64::INFINITY, f64::min);
    let max_y = roi
        .iter()
        .map(|point| point[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let max_x = roi
        .iter()
        .map(|point| point[1])
        .fold(f64::NEG_INFINITY, f64::max);
    let height = max_y - min_y;
    let width = max_x - min_x;

    ([min_y + height / 2.0, min_x + width / 2.0], height, width)
}

fn add3(left: Vec3, right: Vec3) -> Vec3 {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

fn sub2(left: Vec2, right: Vec2) -> Vec2 {
    [left[0] - right[0], left[1] - right[1]]
}

fn sub3(left: Vec3, right: Vec3) -> Vec3 {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale3(vector: Vec3, scale: f64) -> Vec3 {
    [vector[0] * scale, vector[1] * scale, vector[2] * scale]
}

fn dot2(left: Vec2, right: Vec2) -> f64 {
    left[0] * right[0] + left[1] * right[1]
}

fn dot3(left: Vec3, right: Vec3) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn cross3(left: Vec3, right: Vec3) -> Vec3 {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn norm2(vector: Vec2) -> f64 {
    dot2(vector, vector).sqrt()
}

fn norm3(vector: Vec3) -> f64 {
    dot3(vector, vector).sqrt()
}

fn normalize2(vector: Vec2) -> Vec2 {
    let norm = norm2(vector);
    [vector[0] / norm, vector[1] / norm]
}

fn normalize3(vector: Vec3) -> Vec3 {
    let norm = norm3(vector);
    [vector[0] / norm, vector[1] / norm, vector[2] / norm]
}

fn identity3() -> Mat3 {
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
}

fn mat3_add(left: Mat3, right: Mat3) -> Mat3 {
    [
        [
            left[0][0] + right[0][0],
            left[0][1] + right[0][1],
            left[0][2] + right[0][2],
        ],
        [
            left[1][0] + right[1][0],
            left[1][1] + right[1][1],
            left[1][2] + right[1][2],
        ],
        [
            left[2][0] + right[2][0],
            left[2][1] + right[2][1],
            left[2][2] + right[2][2],
        ],
    ]
}

fn mat3_scale(matrix: Mat3, scale: f64) -> Mat3 {
    [
        [
            matrix[0][0] * scale,
            matrix[0][1] * scale,
            matrix[0][2] * scale,
        ],
        [
            matrix[1][0] * scale,
            matrix[1][1] * scale,
            matrix[1][2] * scale,
        ],
        [
            matrix[2][0] * scale,
            matrix[2][1] * scale,
            matrix[2][2] * scale,
        ],
    ]
}

fn mat3_mul(left: Mat3, right: Mat3) -> Mat3 {
    let mut result = [[0.0; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            result[row][col] = (0..3)
                .map(|inner| left[row][inner] * right[inner][col])
                .sum();
        }
    }
    result
}

fn mat3_vec_mul(matrix: Mat3, vector: Vec3) -> Vec3 {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}
