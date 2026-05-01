use std::path::PathBuf;

use napari_rs::utils::misc::{
    MiscError, PathOrUrl, SequenceOfIterables, abspath_or_url, camel_to_snake, camel_to_spaces,
    ensure_iterable, ensure_n_tuple, ensure_sequence_of_iterables, is_scalar_iterable, is_sequence,
    is_slice_iterable, reorder_after_dim_reduction, str_to_rgb,
};

#[test]
fn str_to_rgb_matches_python_behavior() {
    assert_eq!(str_to_rgb("rgb(1,2,3)"), Ok([1, 2, 3]));
    assert_eq!(str_to_rgb("rgb(10, 20, 30)"), Ok([10, 20, 30]));
    assert!(matches!(
        str_to_rgb("rgba(1,2,3)"),
        Err(MiscError::InvalidRgb(_))
    ));
    assert!(matches!(
        str_to_rgb("rgb(1,-2,3)"),
        Err(MiscError::InvalidRgb(_))
    ));
}

#[test]
fn ensure_iterable_returns_existing_sequence_or_repeats_scalar() {
    assert_eq!(ensure_iterable(Some(&[0, 1, 2]), 3, 9), vec![0, 1, 2]);
    assert_eq!(ensure_iterable(None, 3, 1), vec![1, 1, 1]);
}

#[test]
fn iterable_and_sequence_helpers_express_rust_type_level_equivalents() {
    assert!(is_slice_iterable(&[1]));
    assert!(is_sequence(&[1, 2]));
    assert!(!is_scalar_iterable(&1));
}

#[test]
fn ensure_sequence_of_iterables_returns_nested_sequence_or_repeated_empty() {
    let nested = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
    assert_eq!(
        ensure_sequence_of_iterables(&nested, Some(3), true),
        Ok(SequenceOfIterables::Sequence(nested))
    );
    assert_eq!(
        ensure_sequence_of_iterables::<i32>(&[], None, true),
        Ok(SequenceOfIterables::Repeated(Vec::new()))
    );
    assert!(matches!(
        ensure_sequence_of_iterables(&[vec![0, 1]], Some(4), false),
        Err(MiscError::Length {
            expected: 4,
            actual: 1
        })
    ));
}

#[test]
fn camel_case_helpers_match_napari_string_utilities() {
    assert_eq!(camel_to_snake("LayerList"), "layer_list");
    assert_eq!(camel_to_snake("HTTPRequest"), "httprequest");
    assert_eq!(camel_to_spaces("LayerList"), "Layer List");
    assert_eq!(camel_to_spaces("QtViewer"), "Qt Viewer");
}

#[test]
fn abspath_or_url_keeps_urls_and_absolutizes_paths() {
    assert_eq!(
        abspath_or_url("https://something", false),
        Ok(PathOrUrl::Url("https://something".to_owned()))
    );
    assert_eq!(
        abspath_or_url("s3://something", false),
        Ok(PathOrUrl::Url("s3://something".to_owned()))
    );

    let expected = std::env::current_dir().unwrap().join("something");
    assert_eq!(
        abspath_or_url("something", false),
        Ok(PathOrUrl::Path(expected))
    );
}

#[test]
fn abspath_or_url_expands_home_and_checks_existence_when_requested() {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
    assert_eq!(
        abspath_or_url("~/something", false),
        Ok(PathOrUrl::Path(home.join("something")))
    );

    assert!(matches!(
        abspath_or_url("definitely_missing_napari_rs_path", true),
        Err(MiscError::MissingPath(_))
    ));
}

#[test]
fn ensure_n_tuple_matches_python_before_and_after_fill_behavior() {
    assert_eq!(ensure_n_tuple([1, 2], 3, 0, true), Ok(vec![0, 1, 2]));
    assert_eq!(ensure_n_tuple([1, 2], 3, 0, false), Ok(vec![1, 2, 0]));
    assert_eq!(ensure_n_tuple([1, 2, 3, 4], 3, 0, true), Ok(vec![2, 3, 4]));
    assert_eq!(ensure_n_tuple([1, 2, 3, 4], 3, 0, false), Ok(vec![1, 2, 3]));
    assert_eq!(
        ensure_n_tuple([1], 0, 0, true),
        Err(MiscError::EmptyTupleLength)
    );
}

#[test]
fn reorder_after_dim_reduction_matches_python_rank_ordering() {
    assert_eq!(reorder_after_dim_reduction(&[2, 0]), vec![1, 0]);
    assert_eq!(reorder_after_dim_reduction(&[0, 1, 2]), vec![0, 1, 2]);
    assert_eq!(reorder_after_dim_reduction(&[4, 0, 2]), vec![2, 0, 1]);
}
