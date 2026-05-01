use napari_rs::layers::utils::color_transformations::normalize_and_broadcast_colors;

#[test]
fn normalize_and_broadcast_colors_returns_matching_length_colors_unchanged() {
    let colors = vec![
        [1.0, 1.0, 1.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.5],
    ];

    assert_eq!(normalize_and_broadcast_colors(3, &colors), colors);
}

#[test]
fn normalize_and_broadcast_colors_tiles_single_color_like_python_helper() {
    assert_eq!(
        normalize_and_broadcast_colors(4, &[[0.0, 1.0, 0.0, 0.75]]),
        vec![[0.0, 1.0, 0.0, 0.75]; 4]
    );
}

#[test]
fn normalize_and_broadcast_colors_resets_mismatched_colors_to_white() {
    assert_eq!(
        normalize_and_broadcast_colors(3, &[[1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]]),
        vec![[1.0, 1.0, 1.0, 1.0]; 3]
    );
    assert_eq!(
        normalize_and_broadcast_colors(3, &[]),
        vec![[1.0, 1.0, 1.0, 1.0]; 3]
    );
}

#[test]
fn normalize_and_broadcast_colors_keeps_input_when_num_entries_is_zero() {
    assert_eq!(
        normalize_and_broadcast_colors(0, &[[1.0, 0.0, 0.0, 1.0]]),
        vec![[1.0, 0.0, 0.0, 1.0]]
    );
    assert_eq!(
        normalize_and_broadcast_colors(0, &[]),
        Vec::<[f32; 4]>::new()
    );
}
