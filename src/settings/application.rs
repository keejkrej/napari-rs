use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

use crate::settings::constants::{BrushSizeOnMouseModifiers, LabelDType, LoopMode};
use crate::utils::base::DEFAULT_LOCALE;
use crate::utils::camera_orientations::{
    DEFAULT_ORIENTATION, DepthAxisOrientation, HorizontalAxisOrientation, VerticalAxisOrientation,
};
use crate::utils::notifications::NotificationSeverity;

pub const MAX_GRID_SPACING: f64 = 1500.0;
pub const DEFAULT_MEM_FRACTION: f64 = 0.25;

pub const APPLICATION_PREFERENCES_EXCLUDE: &[&str] = &[
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
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidWindowState;

impl fmt::Display for InvalidWindowState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("QByte strings must start with '!QBYTE_'")
    }
}

impl std::error::Error for InvalidWindowState {}

pub fn validate_window_state(value: Option<&str>) -> Result<Option<String>, InvalidWindowState> {
    match value {
        None | Some("") => Ok(value.map(str::to_string)),
        Some(value) if value.starts_with("!QBYTE_") => Ok(Some(value.to_string())),
        Some(_) => Err(InvalidWindowState),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DaskSettings {
    pub enabled: bool,
    pub cache_gb: Option<f64>,
}

impl Default for DaskSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_gb: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationSettings {
    pub first_time: bool,
    pub ipy_interactive: bool,
    pub language: String,
    pub save_window_geometry: bool,
    pub save_window_state: bool,
    pub window_position: Option<(i32, i32)>,
    pub window_size: Option<(u32, u32)>,
    pub window_maximized: bool,
    pub window_fullscreen: bool,
    pub window_state: Option<String>,
    pub window_statusbar: bool,
    pub preferences_size: Option<(u32, u32)>,
    pub gui_notification_level: NotificationSeverity,
    pub console_notification_level: NotificationSeverity,
    pub open_history: Vec<String>,
    pub save_history: Vec<String>,
    pub playback_fps: i32,
    pub playback_mode: LoopMode,
    pub depth_axis_orientation: DepthAxisOrientation,
    pub vertical_axis_orientation: VerticalAxisOrientation,
    pub horizontal_axis_orientation: HorizontalAxisOrientation,
    pub grid_stride: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub grid_spacing: f64,
    pub confirm_close_window: bool,
    pub hold_button_delay: f64,
    pub brush_size_on_mouse_move_modifiers: BrushSizeOnMouseModifiers,
    pub dask: DaskSettings,
    pub new_labels_dtype: LabelDType,
    pub plugin_widget_positions: BTreeMap<String, String>,
    pub startup_script: PathBuf,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            first_time: true,
            ipy_interactive: true,
            language: DEFAULT_LOCALE.to_string(),
            save_window_geometry: true,
            save_window_state: false,
            window_position: None,
            window_size: None,
            window_maximized: false,
            window_fullscreen: false,
            window_state: None,
            window_statusbar: true,
            preferences_size: None,
            gui_notification_level: NotificationSeverity::Info,
            console_notification_level: NotificationSeverity::None,
            open_history: Vec::new(),
            save_history: Vec::new(),
            playback_fps: 10,
            playback_mode: LoopMode::Loop,
            depth_axis_orientation: DEFAULT_ORIENTATION.0,
            vertical_axis_orientation: DEFAULT_ORIENTATION.1,
            horizontal_axis_orientation: DEFAULT_ORIENTATION.2,
            grid_stride: 1,
            grid_width: -1,
            grid_height: -1,
            grid_spacing: 0.0,
            confirm_close_window: true,
            hold_button_delay: 0.5,
            brush_size_on_mouse_move_modifiers: BrushSizeOnMouseModifiers::Alt,
            dask: DaskSettings::default(),
            new_labels_dtype: LabelDType::Uint8,
            plugin_widget_positions: BTreeMap::new(),
            startup_script: PathBuf::new(),
        }
    }
}
