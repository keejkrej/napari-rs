use napari_rs::layers::image::image_utils::{
    GuessMultiscaleError, ImageData, guess_labels, guess_multiscale, guess_rgb,
};
use napari_rs::layers::multiscale_data::{ArrayMetadata, MultiScaleData};
use napari_rs::utils::dtype::DType;

fn array(shape: &[usize]) -> ArrayMetadata {
    ArrayMetadata::new(shape.to_vec(), DType::Float32)
}

#[test]
fn guess_rgb_rejects_non_rgb_shapes_like_python_helper() {
    assert!(!guess_rgb(&[10, 15], 30));
    assert!(!guess_rgb(&[40, 45, 6], 30));
    assert!(!guess_rgb(&[29, 29, 3], 30));
    assert!(!guess_rgb(&[29, 31, 3], 30));
}

#[test]
fn guess_rgb_accepts_large_trailing_rgb_or_rgba_shapes() {
    assert!(guess_rgb(&[31, 31, 3], 30));
    assert!(guess_rgb(&[512, 512, 3], 30));
    assert!(guess_rgb(&[100, 100, 4], 30));
    assert!(guess_rgb(&[10, 10, 3], 5));
}

#[test]
fn guess_multiscale_returns_existing_multiscale_data_unchanged() {
    let multiscale = MultiScaleData::new(vec![array(&[10, 15]), array(&[5, 7])]).unwrap();

    let result = guess_multiscale(ImageData::MultiScale(multiscale.clone())).unwrap();

    assert_eq!(result, (true, ImageData::MultiScale(multiscale)));
}

#[test]
fn guess_multiscale_treats_single_array_as_not_multiscale() {
    let image = array(&[10, 15]);

    let result = guess_multiscale(ImageData::Array(image.clone())).unwrap();

    assert_eq!(result, (false, ImageData::Array(image)));
}

#[test]
fn guess_multiscale_unwraps_single_item_sequences() {
    let image = array(&[10, 15, 6]);

    let result = guess_multiscale(ImageData::Sequence(vec![image.clone()])).unwrap();

    assert_eq!(result, (false, ImageData::Array(image)));
}

#[test]
fn guess_multiscale_accepts_strictly_decreasing_sizes() {
    let levels = vec![array(&[10, 15, 6]), array(&[5, 7, 3])];

    let result = guess_multiscale(ImageData::Sequence(levels.clone())).unwrap();

    assert!(result.0);
    match result.1 {
        ImageData::MultiScale(multiscale) => assert_eq!(multiscale.levels(), levels.as_slice()),
        _ => panic!("expected multiscale data"),
    }
}

#[test]
fn guess_multiscale_accepts_equal_dimensions_when_total_size_decreases() {
    let levels = vec![array(&[10, 15, 6]), array(&[10, 7, 3])];

    let result = guess_multiscale(ImageData::Sequence(levels.clone())).unwrap();

    assert!(result.0);
    match result.1 {
        ImageData::MultiScale(multiscale) => assert_eq!(multiscale.levels(), levels.as_slice()),
        _ => panic!("expected multiscale data"),
    }
}

#[test]
fn guess_multiscale_rejects_equal_sizes_like_python_helper() {
    let levels = vec![array(&[10, 15, 6]), array(&[15, 10, 6])];

    let result = guess_multiscale(ImageData::Sequence(levels));

    assert_eq!(result, Err(GuessMultiscaleError::EqualSingleSize(900)));
}

#[test]
fn guess_multiscale_rejects_incorrect_size_order_like_python_helper() {
    let levels = vec![array(&[5, 7, 3]), array(&[10, 15, 6])];

    let result = guess_multiscale(ImageData::Sequence(levels));

    assert_eq!(
        result,
        Err(GuessMultiscaleError::IncorrectOrder(vec![105, 900]))
    );
}

#[test]
fn guess_labels_only_promotes_large_integer_dtypes() {
    assert_eq!(guess_labels(DType::Int32), "labels");
    assert_eq!(guess_labels(DType::UInt32), "labels");
    assert_eq!(guess_labels(DType::Int64), "labels");
    assert_eq!(guess_labels(DType::UInt64), "labels");
    assert_eq!(guess_labels(DType::UInt16), "image");
    assert_eq!(guess_labels(DType::Float32), "image");
}
