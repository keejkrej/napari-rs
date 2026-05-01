use napari_rs::layers::shapes::constants::ShapeType;
use napari_rs::layers::shapes::shapes_utils::{
    EllipseTriangulation, ExtractedShapeType, PlanarAxis, PlanarPoints, PointToLine, ShapeData,
    ShapeInput, ShapeTypeMetadata, ShapesUtilsError, center_radii_to_corners, create_box,
    cull_triangles_not_in_poly, extract_shape_type, fan_triangulation, find_corners,
    find_planar_axis, fix_vertices_if_needed, get_default_shape_type, get_shape_ndim, inside_boxes,
    is_collinear, lines_intersect, number_of_shapes, on_segment, path_to_mask,
    perpendicular_distance, point_to_lines, points_in_poly, rdp, rectangle_to_box,
    triangle_edges_intersect_box, triangle_vertices_inside_box, triangles_intersect_box,
    triangulate_ellipse, validate_num_vertices, vectorized_lines_intersect,
};

#[test]
fn find_planar_axis_matches_python_2d_and_planar_3d_behavior() {
    let points_2d = vec![[0.0, 0.0], [1.0, 2.0]];
    assert_eq!(
        find_planar_axis(PlanarPoints::Points2D(points_2d.clone())),
        PlanarAxis {
            points: points_2d,
            axis: None,
            value: None,
        }
    );

    assert_eq!(
        find_planar_axis(PlanarPoints::Points3D(vec![
            [5.0, 0.0, 0.0],
            [5.0, 1.0, 0.0],
            [5.0, 0.0, 1.0],
        ])),
        PlanarAxis {
            points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
            axis: Some(0),
            value: Some(5.0),
        }
    );

    assert_eq!(
        find_planar_axis(PlanarPoints::Points3D(vec![
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 0.0, 1.0],
        ])),
        PlanarAxis {
            points: Vec::new(),
            axis: None,
            value: None,
        }
    );
}

#[test]
fn fan_triangulation_matches_python_indices() {
    let (vertices, triangles) =
        fan_triangulation(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);

    assert_eq!(
        vertices,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
    );
    assert_eq!(triangles, vec![[0, 1, 2], [0, 2, 3]]);
}

