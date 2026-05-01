use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorRecord {
    pub start: Vec<f64>,
    pub projection: Vec<f64>,
}

impl VectorRecord {
    pub fn new(start: impl Into<Vec<f64>>, projection: impl Into<Vec<f64>>) -> Self {
        Self {
            start: start.into(),
            projection: projection.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawVectorData {
    Empty {
        ndim: Option<usize>,
    },
    Coordinate(Vec<VectorRecord>),
    Single(VectorRecord),
    Image {
        spatial_shape: Vec<usize>,
        projections: Vec<Vec<f64>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorDataError {
    InvalidVectorDimension {
        start: usize,
        projection: usize,
    },
    InvalidImageVectorDimension {
        spatial_ndim: usize,
        vector_ndim: usize,
    },
    InvalidImageProjectionCount {
        expected: usize,
        found: usize,
    },
    DimensionMismatch {
        data_ndim: usize,
        ndim: usize,
    },
}

impl fmt::Display for VectorDataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVectorDimension { start, projection } => write!(
                formatter,
                "vector start dimension ({start}) must match projection dimension ({projection})"
            ),
            Self::InvalidImageVectorDimension {
                spatial_ndim,
                vector_ndim,
            } => write!(
                formatter,
                "image-like vector dimension ({vector_ndim}) must match spatial dimension ({spatial_ndim})"
            ),
            Self::InvalidImageProjectionCount { expected, found } => write!(
                formatter,
                "image-like vector projection count ({found}) must match spatial size ({expected})"
            ),
            Self::DimensionMismatch { data_ndim, ndim } => {
                write!(
                    formatter,
                    "vectors dimensions ({data_ndim}) must be equal to ndim ({ndim})"
                )
            }
        }
    }
}

impl std::error::Error for VectorDataError {}

pub fn convert_image_to_coordinates(
    spatial_shape: &[usize],
    projections: &[Vec<f64>],
) -> Result<Vec<VectorRecord>, VectorDataError> {
    let spatial_size: usize = spatial_shape.iter().product();
    if projections.len() != spatial_size {
        return Err(VectorDataError::InvalidImageProjectionCount {
            expected: spatial_size,
            found: projections.len(),
        });
    }

    let ndim = spatial_shape.len();
    for projection in projections {
        if projection.len() != ndim {
            return Err(VectorDataError::InvalidImageVectorDimension {
                spatial_ndim: ndim,
                vector_ndim: projection.len(),
            });
        }
    }

    Ok(projections
        .iter()
        .enumerate()
        .map(|(index, projection)| {
            VectorRecord::new(unravel_index(index, spatial_shape), projection.clone())
        })
        .collect())
}

pub fn fix_data_vectors(
    vectors: Option<RawVectorData>,
    ndim: Option<usize>,
) -> Result<(Vec<VectorRecord>, usize), VectorDataError> {
    let (vectors, data_ndim) = match vectors {
        None | Some(RawVectorData::Empty { ndim: None }) => (Vec::new(), ndim.unwrap_or(2)),
        Some(RawVectorData::Empty {
            ndim: Some(data_ndim),
        }) => (Vec::new(), data_ndim),
        Some(RawVectorData::Coordinate(vectors)) => {
            let data_ndim = validate_coordinate_vectors(&vectors)?;
            (vectors, data_ndim)
        }
        Some(RawVectorData::Single(vector)) => {
            let data_ndim = validate_vector(&vector)?;
            (vec![vector], data_ndim)
        }
        Some(RawVectorData::Image {
            spatial_shape,
            projections,
        }) => {
            let data_ndim = spatial_shape.len();
            (
                convert_image_to_coordinates(&spatial_shape, &projections)?,
                data_ndim,
            )
        }
    };

    if let Some(ndim) = ndim
        && ndim != data_ndim
    {
        return Err(VectorDataError::DimensionMismatch { data_ndim, ndim });
    }

    Ok((vectors, data_ndim))
}

fn validate_coordinate_vectors(vectors: &[VectorRecord]) -> Result<usize, VectorDataError> {
    let Some(first) = vectors.first() else {
        return Ok(2);
    };
    let data_ndim = validate_vector(first)?;
    for vector in vectors {
        let vector_ndim = validate_vector(vector)?;
        if vector_ndim != data_ndim {
            return Err(VectorDataError::InvalidVectorDimension {
                start: vector.start.len(),
                projection: vector.projection.len(),
            });
        }
    }
    Ok(data_ndim)
}

fn validate_vector(vector: &VectorRecord) -> Result<usize, VectorDataError> {
    if vector.start.len() != vector.projection.len() {
        return Err(VectorDataError::InvalidVectorDimension {
            start: vector.start.len(),
            projection: vector.projection.len(),
        });
    }
    Ok(vector.projection.len())
}

fn unravel_index(mut index: usize, shape: &[usize]) -> Vec<f64> {
    let mut coords = vec![0.0; shape.len()];
    for (axis, size) in shape.iter().enumerate().rev() {
        coords[axis] = (index % size) as f64;
        index /= size;
    }
    coords
}
