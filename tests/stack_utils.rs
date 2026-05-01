use napari_rs::layers::multiscale_data::ArrayMetadata;
use napari_rs::layers::utils::stack_utils::{
    CMYBGR, MAGENTA_GREEN, StackUtilsError, channel_blending_values,
    default_channel_colormaps, slice_shape_from_axis, split_channel_shapes,
    split_multiscale_channel_shapes,
};
use napari_rs::utils::dtype::DType;

fn array(shape: &[usize]) -> ArrayMetadata {
    ArrayMetadata::new(shape.to_vec(), DType::Float32)
}

#[test]
fn slice_shape_from_axis_removes_selected_axis_like_python_slice() {
    assert_eq!(
        slice_shape_from_axis(&[10, 2, 128, 128], 1, 0),
        Ok(vec![10, 128, 128])
    );
    assert_eq!(
        slice_shape_from_axis(&[10, 128, 128, 3], -1, 2),
        Ok(vec![10, 128, 128])
    );
}

#[test]
fn slice_shape_from_axis_validates_axis_and_element() {
    assert_eq!(
        slice_shape_from_axis(&[], 0, 0),
        Err(StackUtilsError::EmptyShape)
    );
    assert_eq!(
        slice_shape_from_axis(&[10, 2], 2, 0),
        Err(StackUtilsError::AxisOutOfBounds { axis: 2, ndim: 2 })
    );
    assert_eq!(
        slice_shape_from_axis(&[10, 2], -3, 0),
        Err(StackUtilsError::AxisOutOfBounds { axis: -3, ndim: 2 })
    );
    assert_eq!(
        slice_shape_from_axis(&[10, 2], 1, 2),
        Err(StackUtilsError::ElementOutOfBounds {
            element: 2,
            axis_len: 2,
        })
    );
}

#[test]
fn split_channel_shapes_returns_one_shape_per_channel() {
    assert_eq!(
        split_channel_shapes(&[10, 2, 128, 128], 1),
        Ok(vec![vec![10, 128, 128], vec![10, 128, 128]])
    );
    assert_eq!(
        split_channel_shapes(&[10, 128, 128, 3], -1),
        Ok(vec![
            vec![10, 128, 128],
            vec![10, 128, 128],
            vec![10, 128, 128],
        ])
    );
}

#[test]
fn split_multiscale_channel_shapes_preserves_levels_for_each_channel() {
    let levels = vec![
        array(&[3, 128, 128]),
        array(&[3, 64, 64]),
        array(&[3, 32, 32]),
    ];

    let channels = split_multiscale_channel_shapes(&levels, 0).unwrap();

    assert_eq!(channels.len(), 3);
    assert_eq!(
        channels[0],
        vec![array(&[128, 128]), array(&[64, 64]), array(&[32, 32])]
    );
    assert_eq!(channels[1], channels[0]);
    assert_eq!(channels[2], channels[0]);
}

#[test]
fn split_multiscale_channel_shapes_validates_level_channel_counts() {
    let levels = vec![array(&[3, 128, 128]), array(&[2, 64, 64])];

    assert_eq!(
        split_multiscale_channel_shapes(&levels, 0),
        Err(StackUtilsError::MultiscaleChannelLengthMismatch {
            level: 1,
            expected: 3,
            actual: 2,
        })
    );
}

#[test]
fn default_channel_colormaps_match_python_split_channels_defaults() {
    assert_eq!(MAGENTA_GREEN, ["magenta", "green"]);
    assert_eq!(
        CMYBGR,
        ["cyan", "magenta", "yellow", "blue", "green", "red"]
    );
    assert_eq!(default_channel_colormaps(0), Vec::<&str>::new());
    assert_eq!(default_channel_colormaps(1), vec!["gray"]);
    assert_eq!(default_channel_colormaps(2), vec!["magenta", "green"]);
    assert_eq!(
        default_channel_colormaps(8),
        vec![
            "cyan", "magenta", "yellow", "blue", "green", "red", "cyan", "magenta",
        ]
    );
}

#[test]
fn channel_blending_values_match_python_split_channels_defaults_and_overrides() {
    assert_eq!(channel_blending_values(0, None), Vec::<String>::new());
    assert_eq!(
        channel_blending_values(1, None),
        vec!["translucent_no_depth"]
    );
    assert_eq!(
        channel_blending_values(4, None),
        vec![
            "translucent_no_depth".to_owned(),
            "additive".to_owned(),
            "additive".to_owned(),
            "additive".to_owned(),
        ]
    );
    assert_eq!(
        channel_blending_values(3, Some("translucent")),
        vec!["translucent", "translucent", "translucent"]
    );
}
