use napari_rs::components::grid::GridCanvas;

#[test]
fn grid_defaults_match_python_model_defaults() {
    let grid = GridCanvas::default();

    assert!(!grid.enabled);
    assert_eq!(grid.shape, (-1, -1));
    assert_eq!(grid.stride, 1);
    assert_eq!(grid.spacing, 0.0);
}

#[test]
fn grid_creation_allows_shape_and_stride_override() {
    let grid = GridCanvas {
        shape: (3, 4),
        stride: 2,
        ..GridCanvas::default()
    };

    assert_eq!(grid.shape, (3, 4));
    assert_eq!(grid.stride, 2);
}

#[test]
fn actual_shape_and_position_match_python_cases() {
    let grid = GridCanvas {
        enabled: true,
        ..GridCanvas::default()
    };

    assert_eq!(grid.actual_shape(9), (3, 3));
    assert_eq!(grid.position(0, 9), (0, 0));
    assert_eq!(grid.position(2, 9), (0, 2));
    assert_eq!(grid.position(3, 9), (1, 0));
    assert_eq!(grid.position(8, 9), (2, 2));

    assert_eq!(grid.actual_shape(5), (2, 3));
    assert_eq!(grid.position(0, 5), (0, 0));
    assert_eq!(grid.position(2, 5), (0, 2));
    assert_eq!(grid.position(3, 5), (1, 0));

    assert_eq!(grid.actual_shape(10), (3, 4));
    assert_eq!(grid.position(0, 10), (0, 0));
    assert_eq!(grid.position(2, 10), (0, 2));
    assert_eq!(grid.position(3, 10), (0, 3));
    assert_eq!(grid.position(8, 10), (2, 0));
}

#[test]
fn actual_shape_with_stride_matches_python_cases() {
    let grid = GridCanvas {
        enabled: true,
        stride: 2,
        ..GridCanvas::default()
    };

    assert_eq!(grid.actual_shape(7), (2, 2));
    assert_eq!(grid.position(0, 7), (0, 0));
    assert_eq!(grid.position(1, 7), (0, 0));
    assert_eq!(grid.position(2, 7), (0, 1));
    assert_eq!(grid.position(3, 7), (0, 1));
    assert_eq!(grid.position(6, 7), (1, 1));

    assert_eq!(grid.actual_shape(3), (1, 2));
    assert_eq!(grid.position(0, 3), (0, 0));
    assert_eq!(grid.position(1, 3), (0, 0));
    assert_eq!(grid.position(2, 3), (0, 1));
}

#[test]
fn negative_stride_reverses_layer_order_like_python() {
    let grid = GridCanvas {
        enabled: true,
        stride: -1,
        ..GridCanvas::default()
    };

    assert_eq!(grid.actual_shape(9), (3, 3));
    assert_eq!(grid.position(0, 9), (2, 2));
    assert_eq!(grid.position(2, 9), (2, 0));
    assert_eq!(grid.position(3, 9), (1, 2));
    assert_eq!(grid.position(8, 9), (0, 0));
}

#[test]
fn disabled_grid_has_single_shape_and_origin_position() {
    let grid = GridCanvas::default();

    assert_eq!(grid.actual_shape(9), (1, 1));
    assert_eq!(grid.position(3, 9), (0, 0));
    assert_eq!(grid.contents_at((0, 0), 4), vec![0, 1, 2, 3]);
}

#[test]
fn contents_and_viewboxes_report_layer_indices_for_positions() {
    let grid = GridCanvas {
        enabled: true,
        stride: 2,
        ..GridCanvas::default()
    };

    assert_eq!(grid.contents_at((0, 0), 7), vec![0, 1]);
    assert_eq!(grid.contents_at((0, 1), 7), vec![2, 3]);
    assert_eq!(grid.contents_at((1, 1), 7), vec![6]);

    let viewboxes = grid.viewboxes(3);
    assert_eq!(viewboxes, vec![((0, 0), vec![0, 1]), ((0, 1), vec![2])]);
}

#[test]
fn canvas_spacing_matches_raw_and_safety_limited_python_logic() {
    let proportional = GridCanvas {
        enabled: true,
        spacing: 0.1,
        ..GridCanvas::default()
    };
    assert_eq!(proportional.compute_canvas_spacing_raw((300, 200), 4), 12);
    assert_eq!(proportional.compute_canvas_spacing((300, 200), 4), 12);

    let pixels = GridCanvas {
        enabled: true,
        spacing: 25.0,
        ..GridCanvas::default()
    };
    assert_eq!(pixels.compute_canvas_spacing_raw((300, 200), 4), 25);
    assert_eq!(pixels.compute_canvas_spacing((300, 200), 4), 25);

    let too_large = GridCanvas {
        enabled: true,
        spacing: 500.0,
        ..GridCanvas::default()
    };
    assert_eq!(too_large.compute_canvas_spacing((100, 100), 9), 20);
}
