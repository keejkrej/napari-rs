pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TransformUtilsError {
    InvalidMatrix,
    InvalidShear,
    SingularMatrix,
    StrangeShearElementCount(usize),
}

impl std::fmt::Display for TransformUtilsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMatrix => formatter.write_str("improper transform matrix"),
            Self::InvalidShear => formatter.write_str("invalid shear"),
            Self::SingularMatrix => formatter.write_str("matrix is singular"),
            Self::StrangeShearElementCount(count) => {
                write!(formatter, "{count} is a strange number of shear elements")
            }
        }
    }
}

impl std::error::Error for TransformUtilsError {}

#[derive(Debug, Clone, PartialEq)]
pub enum RotationInput {
    Angle2D(f64),
    Angles3D([f64; 3]),
    Matrix(Matrix),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShearInput {
    Vector(Vec<f64>),
    Matrix(Matrix),
}

pub fn translate_to_vector(translate: Option<&[f64]>, ndim: usize) -> Vec<f64> {
    let mut vector = vec![0.0; ndim];
    if let Some(translate) = translate {
        let retained = translate.len().min(ndim);
        let start = ndim - retained;
        let source_start = translate.len() - retained;
        for (index, &value) in translate[source_start..].iter().enumerate() {
            vector[start + index] = value;
        }
    }
    vector
}

pub fn scale_to_vector(scale: Option<&[f64]>, ndim: usize) -> Vec<f64> {
    let mut vector = vec![1.0; ndim];
    if let Some(scale) = scale {
        let retained = scale.len().min(ndim);
        let start = ndim - retained;
        let source_start = scale.len() - retained;
        for (index, &value) in scale[source_start..].iter().enumerate() {
            vector[start + index] = value;
        }
    }
    vector
}

pub fn rotate_to_matrix(rotate: Option<RotationInput>, ndim: usize) -> Matrix {
    let mut full_rotate = identity(ndim);
    if let Some(rotate) = rotate {
        let rotate_matrix = make_rotate_matrix(rotate);
        embed_bottom_right(&mut full_rotate, &rotate_matrix);
    }
    full_rotate
}

pub fn shear_to_matrix(
    shear: Option<ShearInput>,
    ndim: usize,
) -> Result<Matrix, TransformUtilsError> {
    let mut full_shear = identity(ndim);
    if let Some(shear) = shear {
        let shear_matrix = make_shear_matrix(shear)?;
        embed_bottom_right(&mut full_shear, &shear_matrix);
    }
    Ok(full_shear)
}

pub fn compose_linear_matrix(
    rotate: RotationInput,
    scale: &[f64],
    shear: ShearInput,
) -> Result<Matrix, TransformUtilsError> {
    let rotate_matrix = make_rotate_matrix(rotate);
    let scale_matrix = diagonal(scale);
    let shear_matrix = make_shear_matrix(shear)?;
    let ndim = rotate_matrix
        .len()
        .max(scale_matrix.len())
        .max(shear_matrix.len());

    let full_rotate = embed_in_identity_matrix(&rotate_matrix, ndim)?;
    let full_scale = embed_in_identity_matrix(&scale_matrix, ndim)?;
    let full_shear = embed_in_identity_matrix(&shear_matrix, ndim)?;

    Ok(mat_mul(&mat_mul(&full_rotate, &full_shear), &full_scale))
}

pub fn decompose_linear_matrix(
    matrix: &Matrix,
    upper_triangular: bool,
) -> Result<(Matrix, Vec<f64>, ShearInput), TransformUtilsError> {
    validate_square(matrix)?;
    let n = matrix.len();
    let (mut rotate, mut tri) = if upper_triangular {
        qr(matrix)?
    } else {
        let transposed = transpose(matrix);
        let (upper_tri, rotate) = rq(&transposed)?;
        (transpose(&rotate), transpose(&upper_tri))
    };

    let mut scale_with_sign = vec![0.0; n];
    for index in 0..n {
        scale_with_sign[index] = tri[index][index];
    }
    let scale = scale_with_sign
        .iter()
        .map(|value| value.abs())
        .collect::<Vec<_>>();
    let mut normalize = vec![1.0; n];
    for index in 0..n {
        let denom = scale_with_sign[index];
        if denom != 0.0 {
            normalize[index] = scale[index] / denom;
        }
    }

    for ((tri_row, rotate_row), &row_normalize) in
        tri.iter_mut().zip(rotate.iter_mut()).zip(&normalize)
    {
        for value in tri_row.iter_mut().take(n) {
            *value *= row_normalize;
        }
        for value in rotate_row.iter_mut().take(n) {
            *value *= row_normalize;
        }
    }

    for tri_row in tri.iter_mut().take(n) {
        for (col, value) in tri_row.iter_mut().enumerate().take(n) {
            if scale[col] == 0.0 {
                *value = 0.0;
            } else {
                *value /= scale[col];
            }
        }
    }

    let shear = if upper_triangular {
        let mut shear = Vec::new();
        for (row, tri_row) in tri.iter().enumerate().take(n) {
            for &value in tri_row.iter().take(n).skip(row + 1) {
                shear.push(value);
            }
        }
        ShearInput::Vector(shear)
    } else {
        ShearInput::Matrix(tri)
    };

    Ok((rotate, scale, shear))
}

pub fn infer_ndim(
    scale: Option<&[f64]>,
    translate: Option<&[f64]>,
    rotate: Option<RotationInput>,
    shear: Option<ShearInput>,
) -> Result<usize, TransformUtilsError> {
    let mut ndim = 0;
    if let Some(scale) = scale {
        ndim = ndim.max(scale.len());
    }
    if let Some(translate) = translate {
        ndim = ndim.max(translate.len());
    }
    if let Some(rotate) = rotate {
        ndim = ndim.max(make_rotate_matrix(rotate).len());
    }
    if let Some(shear) = shear {
        ndim = ndim.max(make_shear_matrix(shear)?.len());
    }
    Ok(ndim)
}

pub fn expand_upper_triangular(vector: &[f64]) -> Result<Matrix, TransformUtilsError> {
    let n = vector.len();
    let size = (((8 * n + 1) as f64).sqrt() - 1.0) / 2.0 + 1.0;
    if size.floor() != size {
        return Err(TransformUtilsError::StrangeShearElementCount(n));
    }

    let size = size as usize;
    let mut matrix = identity(size);
    let mut values = vector.iter();
    for (row_index, row) in matrix.iter_mut().enumerate() {
        for value in row.iter_mut().skip(row_index + 1) {
            *value = *values.next().expect("validated triangular element count");
        }
    }
    Ok(matrix)
}

pub fn embed_in_identity_matrix(
    matrix: &Matrix,
    ndim: usize,
) -> Result<Matrix, TransformUtilsError> {
    validate_square(matrix)?;
    if matrix.len() > ndim {
        return Err(TransformUtilsError::InvalidMatrix);
    }
    if matrix.len() == ndim {
        return Ok(matrix.clone());
    }

    let mut full = identity(ndim);
    embed_bottom_right(&mut full, matrix);
    Ok(full)
}

pub fn shear_matrix_from_angle(angle: f64, ndim: usize, axes: (usize, usize)) -> Matrix {
    let mut matrix = identity(ndim);
    matrix[axes.0][axes.1] = (90.0 - angle).to_radians().tan();
    matrix
}

pub fn is_matrix_upper_triangular(matrix: &Matrix) -> bool {
    matrix.iter().enumerate().all(|(row, values)| {
        values
            .iter()
            .take(row)
            .all(|value| value.abs() <= f64::EPSILON)
    })
}

pub fn is_matrix_lower_triangular(matrix: &Matrix) -> bool {
    matrix.iter().enumerate().all(|(row, values)| {
        values
            .iter()
            .enumerate()
            .skip(row + 1)
            .all(|(_, value)| value.abs() <= f64::EPSILON)
    })
}

pub fn is_matrix_triangular(matrix: &Matrix) -> bool {
    is_matrix_upper_triangular(matrix) || is_matrix_lower_triangular(matrix)
}

pub fn is_diagonal(matrix: &Matrix, tolerance: f64) -> Result<bool, TransformUtilsError> {
    validate_square(matrix)?;
    let max_off_diagonal = matrix
        .iter()
        .enumerate()
        .flat_map(|(row, values)| {
            values
                .iter()
                .enumerate()
                .filter_map(move |(col, value)| (row != col).then_some(value.abs()))
        })
        .fold(0.0_f64, f64::max);
    if tolerance == 0.0 {
        Ok(max_off_diagonal == 0.0)
    } else {
        Ok(max_off_diagonal <= tolerance)
    }
}

pub fn identity(ndim: usize) -> Matrix {
    let mut matrix = vec![vec![0.0; ndim]; ndim];
    for (index, row) in matrix.iter_mut().enumerate() {
        row[index] = 1.0;
    }
    matrix
}

pub fn diagonal(values: &[f64]) -> Matrix {
    let mut matrix = vec![vec![0.0; values.len()]; values.len()];
    for (index, &value) in values.iter().enumerate() {
        matrix[index][index] = value;
    }
    matrix
}

pub fn mat_mul(left: &Matrix, right: &Matrix) -> Matrix {
    let mut result = vec![vec![0.0; right[0].len()]; left.len()];
    for row in 0..left.len() {
        for col in 0..right[0].len() {
            result[row][col] = (0..right.len())
                .map(|inner| left[row][inner] * right[inner][col])
                .sum();
        }
    }
    result
}

fn transpose(matrix: &Matrix) -> Matrix {
    if matrix.is_empty() {
        return vec![];
    }
    let mut output = vec![vec![0.0; matrix.len()]; matrix[0].len()];
    for (row, values) in matrix.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            output[col][row] = *value;
        }
    }
    output
}

