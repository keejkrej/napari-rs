use std::fmt;

use crate::layers::multiscale_data::{ArrayMetadata, MultiScaleData, MultiScaleDataError};
use crate::utils::dtype::DType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageData {
    Array(ArrayMetadata),
    Sequence(Vec<ArrayMetadata>),
    MultiScale(MultiScaleData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuessMultiscaleError {
    EqualSingleSize(usize),
    IncorrectOrder(Vec<usize>),
    EmptyMultiscale(MultiScaleDataError),
}

impl fmt::Display for GuessMultiscaleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EqualSingleSize(size) => write!(
                formatter,
                "multiscale data has equal-sized levels with size {size}"
            ),
            Self::IncorrectOrder(sizes) => {
                write!(
                    formatter,
                    "multiscale sizes must be strictly decreasing: {sizes:?}"
                )
            }
            Self::EmptyMultiscale(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GuessMultiscaleError {}

pub fn guess_rgb(shape: &[usize], min_side_len: usize) -> bool {
    shape.len() > 2
        && matches!(shape.last(), Some(3 | 4))
        && shape[shape.len() - 3] > min_side_len
        && shape[shape.len() - 2] > min_side_len
}

pub fn guess_multiscale(data: ImageData) -> Result<(bool, ImageData), GuessMultiscaleError> {
    match data {
        ImageData::MultiScale(data) => Ok((true, ImageData::MultiScale(data))),
        ImageData::Array(array) => Ok((false, ImageData::Array(array))),
        ImageData::Sequence(levels) if levels.len() == 1 => {
            let array = levels.into_iter().next().expect("length checked");
            Ok((false, ImageData::Array(array)))
        }
        ImageData::Sequence(levels) => guess_multiscale_sequence(levels),
    }
}

pub fn guess_labels(dtype: DType) -> &'static str {
    match dtype {
        DType::Int32 | DType::UInt32 | DType::Int64 | DType::UInt64 => "labels",
        _ => "image",
    }
}

fn guess_multiscale_sequence(
    levels: Vec<ArrayMetadata>,
) -> Result<(bool, ImageData), GuessMultiscaleError> {
    let sizes: Vec<usize> = levels.iter().map(ArrayMetadata::size).collect();

    if sizes.len() <= 1 {
        return Ok((false, ImageData::Sequence(levels)));
    }
    if sizes.windows(2).all(|pair| pair[0] == pair[1]) {
        return Err(GuessMultiscaleError::EqualSingleSize(sizes[0]));
    }
    if !sizes.windows(2).all(|pair| pair[0] > pair[1]) {
        return Err(GuessMultiscaleError::IncorrectOrder(sizes));
    }

    let multiscale = MultiScaleData::new(levels).map_err(GuessMultiscaleError::EmptyMultiscale)?;
    Ok((true, ImageData::MultiScale(multiscale)))
}
