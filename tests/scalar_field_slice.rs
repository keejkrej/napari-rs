use napari_rs::layers::scalar_field::slice::{
    ScalarFieldProjectionMode, SliceElement, data_slice_to_slices, displayed_slice_order,
    point_to_slices, slice_out_of_bounds,
};
use napari_rs::layers::utils::slice_input::{SliceInput, ThickNdSlice};

fn thick_slice(point: &[f64], left: &[f64], right: &[f64]) -> ThickNdSlice<f64> {
    ThickNdSlice {
        point: point.to_vec(),
        margin_left: left.to_vec(),
        margin_right: right.to_vec(),
    }
}

fn slice_input(ndim: usize, ndisplay: usize, order: Vec<usize>) -> SliceInput {
    SliceInput {
        ndisplay,
        world_slice: ThickNdSlice::make_full(None, None, None, Some(ndim)).unwrap(),
        order,
    }
}

#[test]
fn point_to_slices_matches_python_numpy_rounding_and_nan_all_slice() {
    assert_eq!(
        point_to_slices(&[f64::NAN, 10.1, 2.6, 4.0]),
        vec![
            SliceElement::All,
            SliceElement::Index(10),
            SliceElement::Index(3),
            SliceElement::Index(4),
        ]
    );
    assert_eq!(
        point_to_slices(&[0.5, 1.5, 2.5, -0.5]),
        vec![
            SliceElement::Index(0),
            SliceElement::Index(2),
            SliceElement::Index(2),
            SliceElement::Index(0),
        ]
    );
}

#[test]
fn data_slice_to_slices_matches_python_image_utils_case() {
    let data_slice = thick_slice(
        &[f64::NAN, 10.1, 2.6, 4.0, -1.0],
        &[f64::NAN, 0.0, 1.6, 0.3, 1.0],
        &[f64::NAN, 0.1, 0.3, 0.5, 0.6],
    );

    assert_eq!(
        data_slice_to_slices(&data_slice, &[0]),
        vec![
            SliceElement::All,
            SliceElement::Range {
                start: 10,
                stop: 11
            },
            SliceElement::Range { start: 1, stop: 3 },
            SliceElement::Range { start: 4, stop: 5 },
            SliceElement::Range { start: 0, stop: 1 },
        ]
    );
}

#[test]
fn data_slice_to_slices_rounds_up_exact_integer_high_and_keeps_one_sample() {
    let data_slice = thick_slice(&[4.0, 8.4], &[0.0, 0.4], &[0.0, 0.6]);

    assert_eq!(
        data_slice_to_slices(&data_slice, &[]),
        vec![
            SliceElement::Range { start: 4, stop: 5 },
            SliceElement::Range { start: 8, stop: 10 },
        ]
    );
}

#[test]
fn displayed_slice_order_matches_reduction_and_rgb_final_axis_behavior() {
    assert_eq!(displayed_slice_order(&[3, 7], false), vec![0, 1]);
    assert_eq!(displayed_slice_order(&[5, 2], false), vec![1, 0]);
    assert_eq!(displayed_slice_order(&[4, 0, 1], false), vec![2, 0, 1]);
    assert_eq!(displayed_slice_order(&[4, 0, 1], true), vec![2, 0, 1, 3]);
}

#[test]
fn slice_out_of_bounds_matches_point_projection_behavior() {
    let slice_input = slice_input(4, 2, vec![0, 1, 2, 3]);
    let data_slice = thick_slice(&[0.0, 9.0, f64::NAN, f64::NAN], &[0.0; 4], &[0.0; 4]);
    assert!(!slice_out_of_bounds(
        &[10, 10, 20, 20],
        &slice_input,
        &data_slice,
        ScalarFieldProjectionMode::None,
    ));

    let data_slice = thick_slice(&[0.0, 10.0, f64::NAN, f64::NAN], &[0.0; 4], &[0.0; 4]);
    assert!(slice_out_of_bounds(
        &[10, 10, 20, 20],
        &slice_input,
        &data_slice,
        ScalarFieldProjectionMode::None,
    ));

    let data_slice = thick_slice(&[-0.5, 0.0, f64::NAN, f64::NAN], &[0.0; 4], &[0.0; 4]);
    assert!(!slice_out_of_bounds(
        &[10, 10, 20, 20],
        &slice_input,
        &data_slice,
        ScalarFieldProjectionMode::None,
    ));
}

#[test]
fn slice_out_of_bounds_matches_thick_projection_overlap_behavior() {
    let slice_input = slice_input(3, 2, vec![0, 1, 2]);

    let overlaps_low_edge = thick_slice(&[-1.0, f64::NAN, f64::NAN], &[0.0; 3], &[1.0, 0.0, 0.0]);
    assert!(!slice_out_of_bounds(
        &[10, 10, 10],
        &slice_input,
        &overlaps_low_edge,
        ScalarFieldProjectionMode::Other,
    ));

    let below = thick_slice(&[-2.0, f64::NAN, f64::NAN], &[0.0; 3], &[1.0, 0.0, 0.0]);
    assert!(slice_out_of_bounds(
        &[10, 10, 10],
        &slice_input,
        &below,
        ScalarFieldProjectionMode::Other,
    ));

    let overlaps_high_edge = thick_slice(&[10.0, f64::NAN, f64::NAN], &[1.0, 0.0, 0.0], &[0.0; 3]);
    assert!(!slice_out_of_bounds(
        &[10, 10, 10],
        &slice_input,
        &overlaps_high_edge,
        ScalarFieldProjectionMode::Other,
    ));

    let above = thick_slice(&[11.0, f64::NAN, f64::NAN], &[1.0, 0.0, 0.0], &[0.0; 3]);
    assert!(slice_out_of_bounds(
        &[10, 10, 10],
        &slice_input,
        &above,
        ScalarFieldProjectionMode::Other,
    ));
}
