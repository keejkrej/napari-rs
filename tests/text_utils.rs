use napari_rs::layers::utils::text_constants::Anchor;
use napari_rs::layers::utils::text_utils::{
    BBoxExtents, TextAnchorData, TextAnchorError, ViewData, calculate_anchor_center,
    calculate_anchor_lower_left, calculate_anchor_lower_right, calculate_anchor_upper_left,
    calculate_anchor_upper_right, calculate_bbox_centers, calculate_bbox_extents, get_text_anchors,
};

fn square_coords() -> Vec<Vec<f64>> {
    vec![
        vec![0.0, 0.0],
        vec![10.0, 0.0],
        vec![0.0, 10.0],
        vec![10.0, 10.0],
    ]
}

#[test]
fn text_anchor_strings_match_python_string_enum_values() {
    assert_eq!(Anchor::Center.to_string(), "center");
    assert_eq!(Anchor::UpperLeft.to_string(), "upper_left");
    assert_eq!(Anchor::UpperRight.to_string(), "upper_right");
    assert_eq!(Anchor::LowerLeft.to_string(), "lower_left");
    assert_eq!(Anchor::LowerRight.to_string(), "lower_right");
    assert_eq!("LOWER_RIGHT".parse(), Ok(Anchor::LowerRight));
}

#[test]
fn bbox_center_matches_python_for_list_and_coordinate_array_inputs() {
    let list_data = ViewData::Items(vec![square_coords()]);
    let coord_data = ViewData::Coordinates(square_coords());

    assert_eq!(
        calculate_bbox_centers(&list_data).unwrap(),
        vec![vec![5.0, 5.0]]
    );
    assert_eq!(
        calculate_bbox_centers(&coord_data).unwrap(),
        square_coords()
    );
    assert_eq!(
        calculate_anchor_center(&list_data).unwrap(),
        TextAnchorData {
            coordinates: vec![vec![5.0, 5.0]],
            anchor_x: "center",
            anchor_y: "center",
        }
    );
}

#[test]
fn bbox_extents_match_python_for_list_and_coordinate_array_inputs() {
    let list_data = ViewData::Items(vec![square_coords()]);
    let coord_data = ViewData::Coordinates(square_coords());

    assert_eq!(
        calculate_bbox_extents(&list_data).unwrap(),
        BBoxExtents {
            min: vec![vec![0.0, 0.0]],
            max: vec![vec![10.0, 10.0]],
        }
    );
    assert_eq!(
        calculate_bbox_extents(&coord_data).unwrap(),
        BBoxExtents {
            min: square_coords(),
            max: square_coords(),
        }
    );
}

#[test]
fn two_dimensional_anchor_positions_match_python_helpers() {
    let data = ViewData::Items(vec![square_coords()]);

    assert_eq!(
        calculate_anchor_upper_left(&data, 2).unwrap(),
        TextAnchorData {
            coordinates: vec![vec![0.0, 0.0]],
            anchor_x: "left",
            anchor_y: "top",
        }
    );
    assert_eq!(
        calculate_anchor_upper_right(&data, 2).unwrap(),
        TextAnchorData {
            coordinates: vec![vec![0.0, 10.0]],
            anchor_x: "right",
            anchor_y: "top",
        }
    );
    assert_eq!(
        calculate_anchor_lower_left(&data, 2).unwrap(),
        TextAnchorData {
            coordinates: vec![vec![10.0, 0.0]],
            anchor_x: "left",
            anchor_y: "bottom",
        }
    );
    assert_eq!(
        calculate_anchor_lower_right(&data, 2).unwrap(),
        TextAnchorData {
            coordinates: vec![vec![10.0, 10.0]],
            anchor_x: "right",
            anchor_y: "bottom",
        }
    );
}

#[test]
fn non_2d_corner_anchors_fall_back_to_center_like_python_helpers() {
    let data = ViewData::Items(vec![square_coords()]);

    for anchor in [
        Anchor::Center,
        Anchor::UpperLeft,
        Anchor::UpperRight,
        Anchor::LowerLeft,
        Anchor::LowerRight,
    ] {
        assert_eq!(
            get_text_anchors(&data, 3, anchor).unwrap(),
            TextAnchorData {
                coordinates: vec![vec![5.0, 5.0]],
                anchor_x: "center",
                anchor_y: "center",
            }
        );
    }
}

#[test]
fn stacked_vertices_are_reduced_over_the_vertex_axis_like_numpy_axis_zero() {
    let data = ViewData::Vertices(vec![
        vec![vec![0.0, 0.0], vec![10.0, 10.0]],
        vec![vec![10.0, 0.0], vec![20.0, 10.0]],
        vec![vec![0.0, 10.0], vec![10.0, 20.0]],
        vec![vec![10.0, 10.0], vec![20.0, 20.0]],
    ]);

    assert_eq!(
        calculate_bbox_centers(&data).unwrap(),
        vec![vec![5.0, 5.0], vec![15.0, 15.0]]
    );
    assert_eq!(
        calculate_bbox_extents(&data).unwrap(),
        BBoxExtents {
            min: vec![vec![0.0, 0.0], vec![10.0, 10.0]],
            max: vec![vec![10.0, 10.0], vec![20.0, 20.0]],
        }
    );
}

#[test]
fn text_anchor_utils_reject_invalid_coordinate_shapes() {
    let data = ViewData::Coordinates(vec![vec![1.0]]);
    assert_eq!(
        calculate_bbox_centers(&data),
        Err(TextAnchorError::InvalidCoordinateDimension(1))
    );

    let ragged = ViewData::Vertices(vec![
        vec![vec![0.0, 0.0]],
        vec![vec![1.0, 1.0], vec![2.0, 2.0]],
    ]);
    assert_eq!(
        calculate_bbox_centers(&ragged),
        Err(TextAnchorError::RaggedCoordinateData)
    );
}
