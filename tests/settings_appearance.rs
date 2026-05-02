use napari_rs::settings::appearance::{
    APPEARANCE_PREFERENCES_EXCLUDE, AVAILABLE_BUILTIN_LOGOS, AVAILABLE_BUILTIN_THEMES,
    AppearanceSettings, HighlightSettings, default_theme_font_size, theme_default_font_size,
    validate_builtin_logo, validate_builtin_theme,
};

#[test]
fn appearance_settings_defaults_match_python_model_defaults() {
    let settings = AppearanceSettings::default();

    assert_eq!(settings.theme, "dark");
    assert_eq!(settings.logo, "auto");
    assert_eq!(settings.font_size, default_theme_font_size());
    assert_eq!(settings.highlight, HighlightSettings::default());
    assert!(!settings.layer_tooltip_visibility);
    assert!(settings.update_status_based_on_layer);

    let highlight = HighlightSettings::default();
    assert_eq!(highlight.highlight_thickness, 1);
    assert_eq!(highlight.highlight_color, [0.0, 0.6, 1.0, 1.0]);
    assert_eq!(APPEARANCE_PREFERENCES_EXCLUDE, &["schema_version"]);
}

#[test]
fn builtin_theme_and_logo_validation_matches_python_strfield_lowering() {
    assert_eq!(AVAILABLE_BUILTIN_THEMES, &["dark", "light", "system"]);
    assert_eq!(validate_builtin_theme("DARK").unwrap(), "dark");
    assert_eq!(validate_builtin_theme("system").unwrap(), "system");
    assert!(validate_builtin_theme("vaporwave").is_err());

    assert!(AVAILABLE_BUILTIN_LOGOS.contains(&"auto"));
    assert!(AVAILABLE_BUILTIN_LOGOS.contains(&"gradient"));
    assert_eq!(validate_builtin_logo("AUTO").unwrap(), "auto");
    assert_eq!(validate_builtin_logo("jedi").unwrap(), "jedi");
    assert!(validate_builtin_logo("missing").is_err());
}

#[test]
fn theme_font_size_update_matches_python_appearance_update_rule() {
    let default_font_size = default_theme_font_size();
    assert_eq!(theme_default_font_size("dark"), Some(default_font_size));
    assert_eq!(theme_default_font_size("light"), Some(default_font_size));
    assert_eq!(theme_default_font_size("unknown"), None);

    let mut settings = AppearanceSettings::default();
    settings.set_theme("light").unwrap();
    assert_eq!(settings.theme, "light");
    assert_eq!(settings.font_size, default_font_size);

    settings.font_size = 15;
    settings.set_theme("dark").unwrap();
    assert_eq!(settings.theme, "dark");
    assert_eq!(settings.font_size, 15);
}

#[test]
fn logo_setter_stores_valid_lowercase_value() {
    let mut settings = AppearanceSettings::default();
    settings.set_logo("JEDI").unwrap();
    assert_eq!(settings.logo, "jedi");

    let previous = settings.logo.clone();
    assert!(settings.set_logo("not-a-logo").is_err());
    assert_eq!(settings.logo, previous);
}