fn qr(matrix: &Matrix) -> Result<(Matrix, Matrix), TransformUtilsError> {
    validate_square(matrix)?;
    let n = matrix.len();

    let mut q = identity(n);
    let mut r = vec![vec![0.0; n]; n];
    let columns: Vec<Vec<f64>> = (0..n)
        .map(|col| (0..n).map(|row| matrix[row][col]).collect())
        .collect();

    for j in 0..n {
        let mut v = columns[j].clone();
        for i in 0..j {
            let qi: Vec<f64> = (0..n).map(|row| q[row][i]).collect();
            let projection = dot(&qi, &v);
            r[i][j] = projection;
            for row in 0..n {
                v[row] -= projection * qi[row];
            }
        }

        let norm = l2_norm(&v);
        if norm.abs() < 1e-12 {
            r[j][j] = 0.0;
            continue;
        }
        r[j][j] = norm;
        for row in 0..n {
            q[row][j] = v[row] / norm;
        }
    }

    Ok((q, r))
}

fn rq(matrix: &Matrix) -> Result<(Matrix, Matrix), TransformUtilsError> {
    validate_square(matrix)?;
    let reversed = reverse_rows_cols(&transpose(matrix));
    let (q, r) = qr(&reversed)?;
    Ok((
        reverse_rows_cols(&transpose(&r)),
        reverse_rows_cols(&transpose(&q)),
    ))
}

