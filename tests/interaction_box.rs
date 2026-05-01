use napari_rs::components::overlays::{SelectionBoxOverlay, TransformBoxOverlay};
use napari_rs::layers::base::constants::{Blending, InteractionBoxHandle};
use napari_rs::layers::utils::interaction_box::{
    calculate_bounds_from_contained_points, calculate_bounds_from_dynamic_points,
    generate_interaction_box_vertices, get_nearby_handle,
};

#[test]
fn transform_box_vertices_from_bounds_match_python_cases() {
    let vertices = generate_interaction_box_vertices([0.0, 0.0], [10.0, 10.0], false, true);
    assert_eq!(
        vertices,
        vec![[0.0, 0.0], [0.0, 10.0], [10.0, 0.0], [10.0, 10.0]]
    );

    let vertices = generate_interaction_box_vertices([0.0, 0.0], [10.0, 10.0], true, true);
    assert_eq!(
        vertices,
        vec![
            [0.0, 0.0],
            [0.0, 10.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [5.0, 0.0],
            [0.0, 5.0],
            [10.0, 5.0],
            [5.0, 10.0],
            [5.0, -1.0],
        ]
    );
}

#[test]
fn transform_box_get_nearby_handle_matches_python_cases() {
    let vertices = vec![
        [0.0, 0.0],
        [10.0, 0.0],
        [0.0, 10.0],
        [10.0, 10.0],
        [0.0, 5.0],
        [5.0, 0.0],
        [5.0, 10.0],
        [10.0, 5.0],
        [-1.0, 5.0],
    ];

    assert_eq!(
        get_nearby_handle([0.04, -0.05], &vertices),
        Some(InteractionBoxHandle::TopLeft)
    );
    assert_eq!(
        get_nearby_handle([-1.05, 4.95], &vertices),
        Some(InteractionBoxHandle::Rotation)
    );
    assert_eq!(
        get_nearby_handle([5.0, 5.0], &vertices),
        Some(InteractionBoxHandle::Inside)
    );
    assert_eq!(get_nearby_handle([12.0, -1.0], &vertices), None);
}

#[test]
fn bounds_from_contained_points_match_selection_box_python_case() {
    let points = [[0.0, 5.0], [-3.0, 0.0], [0.0, 7.0]];
    assert_eq!(
        calculate_bounds_from_contained_points(&points),
        Some(([-3.0, 0.0], [0.0, 7.0]))
    );

    let mut selection_box = SelectionBoxOverlay::default();
    selection_box.update_from_points(&points);
    assert_eq!(selection_box.bounds, ([-3.0, 0.0], [0.0, 7.0]));
    assert!(!selection_box.handles);
    assert_eq!(selection_box.selected_handle, None);
    assert_eq!(selection_box.base.overlay.blending, Blending::Translucent);
}

#[test]
fn dynamic_bounds_validation_rejects_non_2d_points() {
    assert_eq!(
        calculate_bounds_from_dynamic_points(&[vec![0.0, 1.0], vec![2.0, 3.0]]).unwrap(),
        Some(([0.0, 2.0], [1.0, 3.0]))
    );
    assert!(calculate_bounds_from_dynamic_points(&[vec![0.0, 1.0, 2.0]]).is_err());
}

#[test]
fn transform_box_overlay_defaults_match_python_model() {
    let transform_box = TransformBoxOverlay::default();
    assert_eq!(transform_box.selected_handle, None);
    assert_eq!(transform_box.base.overlay.blending, Blending::Translucent);
}
