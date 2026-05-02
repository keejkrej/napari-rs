use std::collections::{BTreeMap, BTreeSet};

use napari_rs::settings::plugins::{
    CallOrderMap, PLUGINS_PREFERENCES_EXCLUDE, PluginHookOption, PluginsSettings,
};

#[test]
fn plugins_settings_defaults_match_python_model_defaults() {
    let settings = PluginsSettings::default();

    assert_eq!(settings.disabled_plugins, BTreeSet::new());
    assert_eq!(settings.extension2reader, BTreeMap::new());
    assert_eq!(settings.extension2writer, BTreeMap::new());
    assert_eq!(
        PLUGINS_PREFERENCES_EXCLUDE,
        &["schema_version", "disabled_plugins", "extension2writer"]
    );
}

#[test]
fn plugin_hook_option_matches_python_typed_dict_shape() {
    let option = PluginHookOption::new("napari-svg", true);
    assert_eq!(option.plugin, "napari-svg");
    assert!(option.enabled);

    let mut order = CallOrderMap::new();
    order.insert(
        "napari_get_reader".to_string(),
        vec![
            PluginHookOption::new("first-plugin", true),
            PluginHookOption::new("disabled-plugin", false),
        ],
    );

    assert_eq!(order["napari_get_reader"].len(), 2);
    assert_eq!(order["napari_get_reader"][1].plugin, "disabled-plugin");
    assert!(!order["napari_get_reader"][1].enabled);
}
