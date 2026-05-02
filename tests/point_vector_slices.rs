use napari_rs::layers::points::constants::PointsProjectionMode;
use napari_rs::layers::points::slice::{PointSliceRequest, SliceScale};
use napari_rs::layers::utils::slice_input::{SliceInput, ThickNdSlice};
use napari_rs::layers::vectors::constants::VectorsProjectionMode;
use napari_rs::layers::vectors::slice::{VectorData, VectorSliceRequest};

fn slice_input(ndim: usize, ndisplay: usize) -> SliceInput {
    SliceInput {
        ndisplay,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(ndim)).unwrap(),
        order: (0..ndim).collect(),
    }
}

fn data_slice(point: &[f64], left: &[f64], right: &[f64]) -> ThickNdSlice<f64> {
    ThickNdSlice {
        point: point.to_vec(),
        margin_left: left.to_vec(),
        margin_right: right.to_vec(),
    }
}

#[test]
fn point_slice_returns_empty_response_for_empty_data() {
    let request = PointSliceRequest::new(
        slice_input(3, 2),
        Vec::new(),
        data_slice(&[0.0, 0.0, 0.0], &[0.0; 3], &[0.0; 3]),
        PointsProjectionMode::None,
        Vec::new(),
        false,
    );

    let response = request.call();

    assert!(response.indices.is_empty());
    assert_eq!(response.scale, SliceScale::Values(Vec::new()));
    assert_eq!(response.request_id, request.id);
}

#[test]
fn point_slice_uses_all_indices_when_every_dimension_is_displayed() {
    let request = PointSliceRequest::new(
        slice_input(2, 2),
        vec![vec![0.0, 0.0], vec![1.0, 1.0], vec![2.0, 2.0]],
        data_slice(&[0.0, 0.0], &[0.0; 2], &[0.0; 2]),
        PointsProjectionMode::None,
        vec![1.0; 3],
        false,
    );

    let response = request.call();

    assert_eq!(response.indices, vec![0, 1, 2]);
    assert_eq!(response.scale, SliceScale::Scalar(1.0));
}

#[test]
fn point_slice_selects_points_inside_exact_slice_with_half_pixel_thickness() {
    let request = PointSliceRequest::new(
        slice_input(3, 2),
        vec![
            vec![0.0, 0.0, 0.0],
            vec![0.49, 1.0, 1.0],
            vec![0.51, 2.0, 2.0],
            vec![-0.5, 3.0, 3.0],
        ],
        data_slice(&[0.0, 0.0, 0.0], &[0.0; 3], &[0.0; 3]),
        PointsProjectionMode::None,
        vec![1.0; 4],
        false,
    );

    let response = request.call();

    assert_eq!(response.indices, vec![0, 1, 3]);
    assert_eq!(response.scale, SliceScale::Scalar(1.0));
}

#[test]
fn point_slice_uses_margins_for_all_projection_mode() {
    let request = PointSliceRequest::new(
        slice_input(3, 2),
        vec![
            vec![8.9, 0.0, 0.0],
            vec![9.0, 1.0, 1.0],
            vec![10.0, 2.0, 2.0],
            vec![12.0, 3.0, 3.0],
            vec![12.1, 4.0, 4.0],
        ],
        data_slice(&[10.0, 0.0, 0.0], &[1.0, 0.0, 0.0], &[2.0, 0.0, 0.0]),
        PointsProjectionMode::All,
        vec![1.0; 5],
        false,
    );

    let response = request.call();

    assert_eq!(response.indices, vec![1, 2, 3]);
}

