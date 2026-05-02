use std::ffi::OsStr;

use napari_rs::utils::base::{DEFAULT_LOCALE, FILENAME, default_config_path};
use napari_rs::utils::config::{MONITOR_ENV_VAR, env_value_is_set};

#[test]
fn env_value_is_set_matches_python_config_set_helper() {
    assert!(!env_value_is_set(None));
    assert!(!env_value_is_set(Some(OsStr::new("0"))));
    assert!(env_value_is_set(Some(OsStr::new(""))));
    assert!(env_value_is_set(Some(OsStr::new("1"))));
    assert!(env_value_is_set(Some(OsStr::new("/tmp/monitor.yaml"))));
}

#[test]
fn monitor_env_var_name_matches_python_config_module() {
    assert_eq!(MONITOR_ENV_VAR, "NAPARI_MON");
}

#[test]
fn base_settings_defaults_match_python_base_module() {
    assert_eq!(FILENAME, "settings.yaml");
    assert_eq!(DEFAULT_LOCALE, "en");
    assert_eq!(
        default_config_path("/tmp/napari-config"),
        std::path::PathBuf::from("/tmp/napari-config/settings.yaml")
    );
}