#[test]
fn inside_boxes_checks_origin_containment_like_python_helper() {
    let containing = [
        [-1.0, -1.0],
        [0.0, 0.0],
        [1.0, -1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [-1.0, 1.0],
        [0.0, 0.0],
    ];
    let offset = [
        [2.0, 2.0],
        [0.0, 0.0],
        [3.0, 2.0],
        [0.0, 0.0],
        [3.0, 3.0],
        [0.0, 0.0],
        [2.0, 3.0],
        [0.0, 0.0],
    ];

    assert_eq!(inside_boxes(&[containing, offset]), vec![true, false]);
}

#[test]
fn line_intersection_helpers_cover_general_and_collinear_cases() {
    assert!(lines_intersect(
        [0.0, 0.0],
        [2.0, 2.0],
        [0.0, 2.0],
        [2.0, 0.0]
    ));
    assert!(lines_intersect(
        [0.0, 0.0],
        [2.0, 0.0],
        [1.0, 0.0],
        [3.0, 0.0]
    ));
    assert!(!lines_intersect(
        [0.0, 0.0],
        [1.0, 0.0],
        [2.0, 0.0],
        [3.0, 0.0]
    ));
    assert!(on_segment([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]));
    assert_eq!(
        vectorized_lines_intersect(
            &[[0.0, 0.0], [0.0, 0.0]],
            &[[2.0, 2.0], [1.0, 0.0]],
            [0.0, 2.0],
            [2.0, 0.0],
        ),
        vec![true, false]
    );
}

#[test]
fn triangle_box_intersection_helpers_match_expected_cases() {
    let triangles = [
        [[0.0, 0.0], [2.0, 0.0], [1.0, 2.0]],
        [[5.0, 5.0], [6.0, 5.0], [5.0, 6.0]],
        [[-1.0, 0.5], [3.0, 0.5], [1.0, -2.0]],
    ];
    let corners = [[0.5, 0.5], [1.5, 1.5]];

    assert_eq!(
        triangle_vertices_inside_box(&triangles, &corners),
        vec![false, false, false]
    );
    assert_eq!(
        triangle_edges_intersect_box(&triangles, &corners),
        vec![true, false, true]
    );
    assert_eq!(
        triangles_intersect_box(&triangles, &corners),
        vec![true, false, true]
    );
}

#[test]
fn collinearity_and_point_to_lines_match_python_logic() {
    assert!(is_collinear(&[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]));
    assert!(!is_collinear(&[[0.0, 0.0], [1.0, 1.0], [2.0, 3.0]]));

    assert_eq!(
        point_to_lines(
            [1.0, 1.0],
            &[
                [[0.0, 0.0], [2.0, 0.0]],
                [[0.0, 0.0], [0.0, 2.0]],
                [[5.0, 5.0], [5.0, 5.0]],
            ],
        ),
        Ok(PointToLine {
            index: 0,
            location: 0.5,
        })
    );
    assert_eq!(
        point_to_lines([0.0, 0.0], &[]),
        Err(ShapesUtilsError::NoLines)
    );
}

#[test]
fn shape_box_helpers_match_python_outputs() {
    assert_eq!(
        create_box(&[[0.0, 3.0], [5.0, 0.0], [2.5, 5.0]]).unwrap(),
        [
            [0.0, 0.0],
            [2.5, 0.0],
            [5.0, 0.0],
            [5.0, 2.5],
            [5.0, 5.0],
            [2.5, 5.0],
            [0.0, 5.0],
            [0.0, 2.5],
            [2.5, 2.5],
        ]
    );
    assert_eq!(
        find_corners(&[[0.0, 3.0], [5.0, 0.0], [2.5, 5.0]]),
        Ok([[0.0, 0.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]])
    );
    assert_eq!(
        center_radii_to_corners([2.0, 3.0], [1.0, 2.0]),
        [[1.0, 1.0], [3.0, 1.0], [3.0, 5.0], [1.0, 5.0]]
    );
}

#[test]
fn rectangle_to_box_inserts_midpoints_and_center() {
    let rectangle = vec![
        vec![0.0, 0.0],
        vec![2.0, 0.0],
        vec![2.0, 2.0],
        vec![0.0, 2.0],
    ];

    assert_eq!(
        rectangle_to_box(&rectangle),
        Ok(vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![2.0, 0.0],
            vec![2.0, 1.0],
            vec![2.0, 2.0],
            vec![1.0, 2.0],
            vec![0.0, 2.0],
            vec![0.0, 1.0],
            vec![1.0, 1.0],
        ])
    );
    assert_eq!(
        rectangle_to_box(&rectangle[..3]),
        Err(ShapesUtilsError::RectangleCornerCount(3))
    );
}

#[test]
fn get_default_shape_type_matches_python_lowest_common_denominator() {
    assert_eq!(get_default_shape_type(&[]), ShapeType::Polygon);
    assert_eq!(
        get_default_shape_type(&[ShapeType::Rectangle, ShapeType::Rectangle]),
        ShapeType::Rectangle
    );
    assert_eq!(
        get_default_shape_type(&[ShapeType::Ellipse, ShapeType::Ellipse]),
        ShapeType::Ellipse
    );
    assert_eq!(
        get_default_shape_type(&[ShapeType::Ellipse, ShapeType::Rectangle]),
        ShapeType::Polygon
    );
}