#[test]
fn point_slice_out_of_slice_display_adds_spilling_points_with_scale() {
    let request = PointSliceRequest::new(
        slice_input(3, 2),
        vec![
            vec![10.0, 0.0, 0.0],
            vec![8.8, 1.0, 1.0],
            vec![12.3, 2.0, 2.0],
            vec![13.0, 3.0, 3.0],
        ],
        data_slice(&[10.0, 0.0, 0.0], &[1.0, 0.0, 0.0], &[2.0, 0.0, 0.0]),
        PointsProjectionMode::All,
        vec![2.0; 4],
        true,
    );

    let response = request.call();

    assert_eq!(response.indices, vec![0, 1, 2, 3]);
    match response.scale {
        SliceScale::Values(values) => {
            assert_eq!(values.len(), 4);
            assert!((values[0] - 1.0).abs() < 1e-10);
            assert!((values[1] - 0.8).abs() < 1e-10);
            assert!((values[2] - 0.7).abs() < 1e-10);
            assert!((values[3] - 0.0).abs() < 1e-10);
        }
        SliceScale::Scalar(_) => panic!("expected per-point scales"),
    }
}

#[test]
fn vector_slice_returns_empty_and_all_displayed_responses_like_python_request() {
    let empty = VectorSliceRequest::new(
        slice_input(3, 2),
        Vec::new(),
        data_slice(&[0.0, 0.0, 0.0], &[0.0; 3], &[0.0; 3]),
        VectorsProjectionMode::None,
        1.0,
        false,
    );
    assert_eq!(empty.call().alphas, SliceScale::Values(Vec::new()));

    let all = VectorSliceRequest::new(
        slice_input(2, 2),
        vec![
            VectorData {
                start: vec![0.0, 0.0],
                direction: vec![1.0, 0.0],
            },
            VectorData {
                start: vec![1.0, 1.0],
                direction: vec![0.0, 1.0],
            },
        ],
        data_slice(&[0.0, 0.0], &[0.0; 2], &[0.0; 2]),
        VectorsProjectionMode::None,
        1.0,
        false,
    );

    let response = all.call();
    assert_eq!(response.indices, vec![0, 1]);
    assert_eq!(response.alphas, SliceScale::Scalar(1.0));
}

#[test]
fn vector_slice_selects_by_start_position_and_exact_slice_bounds() {
    let request = VectorSliceRequest::new(
        slice_input(3, 2),
        vec![
            VectorData {
                start: vec![0.0, 0.0, 0.0],
                direction: vec![1.0, 0.0, 0.0],
            },
            VectorData {
                start: vec![0.5, 1.0, 1.0],
                direction: vec![1.0, 0.0, 0.0],
            },
            VectorData {
                start: vec![0.6, 2.0, 2.0],
                direction: vec![1.0, 0.0, 0.0],
            },
        ],
        data_slice(&[0.0, 0.0, 0.0], &[0.0; 3], &[0.0; 3]),
        VectorsProjectionMode::None,
        1.0,
        false,
    );

    assert_eq!(request.call().indices, vec![0, 1]);
}

#[test]
fn vector_slice_out_of_slice_display_adds_spilling_vectors_with_alpha() {
    let request = VectorSliceRequest::new(
        slice_input(3, 2),
        vec![
            VectorData {
                start: vec![10.0, 0.0, 0.0],
                direction: vec![1.0, 0.0, 0.0],
            },
            VectorData {
                start: vec![8.5, 1.0, 1.0],
                direction: vec![1.0, 0.0, 0.0],
            },
            VectorData {
                start: vec![12.5, 2.0, 2.0],
                direction: vec![2.0, 0.0, 0.0],
            },
            VectorData {
                start: vec![13.2, 3.0, 3.0],
                direction: vec![0.5, 0.0, 0.0],
            },
        ],
        data_slice(&[10.0, 0.0, 0.0], &[1.0, 0.0, 0.0], &[2.0, 0.0, 0.0]),
        VectorsProjectionMode::All,
        1.0,
        true,
    );

    let response = request.call();

    assert_eq!(response.indices, vec![0, 1, 2]);
    match response.alphas {
        SliceScale::Values(values) => {
            assert_eq!(values.len(), 3);
            assert!((values[0] - 1.0).abs() < 1e-10);
            assert!((values[1] - 0.5).abs() < 1e-10);
            assert!((values[2] - 0.75).abs() < 1e-10);
        }
        SliceScale::Scalar(_) => panic!("expected per-vector alphas"),
    }
}
