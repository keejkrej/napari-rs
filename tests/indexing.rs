use std::collections::BTreeMap;

use napari_rs::utils::indexing::{elements_in_slice, index_in_slice};

#[test]
fn elements_in_slice_returns_visible_mask_for_single_axis() {
    let index = vec![
        vec![0, 1, 2, 3, 4],
        vec![1, 1, 1, 1, 1],
        vec![4, 5, 6, 7, 8],
    ];
    let position_in_axes = BTreeMap::from([(0, 3)]);

    assert_eq!(
        elements_in_slice(&index, &position_in_axes),
        vec![false, false, false, true, false]
    );
}

#[test]
fn elements_in_slice_reduces_multiple_axis_queries() {
    let index = vec![
        vec![0, 1, 2, 3, 4],
        vec![1, 1, 1, 1, 1],
        vec![4, 5, 6, 7, 8],
    ];
    let position_in_axes = BTreeMap::from([(1, 1), (2, 8)]);

    assert_eq!(
        elements_in_slice(&index, &position_in_axes),
        vec![false, false, false, false, true]
    );
}

#[test]
fn index_in_slice_matches_python_doc_examples() {
    let index = vec![
        vec![0, 1, 2, 3, 4],
        vec![1, 1, 1, 1, 1],
        vec![4, 5, 6, 7, 8],
    ];

    assert_eq!(
        index_in_slice(&index, &BTreeMap::from([(0, 3)]), &[0, 1, 2]),
        vec![vec![1], vec![7]]
    );
    assert_eq!(
        index_in_slice(&index, &BTreeMap::from([(1, 1), (2, 8)]), &[0, 1, 2]),
        vec![vec![4]]
    );
}

#[test]
fn index_in_slice_respects_custom_indices_order() {
    let index = vec![vec![0, 1, 2], vec![5, 5, 6], vec![9, 8, 7]];
    let position_in_axes = BTreeMap::from([(1, 5)]);

    assert_eq!(
        index_in_slice(&index, &position_in_axes, &[2, 0, 1]),
        vec![vec![9, 8], vec![0, 1]]
    );
}