#[test]
fn extract_shape_type_separates_embedded_shape_type_like_python_helper() {
    let plain = ShapeData::Single(vec![vec![0.0, 0.0], vec![1.0, 1.0]]);
    assert_eq!(
        extract_shape_type(
            ShapeInput::Plain(plain.clone()),
            ShapeTypeMetadata::Single(ShapeType::Line),
        ),
        ExtractedShapeType {
            data: plain,
            shape_type: ShapeTypeMetadata::Single(ShapeType::Line),
        }
    );

    let typed_data = ShapeData::Many(vec![
        vec![vec![0.0, 0.0], vec![1.0, 0.0]],
        vec![vec![0.0, 1.0], vec![1.0, 1.0]],
    ]);
    assert_eq!(
        extract_shape_type(
            ShapeInput::Typed(
                typed_data.clone(),
                ShapeTypeMetadata::Single(ShapeType::Path),
            ),
            ShapeTypeMetadata::Single(ShapeType::Polygon),
        ),
        ExtractedShapeType {
            data: typed_data,
            shape_type: ShapeTypeMetadata::Single(ShapeType::Path),
        }
    );

    assert_eq!(
        extract_shape_type(
            ShapeInput::ManyTyped(vec![
                (vec![vec![0.0, 0.0], vec![1.0, 0.0]], ShapeType::Line),
                (
                    vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0]],
                    ShapeType::Polygon,
                ),
            ]),
            ShapeTypeMetadata::None,
        ),
        ExtractedShapeType {
            data: ShapeData::Many(vec![
                vec![vec![0.0, 0.0], vec![1.0, 0.0]],
                vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0]],
            ]),
            shape_type: ShapeTypeMetadata::Many(vec![ShapeType::Line, ShapeType::Polygon]),
        }
    );
}

#[test]
fn number_of_shapes_matches_python_empty_single_and_many_cases() {
    assert_eq!(number_of_shapes(&ShapeData::Empty), 0);
    assert_eq!(
        number_of_shapes(&ShapeData::Single(vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
        ])),
        1
    );
    assert_eq!(
        number_of_shapes(&ShapeData::Many(vec![
            vec![vec![0.0, 0.0], vec![1.0, 0.0]],
            vec![vec![0.0, 1.0], vec![1.0, 1.0]],
        ])),
        2
    );
}

#[test]
fn get_shape_ndim_matches_python_single_and_many_shape_cases() {
    assert_eq!(
        get_shape_ndim(&ShapeData::Single(vec![
            vec![0.0, 0.0, 1.0],
            vec![1.0, 0.0, 1.0],
        ])),
        Ok(3)
    );
    assert_eq!(
        get_shape_ndim(&ShapeData::Many(vec![
            vec![vec![0.0, 0.0], vec![1.0, 0.0]],
            vec![vec![0.0, 1.0], vec![1.0, 1.0], vec![2.0, 1.0]],
        ])),
        Ok(2)
    );
    assert_eq!(
        get_shape_ndim(&ShapeData::Empty),
        Err(ShapesUtilsError::EmptyData)
    );
}

#[test]
fn validate_num_vertices_matches_python_shape_rules() {
    let rectangles = ShapeData::Many(vec![
        vec![vec![0.0, 0.0], vec![1.0, 1.0]],
        vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
        ],
    ]);
    assert_eq!(
        validate_num_vertices(&rectangles, ShapeType::Rectangle, None, Some(&[2, 4])),
        Ok(())
    );

    let line = ShapeData::Single(vec![vec![0.0, 0.0], vec![1.0, 1.0], vec![2.0, 2.0]]);
    assert_eq!(
        validate_num_vertices(&line, ShapeType::Line, None, Some(&[2])),
        Err(ShapesUtilsError::InvalidVertexCount {
            shape_type: ShapeType::Line,
            vertices: 3,
        })
    );

    let polygon = ShapeData::Single(vec![vec![0.0, 0.0], vec![1.0, 0.0]]);
    assert_eq!(
        validate_num_vertices(&polygon, ShapeType::Polygon, Some(3), None),
        Err(ShapesUtilsError::InvalidVertexCount {
            shape_type: ShapeType::Polygon,
            vertices: 2,
        })
    );
    assert_eq!(
        validate_num_vertices(&ShapeData::Empty, ShapeType::Path, Some(2), None),
        Ok(())
    );
}

