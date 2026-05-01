use napari_rs::layers::utils::string_encoding::{
    StringEncodingSpec, is_format_string, parse_format_fields,
    validate_string_encoding_from_sequence, validate_string_encoding_from_str,
};

#[test]
fn is_format_string_matches_python_formatter_field_detection() {
    assert!(is_format_string("{class}: {score:.2f}"));
    assert!(is_format_string("{index}: {confidence:.2f}"));
    assert!(!is_format_string("class"));
    assert!(!is_format_string("{{class}}"));
    assert!(!is_format_string("{class}: {confidence:.2f"));
    assert!(!is_format_string("class}"));
}

#[test]
fn parse_format_fields_extracts_field_names_before_format_specs() {
    assert_eq!(
        parse_format_fields("{class}: {score:.2f}"),
        Some(vec!["class".to_owned(), "score".to_owned()])
    );
    assert_eq!(
        parse_format_fields("{label:d}: {confidence:.2f}"),
        Some(vec!["label".to_owned(), "confidence".to_owned()])
    );
    assert_eq!(parse_format_fields("{{escaped}}"), Some(Vec::new()));
    assert_eq!(parse_format_fields("{broken"), None);
}

#[test]
fn validate_string_encoding_from_str_matches_python_direct_vs_format_choice() {
    assert_eq!(
        validate_string_encoding_from_str("{class}: {score:.2f}"),
        StringEncodingSpec::Format {
            format: "{class}: {score:.2f}".to_owned(),
        }
    );
    assert_eq!(
        validate_string_encoding_from_str("class"),
        StringEncodingSpec::Direct {
            feature: "class".to_owned(),
        }
    );
}

#[test]
fn validate_string_encoding_from_sequence_matches_python_manual_choice() {
    let values = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];

    assert_eq!(
        validate_string_encoding_from_sequence(&values),
        StringEncodingSpec::Manual {
            array: values,
            default: String::new(),
        }
    );
}
