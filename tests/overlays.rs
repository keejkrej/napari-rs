use napari_rs::components::overlays::base::{CanvasOverlay, Overlay, SceneOverlay};
use napari_rs::components::overlays::{
    AxesOverlay, BoundingBoxOverlay, BrushCircleOverlay, ColorBarOverlay, CurrentSliceOverlay,
    LabelsPolygonOverlay, LayerNameOverlay, ScaleBarOverlay, TextOverlay, WelcomeOverlay,
    ZoomOverlay,
};
use napari_rs::components::tooltip::Tooltip;
use napari_rs::components::viewer_constants::CanvasPosition;
use napari_rs::layers::base::constants::Blending;

#[test]
fn overlay_base_defaults_match_python_models() {
    let canvas = CanvasOverlay::default();
    assert!(!canvas.overlay.visible);
    assert_eq!(canvas.overlay.opacity, 1.0);
    assert_eq!(canvas.overlay.order, Overlay::DEFAULT_ORDER);
    assert_eq!(canvas.overlay.blending, Blending::TranslucentNoDepth);
    assert_eq!(canvas.position, CanvasPosition::BottomRight);
    assert!(canvas.box_visible);
    assert_eq!(canvas.box_color, None);
    assert!(!canvas.gridded);

    let scene = SceneOverlay::default();
    assert_eq!(scene.overlay.blending, Blending::Translucent);
}

#[test]
fn tooltip_defaults_match_python_model() {
    let tooltip = Tooltip::default();
    assert!(!tooltip.visible);
    assert_eq!(tooltip.text, "");
}

#[test]
fn axes_defaults_match_python_model() {
    let axes = AxesOverlay::default();
    assert!(axes.labels);
    assert!(axes.colored);
    assert!(!axes.dashed);
    assert!(axes.arrows);
    assert_eq!(axes.base.overlay.blending, Blending::Translucent);
}

#[test]
fn brush_circle_defaults_match_python_model() {
    let brush_circle = BrushCircleOverlay::default();
    assert_eq!(brush_circle.size, 10);
    assert_eq!(brush_circle.position, (0, 0));
    assert!(!brush_circle.position_is_frozen);
}

#[test]
fn bounding_box_defaults_match_python_model() {
    let bounding_box = BoundingBoxOverlay::default();
    assert!(bounding_box.lines);
    assert_eq!(bounding_box.line_thickness, 1.0);
    assert_eq!(bounding_box.line_color, [1.0, 0.0, 0.0, 1.0]);
    assert!(bounding_box.points);
    assert_eq!(bounding_box.point_size, 5.0);
    assert_eq!(bounding_box.point_color, [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(bounding_box.base.overlay.blending, Blending::Translucent);
}

#[test]
fn colorbar_defaults_match_python_model() {
    let colorbar = ColorBarOverlay::default();
    assert_eq!(colorbar.color, None);
    assert_eq!(colorbar.size, (25.0, 150.0));
    assert_eq!(colorbar.tick_length, 5.0);
    assert_eq!(colorbar.font_size, 10.0);
    assert_eq!(colorbar.base.position, CanvasPosition::TopRight);
}

#[test]
fn scale_bar_defaults_and_fixed_length_match_python_model() {
    let scale_bar = ScaleBarOverlay::default();
    assert!(!scale_bar.colored);
    assert_eq!(scale_bar.color, [1.0, 0.0, 1.0, 1.0]);
    assert!(scale_bar.ticks);
    assert_eq!(scale_bar.font_size, 10.0);
    assert_eq!(scale_bar.unit, None);
    assert_eq!(scale_bar.length, None);

    let scale_bar = ScaleBarOverlay {
        length: Some(50.0),
        ..ScaleBarOverlay::default()
    };
    assert_eq!(scale_bar.length, Some(50.0));
}

#[test]
fn text_overlay_defaults_match_python_model() {
    let text = TextOverlay::default();
    assert_eq!(text.color, None);
    assert_eq!(text.font_size, 10.0);
    assert_eq!(text.text, "");
    assert_eq!(text.base.position, CanvasPosition::BottomRight);

    let layer_name = LayerNameOverlay::default();
    assert_eq!(layer_name.text.base.position, CanvasPosition::TopLeft);

    let current_slice = CurrentSliceOverlay::default();
    assert_eq!(
        current_slice.text.base.position,
        CanvasPosition::BottomRight
    );
}

#[test]
fn zoom_overlay_defaults_and_validation_match_python_model() {
    let mut zoom = ZoomOverlay::default();
    assert_eq!(zoom.position, ((0.0, 0.0), (0.0, 0.0)));
    assert_eq!(zoom.zoom_area, ((0.0, 0.0), (0.0, 0.0)));

    zoom.set_position_from_points(&[vec![0.0, 0.0], vec![300.0, 200.0]])
        .unwrap();
    assert_eq!(zoom.position, ((0.0, 0.0), (300.0, 200.0)));

    assert!(
        zoom.set_position_from_points(&[vec![0.0, 0.0, 0.0], vec![300.0, 200.0, 100.0]])
            .is_err()
    );
    assert_eq!(zoom.position, ((0.0, 0.0), (300.0, 200.0)));
}

#[test]
fn blending_strings_match_python_string_enum_values() {
    assert_eq!(Blending::Translucent.to_string(), "translucent");
    assert_eq!(
        Blending::TranslucentNoDepth.to_string(),
        "translucent_no_depth"
    );
    assert_eq!(Blending::Additive.to_string(), "additive");
    assert_eq!(Blending::Minimum.to_string(), "minimum");
    assert_eq!(Blending::Opaque.to_string(), "opaque");
    assert_eq!(Blending::Multiplicative.to_string(), "multiplicative");
    assert_eq!("OPAQUE".parse(), Ok(Blending::Opaque));
}

#[test]
fn welcome_overlay_defaults_match_python_model() {
    let welcome = WelcomeOverlay::default();
    assert_eq!(welcome.position, None);
    assert_eq!(welcome.overlay.order, Overlay::DEFAULT_ORDER + 10);
    assert!(!welcome.gridded);
    assert_eq!(welcome.version, "not-installed");
    assert_eq!(welcome.shortcuts.len(), 4);
    assert_eq!(welcome.tips.len(), 10);
    assert!(
        welcome
            .shortcuts
            .contains(&"napari.window.view.toggle_command_palette".to_owned())
    );
}

#[test]
fn labels_polygon_overlay_tracks_and_clears_pending_polygon() {
    let mut overlay = LabelsPolygonOverlay::default();
    assert!(!overlay.enabled);
    assert!(!overlay.use_double_click_completion_radius);
    assert_eq!(overlay.completion_radius, 20.0);

    overlay.points = vec![vec![0.0, 0.0], vec![1.0, 0.0]];
    assert_eq!(overlay.take_polygon_for_paint(), None);
    assert!(overlay.points.is_empty());

    overlay.points = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0]];
    assert_eq!(
        overlay.take_polygon_for_paint(),
        Some(vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0]])
    );
    assert!(overlay.points.is_empty());
}
