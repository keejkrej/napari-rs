use napari_rs::components::overlays::base::{CanvasOverlay, Overlay, SceneOverlay};
use napari_rs::components::overlays::{
    AxesOverlay, BrushCircleOverlay, CurrentSliceOverlay, LayerNameOverlay, ScaleBarOverlay,
    TextOverlay, ZoomOverlay,
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
