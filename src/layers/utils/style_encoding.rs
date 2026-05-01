use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyleEncodingError {
    IndexOutOfBounds { index: usize, len: usize },
}

impl fmt::Display for StyleEncodingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StyleEncodingError::IndexOutOfBounds { index, len } => {
                write!(
                    formatter,
                    "style value index {index} is out of bounds for length {len}"
                )
            }
        }
    }
}

impl std::error::Error for StyleEncodingError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleValueSource<'a, T> {
    Single(&'a T),
    Multiple(&'a [T]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyleValueSelection<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub fn get_style_values<T: Clone>(
    values: StyleValueSource<'_, T>,
    indices: &[usize],
) -> Result<StyleValueSelection<T>, StyleEncodingError> {
    match values {
        StyleValueSource::Single(value) => Ok(StyleValueSelection::Single(value.clone())),
        StyleValueSource::Multiple(values) => {
            let mut selected = Vec::with_capacity(indices.len());
            for &index in indices {
                let value = values
                    .get(index)
                    .ok_or(StyleEncodingError::IndexOutOfBounds {
                        index,
                        len: values.len(),
                    })?;
                selected.push(value.clone());
            }
            Ok(StyleValueSelection::Multiple(selected))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantStyleEncoding<T> {
    pub constant: T,
}

impl<T: Clone> ConstantStyleEncoding<T> {
    pub fn values(&self) -> T {
        self.constant.clone()
    }

    pub fn apply(&mut self, _num_rows: usize) {}

    pub fn append(&mut self, _values: &[T]) {}

    pub fn delete(&mut self, _indices: &[usize]) {}

    pub fn clear(&mut self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManualStyleEncoding<T> {
    pub array: Vec<T>,
    pub default: T,
}

impl<T: Clone> ManualStyleEncoding<T> {
    pub fn apply(&mut self, num_rows: usize) {
        self.array = self.call(num_rows);
    }

    pub fn call(&self, num_rows: usize) -> Vec<T> {
        if num_rows > self.array.len() {
            let mut out = self.array.clone();
            out.extend(std::iter::repeat_n(
                self.default.clone(),
                num_rows - self.array.len(),
            ));
            out
        } else {
            self.array[..num_rows].to_vec()
        }
    }

    pub fn append(&mut self, values: &[T]) {
        self.array.extend_from_slice(values);
    }

    pub fn delete(&mut self, indices: &[usize]) {
        delete_indices(&mut self.array, indices);
    }

    pub fn clear(&mut self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedStyleEncoding<T> {
    pub cached: Vec<T>,
    pub fallback: T,
}

impl<T: Clone> DerivedStyleEncoding<T> {
    pub fn new(fallback: T) -> Self {
        Self {
            cached: Vec::new(),
            fallback,
        }
    }

    pub fn values(&self) -> &[T] {
        &self.cached
    }

    pub fn apply_with<E>(
        &mut self,
        num_rows: usize,
        derive_tail: impl FnOnce(usize, usize) -> Result<Vec<T>, E>,
    ) {
        let num_cached = self.cached.len();
        if num_cached < num_rows {
            let tail_len = num_rows - num_cached;
            let tail = derive_tail(num_cached, num_rows)
                .unwrap_or_else(|_| std::iter::repeat_n(self.fallback.clone(), tail_len).collect());
            self.append(&tail);
        } else if num_cached > num_rows {
            self.cached.truncate(num_rows);
        }
    }

    pub fn append(&mut self, values: &[T]) {
        self.cached.extend_from_slice(values);
    }

    pub fn delete(&mut self, indices: &[usize]) {
        delete_indices(&mut self.cached, indices);
    }

    pub fn clear(&mut self) {
        self.cached.clear();
    }
}

fn delete_indices<T>(values: &mut Vec<T>, indices: &[usize]) {
    let mut sorted = indices.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    for index in sorted.into_iter().rev() {
        if index < values.len() {
            values.remove(index);
        }
    }
}
