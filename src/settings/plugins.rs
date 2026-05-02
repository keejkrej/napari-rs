use std::collections::{BTreeMap, BTreeSet};

pub const PLUGINS_PREFERENCES_EXCLUDE: &[&str] =
    &["schema_version", "disabled_plugins", "extension2writer"];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginHookOption {
    pub plugin: String,
    pub enabled: bool,
}

impl PluginHookOption {
    pub fn new(plugin: impl Into<String>, enabled: bool) -> Self {
        Self {
            plugin: plugin.into(),
            enabled,
        }
    }
}

pub type CallOrderMap = BTreeMap<String, Vec<PluginHookOption>>;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct PluginsSettings {
    pub disabled_plugins: BTreeSet<String>,
    pub extension2reader: BTreeMap<String, String>,
    pub extension2writer: BTreeMap<String, String>,
}
