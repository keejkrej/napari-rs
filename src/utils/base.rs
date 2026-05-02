use std::path::{Path, PathBuf};

pub const FILENAME: &str = "settings.yaml";
pub const DEFAULT_LOCALE: &str = "en";

pub fn default_config_path(user_config_dir: impl AsRef<Path>) -> PathBuf {
    user_config_dir.as_ref().join(FILENAME)
}
