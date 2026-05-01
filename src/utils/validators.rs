use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    Length {
        expected: usize,
        actual: usize,
    },
    Type {
        expected: &'static str,
        index: usize,
    },
    NotIncreasing,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length { expected, actual } => {
                write!(
                    formatter,
                    "object must have length {expected}, got {actual}"
                )
            }
            Self::Type { expected, index } => {
                write!(
                    formatter,
                    "item at index {index} must be of type {expected}"
                )
            }
            Self::NotIncreasing => write!(formatter, "sequence must be monotonically increasing"),
        }
    }
}

impl Error for ValidationError {}

pub fn validate_n_seq<T>(values: &[T], n: usize) -> Result<(), ValidationError> {
    if values.len() != n {
        return Err(ValidationError::Length {
            expected: n,
            actual: values.len(),
        });
    }

    Ok(())
}

pub fn validate_n_seq_by<T>(
    values: &[T],
    n: usize,
    expected_type: &'static str,
    predicate: impl Fn(&T) -> bool,
) -> Result<(), ValidationError> {
    validate_n_seq(values, n)?;

    for (index, value) in values.iter().enumerate() {
        if !predicate(value) {
            return Err(ValidationError::Type {
                expected: expected_type,
                index,
            });
        }
    }

    Ok(())
}

pub fn pairwise<T: Clone>(values: &[T]) -> Vec<(T, T)> {
    values
        .windows(2)
        .map(|window| (window[0].clone(), window[1].clone()))
        .collect()
}

pub fn validate_increasing<T: PartialOrd>(values: &[T]) -> Result<(), ValidationError> {
    if values.windows(2).any(|window| window[0] >= window[1]) {
        return Err(ValidationError::NotIncreasing);
    }

    Ok(())
}
