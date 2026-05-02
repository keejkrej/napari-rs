use std::error::Error;
use std::fmt;

use crate::utils::misc::{argsort, reorder_after_dim_reduction};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeTuple {
    pub start: f64,
    pub stop: f64,
    pub step: f64,
}

impl RangeTuple {
    pub fn new(start: f64, stop: f64, step: f64) -> Result<Self, DimsError> {
        let range = Self { start, stop, step };
        validate_range(range, None)?;
        Ok(range)
    }
}

impl Default for RangeTuple {
    fn default() -> Self {
        Self {
            start: 0.0,
            stop: 2.0,
            step: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dims {
    pub ndim: usize,
    pub ndisplay: usize,
    pub order: Vec<usize>,
    pub axis_labels: Vec<String>,
    pub rollable: Vec<bool>,
    pub range: Vec<RangeTuple>,
    pub margin_left: Vec<f64>,
    pub margin_right: Vec<f64>,
    pub point: Vec<f64>,
    pub last_used: usize,
}

impl Default for Dims {
    fn default() -> Self {
        Self::new(2).expect("default dimensions are valid")
    }
}

impl Dims {
    pub fn new(ndim: usize) -> Result<Self, DimsError> {
        let mut dims = Self {
            ndim,
            ndisplay: 2,
            order: Vec::new(),
            axis_labels: Vec::new(),
            rollable: Vec::new(),
            range: Vec::new(),
            margin_left: Vec::new(),
            margin_right: Vec::new(),
            point: Vec::new(),
            last_used: 0,
        };
        dims.normalize()?;
        Ok(dims)
    }

    pub fn with_options(
        ndim: usize,
        ndisplay: usize,
        order: Vec<usize>,
        axis_labels: Vec<String>,
        range: Vec<RangeTuple>,
        point: Vec<f64>,
    ) -> Result<Self, DimsError> {
        let mut dims = Self {
            ndim,
            ndisplay,
            order,
            axis_labels,
            rollable: Vec::new(),
            range,
            margin_left: Vec::new(),
            margin_right: Vec::new(),
            point,
            last_used: 0,
        };
        dims.normalize()?;
        Ok(dims)
    }

    pub fn set_ndim(&mut self, ndim: usize) -> Result<(), DimsError> {
        self.ndim = ndim;
        self.normalize()
    }

    pub fn set_ndisplay(&mut self, ndisplay: usize) -> Result<(), DimsError> {
        if !matches!(ndisplay, 2 | 3) {
            return Err(DimsError::InvalidNDisplay(ndisplay));
        }
        self.ndisplay = ndisplay;
        self.normalize()
    }

    pub fn set_order(&mut self, order: Vec<usize>) -> Result<(), DimsError> {
        self.order = order;
        self.normalize()
    }

    pub fn set_axis_labels(&mut self, labels: Vec<String>) -> Result<(), DimsError> {
        self.axis_labels = labels;
        self.normalize()
    }

    pub fn set_axis_labels_from_str(&mut self, labels: &str) -> Result<(), DimsError> {
        self.set_axis_labels(labels.chars().map(|char| char.to_string()).collect())
    }

    pub fn set_range(&mut self, axis: isize, range: RangeTuple) -> Result<(), DimsError> {
        let axis = ensure_axis_in_bounds(axis, self.ndim)?;
        validate_range(range, Some(axis))?;
        self.range[axis] = range;
        self.normalize()
    }

    pub fn set_ranges(&mut self, axes: &[isize], ranges: &[RangeTuple]) -> Result<(), DimsError> {
        if axes.len() != ranges.len() {
            return Err(DimsError::MismatchedInputLength);
        }
        let mut full_range = self.range.clone();
        for (&axis, &range) in axes.iter().zip(ranges) {
            let axis = ensure_axis_in_bounds(axis, self.ndim)?;
            validate_range(range, Some(axis))?;
            full_range[axis] = range;
        }
        self.range = full_range;
        self.normalize()
    }

    pub fn set_point(&mut self, axis: isize, value: f64) -> Result<(), DimsError> {
        let axis = ensure_axis_in_bounds(axis, self.ndim)?;
        self.point[axis] = value;
        self.normalize()
    }

    pub fn set_points(&mut self, axes: &[isize], values: &[f64]) -> Result<(), DimsError> {
        if axes.len() != values.len() {
            return Err(DimsError::MismatchedInputLength);
        }
        let mut full_point = self.point.clone();
        for (&axis, &value) in axes.iter().zip(values) {
            let axis = ensure_axis_in_bounds(axis, self.ndim)?;
            full_point[axis] = value;
        }
        self.point = full_point;
        self.normalize()
    }

    pub fn set_current_step(&mut self, axis: isize, value: isize) -> Result<(), DimsError> {
        let axis = ensure_axis_in_bounds(axis, self.ndim)?;
        let range = self.range[axis];
        self.set_point(axis as isize, range.start + (value as f64 * range.step))
    }

    pub fn set_current_steps(&mut self, axes: &[isize], values: &[isize]) -> Result<(), DimsError> {
        if axes.len() != values.len() {
            return Err(DimsError::MismatchedInputLength);
        }
        let mut points = Vec::with_capacity(values.len());
        for (&axis, &value) in axes.iter().zip(values) {
            let axis = ensure_axis_in_bounds(axis, self.ndim)?;
            let range = self.range[axis];
            points.push(range.start + (value as f64 * range.step));
        }
        self.set_points(axes, &points)
    }

    pub fn set_axis_label(
        &mut self,
        axis: isize,
        label: impl Into<String>,
    ) -> Result<(), DimsError> {
        let axis = ensure_axis_in_bounds(axis, self.ndim)?;
        self.axis_labels[axis] = label.into();
        self.normalize()
    }

    pub fn set_axis_labels_for_axes(
        &mut self,
        axes: &[isize],
        labels: &[String],
    ) -> Result<(), DimsError> {
        if axes.len() != labels.len() {
            return Err(DimsError::MismatchedInputLength);
        }
        let mut axis_labels = self.axis_labels.clone();
        for (&axis, label) in axes.iter().zip(labels) {
            let axis = ensure_axis_in_bounds(axis, self.ndim)?;
            axis_labels[axis] = label.clone();
        }
        self.axis_labels = axis_labels;
        self.normalize()
    }

    pub fn set_nsteps(&mut self, nsteps: &[usize]) -> Result<(), DimsError> {
        if nsteps.len() != self.range.len() {
            return Err(DimsError::MismatchedInputLength);
        }
        for (range, &nsteps) in self.range.iter_mut().zip(nsteps) {
            range.step = (range.stop - range.start) / ((nsteps - 1) as f64);
        }
        self.normalize()
    }

    pub fn set_thickness(&mut self, thickness: &[f64]) -> Result<(), DimsError> {
        if thickness.len() != self.ndim {
            return Err(DimsError::MismatchedInputLength);
        }
        self.margin_left = thickness.iter().map(|value| value / 2.0).collect();
        self.margin_right = self.margin_left.clone();
        self.normalize()
    }

    pub fn set_thickness_step(&mut self, thickness_step: &[isize]) -> Result<(), DimsError> {
        if thickness_step.len() != self.ndim {
            return Err(DimsError::MismatchedInputLength);
        }
        let margin_steps: Vec<isize> = thickness_step.iter().map(|value| value / 2).collect();
        self.set_margin_left_step(&margin_steps)?;
        self.set_margin_right_step(&margin_steps)
    }

    pub fn set_margin_left_step(&mut self, steps: &[isize]) -> Result<(), DimsError> {
        if steps.len() != self.ndim {
            return Err(DimsError::MismatchedInputLength);
        }
        self.margin_left = steps
            .iter()
            .zip(&self.range)
            .map(|(&step, range)| step as f64 * range.step)
            .collect();
        self.normalize()
    }

    pub fn set_margin_right_step(&mut self, steps: &[isize]) -> Result<(), DimsError> {
        if steps.len() != self.ndim {
            return Err(DimsError::MismatchedInputLength);
        }
        self.margin_right = steps
            .iter()
            .zip(&self.range)
            .map(|(&step, range)| step as f64 * range.step)
            .collect();
        self.normalize()
    }

    pub fn reset(&mut self) {
        self.range = vec![RangeTuple::default(); self.ndim];
        self.point = vec![0.0; self.ndim];
        self.order = (0..self.ndim).collect();
        self.margin_left = vec![0.0; self.ndim];
        self.margin_right = vec![0.0; self.ndim];
        self.rollable = vec![true; self.ndim];
    }

    pub fn transpose(&mut self) {
        let last = self.order.len() - 1;
        self.order.swap(last - 1, last);
    }

    pub fn increment_dims_right(&mut self, axis: Option<isize>) -> Result<(), DimsError> {
        let axis = axis.unwrap_or(self.last_used as isize);
        let axis_index = ensure_axis_in_bounds(axis, self.ndim)?;
        self.set_current_step(axis, self.current_step()[axis_index] + 1)
    }

    pub fn increment_dims_left(&mut self, axis: Option<isize>) -> Result<(), DimsError> {
        let axis = axis.unwrap_or(self.last_used as isize);
        let axis_index = ensure_axis_in_bounds(axis, self.ndim)?;
        self.set_current_step(axis, self.current_step()[axis_index] - 1)
    }

    pub fn focus_up(&mut self) {
        let sliders = self.sliders();
        if sliders.is_empty() {
            return;
        }
        if let Some(index) = sliders.iter().position(|&axis| axis == self.last_used) {
            self.last_used = sliders[(index + 1) % sliders.len()];
        }
    }

    pub fn focus_down(&mut self) {
        let sliders = self.sliders();
        if sliders.is_empty() {
            return;
        }
        if let Some(index) = sliders.iter().position(|&axis| axis == self.last_used) {
            self.last_used = sliders[(index + sliders.len() - 1) % sliders.len()];
        }
    }

    pub fn roll(&mut self) -> Result<(), DimsError> {
        let nsteps = self.nsteps();
        let mut order = self.order.clone();
        let valid_positions: Vec<usize> = order
            .iter()
            .enumerate()
            .filter_map(|(position, &axis)| {
                (self.rollable[axis] && nsteps[axis] > 1).then_some(position)
            })
            .collect();

        if valid_positions.len() > 1 {
            let values: Vec<usize> = valid_positions
                .iter()
                .map(|&position| order[position])
                .collect();
            for (index, &position) in valid_positions.iter().enumerate() {
                order[position] = values[(index + values.len() - 1) % values.len()];
            }
        }

        self.set_order(order)
    }

    pub fn go_to_center_step(&mut self) -> Result<(), DimsError> {
        let center_steps: Vec<isize> = self
            .nsteps()
            .into_iter()
            .map(|steps| ((steps - 1) / 2) as isize)
            .collect();
        let axes: Vec<isize> = (0..self.ndim).map(|axis| axis as isize).collect();
        self.set_current_steps(&axes, &center_steps)
    }

    pub fn nsteps(&self) -> Vec<usize> {
        nsteps_from_range(&self.range)
    }

    pub fn current_step(&self) -> Vec<isize> {
        self.point
            .iter()
            .zip(&self.range)
            .map(|(&point, range)| ((point - range.start) / range.step_or_one()).round() as isize)
            .collect()
    }

    pub fn margin_left_step(&self) -> Vec<isize> {
        self.margin_left
            .iter()
            .zip(&self.range)
            .map(|(&margin, range)| (margin / range.step_or_one()).round() as isize)
            .collect()
    }

    pub fn margin_right_step(&self) -> Vec<isize> {
        self.margin_right
            .iter()
            .zip(&self.range)
            .map(|(&margin, range)| (margin / range.step_or_one()).round() as isize)
            .collect()
    }

    pub fn thickness(&self) -> Vec<f64> {
        self.margin_left
            .iter()
            .zip(&self.margin_right)
            .map(|(&left, &right)| left + right)
            .collect()
    }

    pub fn thickness_step(&self) -> Vec<isize> {
        self.margin_left_step()
            .into_iter()
            .zip(self.margin_right_step())
            .map(|(left, right)| left + right)
            .collect()
    }

    pub fn displayed(&self) -> &[usize] {
        &self.order[self.order.len() - self.ndisplay..]
    }

    pub fn not_displayed(&self) -> &[usize] {
        &self.order[..self.order.len() - self.ndisplay]
    }

    pub fn displayed_order(&self) -> Vec<usize> {
        argsort(self.displayed())
    }

    fn normalize(&mut self) -> Result<(), DimsError> {
        if !matches!(self.ndisplay, 2 | 3) {
            return Err(DimsError::InvalidNDisplay(self.ndisplay));
        }
        if self.ndisplay > self.ndim {
            self.ndisplay = self.ndim;
        }

        self.range = ensure_len(self.range.clone(), self.ndim, RangeTuple::default());
        for (axis, &range) in self.range.iter().enumerate() {
            validate_range(range, Some(axis))?;
        }

        self.point = ensure_len(self.point.clone(), self.ndim, 0.0);
        for (point, range) in self.point.iter_mut().zip(&self.range) {
            *point = point.clamp(range.start, range.stop);
        }

        self.margin_left = ensure_len(self.margin_left.clone(), self.ndim, 0.0);
        self.margin_right = ensure_len(self.margin_right.clone(), self.ndim, 0.0);

        self.normalize_order()?;
        self.normalize_axis_labels();
        self.rollable = ensure_len(self.rollable.clone(), self.ndim, true);
        self.normalize_last_used();

        Ok(())
    }

    fn normalize_order(&mut self) -> Result<(), DimsError> {
        if self.order.len() < self.ndim {
            let order_ndim = self.order.len();
            let mut order: Vec<usize> = (0..self.ndim - order_ndim).collect();
            order.extend(self.order.iter().map(|axis| axis + self.ndim - order_ndim));
            self.order = order;
        } else if self.order.len() > self.ndim {
            let retained = self.order[self.order.len() - self.ndim..].to_vec();
            self.order = reorder_after_dim_reduction(&retained);
        }

        let mut sorted = self.order.clone();
        sorted.sort_unstable();
        if sorted != (0..self.ndim).collect::<Vec<_>>() {
            return Err(DimsError::InvalidOrder {
                order: self.order.clone(),
                ndim: self.ndim,
            });
        }
        Ok(())
    }

    fn normalize_axis_labels(&mut self) {
        let labels_ndim = self.axis_labels.len();
        if labels_ndim < self.ndim {
            let mut labels: Vec<String> = (-(self.ndim as isize)..-(labels_ndim as isize))
                .map(|axis| axis.to_string())
                .collect();
            labels.extend(self.axis_labels.clone());
            self.axis_labels = labels;
        } else if labels_ndim > self.ndim {
            self.axis_labels = self.axis_labels[labels_ndim - self.ndim..].to_vec();
        }
    }

    fn normalize_last_used(&mut self) {
        let nsteps = self.nsteps();
        let not_displayed: Vec<usize> = self
            .not_displayed()
            .iter()
            .copied()
            .filter(|&axis| nsteps[axis] > 1)
            .collect();
        if !not_displayed.is_empty() && !not_displayed.contains(&self.last_used) {
            self.last_used = not_displayed[0];
        }
    }

    fn sliders(&self) -> Vec<usize> {
        let nsteps = self.nsteps();
        self.not_displayed()
            .iter()
            .copied()
            .filter(|&axis| nsteps[axis] > 1)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimsError {
    AxisOutOfBounds {
        axis: isize,
        ndim: usize,
    },
    InvalidNDisplay(usize),
    InvalidOrder {
        order: Vec<usize>,
        ndim: usize,
    },
    InvalidRangeOrder {
        axis: Option<usize>,
        start: String,
        stop: String,
    },
    InvalidRangeStep {
        axis: Option<usize>,
        step: String,
    },
    MismatchedInputLength,
}

impl fmt::Display for DimsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AxisOutOfBounds { axis, ndim } => write!(
                formatter,
                "axis {axis} not defined for dimensionality {ndim}. must be in [-{ndim}, {ndim})"
            ),
            Self::InvalidNDisplay(ndisplay) => {
                write!(formatter, "ndisplay must be 2 or 3, got {ndisplay}")
            }
            Self::InvalidOrder { order, ndim } => {
                write!(
                    formatter,
                    "invalid ordering {order:?} for {ndim} dimensions"
                )
            }
            Self::InvalidRangeOrder { axis, start, stop } => write!(
                formatter,
                "start and stop must be strictly increasing, but got ({start}, {stop}) for axis {axis:?}"
            ),
            Self::InvalidRangeStep { axis, step } => write!(
                formatter,
                "step must be strictly positive, but got {step} for axis {axis:?}"
            ),
            Self::MismatchedInputLength => {
                formatter.write_str("axis and value sequences must have equal length")
            }
        }
    }
}

impl Error for DimsError {}

pub fn ensure_len<T: Clone>(mut value: Vec<T>, length: usize, pad_width: T) -> Vec<T> {
    if value.len() < length {
        let mut padded = vec![pad_width; length - value.len()];
        padded.extend(value);
        padded
    } else if value.len() > length {
        value.drain(0..value.len() - length);
        value
    } else {
        value
    }
}

pub fn ensure_axis_in_bounds(axis: isize, ndim: usize) -> Result<usize, DimsError> {
    let ndim = ndim as isize;
    if axis < -ndim || axis >= ndim {
        return Err(DimsError::AxisOutOfBounds {
            axis,
            ndim: ndim as usize,
        });
    }
    Ok(axis.rem_euclid(ndim) as usize)
}

pub fn nsteps_from_range(dims_range: &[RangeTuple]) -> Vec<usize> {
    dims_range
        .iter()
        .map(|range| ((range.stop - range.start) / range.step_or_one()) as usize + 1)
        .collect()
}

fn validate_range(range: RangeTuple, axis: Option<usize>) -> Result<(), DimsError> {
    if range.start > range.stop {
        return Err(DimsError::InvalidRangeOrder {
            axis,
            start: range.start.to_string(),
            stop: range.stop.to_string(),
        });
    }
    if range.step <= 0.0 {
        return Err(DimsError::InvalidRangeStep {
            axis,
            step: range.step.to_string(),
        });
    }
    Ok(())
}

impl RangeTuple {
    fn step_or_one(self) -> f64 {
        if self.step == 0.0 { 1.0 } else { self.step }
    }
}
