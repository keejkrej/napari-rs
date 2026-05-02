use crate::layers::base::slice::next_request_id;
use crate::layers::points::constants::PointsProjectionMode;
use crate::layers::utils::slice_input::{SliceInput, ThickNdSlice};

#[derive(Debug, Clone, PartialEq)]
pub enum SliceScale {
    Scalar(f64),
    Values(Vec<f64>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointSliceResponse {
    pub indices: Vec<usize>,
    pub scale: SliceScale,
    pub slice_input: SliceInput,
    pub request_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointSliceRequest {
    pub slice_input: SliceInput,
    pub data: Vec<Vec<f64>>,
    pub data_slice: ThickNdSlice<f64>,
    pub projection_mode: PointsProjectionMode,
    pub size: Vec<f64>,
    pub out_of_slice_display: bool,
    pub id: usize,
}

impl PointSliceRequest {
    pub fn new(
        slice_input: SliceInput,
        data: Vec<Vec<f64>>,
        data_slice: ThickNdSlice<f64>,
        projection_mode: PointsProjectionMode,
        size: Vec<f64>,
        out_of_slice_display: bool,
    ) -> Self {
        Self {
            slice_input,
            data,
            data_slice,
            projection_mode,
            size,
            out_of_slice_display,
            id: next_request_id(),
        }
    }

    pub fn call(&self) -> PointSliceResponse {
        if self.data.is_empty() {
            return PointSliceResponse {
                indices: Vec::new(),
                scale: SliceScale::Values(Vec::new()),
                slice_input: self.slice_input.clone(),
                request_id: self.id,
            };
        }

        let not_displayed = self.slice_input.not_displayed();
        if not_displayed.is_empty() {
            return PointSliceResponse {
                indices: (0..self.data.len()).collect(),
                scale: SliceScale::Scalar(1.0),
                slice_input: self.slice_input.clone(),
                request_id: self.id,
            };
        }

        let (indices, scale) = self.get_slice_data(&not_displayed);
        PointSliceResponse {
            indices,
            scale,
            slice_input: self.slice_input.clone(),
            request_id: self.id,
        }
    }

    fn get_slice_data(&self, not_displayed: &[usize]) -> (Vec<usize>, SliceScale) {
        let (low, high) = slice_bounds(
            &self.data_slice,
            not_displayed,
            self.projection_mode == PointsProjectionMode::None,
        );
        let data: Vec<Vec<f64>> = self
            .data
            .iter()
            .map(|point| not_displayed.iter().map(|&axis| point[axis]).collect())
            .collect();
        let inside_slice = inside_slice_mask(&data, &low, &high);
        let mut slice_indices: Vec<usize> = inside_slice
            .iter()
            .enumerate()
            .filter_map(|(index, &inside)| inside.then_some(index))
            .collect();

        if self.out_of_slice_display && self.slice_input.ndim() > 2 {
            let sizes: Vec<f64> = self.size.iter().map(|size| size / 2.0).collect();
            let mut matches = Vec::new();
            let mut scales = Vec::new();
            for (index, coords) in data.iter().enumerate() {
                let distances = distances_from_slice(coords, &low, &high, inside_slice[index]);
                let size = sizes[index];
                if distances.iter().all(|&distance| distance <= size) {
                    matches.push(index);
                    scales.push(
                        distances
                            .into_iter()
                            .map(|distance| (size - distance) / size)
                            .product(),
                    );
                }
            }

            if matches.is_empty() {
                return (Vec::new(), SliceScale::Scalar(1.0));
            }
            slice_indices = matches;
            return (slice_indices, SliceScale::Values(scales));
        }

        (slice_indices, SliceScale::Scalar(1.0))
    }
}

pub(crate) fn slice_bounds(
    data_slice: &ThickNdSlice<f64>,
    axes: &[usize],
    exact_slice: bool,
) -> (Vec<f64>, Vec<f64>) {
    let selected = data_slice.select_axes(axes);
    let mut low = selected.point.clone();
    let mut high = selected.point.clone();
    if !exact_slice {
        for ((low, high), (&point, (&left, &right))) in low.iter_mut().zip(&mut high).zip(
            selected
                .point
                .iter()
                .zip(selected.margin_left.iter().zip(&selected.margin_right)),
        ) {
            *low = point - left;
            *high = point + right;
        }
    }

    for (low, high) in low.iter_mut().zip(&mut high) {
        if (*high - *low).abs() <= 1e-8 {
            *low -= 0.5;
            *high += 0.5;
        }
    }

    (low, high)
}

pub(crate) fn inside_slice_mask(data: &[Vec<f64>], low: &[f64], high: &[f64]) -> Vec<bool> {
    data.iter()
        .map(|point| {
            point
                .iter()
                .zip(low.iter().zip(high))
                .all(|(&value, (&low, &high))| value >= low && value <= high)
        })
        .collect()
}

pub(crate) fn distances_from_slice(
    point: &[f64],
    low: &[f64],
    high: &[f64],
    inside: bool,
) -> Vec<f64> {
    if inside {
        return vec![0.0; point.len()];
    }
    point
        .iter()
        .zip(low.iter().zip(high))
        .map(|(&value, (&low, &high))| (value - low).abs().min((value - high).abs()))
        .collect()
}
