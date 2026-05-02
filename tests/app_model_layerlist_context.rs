use napari_rs::app_model::layerlist_context::{
    LayerInfo, LayerSelection, active_layer_dtype, active_layer_is_image_3d, active_layer_is_rgb,
    active_layer_ndim, active_layer_shape, active_layer_supports_features, active_layer_type,
    all_selected_layers_of_type, all_selected_layers_same_shape, all_selected_layers_same_type,
    all_selected_layers_support_colorbar, count_selected_layers_of_type, num_layers,
    num_selected_layers, selected_empty_shapes_layer,
};

#[test]
fn layerlist_context_counts_match_python_len_and_type_helpers() {
    let layers = vec![
        LayerInfo::new("image"),
        LayerInfo::new("labels"),
        LayerInfo::new("image"),
    ];
    let selection = LayerSelection::new(layers.clone(), Some(0));

    assert_eq!(num_layers(&layers), 3);
    assert_eq!(num_selected_layers(&selection), 3);
    assert_eq!(count_selected_layers_of_type(&selection, "image"), 2);
    assert_eq!(count_selected_layers_of_type(&selection, "labels"), 1);
    assert_eq!(count_selected_layers_of_type(&selection, "points"), 0);
    assert!(!all_selected_layers_of_type(&selection, "image"));
    assert!(all_selected_layers_of_type(
        &LayerSelection::new(vec![LayerInfo::new("image")], Some(0)),
        "image"
    ));
    assert!(!all_selected_layers_of_type(
        &LayerSelection::empty(),
        "image"
    ));
}

#[test]
fn active_layer_context_helpers_match_python_getattr_behavior() {
    let active = LayerInfo::new("image")
        .with_ndim(4)
        .with_shape(vec![3, 4, 5, 6])
        .with_dtype("uint8")
        .with_rgb(true)
        .with_features(true)
        .with_colorbar(true);
    let selection = LayerSelection::new(vec![LayerInfo::new("labels"), active], Some(1));

    assert!(active_layer_is_rgb(&selection));
    assert_eq!(active_layer_type(&selection), Some("image"));
    assert_eq!(active_layer_ndim(&selection), Some(4));
    assert_eq!(
        active_layer_shape(&selection),
        Some([3, 4, 5, 6].as_slice())
    );
    assert_eq!(active_layer_dtype(&selection), Some("uint8"));
    assert!(active_layer_supports_features(&selection));

    let empty = LayerSelection::empty();
    assert!(!active_layer_is_rgb(&empty));
    assert_eq!(active_layer_type(&empty), None);
    assert_eq!(active_layer_ndim(&empty), None);
    assert_eq!(active_layer_shape(&empty), None);
    assert_eq!(active_layer_dtype(&empty), None);
    assert!(!active_layer_supports_features(&empty));
}

#[test]
fn same_type_shape_and_colorbar_helpers_match_python_selection_logic() {
    let images = LayerSelection::new(
        vec![
            LayerInfo::new("image")
                .with_shape(vec![4, 5])
                .with_colorbar(true),
            LayerInfo::new("image")
                .with_shape(vec![4, 5])
                .with_colorbar(true),
        ],
        Some(0),
    );
    assert!(all_selected_layers_same_type(&images));
    assert!(all_selected_layers_same_shape(&images));
    assert!(all_selected_layers_support_colorbar(&images));

    let mixed = LayerSelection::new(
        vec![
            LayerInfo::new("image")
                .with_shape(vec![4, 5])
                .with_colorbar(true),
            LayerInfo::new("labels")
                .with_shape(vec![5, 5])
                .with_colorbar(false),
        ],
        Some(0),
    );
    assert!(!all_selected_layers_same_type(&mixed));
    assert!(!all_selected_layers_same_shape(&mixed));
    assert!(!all_selected_layers_support_colorbar(&mixed));

    assert!(all_selected_layers_same_type(&LayerSelection::empty()));
    assert!(all_selected_layers_same_shape(&LayerSelection::empty()));
    assert!(!all_selected_layers_support_colorbar(
        &LayerSelection::empty()
    ));
}

#[test]
fn image_3d_and_empty_shapes_checks_match_python_helpers() {
    let rgb_3d = LayerSelection::new(
        vec![LayerInfo::new("image").with_ndim(3).with_rgb(true)],
        Some(0),
    );
    assert!(!active_layer_is_image_3d(&rgb_3d));

    let grayscale_3d = LayerSelection::new(
        vec![LayerInfo::new("image").with_ndim(3).with_rgb(false)],
        Some(0),
    );
    assert!(active_layer_is_image_3d(&grayscale_3d));

    let four_dim_rgb = LayerSelection::new(
        vec![LayerInfo::new("image").with_ndim(4).with_rgb(true)],
        Some(0),
    );
    assert!(active_layer_is_image_3d(&four_dim_rgb));

    let shapes = LayerSelection::new(
        vec![
            LayerInfo::new("shapes").with_data_len(2),
            LayerInfo::new("shapes").with_data_len(0),
        ],
        Some(0),
    );
    assert!(selected_empty_shapes_layer(&shapes));

    let non_empty_shapes =
        LayerSelection::new(vec![LayerInfo::new("shapes").with_data_len(1)], Some(0));
    assert!(!selected_empty_shapes_layer(&non_empty_shapes));
}
