use std::fmt;

pub const APPEARANCE_PREFERENCES_EXCLUDE: &[&str] = &["schema_version"];

pub const AVAILABLE_BUILTIN_THEMES: &[&str] = &["dark", "light", "system"];
pub const AVAILABLE_BUILTIN_LOGOS: &[&str] = &[
    "auto",
    "christmas",
    "gradient",
    "halloween",
    "jedi",
    "maythefourth",
    "sith",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidAppearanceValue {
    field: &'static str,
    value: String,
}

impl InvalidAppearanceValue {
    fn new(field: &'static str, value: &str) -> Self {
        Self {
            field,
            value: value.to_string(),
        }
    }
}

impl fmt::Display for InvalidAppearanceValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid {} value: {}", self.field, self.value)
    }
}

impl std::error::Error for InvalidAppearanceValue {}

pub fn validate_builtin_theme(value: &str) -> Result<String, InvalidAppearanceValue> {
    let value = value.to_ascii_lowercase();
    if AVAILABLE_BUILTIN_THEMES.contains(&value.as_str()) {
        Ok(value)
    } else {
        Err(InvalidAppearanceValue::new("theme", &value))
    }
}

pub fn validate_builtin_logo(value: &str) -> Result<String, InvalidAppearanceValue> {
    let value = value.to_ascii_lowercase();
    if AVAILABLE_BUILTIN_LOGOS.contains(&value.as_str()) {
        Ok(value)
    } else {
        Err(InvalidAppearanceValue::new("logo", &value))
    }
}

pub const fn default_theme_font_size() -> i32 {
    if cfg!(target_os = "macos") { 12 } else { 9 }
}

pub fn theme_default_font_size(theme: &str) -> Option<i32> {
    match theme {
        "dark" | "light" | "system" => Some(default_theme_font_size()),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HighlightSettings {
    pub highlight_thickness: i32,
    pub highlight_color: [f64; 4],
}

impl Default for HighlightSettings {
    fn default() -> Self {
        Self {
            highlight_thickness: 1,
            highlight_color: [0.0, 0.6, 1.0, 1.0],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppearanceSettings {
    pub theme: String,
    pub logo: String,
    pub font_size: i32,
    pub highlight: HighlightSettings,
    pub layer_tooltip_visibility: bool,
    pub update_status_based_on_layer: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            logo: "auto".to_string(),
            font_size: default_theme_font_size(),
            highlight: HighlightSettings::default(),
            layer_tooltip_visibility: false,
            update_status_based_on_layer: true,
        }
    }
}

impl AppearanceSettings {
    pub fn set_theme(&mut self, theme: &str) -> Result<(), InvalidAppearanceValue> {
        let next_theme = validate_builtin_theme(theme)?;
        if next_theme != self.theme {
            if let (Some(current_font_size), Some(next_font_size)) = (
                theme_default_font_size(&self.theme),
                theme_default_font_size(&next_theme),
            ) && self.font_size == current_font_size
            {
                self.font_size = next_font_size;
            }
            self.theme = next_theme;
        }
        Ok(())
    }

    pub fn set_logo(&mut self, logo: &str) -> Result<(), InvalidAppearanceValue> {
        self.logo = validate_builtin_logo(logo)?;
        Ok(())
    }
}
