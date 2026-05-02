use std::cell::Cell;
use std::rc::Rc;

use napari_rs::app_model::context::{ContextEntry, ContextMapping, ContextValue};

#[test]
fn context_mapping_reports_initial_keys_like_python_mapping() {
    let mapping = ContextMapping::new([
        ("visible".to_string(), ContextEntry::Value(true.into())),
        ("count".to_string(), ContextEntry::Value(3usize.into())),
    ]);

    assert_eq!(mapping.len(), 2);
    assert!(!mapping.is_empty());
    assert!(mapping.contains_key("visible"));
    assert!(!mapping.contains_key("missing"));
    assert_eq!(mapping.keys().collect::<Vec<_>>(), vec!["count", "visible"]);
}

#[test]
fn context_mapping_evaluates_lazy_values_once_and_caches_them() {
    let calls = Rc::new(Cell::new(0));
    let calls_for_getter = Rc::clone(&calls);
    let mut mapping = ContextMapping::new([(
        "computed".to_string(),
        ContextEntry::Lazy(Box::new(move || {
            let next = calls_for_getter.get() + 1;
            calls_for_getter.set(next);
            ContextValue::Int(next)
        })),
    )]);

    assert_eq!(calls.get(), 0);
    assert_eq!(mapping.get("computed").unwrap(), &ContextValue::Int(1));
    assert_eq!(calls.get(), 1);
    assert_eq!(mapping.get("computed").unwrap(), &ContextValue::Int(1));
    assert_eq!(calls.get(), 1);
}

#[test]
fn context_mapping_caches_static_values_after_first_lookup() {
    let mut mapping = ContextMapping::new([
        (
            "status".to_string(),
            ContextEntry::Value(ContextValue::String("ready".to_string())),
        ),
        ("none".to_string(), ContextEntry::Value(ContextValue::None)),
    ]);

    assert_eq!(
        mapping.get("status").unwrap(),
        &ContextValue::String("ready".to_string())
    );
    assert_eq!(mapping.get("none").unwrap(), &ContextValue::None);
}

#[test]
fn context_mapping_missing_key_error_matches_python_message_shape() {
    let mut mapping = ContextMapping::empty();
    let error = mapping.get("missing").unwrap_err();

    assert_eq!(error.key(), "missing");
    assert_eq!(error.to_string(), "Key \"missing\" not found");
}
