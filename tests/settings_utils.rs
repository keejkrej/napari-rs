use std::collections::BTreeMap;

use napari_rs::settings::utils::coerce_extensions_to_globs;

fn settings(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(pattern, reader)| ((*pattern).to_owned(), (*reader).to_owned()))
        .collect()
}

#[test]
fn extension_coercion_to_glob_replaces_bare_extensions_like_python() {
    let original = settings(&[(".tif", "fake-plugin"), ("*.csv", "other-plugin")]);

    let coerced = coerce_extensions_to_globs(&original);

    assert!(!coerced.contains_key(".tif"));
    assert_eq!(coerced.get("*.tif").unwrap(), "fake-plugin");
    assert_eq!(coerced.get("*.csv").unwrap(), "other-plugin");
}

#[test]
fn extension_coercion_preserves_complex_glob_patterns_like_python() {
    let original = settings(&[(".blah*.tif", "fake-plugin"), ("*.csv", "other-plugin")]);

    let coerced = coerce_extensions_to_globs(&original);

    assert_eq!(coerced.get(".blah*.tif").unwrap(), "fake-plugin");
    assert_eq!(coerced.get("*.csv").unwrap(), "other-plugin");
}

#[test]
fn extension_coercion_returns_new_map_without_mutating_input() {
    let original = settings(&[("*.tif", "fake-plugin"), (".csv", "other-plugin")]);

    let coerced = coerce_extensions_to_globs(&original);

    assert_eq!(
        coerced,
        settings(&[("*.tif", "fake-plugin"), ("*.csv", "other-plugin")])
    );
    assert_eq!(
        original,
        settings(&[("*.tif", "fake-plugin"), (".csv", "other-plugin")])
    );
}
