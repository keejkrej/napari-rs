use napari_rs::utils::dtype::{
    DType, DTypeError, VISPY_TEXTURE_DTYPE, get_dtype_limits, normalize_dtype,
};

#[test]
fn normalize_dtype_handles_numpy_style_names() {
    assert_eq!(normalize_dtype("uint8"), Ok(DType::UInt8));
    assert_eq!(normalize_dtype("uint16"), Ok(DType::UInt16));
    assert_eq!(normalize_dtype("uint32"), Ok(DType::UInt32));
    assert_eq!(normalize_dtype("uint64"), Ok(DType::UInt64));
    assert_eq!(normalize_dtype("int8"), Ok(DType::Int8));
    assert_eq!(normalize_dtype("int16"), Ok(DType::Int16));
    assert_eq!(normalize_dtype("int32"), Ok(DType::Int32));
    assert_eq!(normalize_dtype("int64"), Ok(DType::Int64));
    assert_eq!(normalize_dtype("float32"), Ok(DType::Float32));
    assert_eq!(normalize_dtype("float64"), Ok(DType::Float64));
    assert_eq!(normalize_dtype("complex64"), Ok(DType::Complex64));
    assert_eq!(normalize_dtype("complex128"), Ok(DType::Complex128));
    assert_eq!(normalize_dtype("bool"), Ok(DType::Bool));
}

#[test]
fn normalize_dtype_handles_pure_python_aliases_like_numpy() {
    assert_eq!(normalize_dtype("int"), Ok(DType::Int64));
    assert_eq!(normalize_dtype("float"), Ok(DType::Float64));
}

#[test]
fn normalize_dtype_handles_endian_and_short_dtype_codes() {
    assert_eq!(normalize_dtype(">f4"), Ok(DType::Float32));
    assert_eq!(normalize_dtype(">f8"), Ok(DType::Float64));

    for (dtype_str, dtype) in [
        ("<i1", DType::Int8),
        (">i1", DType::Int8),
        ("<i2", DType::Int16),
        (">i2", DType::Int16),
        ("<i4", DType::Int32),
        (">i4", DType::Int32),
        ("<i8", DType::Int64),
        (">i8", DType::Int64),
        ("<u1", DType::UInt8),
        (">u1", DType::UInt8),
        ("<u2", DType::UInt16),
        (">u2", DType::UInt16),
        ("<u4", DType::UInt32),
        (">u4", DType::UInt32),
        ("<u8", DType::UInt64),
        (">u8", DType::UInt64),
    ] {
        assert_eq!(normalize_dtype(dtype_str), Ok(dtype));
    }
}

#[test]
fn normalize_dtype_handles_duck_array_string_representations() {
    assert_eq!(normalize_dtype("torch.float32"), Ok(DType::Float32));
    assert_eq!(normalize_dtype("torch.uint8"), Ok(DType::UInt8));
    assert_eq!(normalize_dtype("tensorstore.dtype.int16"), Ok(DType::Int16));
    assert_eq!(normalize_dtype("numpy.complex128"), Ok(DType::Complex128));
    assert_eq!(normalize_dtype("numpy.bool_"), Ok(DType::Bool));
}

#[test]
fn get_dtype_limits_returns_integer_and_float_machine_limits() {
    assert_eq!(get_dtype_limits("uint8"), Ok((0.0, 255.0)));
    assert_eq!(get_dtype_limits("int8"), Ok((-128.0, 127.0)));
    assert_eq!(get_dtype_limits("uint16"), Ok((0.0, 65535.0)));
    assert_eq!(get_dtype_limits("int16"), Ok((-32768.0, 32767.0)));
    assert_eq!(get_dtype_limits("float16"), Ok((-65504.0, 65504.0)));
    assert_eq!(
        get_dtype_limits("float32"),
        Ok((f32::MIN as f64, f32::MAX as f64))
    );
    assert_eq!(get_dtype_limits("float64"), Ok((f64::MIN, f64::MAX)));
}

#[test]
fn get_dtype_limits_rejects_bool_and_complex_like_python_iinfo_finfo_path() {
    assert_eq!(
        get_dtype_limits("bool"),
        Err(DTypeError::NonNumeric(DType::Bool))
    );
    assert_eq!(
        get_dtype_limits("complex64"),
        Err(DTypeError::NonNumeric(DType::Complex64))
    );
}

#[test]
fn vispy_texture_dtype_matches_python_constant() {
    assert_eq!(VISPY_TEXTURE_DTYPE, DType::Float32);
}