fn reverse_rows_cols(matrix: &Matrix) -> Matrix {
    matrix
        .iter()
        .rev()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(left, right)| left * right)
        .sum()
}

fn l2_norm(vector: &[f64]) -> f64 {
    vector.iter().map(|value| value * value).sum::<f64>().sqrt()
}

pub fn mat_vec_mul(matrix: &Matrix, vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| row.iter().zip(vector.iter()).map(|(a, b)| a * b).sum())
        .collect()
}

pub fn invert_matrix(matrix: &Matrix) -> Result<Matrix, TransformUtilsError> {
    validate_square(matrix)?;
    let size = matrix.len();
    if size == 0 {
        return Ok(vec![]);
    }

    let mut workspace = vec![vec![0.0; size * 2]; size];
    for (row_index, row) in matrix.iter().enumerate() {
        workspace[row_index][..size].copy_from_slice(row);
        workspace[row_index][size + row_index] = 1.0;
    }

    for pivot in 0..size {
        let mut swap_index = pivot;
        while swap_index < size && workspace[swap_index][pivot].abs() < 1e-12 {
            swap_index += 1;
        }
        if swap_index == size {
            return Err(TransformUtilsError::SingularMatrix);
        }
        if swap_index != pivot {
            workspace.swap(pivot, swap_index);
        }

        let pivot_value = workspace[pivot][pivot];
        if pivot_value.abs() < 1e-12 {
            return Err(TransformUtilsError::SingularMatrix);
        }
        for value in workspace[pivot].iter_mut() {
            *value /= pivot_value;
        }

        for row_index in 0..size {
            if row_index == pivot {
                continue;
            }
            let factor = workspace[row_index][pivot];
            if factor == 0.0 {
                continue;
            }
            let pivot_values = workspace[pivot][pivot..(size * 2)].to_vec();
            for (value, pivot_value) in workspace[row_index][pivot..(size * 2)]
                .iter_mut()
                .zip(pivot_values)
            {
                *value -= factor * pivot_value;
            }
        }
    }

    let mut inverse = vec![vec![0.0; size]; size];
    for row in 0..size {
        inverse[row][..].copy_from_slice(&workspace[row][size..]);
    }
    Ok(inverse)
}

