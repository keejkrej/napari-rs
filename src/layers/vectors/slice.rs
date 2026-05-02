use crate::layers::base::slice::next_request_id;
use crate::layers::points::slice::{
    SliceScale, distances_from_slice, inside_slice_mask, slice_bounds,
};
use crate::layers::utils::slice_input::{SliceInput, ThickNdSlice};
use crate::layers::vectors::constants::VectorsProjectionMode;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorData {
    pub start: Vec<f64>,
    pub direction: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorSliceResponse {
    pub indices: Vec<usize>,
    pub alphas: SliceScale,
    pub slice_input: SliceInput,
    pub request_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorSliceRequest {
    pub slice_input: SliceInput,
    pub data: Vec<VectorData>,
    pub data_slice: ThickNdSlice<f64>,
    pub projection_mode: VectorsProjectionMode,
    pub length: f64,
    pub out_of_slice_display: bool,
    pub id: usize,
}

impl VectorSliceRequest {
    pub fn new(
        slice_input: SliceInput,
        data: Vec<VectorData>,
        data_slice: ThickNdSlice<f64>,
        projection_mode: VectorsProjectionMode,
        length: f64,
        out_of_slice_display: bool,
    ) -> Self {
        Self {
            slice_input,
            data,
            data_slice,
            projection_mode,
            length,
            out_of_slice_display,
            id: next_request_id(),
        }
    }

    pub fn call(&self) -> VectorSliceResponse {
        if self.data.is_empty() {
            return VectorSliceResponse {
                indices: Vec::new(),
                alphas: SliceScale::Values(Vec::new()),
                slice_input: self.slice_input.clone(),
                request_id: self.id,
            };
        }

        let not_displayed = self.slice_input.not_displayed();
        if not_displayed.is_empty() {
            return VectorSliceResponse {
                indices: (0..self.data.len()).collect(),
                alphas: SliceScale::Scalar(1.0),
                slice_input: self.slice_input.clone(),
                request_id: self.id,
            };
        }

        let (indices, alphas) = self.get_slice_data(&not_displayed);
        VectorSliceResponse {
            indices,
            alphas,
            slice_input: self.slice_input.clone(),
            request_id: self.id,
        }
    }

    fn get_slice_data(&self, not_displayed: &[usize]) -> (Vec<usize>, SliceScale) {
        let (low, high) = slice_bounds(
            &self.data_slice,
            not_displayed,
            self.projection_mode == VectorsProjectionMode::None,
        );
        let starts: Vec<Vec<f64>> = self
            .data
            .iter()
            .map(|vector| {
                not_displayed
                    .iter()
                    .map(|&axis| vector.start[axis])
                    .collect()
            })
            .collect();
        let inside_slice = inside_slice_mask(&starts, &low, &high);
        let mut slice_indices: Vec<usize> = inside_slice
            .iter()
            .enumerate()
            .filter_map(|(index, &inside)| inside.then_some(index))
            .collect();

        if self.out_of_slice_display && self.slice_input.ndim() > 2 {
            let mut matches = Vec::new();
            let mut alphas = Vec::new();
            for (index, (coords, vector)) in starts.iter().zip(&self.data).enumerate() {
                let projected_lengths: Vec<f64> = not_displayed
                    .iter()
                    .map(|&axis| (vector.direction[axis] * self.length).abs())
                    .collect();
                let distances = distances_from_slice(coords, &low, &high, inside_slice[index]);
                if distances
                    .iter()
                    .zip(&projected_lengths)
                    .all(|(&distance, &length)| distance <= length)
                {
                    matches.push(index);
                    alphas.push(
                        distances
                            .into_iter()
                            .zip(projected_lengths)
                            .map(|(distance, length)| {
                                let length = if length == 0.0 { 1.0 } else { length };
                                (length - distance) / length
                            })
                            .product(),
                    );
                }
            }
            slice_indices = matches;
            return (slice_indices, SliceScale::Values(alphas));
        }

        (slice_indices, SliceScale::Scalar(1.0))
    }
}
