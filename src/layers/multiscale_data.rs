use crate::utils::dtype::DType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayMetadata {
    pub shape: Vec<usize>,
    pub dtype: DType,
}

impl ArrayMetadata {
    pub fn new(shape: impl Into<Vec<usize>>, dtype: DType) -> Self {
        Self {
            shape: shape.into(),
            dtype,
        }
    }

    pub fn size(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn ndim(&self) -> usize {
        self.shape.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiScaleData {
    levels: Vec<ArrayMetadata>,
}

impl MultiScaleData {
    pub fn new(levels: impl Into<Vec<ArrayMetadata>>) -> Result<Self, MultiScaleDataError> {
        let levels = levels.into();
        if levels.is_empty() {
            return Err(MultiScaleDataError::Empty);
        }
        Ok(Self { levels })
    }

    pub fn size(&self) -> usize {
        self.levels[0].size()
    }

    pub fn ndim(&self) -> usize {
        self.levels[0].ndim()
    }

    pub fn dtype(&self) -> DType {
        self.levels[0].dtype
    }

    pub fn shape(&self) -> &[usize] {
        &self.levels[0].shape
    }

    pub fn shapes(&self) -> Vec<Vec<usize>> {
        self.levels
            .iter()
            .map(|level| level.shape.clone())
            .collect()
    }

    pub fn get(&self, index: usize) -> Option<&ArrayMetadata> {
        self.levels.get(index)
    }

    pub fn len(&self) -> usize {
        self.levels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    pub fn levels(&self) -> &[ArrayMetadata] {
        &self.levels
    }

    pub fn to_lowest_resolution_array(&self) -> &ArrayMetadata {
        self.levels.last().expect("multiscale data is non-empty")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiScaleDataError {
    Empty,
}

impl std::fmt::Display for MultiScaleDataError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Multiscale data must be a non-empty sequence"),
        }
    }
}

impl std::error::Error for MultiScaleDataError {}