fn make_rotate_matrix(rotate: RotationInput) -> Matrix {
    match rotate {
        RotationInput::Angle2D(angle) => make_2d_rotation(angle),
        RotationInput::Angles3D([alpha, beta, gamma]) => make_3d_rotation(alpha, beta, gamma),
        RotationInput::Matrix(matrix) => matrix,
    }
}

fn make_2d_rotation(angle_degrees: f64) -> Matrix {
    let (sin_theta, cos_theta) = angle_degrees.to_radians().sin_cos();
    vec![vec![cos_theta, -sin_theta], vec![sin_theta, cos_theta]]
}

fn make_3d_rotation(alpha_degrees: f64, beta_degrees: f64, gamma_degrees: f64) -> Matrix {
    let (sin_alpha, cos_alpha) = alpha_degrees.to_radians().sin_cos();
    let r_alpha = vec![
        vec![cos_alpha, -sin_alpha, 0.0],
        vec![sin_alpha, cos_alpha, 0.0],
        vec![0.0, 0.0, 1.0],
    ];

    let (sin_beta, cos_beta) = beta_degrees.to_radians().sin_cos();
    let r_beta = vec![
        vec![cos_beta, 0.0, sin_beta],
        vec![0.0, 1.0, 0.0],
        vec![-sin_beta, 0.0, cos_beta],
    ];

    let (sin_gamma, cos_gamma) = gamma_degrees.to_radians().sin_cos();
    let r_gamma = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, cos_gamma, -sin_gamma],
        vec![0.0, sin_gamma, cos_gamma],
    ];

    mat_mul(&mat_mul(&r_alpha, &r_beta), &r_gamma)
}

fn make_shear_matrix(shear: ShearInput) -> Result<Matrix, TransformUtilsError> {
    match shear {
        ShearInput::Vector(vector) => expand_upper_triangular(&vector),
        ShearInput::Matrix(matrix) => {
            if !is_matrix_triangular(&matrix) {
                return Err(TransformUtilsError::InvalidShear);
            }
            Ok(matrix)
        }
    }
}

fn embed_bottom_right(full: &mut Matrix, matrix: &Matrix) {
    let row_offset = full.len() - matrix.len();
    let col_offset = full[0].len() - matrix[0].len();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            full[row_offset + row][col_offset + col] = matrix[row][col];
        }
    }
}

fn validate_square(matrix: &Matrix) -> Result<(), TransformUtilsError> {
    if matrix.is_empty() || matrix.iter().any(|row| row.len() != matrix.len()) {
        return Err(TransformUtilsError::InvalidMatrix);
    }
    Ok(())
}
