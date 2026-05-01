use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    Bool,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float16,
    Float32,
    Float64,
    Complex64,
    Complex128,
}

impl DType {
    pub fn is_integer(self) -> bool {
        matches!(
            self,
            Self::UInt8
                | Self::UInt16
                | Self::UInt32
                | Self::UInt64
                | Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
        )
    }

    pub fn is_floating(self) -> bool {
        matches!(self, Self::Float16 | Self::Float32 | Self::Float64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DTypeError {
    Unrecognized(String),
    NonNumeric(DType),
}

impl fmt::Display for DTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unrecognized(dtype) => write!(formatter, "unrecognized dtype: {dtype}"),
            Self::NonNumeric(dtype) => {
                write!(formatter, "unrecognized or non-numeric dtype: {dtype:?}")
            }
        }
    }
}

impl Error for DTypeError {}

pub fn normalize_dtype(dtype_spec: &str) -> Result<DType, DTypeError> {
    let normalized = dtype_spec.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "bool" | "bool_" | "numpy.bool" | "numpy.bool_" => Ok(DType::Bool),
        "uint8" | "u1" | "<u1" | ">u1" | "|u1" | "numpy.uint8" => Ok(DType::UInt8),
        "uint16" | "u2" | "<u2" | ">u2" | "numpy.uint16" => Ok(DType::UInt16),
        "uint32" | "u4" | "<u4" | ">u4" | "numpy.uint32" => Ok(DType::UInt32),
        "uint64" | "u8" | "<u8" | ">u8" | "numpy.uint64" => Ok(DType::UInt64),
        "int" | "int64" | "i8" | "<i8" | ">i8" | "numpy.int64" => Ok(DType::Int64),
        "int8" | "i1" | "<i1" | ">i1" | "|i1" | "numpy.int8" => Ok(DType::Int8),
        "int16" | "i2" | "<i2" | ">i2" | "numpy.int16" => Ok(DType::Int16),
        "int32" | "i4" | "<i4" | ">i4" | "numpy.int32" => Ok(DType::Int32),
        "float" | "float64" | "f8" | "<f8" | ">f8" | "numpy.float64" => Ok(DType::Float64),
        "float16" | "f2" | "<f2" | ">f2" | "numpy.float16" => Ok(DType::Float16),
        "float32" | "f4" | "<f4" | ">f4" | "numpy.float32" => Ok(DType::Float32),
        "complex64" | "c8" | "<c8" | ">c8" | "numpy.complex64" => Ok(DType::Complex64),
        "complex128" | "c16" | "<c16" | ">c16" | "numpy.complex128" => Ok(DType::Complex128),
        _ if normalized.contains("uint") => normalize_str_by_bit_depth(&normalized, "uint"),
        _ if normalized.contains("int") => normalize_str_by_bit_depth(&normalized, "int"),
        _ if normalized.contains("float") => normalize_str_by_bit_depth(&normalized, "float"),
        _ if normalized.contains("complex") => normalize_str_by_bit_depth(&normalized, "complex"),
        _ if normalized.contains("bool") => Ok(DType::Bool),
        _ => Err(DTypeError::Unrecognized(dtype_spec.to_owned())),
    }
}

pub fn get_dtype_limits(dtype_spec: &str) -> Result<(f64, f64), DTypeError> {
    let dtype = normalize_dtype(dtype_spec)?;
    match dtype {
        DType::UInt8 => Ok((u8::MIN as f64, u8::MAX as f64)),
        DType::UInt16 => Ok((u16::MIN as f64, u16::MAX as f64)),
        DType::UInt32 => Ok((u32::MIN as f64, u32::MAX as f64)),
        DType::UInt64 => Ok((u64::MIN as f64, u64::MAX as f64)),
        DType::Int8 => Ok((i8::MIN as f64, i8::MAX as f64)),
        DType::Int16 => Ok((i16::MIN as f64, i16::MAX as f64)),
        DType::Int32 => Ok((i32::MIN as f64, i32::MAX as f64)),
        DType::Int64 => Ok((i64::MIN as f64, i64::MAX as f64)),
        DType::Float16 => Ok((-65504.0, 65504.0)),
        DType::Float32 => Ok((f32::MIN as f64, f32::MAX as f64)),
        DType::Float64 => Ok((f64::MIN, f64::MAX)),
        DType::Bool | DType::Complex64 | DType::Complex128 => Err(DTypeError::NonNumeric(dtype)),
    }
}

pub const VISPY_TEXTURE_DTYPE: DType = DType::Float32;

fn normalize_str_by_bit_depth(dtype_str: &str, kind: &str) -> Result<DType, DTypeError> {
    if !dtype_str.chars().any(|char| char.is_ascii_digit()) {
        return match kind {
            "int" => Ok(DType::Int64),
            "float" => Ok(DType::Float64),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }
    if dtype_str.contains("128") {
        return match kind {
            "complex" => Ok(DType::Complex128),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }
    if dtype_str.contains('8') {
        return match kind {
            "uint" => Ok(DType::UInt8),
            "int" => Ok(DType::Int8),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }
    if dtype_str.contains("16") {
        return match kind {
            "uint" => Ok(DType::UInt16),
            "int" => Ok(DType::Int16),
            "float" => Ok(DType::Float16),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }
    if dtype_str.contains("32") {
        return match kind {
            "uint" => Ok(DType::UInt32),
            "int" => Ok(DType::Int32),
            "float" => Ok(DType::Float32),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }
    if dtype_str.contains("64") {
        return match kind {
            "uint" => Ok(DType::UInt64),
            "int" => Ok(DType::Int64),
            "float" => Ok(DType::Float64),
            "complex" => Ok(DType::Complex64),
            _ => Err(DTypeError::Unrecognized(dtype_str.to_owned())),
        };
    }

    Err(DTypeError::Unrecognized(dtype_str.to_owned()))
}