#[test]
fn perpendicular_distance_matches_python_higher_dimensional_cases() {
    assert_eq!(
        perpendicular_distance(&[1.0, 0.0], &[0.0, 0.0], &[0.0, 3.0]),
        Ok(1.0)
    );
    assert_eq!(
        perpendicular_distance(&[1.0, 0.0, 0.0], &[0.0, 0.0, 0.0], &[0.0, 0.0, 3.0]),
        Ok(1.0)
    );
    assert_eq!(
        perpendicular_distance(
            &[1.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 3.0],
        ),
        Ok(1.0)
    );
    assert_eq!(
        perpendicular_distance(&[1.0, 0.0, 0.0], &[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]),
        Ok(1.0)
    );
}

#[test]
fn rdp_preserves_vertices_at_zero_epsilon_and_simplifies_as_epsilon_increases() {
    let vertices = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.1],
        vec![2.0, -0.1],
        vec![3.0, 5.0],
        vec![4.0, 6.0],
        vec![5.0, 7.0],
        vec![6.0, 8.1],
        vec![7.0, 9.0],
    ];

    let unchanged = rdp(&vertices, 0.0).unwrap();
    let reduced = rdp(&vertices, 1.0).unwrap();
    let more_reduced = rdp(&vertices, 2.0).unwrap();

    assert_eq!(unchanged.len(), vertices.len());
    assert!(reduced.len() < vertices.len());
    assert!(more_reduced.len() < reduced.len());
    assert_eq!(reduced.first(), vertices.first());
    assert_eq!(reduced.last(), vertices.last());
}

