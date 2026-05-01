use napari_rs::layers::multiscale_data::{ArrayMetadata, MultiScaleData, MultiScaleDataError};
use napari_rs::utils::dtype::DType;

#[test]
fn multiscale_data_rejects_empty_sequences() {
    assert_eq!(
        MultiScaleData::new(Vec::new()),
        Err(MultiScaleDataError::Empty)
    );
}

#[test]
fn multiscale_data_reports_first_scale_metadata_like_python_wrapper() {
    let data = MultiScaleData::new(vec![
        ArrayMetadata::new([100, 80], DType::UInt16),
        ArrayMetadata::new([50, 40], DType::UInt16),
        ArrayMetadata::new([25, 20], DType::UInt16),
    ])
    .unwrap();

    assert_eq!(data.size(), 8000);
    assert_eq!(data.ndim(), 2);
    assert_eq!(data.dtype(), DType::UInt16);
    assert_eq!(data.shape(), &[100, 80]);
    assert_eq!(
        data.shapes(),
        vec![vec![100, 80], vec![50, 40], vec![25, 20]]
    );
    assert_eq!(data.len(), 3);
    assert!(!data.is_empty());
}

#[test]
fn multiscale_data_indexing_and_lowest_resolution_match_python_behavior() {
    let levels = vec![
        ArrayMetadata::new([16, 16, 16], DType::Float32),
        ArrayMetadata::new([8, 8, 8], DType::Float32),
    ];
    let data = MultiScaleData::new(levels.clone()).unwrap();

    assert_eq!(data.get(0), Some(&levels[0]));
    assert_eq!(data.get(1), Some(&levels[1]));
    assert_eq!(data.get(2), None);
    assert_eq!(data.to_lowest_resolution_array(), &levels[1]);
    assert_eq!(data.levels(), levels.as_slice());
}
