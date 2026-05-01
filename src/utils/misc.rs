use std::env;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscError {
    InvalidRgb(String),
    InvalidPathType,
    MissingPath(PathBuf),
    Length { expected: usize, actual: usize },
    EmptyTupleLength,
}

impl fmt::Display for MiscError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRgb(value) => write!(formatter, "arg not in format 'rgb(x,y,z)': {value}"),
            Self::InvalidPathType => write!(formatter, "argument must be a string or path"),
            Self::MissingPath(path) => {
                write!(formatter, "requested path {:?} does not exist", path)
            }
            Self::Length { expected, actual } => {
                write!(formatter, "length must equal {expected}, got {actual}")
            }
            Self::EmptyTupleLength => write!(formatter, "n must be greater than 0"),
        }
    }
}

impl Error for MiscError {}

pub fn str_to_rgb(arg: &str) -> Result<[u32; 3], MiscError> {
    let Some(inner) = arg
        .strip_prefix("rgb(")
        .and_then(|value| value.strip_suffix(')'))
    else {
        return Err(MiscError::InvalidRgb(arg.to_owned()));
    };
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if parts.len() != 3 {
        return Err(MiscError::InvalidRgb(arg.to_owned()));
    }

    let mut rgb = [0; 3];
    for (index, part) in parts.into_iter().enumerate() {
        if part.is_empty() || !part.chars().all(|char| char.is_ascii_digit()) {
            return Err(MiscError::InvalidRgb(arg.to_owned()));
        }
        rgb[index] = part
            .parse()
            .map_err(|_| MiscError::InvalidRgb(arg.to_owned()))?;
    }
    Ok(rgb)
}

pub fn ensure_iterable<T: Clone>(arg: Option<&[T]>, repeat_count: usize, scalar: T) -> Vec<T> {
    arg.map_or_else(|| vec![scalar; repeat_count], ToOwned::to_owned)
}

pub fn is_slice_iterable<T>(_: &[T]) -> bool {
    true
}

pub fn is_scalar_iterable<T>(_: &T) -> bool {
    false
}

pub fn is_sequence<T>(_: &[T]) -> bool {
    true
}

pub fn ensure_sequence_of_iterables<T: Clone>(
    obj: &[Vec<T>],
    length: Option<usize>,
    repeat_empty: bool,
) -> Result<SequenceOfIterables<T>, MiscError> {
    if obj.is_empty() && repeat_empty {
        return Ok(SequenceOfIterables::Repeated(Vec::new()));
    }
    if let Some(length) = length
        && obj.len() != length
    {
        return Err(MiscError::Length {
            expected: length,
            actual: obj.len(),
        });
    }
    Ok(SequenceOfIterables::Sequence(obj.to_vec()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequenceOfIterables<T> {
    Sequence(Vec<Vec<T>>),
    Repeated(Vec<T>),
}

pub fn camel_to_snake(name: &str) -> String {
    let mut output = String::with_capacity(name.len());
    let chars: Vec<char> = name.chars().collect();

    for (index, &char) in chars.iter().enumerate() {
        if char.is_ascii_uppercase()
            && index > 0
            && chars[index - 1] != '_'
            && chars[index - 1].is_ascii_lowercase()
        {
            output.push('_');
        }
        output.push(char.to_ascii_lowercase());
    }

    output
}

pub fn camel_to_spaces(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let chars: Vec<char> = value.chars().collect();

    for (index, &char) in chars.iter().enumerate() {
        let previous_is_lower = index > 0 && chars[index - 1].is_ascii_lowercase();
        let next_is_lower = chars
            .get(index + 1)
            .is_some_and(|next| next.is_ascii_lowercase());
        let is_upper = char.is_ascii_uppercase();
        if is_upper && (previous_is_lower || (index > 0 && next_is_lower && char != 'S')) {
            output.push(' ');
        }
        output.push(char);
    }

    output
}

pub fn abspath_or_url(relpath: impl AsRef<Path>, must_exist: bool) -> Result<PathOrUrl, MiscError> {
    let relpath = relpath.as_ref();
    let relpath_str = relpath.to_string_lossy();
    if is_url_with_netloc(&relpath_str) {
        return Ok(PathOrUrl::Url(relpath_str.into_owned()));
    }

    let expanded = expand_user(&relpath_str);
    let absolute = absolutize(Path::new(&expanded))?;
    if must_exist && !absolute.exists() {
        return Err(MiscError::MissingPath(absolute));
    }

    Ok(PathOrUrl::Path(absolute))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathOrUrl {
    Path(PathBuf),
    Url(String),
}

pub fn ensure_n_tuple<T: Clone>(
    value: impl IntoIterator<Item = T>,
    n: usize,
    fill: T,
    before: bool,
) -> Result<Vec<T>, MiscError> {
    if n == 0 {
        return Err(MiscError::EmptyTupleLength);
    }

    let tuple_value: Vec<T> = value.into_iter().collect();
    if before {
        let mut result = Vec::with_capacity(n);
        let retained = tuple_value.len().min(n);
        result.extend(std::iter::repeat_n(fill, n - retained));
        result.extend(tuple_value);
        if result.len() > n {
            result.drain(0..result.len() - n);
        }
        Ok(result)
    } else {
        let mut result: Vec<T> = tuple_value.into_iter().take(n).collect();
        result.extend(std::iter::repeat_n(fill, n - result.len()));
        Ok(result)
    }
}

pub fn argsort(values: &[usize]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_by_key(|&index| values[index]);
    indices
}

pub fn reorder_after_dim_reduction(order: &[usize]) -> Vec<usize> {
    argsort(&argsort(order))
}

fn is_url_with_netloc(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    !scheme.is_empty() && !rest.is_empty()
}

fn expand_user(path: &str) -> String {
    if (path == "~" || path.starts_with("~/"))
        && let Some(home) = env::var_os("HOME")
    {
        let mut expanded = PathBuf::from(home);
        if path.len() > 2 {
            expanded.push(&path[2..]);
        }
        return expanded.to_string_lossy().into_owned();
    }
    path.to_owned()
}

fn absolutize(path: &Path) -> Result<PathBuf, MiscError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()
            .map_err(|_| MiscError::InvalidPathType)?
            .join(path))
    }
}
