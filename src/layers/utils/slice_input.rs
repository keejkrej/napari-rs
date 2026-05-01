use std::error::Error;
use std::fmt;

use crate::utils::misc::reorder_after_dim_reduction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliceInputError {
    MissingNdim,
    InvalidArrayRows {
        rows: usize,
    },
    MismatchedArrayRowLengths,
    InvalidLinearMatrix {
        rows: usize,
        expected: usize,
    },
    InvalidLinearMatrixRow {
        row: usize,
        columns: usize,
        expected: usize,
    },
}

impl fmt::Display for SliceInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNdim => write!(
                formatter,
                "ndim must be provided if no point or margin values are given"
            ),
            Self::InvalidArrayRows { rows } => {
                write!(formatter, "expected three slice rows, got {rows}")
            }
            Self::MismatchedArrayRowLengths => {
                write!(formatter, "slice rows must all have the same length")
            }
            Self::InvalidLinearMatrix { rows, expected } => {
                write!(
                    formatter,
                    "expected {expected} linear matrix rows, got {rows}"
                )
            }
            Self::InvalidLinearMatrixRow {
                row,
                columns,
                expected,
            } => write!(
                formatter,
                "expected {expected} columns in linear matrix row {row}, got {columns}"
            ),
        }
    }
}

impl Error for SliceInputError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThickNdSlice<T> {
    pub point: Vec<T>,
    pub margin_left: Vec<T>,
    pub margin_right: Vec<T>,
}

impl<T: Clone + Default> ThickNdSlice<T> {
    pub fn make_full(
        point: Option<&[T]>,
        margin_left: Option<&[T]>,
        margin_right: Option<&[T]>,
        ndim: Option<usize>,
    ) -> Result<Self, SliceInputError> {
        let value_ndim = point
            .or(margin_left)
            .or(margin_right)
            .map(<[T]>::len)
            .or(ndim)
            .ok_or(SliceInputError::MissingNdim)?;
        let ndim = ndim.unwrap_or(value_ndim);

        Ok(Self {
            point: normalize_slice_field(point, ndim, value_ndim),
            margin_left: normalize_slice_field(margin_left, ndim, value_ndim),
            margin_right: normalize_slice_field(margin_right, ndim, value_ndim),
        })
    }

    pub fn copy_with(
        &self,
        point: Option<&[T]>,
        margin_left: Option<&[T]>,
        margin_right: Option<&[T]>,
        ndim: Option<usize>,
    ) -> Result<Self, SliceInputError> {
        Self::make_full(
            point.or(Some(&self.point)),
            margin_left.or(Some(&self.margin_left)),
            margin_right.or(Some(&self.margin_right)),
            ndim.or(Some(self.ndim())),
        )
    }
}

impl<T: Clone> ThickNdSlice<T> {
    pub fn ndim(&self) -> usize {
        self.point.len()
    }

    pub fn as_rows(&self) -> Vec<Vec<T>> {
        vec![
            self.point.clone(),
            self.margin_left.clone(),
            self.margin_right.clone(),
        ]
    }

    pub fn from_rows(rows: &[Vec<T>]) -> Result<Self, SliceInputError> {
        if rows.len() != 3 {
            return Err(SliceInputError::InvalidArrayRows { rows: rows.len() });
        }
        let row_len = rows[0].len();
        if rows.iter().any(|row| row.len() != row_len) {
            return Err(SliceInputError::MismatchedArrayRowLengths);
        }
        Ok(Self {
            point: rows[0].clone(),
            margin_left: rows[1].clone(),
            margin_right: rows[2].clone(),
        })
    }

    pub fn select_axes(&self, axes: &[usize]) -> Self {
        Self {
            point: axes.iter().map(|&axis| self.point[axis].clone()).collect(),
            margin_left: axes
                .iter()
                .map(|&axis| self.margin_left[axis].clone())
                .collect(),
            margin_right: axes
                .iter()
                .map(|&axis| self.margin_right[axis].clone())
                .collect(),
        }
    }

    pub fn iter_by_dimension(&self) -> impl Iterator<Item = (&T, &T, &T)> {
        self.point
            .iter()
            .zip(&self.margin_left)
            .zip(&self.margin_right)
            .map(|((point, margin_left), margin_right)| (point, margin_left, margin_right))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliceInput {
    pub ndisplay: usize,
    pub world_slice: ThickNdSlice<f64>,
    pub order: Vec<usize>,
}

impl SliceInput {
    pub fn ndim(&self) -> usize {
        self.order.len()
    }

    pub fn displayed(&self) -> Vec<usize> {
        let split = displayed_split_index(self.order.len(), self.ndisplay);
        self.order[split..].to_vec()
    }

    pub fn not_displayed(&self) -> Vec<usize> {
        let split = displayed_split_index(self.order.len(), self.ndisplay);
        self.order[..split].to_vec()
    }

    pub fn with_ndim(&self, ndim: usize) -> Result<Self, SliceInputError> {
        let old_ndim = self.ndim();
        let world_slice = self.world_slice.copy_with(None, None, None, Some(ndim))?;
        let order = if old_ndim > ndim {
            reorder_after_dim_reduction(&self.order[old_ndim - ndim..])
        } else if old_ndim < ndim {
            let offset = ndim - old_ndim;
            (0..offset)
                .chain(self.order.iter().map(|order| order + offset))
                .collect()
        } else {
            self.order.clone()
        };

        Ok(Self {
            ndisplay: self.ndisplay,
            world_slice,
            order,
        })
    }

    pub fn is_orthogonal_with_linear_matrix(
        &self,
        linear_matrix: &[Vec<f64>],
    ) -> Result<bool, SliceInputError> {
        let ndim = self.ndim();
        validate_linear_matrix(linear_matrix, ndim)?;

        let mut non_displayed_subspace = vec![0.0; ndim];
        for axis in self.not_displayed() {
            non_displayed_subspace[axis] = 1.0;
        }

        let mapped_subspace: Vec<f64> = linear_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&non_displayed_subspace)
                    .map(|(matrix_value, subspace_value)| matrix_value * subspace_value)
                    .sum()
            })
            .collect();

        Ok(self
            .displayed()
            .into_iter()
            .all(|axis| mapped_subspace[axis].abs() < 1e-8))
    }
}

fn normalize_slice_field<T: Clone + Default>(
    value: Option<&[T]>,
    ndim: usize,
    value_ndim: usize,
) -> Vec<T> {
    let mut output = Vec::with_capacity(ndim);
    output.extend(std::iter::repeat_n(
        T::default(),
        ndim.saturating_sub(value_ndim),
    ));
    output.extend_from_slice(value.unwrap_or(&[]));

    if output.len() > ndim {
        output.drain(0..output.len() - ndim);
    }
    if output.len() < ndim {
        output.extend(std::iter::repeat_n(T::default(), ndim - output.len()));
    }
    output
}

fn displayed_split_index(len: usize, ndisplay: usize) -> usize {
    if ndisplay == 0 {
        0
    } else {
        len.saturating_sub(ndisplay)
    }
}

fn validate_linear_matrix(
    linear_matrix: &[Vec<f64>],
    expected: usize,
) -> Result<(), SliceInputError> {
    if linear_matrix.len() != expected {
        return Err(SliceInputError::InvalidLinearMatrix {
            rows: linear_matrix.len(),
            expected,
        });
    }
    for (row, values) in linear_matrix.iter().enumerate() {
        if values.len() != expected {
            return Err(SliceInputError::InvalidLinearMatrixRow {
                row,
                columns: values.len(),
                expected,
            });
        }
    }
    Ok(())
}
