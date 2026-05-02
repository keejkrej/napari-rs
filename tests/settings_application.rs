use std::path::PathBuf;

use napari_rs::settings::application::{
    APPLICATION_PREFERENCES_EXCLUDE, ApplicationSettings, DEFAULT_MEM_FRACTION, DaskSettings,
    MAX_GRID_SPACING, validate_window_state,
};
use napari_rs::settings::constants::{BrushSizeOnMouseModifiers, LabelDType, LoopMode};
use napari_rs::utils::base::DEFAULT_LOCALE;
use napari_rs::utils::camera_orientations::{
    DepthAxisOrientation, HorizontalAxisOrientation, VerticalAxisOrientation,
};
use napari_rs::utils::notifications::NotificationSeverity;

#[test]
fn application_settings_defaults_match_python_model_defaults() {
    let settings = ApplicationSettings::default();

    assert!(settings.first_time);
    assert!(settings.ipy_interactive);
    assert_eq!(settings.language, DEFAULT_LOCALE);
    assert!(settings.save_window_geometry);
    assert!(!settings.save_window_state);
    assert_eq!(settings.window_position, None);
    assert_eq!(settings.window_size, None);
    assert!(!settings.window_maximized);
    assert!(!settings.window_fullscreen);
    assert_eq!(settings.window_state, None);
    assert!(settings.window_statusbar);
    assert_eq!(settings.preferences_size, None);
    assert_eq!(settings.gui_notification_level, NotificationSeverity::Info);
    assert_eq!(
        settings.console_notification_level,
        NotificationSeverity::None
    );
    assert!(settings.open_history.is_empty());
    assert!(settings.save_history.is_empty());
    assert_eq!(settings.playback_fps, 10);
    assert_eq!(settings.playback_mode, LoopMode::Loop);
    assert_eq!(
        settings.depth_axis_orientation,
        DepthAxisOrientation::Towards
    );
    assert_eq!(
        settings.vertical_axis_orientation,
        VerticalAxisOrientation::Down
    );
    assert_eq!(
        settings.horizontal_axis_orientation,
        HorizontalAxisOrientation::Right
    );
    assert_eq!(settings.grid_stride, 1);
    assert_eq!(settings.grid_width, -1);
    assert_eq!(settings.grid_height, -1);
    assert_eq!(settings.grid_spacing, 0.0);
    assert!(settings.confirm_close_window);
    assert_eq!(settings.hold_button_delay, 0.5);
    assert_eq!(
        settings.brush_size_on_mouse_move_modifiers,
        BrushSizeOnMouseModifiers::Alt
    );
    assert_eq!(settings.dask, DaskSettings::default());
    assert_eq!(settings.new_labels_dtype, LabelDType::Uint8);
    assert!(settings.plugin_widget_positions.is_empty());
    assert_eq!(settings.startup_script, PathBuf::new());
}

#[test]
fn application_settings_constants_match_python_module_values() {
    assert_eq!(MAX_GRID_SPACING, 1500.0);
    assert_eq!(DEFAULT_MEM_FRACTION, 0.25);
    assert_eq!(
        APPLICATION_PREFERENCES_EXCLUDE,
        &[
            "schema_version",
            "preferences_size",
            "first_time",
            "window_position",
            "window_size",
            "window_maximized",
            "window_fullscreen",
            "window_state",
            "window_statusbar",
            "open_history",
            "save_history",
            "ipy_interactive",
            "plugin_widget_positions",
        ]
    );
}

#[test]
fn window_state_validator_matches_python_qbyte_prefix_rule() {
    assert_eq!(validate_window_state(None).unwrap(), None);
    assert_eq!(
        validate_window_state(Some("")).unwrap(),
        Some(String::new())
    );
    assert_eq!(
        validate_window_state(Some("!QBYTE_saved")).unwrap(),
        Some("!QBYTE_saved".to_string())
    );
    assert_eq!(
        validate_window_state(Some("bad")).unwrap_err().to_string(),
        "QByte strings must start with '!QBYTE_'"
    );
}