#[test]
fn points_in_poly_matches_python_ray_casting_boundary_behavior() {
    let vertices = [[1.0, 1.0], [1.0, 3.0], [3.0, 3.0], [3.0, 1.0]];
    let points = [
        [2.0, 2.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [3.0, 3.0],
        [1.0, 2.0],
        [3.0, 2.0],
        [2.0, 1.0],
        [2.0, 3.0],
    ];

    assert_eq!(
        points_in_poly(&points, &vertices),
        vec![true, false, true, false, true, false, true, false]
    );
    assert_eq!(points_in_poly(&points, &[]), vec![false; points.len()]);
}

#[test]
fn grid_points_in_poly_matches_python_grid_wrapper() {
    let vertices = [[1.0, 1.0], [1.0, 3.0], [3.0, 3.0], [3.0, 1.0]];

    assert_eq!(
        napari_rs::layers::shapes::shapes_utils::grid_points_in_poly([5, 5], &vertices),
        vec![
            vec![false, false, false, false, false],
            vec![false, true, true, false, false],
            vec![false, true, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]
    );
}

#[test]
fn path_to_mask_rasterizes_edges_like_python_helper() {
    let vertices = [[0.0, 0.0], [0.0, 3.0], [0.0, 3.0], [3.0, 3.0]];

    assert_eq!(
        path_to_mask([5, 5], &vertices),
        vec![
            vec![true, true, true, true, false],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, false],
        ]
    );
}

#[test]
fn path_to_mask_clips_and_rounds_vertices_like_python_helper() {
    let vertices = [[-1.0, -1.0], [2.6, 0.2], [8.0, 8.0]];

    assert_eq!(
        path_to_mask([5, 5], &vertices),
        vec![
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
        ]
    );
    assert_eq!(path_to_mask([0, 5], &vertices), Vec::<Vec<bool>>::new());
}

#[test]
fn triangulate_ellipse_matches_python_vertices_and_indices() {
    let corners = vec![
        vec![1.0, 1.0],
        vec![3.0, 1.0],
        vec![3.0, 5.0],
        vec![1.0, 5.0],
    ];
    let triangulation = triangulate_ellipse(&corners, 5).unwrap();

    assert_vertices_close(
        &triangulation.vertices,
        &[
            &[2.0, 3.0],
            &[3.0, 3.0],
            &[2.0, 5.0],
            &[1.0, 3.0],
            &[2.0, 1.0],
            &[3.0, 3.0],
        ],
    );
    assert_eq!(
        triangulation.triangles,
        vec![[0, 1, 2], [0, 2, 3], [0, 3, 4], [0, 4, 5], [0, 5, 1]]
    );
}

#[test]
fn triangulate_ellipse_supports_planar_3d_corners_and_validates_shape() {
    let corners = vec![
        vec![0.0, 0.0, 7.0],
        vec![2.0, 0.0, 7.0],
        vec![2.0, 2.0, 7.0],
        vec![0.0, 2.0, 7.0],
    ];

    assert_eq!(
        triangulate_ellipse(&corners, 3).unwrap(),
        EllipseTriangulation {
            vertices: vec![
                vec![1.0, 1.0, 7.0],
                vec![2.0, 1.0, 7.0],
                vec![0.0, 1.0000000000000002, 7.0],
                vec![2.0, 0.9999999999999998, 7.0],
            ],
            triangles: vec![[0, 1, 2], [0, 2, 3], [0, 3, 1]],
        }
    );
    assert_eq!(
        triangulate_ellipse(&corners[..3], 5),
        Err(ShapesUtilsError::InvalidCornerShape {
            rows: 3,
            columns: Some(3),
        })
    );
    assert_eq!(
        triangulate_ellipse(&[vec![0.0], vec![1.0], vec![2.0], vec![3.0]], 5),
        Err(ShapesUtilsError::InvalidCornerShape {
            rows: 4,
            columns: Some(1),
        })
    );
}

#[test]
fn cull_triangles_not_in_poly_matches_python_centroid_filter() {
    let vertices = [
        [0.0, 0.0],
        [4.0, 0.0],
        [4.0, 4.0],
        [0.0, 4.0],
        [6.0, 6.0],
        [7.0, 6.0],
        [6.0, 7.0],
    ];
    let triangles = [[0, 1, 2], [0, 2, 3], [4, 5, 6], [1, 4, 2]];
    let poly = [[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]];

    assert_eq!(
        cull_triangles_not_in_poly(&vertices, &triangles, &poly),
        Ok(vec![[0, 1, 2], [0, 2, 3]])
    );
    assert_eq!(
        cull_triangles_not_in_poly(&vertices, &[[0, 1, 99]], &poly),
        Err(ShapesUtilsError::InvalidTriangleIndex {
            index: 99,
            vertices: vertices.len(),
        })
    );
}

#[test]
fn fix_vertices_if_needed_inserts_planar_axis_like_python_helper() {
    let vertices = [[1.0, 2.0], [3.0, 4.0]];

    assert_eq!(
        fix_vertices_if_needed(&vertices, None, Some(9.0)),
        Ok(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
    );
    assert_eq!(
        fix_vertices_if_needed(&vertices, Some(1), None),
        Ok(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
    );
    assert_eq!(
        fix_vertices_if_needed(&vertices, Some(0), Some(9.0)),
        Ok(vec![vec![9.0, 1.0, 2.0], vec![9.0, 3.0, 4.0]])
    );
    assert_eq!(
        fix_vertices_if_needed(&vertices, Some(1), Some(9.0)),
        Ok(vec![vec![1.0, 9.0, 2.0], vec![3.0, 9.0, 4.0]])
    );
    assert_eq!(
        fix_vertices_if_needed(&vertices, Some(2), Some(9.0)),
        Ok(vec![vec![1.0, 2.0, 9.0], vec![3.0, 4.0, 9.0]])
    );
    assert_eq!(
        fix_vertices_if_needed(&vertices, Some(3), Some(9.0)),
        Err(ShapesUtilsError::InvalidAxis { axis: 3, ndim: 3 })
    );
}

fn assert_vertices_close(actual: &[Vec<f64>], expected: &[&[f64]]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.len(), expected.len());
        for (&actual, &expected) in actual.iter().zip(*expected) {
            assert!((actual - expected).abs() < 1e-12);
        }
    }
}
