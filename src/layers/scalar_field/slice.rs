use crate::layers::utils::slice_input::{SliceInput, ThickNdSlice};
use crate::utils::misc::reorder_after_dim_reduction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceElement {
    All,
    Index(isize),
    Range { start: isize, stop: isize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarFieldProjectionMode {
    None,
    Other,
}

pub fn point_to_slices(point: &[f64]) -> Vec<SliceElement> {
    point
        .iter()
        .map(|&point| {
            if point.is_nan() {
                SliceElement::All
            } else {
                SliceElement::Index(round_half_to_even(point) as isize)
            }
        })
        .collect()
}

pub fn data_slice_to_slices(
    data_slice: &ThickNdSlice<f64>,
    dims_displayed: &[usize],
) -> Vec<SliceElement> {
    let mut slices = vec![SliceElement::All; data_slice.ndim()];

    for (dim, (&point, (&margin_left, &margin_right))) in data_slice
        .point
        .iter()
        .zip(data_slice.margin_left.iter().zip(&data_slice.margin_right))
        .enumerate()
    {
        if dims_displayed.contains(&dim) {
            continue;
        }

        let mut low = (round_half_to_even(point - margin_left) as isize).max(0);
        let mut high = (round_half_to_even(point + margin_right) as isize).max(0);
        if is_close(high as f64, point + margin_right) {
            high += 1;
        }
        if low == high {
            high += 1;
        }
        if low > high {
            std::mem::swap(&mut low, &mut high);
        }
        slices[dim] = SliceElement::Range {
            start: low,
            stop: high,
        };
    }

    slices
}

pub fn displayed_slice_order(displayed: &[usize], rgb: bool) -> Vec<usize> {
    let order = reorder_after_dim_reduction(displayed);
    if rgb {
        let mut output = order;
        let final_axis = output.iter().max().map_or(0, |axis| axis + 1);
        output.push(final_axis);
        output
    } else {
        order
    }
}

pub fn slice_out_of_bounds(
    shape: &[usize],
    slice_input: &SliceInput,
    data_slice: &ThickNdSlice<f64>,
    projection_mode: ScalarFieldProjectionMode,
) -> bool {
    for dim in slice_input.not_displayed() {
        let max_idx = shape[dim] as isize - 1;
        let point = data_slice.point[dim];
        if projection_mode == ScalarFieldProjectionMode::None {
            let point = round_half_to_even(point) as isize;
            if point < 0 || point > max_idx {
                return true;
            }
        } else {
            let low = round_half_to_even(point - data_slice.margin_left[dim]) as isize;
            let high = round_half_to_even(point + data_slice.margin_right[dim]) as isize;
            if high < 0 || low > max_idx {
                return true;
            }
        }
    }
    false
}

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-8
}

fn round_half_to_even(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }
    let floor = value.floor();
    let fraction = value - floor;
    if (fraction - 0.5).abs() > 1e-12 {
        value.round()
    } else if (floor as i128).rem_euclid(2) == 0 {
        floor
    } else {
        floor + 1.0
    }
}
