use napari_rs::layers::vectors::vector_utils::{
    RawVectorData, VectorDataError, VectorRecord, convert_image_to_coordinates, fix_data_vectors,
};

#[test]
fn convert_image_to_coordinates_matches_python_meshgrid_indexing() {
    let projections = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
        vec![2.0, 0.0],
        vec![2.0, 1.0],
    ];

    let converted = convert_image_to_coordinates(&[3, 2], &projections).unwrap();

    assert_eq!(
        converted,
        vec![
            VectorRecord::new([0.0, 0.0], [0.0, 0.0]),
            VectorRecord::new([0.0, 1.0], [0.0, 1.0]),
            VectorRecord::new([1.0, 0.0], [1.0, 0.0]),
            VectorRecord::new([1.0, 1.0], [1.0, 1.0]),
            VectorRecord::new([2.0, 0.0], [2.0, 0.0]),
            VectorRecord::new([2.0, 1.0], [2.0, 1.0]),
        ]
    );
}

#[test]
fn fix_data_vectors_defaults_empty_data_to_2d_like_python() {
    let (vectors, ndim) = fix_data_vectors(None, None).unwrap();

    assert!(vectors.is_empty());
    assert_eq!(ndim, 2);
}

#[test]
fn fix_data_vectors_uses_explicit_ndim_for_empty_data() {
    let (vectors, ndim) =
        fix_data_vectors(Some(RawVectorData::Empty { ndim: None }), Some(3)).unwrap();

    assert!(vectors.is_empty());
    assert_eq!(ndim, 3);

    let (vectors, ndim) =
        fix_data_vectors(Some(RawVectorData::Empty { ndim: Some(4) }), None).unwrap();
    assert!(vectors.is_empty());
    assert_eq!(ndim, 4);
}

#[test]
fn fix_data_vectors_keeps_coordinate_like_data() {
    let raw = vec![
        VectorRecord::new([1.0, 2.0], [3.0, 4.0]),
        VectorRecord::new([5.0, 6.0], [7.0, 8.0]),
    ];

    let (vectors, ndim) =
        fix_data_vectors(Some(RawVectorData::Coordinate(raw.clone())), None).unwrap();

    assert_eq!(vectors, raw);
    assert_eq!(ndim, 2);
}

#[test]
fn fix_data_vectors_reshapes_single_vector_like_python_newaxis_case() {
    let vector = VectorRecord::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);

    let (vectors, ndim) =
        fix_data_vectors(Some(RawVectorData::Single(vector.clone())), None).unwrap();

    assert_eq!(vectors, vec![vector]);
    assert_eq!(ndim, 3);
}

#[test]
fn fix_data_vectors_converts_image_like_data() {
    let data = RawVectorData::Image {
        spatial_shape: vec![2, 2],
        projections: vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ],
    };

    let (vectors, ndim) = fix_data_vectors(Some(data), None).unwrap();

    assert_eq!(ndim, 2);
    assert_eq!(
        vectors,
        vec![
            VectorRecord::new([0.0, 0.0], [0.0, 0.0]),
            VectorRecord::new([0.0, 1.0], [0.0, 1.0]),
            VectorRecord::new([1.0, 0.0], [1.0, 0.0]),
            VectorRecord::new([1.0, 1.0], [1.0, 1.0]),
        ]
    );
}

#[test]
fn fix_data_vectors_rejects_incompatible_ndim_like_python_helper() {
    let data = RawVectorData::Empty { ndim: Some(2) };

    assert_eq!(
        fix_data_vectors(Some(data), Some(3)),
        Err(VectorDataError::DimensionMismatch {
            data_ndim: 2,
            ndim: 3,
        })
    );
}

#[test]
fn vector_utils_reject_invalid_vector_shapes() {
    assert_eq!(
        fix_data_vectors(
            Some(RawVectorData::Single(VectorRecord::new(
                [1.0, 2.0],
                [3.0, 4.0, 5.0],
            ))),
            None,
        ),
        Err(VectorDataError::InvalidVectorDimension {
            start: 2,
            projection: 3,
        })
    );

    assert_eq!(
        convert_image_to_coordinates(&[2, 2], &[vec![1.0, 2.0]]),
        Err(VectorDataError::InvalidImageProjectionCount {
            expected: 4,
            found: 1,
        })
    );
}
