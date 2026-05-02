use napari_rs::utils::status_messages::{
    LayerStatusValue, StatusValue, generate_layer_coords_status, generate_layer_status,
    generate_layer_status_strings, status_format,
};

#[test]
fn status_format_matches_python_string_none_numeric_and_nested_cases() {
    let numeric = StatusValue::List(vec![
        StatusValue::Integer(1),
        StatusValue::Integer(10),
        StatusValue::Integer(100),
        StatusValue::Integer(1000),
        StatusValue::Float(1e6),
        StatusValue::Float(-std::f64::consts::TAU),
        StatusValue::Float(123.932_021),
        StatusValue::Float(1_123.939_200_1),
        StatusValue::Float(2.0 * std::f64::consts::PI),
        StatusValue::Float(std::f64::consts::E),
    ]);

    assert_eq!(
        status_format(&numeric),
        "[1, 10, 100, 1000, 1e+06, -6.28, 124, 1.12e+03, 6.28, 2.72]"
    );
    assert_eq!(
        status_format(&StatusValue::from("hello world")),
        "hello world"
    );
    assert_eq!(status_format(&StatusValue::None), "");
    assert_eq!(
        status_format(&StatusValue::List(vec![
            StatusValue::Float(1e6),
            StatusValue::None,
            StatusValue::from("hello world"),
        ])),
        "[1e+06, , hello world]"
    );
}

#[test]
fn layer_status_strings_match_python_coordinate_and_value_formatting() {
    let value = LayerStatusValue::Value(StatusValue::List(vec![
        StatusValue::Integer(12),
        StatusValue::Float(std::f64::consts::PI),
    ]));

    assert_eq!(
        generate_layer_status_strings(Some(&[2.2, 3.5, -1.5]), Some(&value)),
        (" [2 4 -2]".to_owned(), "[12, 3.14]".to_owned())
    );
    assert_eq!(
        generate_layer_coords_status(Some(&[2.2, 3.5]), Some(&value)),
        " [2 4]: [12, 3.14]"
    );
    assert_eq!(
        generate_layer_status("image", Some(&[2.2, 3.5]), Some(&value)),
        "image [2 4]: [12, 3.14]"
    );
    assert_eq!(
        generate_layer_status("image", None, Some(&value)),
        "image: [12, 3.14]"
    );
}

#[test]
fn multiscale_layer_status_values_match_python_tuple_handling() {
    let multiscale = LayerStatusValue::Multiscale {
        level: StatusValue::Integer(2),
        value: Some(StatusValue::Float(42.25)),
    };
    assert_eq!(
        generate_layer_status("labels", Some(&[0.0, 1.0]), Some(&multiscale)),
        "labels [0 1]: 2, 42.2"
    );

    let level_only = LayerStatusValue::Multiscale {
        level: StatusValue::Integer(2),
        value: None,
    };
    assert_eq!(
        generate_layer_status_strings(None, Some(&level_only)),
        (String::new(), "2".to_owned())
    );

    let empty_tuple = LayerStatusValue::Multiscale {
        level: StatusValue::None,
        value: Some(StatusValue::None),
    };
    assert_eq!(
        generate_layer_status("labels", None, Some(&empty_tuple)),
        "labels"
    );
}
