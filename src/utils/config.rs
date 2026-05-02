use std::ffi::OsStr;

pub const MONITOR_ENV_VAR: &str = "NAPARI_MON";

pub fn env_value_is_set(value: Option<&OsStr>) -> bool {
    !matches!(value.and_then(OsStr::to_str), None | Some("0"))
}

pub fn is_env_var_set(env_var: &str) -> bool {
    env_value_is_set(std::env::var_os(env_var).as_deref())
}

pub fn monitor_enabled() -> bool {
    is_env_var_set(MONITOR_ENV_VAR)
}
