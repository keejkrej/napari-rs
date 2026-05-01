use std::collections::BTreeMap;

use napari_rs::layers::utils::color_manager_utils::{
    ColorArgument, ColorManagerUtilsError, calculate_contrast_limits, guess_continuous,
    is_color_mapped, normalize_property_values,
};
use napari_rs::layers::utils::layer_utils::{Properties, PropertyValue};

fn properties(entries: impl IntoIterator<Item = (&'static str, Vec<PropertyValue>)>) -> Properties {
    entries
        .into_iter()
        .map(|(key, values)| (key.to_owned(), values))
        .collect::<BTreeMap<_, _>>()
}

#[test]
fn guess_continuous_matches_python_dtype_and_unique_count_logic() {
    assert!(guess_continuous(&[
        PropertyValue::Float(1.0),
        PropertyValue::Float(2.0),
        PropertyValue::Float(3.0),
    ]));

    assert!(!guess_continuous(&[
        PropertyValue::Bool(true),
        PropertyValue::Bool(false),
    ]));
    assert!(!guess_continuous(&[
        PropertyValue::Int(1),
        PropertyValue::Int(2),
        PropertyValue::Int(3),
    ]));
    assert!(guess_continuous(
        &(0..20).map(PropertyValue::Int).collect::<Vec<_>>()
    ));
    assert!(!guess_continuous(&[
        PropertyValue::String("a".to_owned()),
        PropertyValue::String("b".to_owned()),
    ]));
    assert!(!guess_continuous(&[]));
}

#[test]
fn is_color_mapped_matches_python_color_argument_type_logic() {
    let props = properties([
        ("hello", vec![PropertyValue::Int(1), PropertyValue::Int(2)]),
        ("hi", vec![PropertyValue::String("red".to_owned())]),
    ]);

    assert!(is_color_mapped(
        &ColorArgument::Name("hello".to_owned()),
        &props
    ));
    assert!(!is_color_mapped(
        &ColorArgument::Name("red".to_owned()),
        &props
    ));
    assert!(is_color_mapped(&ColorArgument::Mapped, &props));
    assert!(!is_color_mapped(
        &ColorArgument::Array(vec![[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 0.0, 1.0]]),
        &props
    ));
}

#[test]
fn calculate_contrast_limits_matches_python_quantitative_encoding_helper() {
    assert_eq!(calculate_contrast_limits(&[]), None);
    assert_eq!(calculate_contrast_limits(&[1.0, 1.0, 1.0]), None);
    assert_eq!(
        calculate_contrast_limits(&[3.0, -1.0, 2.0]),
        Some((-1.0, 3.0))
    );
    assert_eq!(calculate_contrast_limits(&[f64::NAN, 1.0]), None);
}

#[test]
fn normalize_property_values_matches_python_map_property_interpolation_core() {
    assert_eq!(
        normalize_property_values(&[-1.0, 0.0, 5.0, 10.0, 11.0], Some((0.0, 10.0))).unwrap(),
        (vec![0.0, 0.0, 0.5, 1.0, 1.0], (0.0, 10.0))
    );
    assert_eq!(
        normalize_property_values(&[3.0, -1.0, 2.0], None).unwrap(),
        (vec![1.0, 0.0, 0.75], (-1.0, 3.0))
    );
}

#[test]
fn normalize_property_values_validates_inputs() {
    assert_eq!(
        normalize_property_values(&[], None),
        Err(ColorManagerUtilsError::EmptyPropertyValues)
    );
    assert_eq!(
        normalize_property_values(&[1.0, 1.0], None),
        Err(ColorManagerUtilsError::InvalidContrastLimits)
    );
    assert_eq!(
        normalize_property_values(&[0.0, 1.0], Some((1.0, 1.0))),
        Err(ColorManagerUtilsError::InvalidContrastLimits)
    );
}
