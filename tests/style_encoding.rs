use napari_rs::layers::utils::style_encoding::{
    ConstantStyleEncoding, DerivedStyleEncoding, ManualStyleEncoding, StyleEncodingError,
    StyleValueSelection, StyleValueSource, get_style_values,
};

#[test]
fn constant_style_encoding_stateful_operations_are_noops_like_python_base() {
    let mut encoding = ConstantStyleEncoding { constant: 0 };

    encoding.apply(3);
    assert_eq!(encoding.values(), 0);
    encoding.append(&[4, 5]);
    assert_eq!(encoding.values(), 0);
    encoding.delete(&[0, 2]);
    assert_eq!(encoding.values(), 0);
    encoding.clear();
    assert_eq!(encoding.values(), 0);
}

#[test]
fn manual_style_encoding_apply_truncates_or_pads_with_default() {
    let mut shorter = ManualStyleEncoding {
        array: vec![1, 2, 3, 4],
        default: -1,
    };
    shorter.apply(3);
    assert_eq!(shorter.array, vec![1, 2, 3]);

    let mut equal = ManualStyleEncoding {
        array: vec![1, 2, 3],
        default: -1,
    };
    equal.apply(3);
    assert_eq!(equal.array, vec![1, 2, 3]);

    let mut longer = ManualStyleEncoding {
        array: vec![1, 2],
        default: -1,
    };
    longer.apply(3);
    assert_eq!(longer.array, vec![1, 2, -1]);
}

#[test]
fn manual_style_encoding_append_delete_and_clear_match_python_base_behavior() {
    let mut encoding = ManualStyleEncoding {
        array: vec![1, 2, 3],
        default: -1,
    };

    encoding.append(&[4, 5]);
    assert_eq!(encoding.array, vec![1, 2, 3, 4, 5]);
    encoding.delete(&[0, 2]);
    assert_eq!(encoding.array, vec![2, 4, 5]);
    encoding.clear();
    assert_eq!(encoding.array, vec![2, 4, 5]);
}

#[test]
fn manual_style_encoding_supports_vector_style_rows() {
    let mut encoding = ManualStyleEncoding {
        array: vec![vec![1, 1], vec![2, 2]],
        default: vec![-1, -1],
    };

    encoding.apply(3);
    assert_eq!(encoding.array, vec![vec![1, 1], vec![2, 2], vec![-1, -1]]);
    encoding.append(&[vec![4, 4], vec![5, 5]]);
    assert_eq!(
        encoding.array,
        vec![vec![1, 1], vec![2, 2], vec![-1, -1], vec![4, 4], vec![5, 5]]
    );
    encoding.delete(&[0, 2]);
    assert_eq!(encoding.array, vec![vec![2, 2], vec![4, 4], vec![5, 5]]);
}

#[test]
fn derived_style_encoding_apply_derives_only_uncached_tail() {
    let mut encoding = DerivedStyleEncoding::new(-1);

    encoding.apply_with(3, |start, end| {
        assert_eq!((start, end), (0, 3));
        Ok::<_, ()>((start..end).map(|index| index as i32 + 1).collect())
    });

    assert_eq!(encoding.values(), &[1, 2, 3]);

    encoding.apply_with(5, |start, end| {
        assert_eq!((start, end), (3, 5));
        Ok::<_, ()>((start..end).map(|index| index as i32 + 1).collect())
    });

    assert_eq!(encoding.values(), &[1, 2, 3, 4, 5]);
}

#[test]
fn derived_style_encoding_apply_truncates_when_row_count_shrinks() {
    let mut encoding = DerivedStyleEncoding {
        cached: vec![1, 2, 3],
        fallback: -1,
    };

    encoding.apply_with(2, |_start, _end| -> Result<Vec<i32>, ()> {
        panic!("derivation should not run when truncating")
    });

    assert_eq!(encoding.values(), &[1, 2]);
}

#[test]
fn derived_style_encoding_apply_uses_fallback_when_derivation_fails() {
    let mut encoding = DerivedStyleEncoding::new(-1);

    encoding.apply_with(3, |_start, _end| Err::<Vec<i32>, _>("missing feature"));

    assert_eq!(encoding.values(), &[-1, -1, -1]);
}

#[test]
fn derived_style_encoding_append_delete_and_clear_update_cached_values() {
    let mut encoding = DerivedStyleEncoding {
        cached: vec![vec![1, 1], vec![2, 2], vec![3, 3]],
        fallback: vec![-1, -1],
    };

    encoding.append(&[vec![4, 4], vec![5, 5]]);
    assert_eq!(
        encoding.values(),
        &[vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]
    );

    encoding.delete(&[0, 2]);
    assert_eq!(encoding.values(), &[vec![2, 2], vec![4, 4], vec![5, 5]]);

    encoding.clear();
    assert!(encoding.values().is_empty());
}

#[test]
fn get_style_values_returns_single_values_without_indexing() {
    let value = vec![1, 2];

    let selection = get_style_values(StyleValueSource::Single(&value), &[0, 2]).unwrap();

    assert_eq!(selection, StyleValueSelection::Single(vec![1, 2]));
}

#[test]
fn get_style_values_indexes_multiple_cached_values() {
    let values = vec![vec![1, 1], vec![2, 2], vec![3, 3]];

    let selection = get_style_values(StyleValueSource::Multiple(&values), &[2, 0]).unwrap();

    assert_eq!(
        selection,
        StyleValueSelection::Multiple(vec![vec![3, 3], vec![1, 1]])
    );
}

#[test]
fn get_style_values_reports_out_of_bounds_indices() {
    let values = vec![1, 2];

    let error = get_style_values(StyleValueSource::Multiple(&values), &[1, 3]).unwrap_err();

    assert_eq!(
        error,
        StyleEncodingError::IndexOutOfBounds { index: 3, len: 2 }
    );
}
