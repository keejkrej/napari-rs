use napari_rs::layers::utils::plane::{
    ClippingPlane, ClippingPlaneList, Plane, PlaneError, SlicingPlane,
};

fn assert_vec3_close(actual: [f64; 3], expected: [f64; 3]) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12);
    }
}

#[test]
fn plane_defaults_and_normalization_match_python_model() {
    assert_eq!(
        Plane::default(),
        Plane {
            position: [0.0, 0.0, 0.0],
            normal: [1.0, 0.0, 0.0],
        }
    );

    let plane = Plane::new([0.0, 0.0, 0.0], [5.0, 0.0, 0.0]).unwrap();
    assert_eq!(plane.normal, [1.0, 0.0, 0.0]);
}

#[test]
fn plane_from_points_uses_cross_product_normal_and_centroid_position() {
    let plane = Plane::from_points([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();

    assert_eq!(plane.normal, [0.0, 0.0, 1.0]);
    assert_vec3_close(plane.position, [1.0 / 3.0, 1.0 / 3.0, 0.0]);
}

#[test]
fn plane_shifts_along_normal_vector() {
    let mut plane = Plane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();

    plane.shift_along_normal_vector(0.5);

    assert_eq!(plane.position, [0.5, 0.0, 0.0]);
}

#[test]
fn plane_intersects_with_line_using_existing_geometry_helper() {
    let plane = Plane::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).unwrap();

    let intersection = plane.intersect_with_line([1.0, 2.0, 0.0], [0.0, 0.0, 1.0]);

    assert_eq!(intersection, [1.0, 2.0, 5.0]);
}

#[test]
fn slicing_plane_array_round_trips_position_and_normal() {
    let array = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

    let plane = SlicingPlane::from_array(array, 2.0).unwrap();

    assert_eq!(plane.thickness, 2.0);
    assert_eq!(plane.as_array(), array);
}

#[test]
fn clipping_plane_defaults_to_enabled_like_python_model() {
    let plane = ClippingPlane::default();

    assert!(plane.enabled);
    assert_eq!(plane.plane, Plane::default());
}

#[test]
fn clipping_plane_list_array_conversion_omits_disabled_planes() {
    let array = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
    let mut plane_list = ClippingPlaneList::from_array(vec![array, array], true).unwrap();
    plane_list.add_plane(ClippingPlane::from_array(array, false).unwrap());

    assert_eq!(plane_list.len(), 3);
    assert_eq!(plane_list.as_array(), vec![array, array]);
}

#[test]
fn clipping_plane_list_from_bounding_box_matches_python_positions_and_normals() {
    let plane_list =
        ClippingPlaneList::from_bounding_box([0.0, 0.0, 0.0], [2.0, 2.0, 2.0], true).unwrap();

    assert_eq!(plane_list.len(), 6);
    assert_eq!(
        plane_list.as_array(),
        vec![
            [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]],
            [[0.0, -1.0, 0.0], [0.0, 1.0, 0.0]],
            [[0.0, 1.0, 0.0], [0.0, -1.0, 0.0]],
            [[0.0, 0.0, -1.0], [0.0, 0.0, 1.0]],
            [[0.0, 0.0, 1.0], [0.0, 0.0, -1.0]],
        ]
    );
    let sum: f64 = plane_list.as_array().iter().flatten().flatten().sum();
    assert_eq!(sum, 0.0);
}

#[test]
fn planes_reject_zero_normals() {
    assert_eq!(
        Plane::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        Err(PlaneError::ZeroNormal)
    );
    assert_eq!(
        Plane::from_points([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 2.0, 2.0]),
        Err(PlaneError::ZeroNormal)
    );
}
