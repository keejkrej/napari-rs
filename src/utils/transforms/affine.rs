use crate::utils::transforms::scale_translate::ScaleTranslate;
use crate::utils::transforms::transform_utils::{
    Matrix, RotationInput, ShearInput, compose_linear_matrix, decompose_linear_matrix,
    embed_in_identity_matrix, identity, infer_ndim, invert_matrix, is_diagonal,
    is_matrix_upper_triangular, mat_mul, mat_vec_mul, rotate_to_matrix, scale_to_vector,
    shear_to_matrix, translate_to_vector,
};
use crate::utils::transforms::units::{
    Quantity, Unit, UnitsError, default_axis_labels, pixel_units,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Transform {
    ScaleTranslate(ScaleTranslate),
    Affine(Affine),
    CompositeAffine(CompositeAffine),
}

impl Transform {
    pub fn compose(&self, transform: &Self) -> TransformChain {
        TransformChain::new(vec![self.clone(), transform.clone()])
    }

    pub fn transform_point(&self, coords: &[f64]) -> Vec<f64> {
        match self {
            Self::ScaleTranslate(transform) => transform.transform_point(coords),
            Self::Affine(transform) => transform.transform_point(coords),
            Self::CompositeAffine(transform) => transform.transform_point(coords),
        }
    }

    pub fn transform_points(&self, coords: &[Vec<f64>]) -> Vec<Vec<f64>> {
        coords
            .iter()
            .map(|coord| self.transform_point(coord))
            .collect()
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::ScaleTranslate(transform) => Self::ScaleTranslate(transform.inverse()),
            Self::Affine(transform) => Self::Affine(transform.inverse()),
            Self::CompositeAffine(transform) => Self::Affine(transform.inverse()),
        }
    }

    pub fn set_slice(&self, axes: &[usize]) -> Self {
        match self {
            Self::ScaleTranslate(transform) => Self::ScaleTranslate(transform.set_slice(axes)),
            Self::Affine(transform) => Self::Affine(transform.set_slice(axes)),
            Self::CompositeAffine(transform) => Self::CompositeAffine(transform.set_slice(axes)),
        }
    }

    pub fn expand_dims(&self, axes: &[usize]) -> Self {
        match self {
            Self::ScaleTranslate(transform) => Self::ScaleTranslate(transform.expand_dims(axes)),
            Self::Affine(transform) => Self::Affine(transform.expand_dims(axes)),
            Self::CompositeAffine(transform) => Self::CompositeAffine(transform.expand_dims(axes)),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        match self {
            Self::ScaleTranslate(_) => true,
            Self::Affine(transform) => transform.is_diagonal(),
            Self::CompositeAffine(transform) => transform.is_diagonal(),
        }
    }

    pub fn to_affine(&self) -> Affine {
        match self {
            Self::ScaleTranslate(transform) => {
                let ndim = transform.scale.len().max(transform.translate.len());
                let scale = scale_to_vector(Some(&transform.scale), ndim);
                let translate = translate_to_vector(Some(&transform.translate), ndim);
                let linear_matrix = diagonal_matrix(&scale);
                Affine::from_linear_matrix(linear_matrix, translate, transform.name.clone())
            }
            Self::Affine(transform) => transform.clone(),
            Self::CompositeAffine(transform) => transform.base.clone(),
        }
    }

    pub fn ndim(&self) -> usize {
        match self {
            Self::ScaleTranslate(transform) => transform.scale.len().max(transform.translate.len()),
            Self::Affine(transform) => transform.ndim(),
            Self::CompositeAffine(transform) => transform.ndim(),
        }
    }
}

impl From<ScaleTranslate> for Transform {
    fn from(value: ScaleTranslate) -> Self {
        Self::ScaleTranslate(value)
    }
}

impl From<Affine> for Transform {
    fn from(value: Affine) -> Self {
        Self::Affine(value)
    }
}

impl From<CompositeAffine> for Transform {
    fn from(value: CompositeAffine) -> Self {
        Self::CompositeAffine(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Affine {
    pub linear_matrix: Matrix,
    pub translate: Vec<f64>,
    pub name: Option<String>,
    pub upper_triangular: bool,
    pub axis_labels: Vec<String>,
    pub units: Vec<Unit>,
}

impl Affine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scale: impl Into<Vec<f64>>,
        translate: impl Into<Vec<f64>>,
        affine_matrix: Option<Matrix>,
        linear_matrix: Option<Matrix>,
        rotate: Option<RotationInput>,
        shear: Option<ShearInput>,
        ndim: Option<usize>,
        name: Option<String>,
    ) -> Self {
        let scale = {
            let scale = scale.into();
            if scale.is_empty() {
                vec![1.0; ndim.unwrap_or(2)]
            } else {
                scale
            }
        };
        let translate = {
            let translate = translate.into();
            if translate.is_empty() {
                vec![0.0; ndim.unwrap_or(2)]
            } else {
                translate
            }
        };

        let mut inferred_ndim = ndim.unwrap_or_else(|| {
            infer_ndim(
                Some(&scale),
                Some(&translate),
                rotate.clone(),
                shear.clone(),
            )
            .unwrap_or(0)
        });
        if inferred_ndim == 0 {
            inferred_ndim = 2;
        }

        if let Some(affine) = affine_matrix {
            return Self::from_affine_matrix_with_ndim(affine, inferred_ndim, name);
        }
        if let Some(linear) = linear_matrix {
            let linear_matrix = embed_in_identity_matrix(&linear, inferred_ndim).unwrap();
            let translate = translate_to_vector(Some(&translate), inferred_ndim);
            let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
            return Self {
                linear_matrix,
                translate,
                upper_triangular,
                name,
                axis_labels: default_axis_labels(inferred_ndim),
                units: pixel_units(inferred_ndim),
            };
        }

        let rotate = rotate.unwrap_or_else(|| RotationInput::Matrix(identity(inferred_ndim)));
        let shear_matrix = shear_to_matrix(shear, inferred_ndim).unwrap();
        let upper_triangular = is_matrix_upper_triangular(&shear_matrix);
        let scale = scale_to_vector(Some(&scale), inferred_ndim);
        let linear_matrix =
            compose_linear_matrix(rotate, &scale, ShearInput::Matrix(shear_matrix)).unwrap();
        let translate = translate_to_vector(Some(&translate), inferred_ndim);
        Self {
            linear_matrix,
            translate,
            name,
            upper_triangular,
            axis_labels: default_axis_labels(inferred_ndim),
            units: pixel_units(inferred_ndim),
        }
    }

    pub fn from_linear_matrix(
        linear_matrix: impl Into<Matrix>,
        translate: impl Into<Vec<f64>>,
        name: Option<String>,
    ) -> Self {
        let linear_matrix = linear_matrix.into();
        let ndim = linear_matrix.len();
        let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
        Self {
            linear_matrix,
            translate: translate_to_vector(Some(&translate.into()), ndim),
            name,
            upper_triangular,
            axis_labels: default_axis_labels(ndim),
            units: pixel_units(ndim),
        }
    }

    pub fn from_affine_matrix(affine_matrix: Matrix, name: Option<String>) -> Self {
        Self::from_affine_matrix_with_ndim(affine_matrix, 0, name)
    }

    pub fn from_affine_matrix_with_ndim(
        affine_matrix: Matrix,
        ndim: usize,
        name: Option<String>,
    ) -> Self {
        let affine_ndim = affine_matrix.len();
        let matrix_ndim = affine_ndim - 1;
        let output_ndim = ndim.max(matrix_ndim);
        let linear_matrix = affine_matrix[..matrix_ndim]
            .iter()
            .map(|row| row[..matrix_ndim].to_vec())
            .collect::<Matrix>();
        let linear_matrix = embed_in_identity_matrix(&linear_matrix, output_ndim).unwrap();
        let translate = affine_matrix[..matrix_ndim]
            .iter()
            .map(|row| row[matrix_ndim])
            .collect::<Vec<_>>();
        let translate = translate_to_vector(Some(&translate), output_ndim);
        let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
        Self {
            linear_matrix,
            translate,
            name,
            upper_triangular,
            axis_labels: default_axis_labels(output_ndim),
            units: pixel_units(output_ndim),
        }
    }

    pub fn decompose_linear_matrix(&self) -> (Matrix, Vec<f64>, ShearInput) {
        decompose_linear_matrix(&self.linear_matrix, self.upper_triangular).unwrap()
    }

    pub fn scale(&self) -> Vec<f64> {
        if self.is_diagonal() {
            return self
                .linear_matrix
                .iter()
                .enumerate()
                .map(|(index, row)| row[index])
                .collect();
        }
        let (_, scale, _) = self.decompose_linear_matrix();
        scale
    }

    pub fn rotate(&self) -> Matrix {
        if self.is_diagonal() {
            return identity(self.ndim());
        }
        let (rotate, _, _) = self.decompose_linear_matrix();
        rotate
    }

    pub fn shear(&self) -> ShearInput {
        if self.is_diagonal() {
            return ShearInput::Vector(vec![0.0; self.ndim().saturating_sub(1)]);
        }
        let (_, _, shear) = self.decompose_linear_matrix();
        shear
    }

    pub fn set_scale(&mut self, scale: impl Into<Vec<f64>>) {
        let scale = scale_to_vector(Some(&scale.into()), self.ndim());
        if self.is_diagonal() {
            for (index, value) in scale.iter().enumerate() {
                self.linear_matrix[index][index] = *value;
            }
            return;
        }

        let (rotate, _, shear) = self.decompose_linear_matrix();
        self.linear_matrix =
            compose_linear_matrix(RotationInput::Matrix(rotate), &scale, shear).unwrap();
    }

    pub fn set_rotate(&mut self, rotate: RotationInput) {
        let rotate = rotate_to_matrix(Some(rotate), self.ndim());
        let (_, scale, shear) = self.decompose_linear_matrix();
        self.linear_matrix =
            compose_linear_matrix(RotationInput::Matrix(rotate), &scale, shear).unwrap();
    }

    pub fn set_shear(&mut self, shear: ShearInput) {
        let shear_matrix = shear_to_matrix(Some(shear), self.ndim()).unwrap();
        self.upper_triangular = is_matrix_upper_triangular(&shear_matrix);
        let rotate = self.rotate();
        let scale = self.scale();
        self.linear_matrix = compose_linear_matrix(
            RotationInput::Matrix(rotate),
            &scale,
            ShearInput::Matrix(shear_matrix),
        )
        .unwrap();
    }

    pub fn ndim(&self) -> usize {
        self.linear_matrix.len()
    }

    pub fn set_axis_labels(
        &mut self,
        axis_labels: impl Into<Vec<String>>,
    ) -> Result<(), AffineMetadataError> {
        let axis_labels = axis_labels.into();
        if axis_labels.len() != self.ndim() {
            return Err(AffineMetadataError::LengthMismatch {
                field: "axis_labels",
                expected: self.ndim(),
                actual: axis_labels.len(),
            });
        }
        self.axis_labels = axis_labels;
        Ok(())
    }

    pub fn set_units(&mut self, units: impl Into<Vec<Unit>>) -> Result<(), AffineMetadataError> {
        let units = units.into();
        if units.len() != self.ndim() {
            return Err(AffineMetadataError::LengthMismatch {
                field: "units",
                expected: self.ndim(),
                actual: units.len(),
            });
        }
        self.units = units;
        Ok(())
    }

    pub fn set_units_from_names(&mut self, units: &[&str]) -> Result<(), AffineMetadataError> {
        let units = units
            .iter()
            .map(|unit| crate::utils::transforms::units::get_unit_from_name(Some(unit)))
            .collect::<Result<Vec<_>, UnitsError>>()?;
        self.set_units(units)
    }

    pub fn physical_scale(&self) -> Vec<Quantity> {
        self.scale()
            .into_iter()
            .zip(self.units.iter().cloned())
            .map(|(magnitude, unit)| Quantity { magnitude, unit })
            .collect()
    }

    pub fn transform_point(&self, coords: &[f64]) -> Vec<f64> {
        let matrix = if coords.len() == self.linear_matrix.len() {
            self.linear_matrix.clone()
        } else if coords.len() > self.linear_matrix.len() {
            embed_in_identity_matrix(&self.linear_matrix, coords.len()).unwrap()
        } else {
            panic!("coordinates have fewer dimensions than affine matrix");
        };
        let translate = translate_to_vector(Some(&self.translate), matrix.len());
        mat_vec_mul(&matrix, coords)
            .into_iter()
            .zip(translate)
            .map(|(value, shift)| value + shift)
            .collect()
    }

    pub fn transform_points(&self, coords: &[Vec<f64>]) -> Vec<Vec<f64>> {
        coords
            .iter()
            .map(|coord| self.transform_point(coord))
            .collect()
    }

    pub fn affine_matrix(&self) -> Matrix {
        let ndim = self.linear_matrix.len();
        let mut matrix = identity(ndim + 1);
        for (index, row) in matrix.iter_mut().enumerate().take(ndim) {
            row[..ndim].copy_from_slice(&self.linear_matrix[index]);
            row[ndim] = self.translate[index];
        }
        matrix
    }

    pub fn inverse(&self) -> Self {
        let affine_matrix = invert_matrix(&self.affine_matrix()).unwrap();
        let mut inverse = Self::from_affine_matrix(affine_matrix, self.name.clone());
        inverse.units = self.units.clone();
        inverse.axis_labels = self.axis_labels.clone();
        inverse
    }

    pub fn compose(&self, transform: &Self) -> Self {
        Self::from_affine_matrix(
            mat_mul(&self.affine_matrix(), &transform.affine_matrix()),
            self.name.clone(),
        )
    }

    pub fn set_slice(&self, axes: &[usize]) -> Self {
        let linear_matrix = if self.is_diagonal() {
            let scale = self
                .linear_matrix
                .iter()
                .enumerate()
                .map(|(axis, row)| row[axis])
                .collect::<Vec<_>>();
            diagonal_matrix(&axes.iter().map(|&axis| scale[axis]).collect::<Vec<_>>())
        } else {
            select_matrix_rows_cols(&self.linear_matrix, axes)
        };
        let translate = axes.iter().map(|&axis| self.translate[axis]).collect();
        let axis_labels = axes
            .iter()
            .map(|&axis| self.axis_labels[axis].clone())
            .collect();
        let units = axes.iter().map(|&axis| self.units[axis].clone()).collect();
        let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
        Self {
            linear_matrix,
            translate,
            name: self.name.clone(),
            upper_triangular,
            axis_labels,
            units,
        }
    }

    pub fn replace_slice(&self, axes: &[usize], transform: &Self) -> Self {
        if axes.len() != transform.ndim() {
            panic!("dimensionality of provided axes list and transform differ");
        }
        let mut linear_matrix = self.linear_matrix.clone();
        set_matrix_rows_cols(&mut linear_matrix, axes, &transform.linear_matrix);
        let mut translate = self.translate.clone();
        for (index, &axis) in axes.iter().enumerate() {
            translate[axis] = transform.translate[index];
        }
        let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
        Self {
            linear_matrix,
            translate,
            name: self.name.clone(),
            upper_triangular,
            axis_labels: default_axis_labels(self.ndim()),
            units: pixel_units(self.ndim()),
        }
    }

    pub fn expand_dims(&self, axes: &[usize]) -> Self {
        let ndim = self.ndim() + axes.len();
        let not_axes = missing_indices(ndim, axes);
        let mut linear_matrix = identity(ndim);
        set_matrix_rows_cols(&mut linear_matrix, &not_axes, &self.linear_matrix);
        let mut translate = vec![0.0; ndim];
        for (&axis, value) in not_axes.iter().zip(&self.translate) {
            translate[axis] = *value;
        }
        let upper_triangular = is_matrix_upper_triangular(&linear_matrix);
        Self {
            linear_matrix,
            translate,
            name: self.name.clone(),
            upper_triangular,
            axis_labels: default_axis_labels(ndim),
            units: pixel_units(ndim),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        is_diagonal(&self.linear_matrix, 1e-8).unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffineMetadataError {
    LengthMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    Units(UnitsError),
}

impl std::fmt::Display for AffineMetadataError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "{field} must have length ndim={expected}, got {actual}"
            ),
            Self::Units(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for AffineMetadataError {}

impl From<UnitsError> for AffineMetadataError {
    fn from(value: UnitsError) -> Self {
        Self::Units(value)
    }
}

impl Default for Affine {
    fn default() -> Self {
        Self::from_linear_matrix(diagonal_matrix(&[1.0, 1.0]), vec![0.0, 0.0], None)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeAffine {
    pub scale: Vec<f64>,
    pub rotate: Matrix,
    pub shear: Matrix,
    pub base: Affine,
}

impl CompositeAffine {
    pub fn new(
        scale: impl Into<Vec<f64>>,
        translate: impl Into<Vec<f64>>,
        rotate: Option<RotationInput>,
        shear: Option<ShearInput>,
        ndim: Option<usize>,
        name: Option<String>,
    ) -> Self {
        let scale = {
            let scale = scale.into();
            if scale.is_empty() {
                vec![1.0; ndim.unwrap_or(2)]
            } else {
                scale
            }
        };
        let translate = translate.into();
        let mut inferred_ndim = ndim.unwrap_or_else(|| {
            infer_ndim(
                Some(&scale),
                Some(&translate),
                rotate.clone(),
                shear.clone(),
            )
            .unwrap_or(0)
        });
        if inferred_ndim == 0 {
            inferred_ndim = 2;
        }

        let scale = scale_to_vector(Some(&scale), inferred_ndim);
        let rotate_input = rotate.unwrap_or_else(|| RotationInput::Matrix(identity(inferred_ndim)));
        let rotate = rotate_to_matrix(Some(rotate_input.clone()), inferred_ndim);
        let shear = shear_to_matrix(shear, inferred_ndim).unwrap();
        let linear_matrix =
            compose_linear_matrix(rotate_input, &scale, ShearInput::Matrix(shear.clone())).unwrap();
        let translate = translate_to_vector(Some(&translate), inferred_ndim);
        let base = Affine::from_linear_matrix(linear_matrix, translate, name);
        Self {
            scale,
            rotate,
            shear,
            base,
        }
    }

    pub fn ndim(&self) -> usize {
        self.base.ndim()
    }

    pub fn set_axis_labels(
        &mut self,
        axis_labels: impl Into<Vec<String>>,
    ) -> Result<(), AffineMetadataError> {
        self.base.set_axis_labels(axis_labels)
    }

    pub fn set_units(&mut self, units: impl Into<Vec<Unit>>) -> Result<(), AffineMetadataError> {
        self.base.set_units(units)
    }

    pub fn set_units_from_names(&mut self, units: &[&str]) -> Result<(), AffineMetadataError> {
        self.base.set_units_from_names(units)
    }

    pub fn physical_scale(&self) -> Vec<Quantity> {
        self.scale
            .iter()
            .copied()
            .zip(self.base.units.iter().cloned())
            .map(|(magnitude, unit)| Quantity { magnitude, unit })
            .collect()
    }

    pub fn transform_point(&self, coords: &[f64]) -> Vec<f64> {
        self.base.transform_point(coords)
    }

    pub fn transform_points(&self, coords: &[Vec<f64>]) -> Vec<Vec<f64>> {
        self.base.transform_points(coords)
    }

    pub fn set_scale(&mut self, scale: impl Into<Vec<f64>>) {
        self.scale = scale_to_vector(Some(&scale.into()), self.ndim());
        self.recompute_linear_matrix();
    }

    pub fn set_rotate(&mut self, rotate: RotationInput) {
        self.rotate = rotate_to_matrix(Some(rotate), self.ndim());
        self.recompute_linear_matrix();
    }

    pub fn set_shear(&mut self, shear: ShearInput) {
        self.shear = shear_to_matrix(Some(shear), self.ndim()).unwrap();
        self.recompute_linear_matrix();
    }

    fn recompute_linear_matrix(&mut self) {
        let rotated_shear = mat_mul(&self.rotate, &self.shear);
        self.base.linear_matrix = mat_mul(&rotated_shear, &diagonal_matrix(&self.scale));
    }

    pub fn set_slice(&self, axes: &[usize]) -> Self {
        let scale = select_vector(&self.scale, axes);
        let rotate = select_matrix_rows_cols(&self.rotate, axes);
        let shear = select_matrix_rows_cols(&self.shear, axes);
        let mut base = Affine::from_linear_matrix(
            rotate_shear_scale_matrix(&rotate, &shear, &scale),
            select_vector(&self.base.translate, axes),
            self.base.name.clone(),
        );
        base.axis_labels = axes
            .iter()
            .map(|&axis| self.base.axis_labels[axis].clone())
            .collect();
        base.units = axes
            .iter()
            .map(|&axis| self.base.units[axis].clone())
            .collect();
        Self {
            scale: scale.clone(),
            rotate: rotate.clone(),
            shear: shear.clone(),
            base,
        }
    }

    pub fn expand_dims(&self, axes: &[usize]) -> Self {
        let ndim = self.ndim() + axes.len();
        let not_axes = missing_indices(ndim, axes);
        let mut scale = vec![1.0; ndim];
        for (&axis, value) in not_axes.iter().zip(&self.scale) {
            scale[axis] = *value;
        }
        let mut rotate = identity(ndim);
        set_matrix_rows_cols(&mut rotate, &not_axes, &self.rotate);
        let mut shear = identity(ndim);
        set_matrix_rows_cols(&mut shear, &not_axes, &self.shear);
        let mut translate = vec![0.0; ndim];
        for (&axis, value) in not_axes.iter().zip(&self.base.translate) {
            translate[axis] = *value;
        }
        let linear_matrix = rotate_shear_scale_matrix(&rotate, &shear, &scale);
        Self {
            scale,
            rotate: rotate.clone(),
            shear: shear.clone(),
            base: Affine::from_linear_matrix(linear_matrix, translate, self.base.name.clone()),
        }
    }

    pub fn inverse(&self) -> Affine {
        self.base.inverse()
    }

    pub fn is_diagonal(&self) -> bool {
        self.base.is_diagonal()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformChain {
    pub transforms: Vec<Transform>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransformChainError {
    EmptyChain,
}

impl TransformChain {
    pub fn new(transforms: Vec<Transform>) -> Self {
        Self {
            transforms,
            name: None,
        }
    }

    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    pub fn transform_point(&self, coords: &[f64]) -> Vec<f64> {
        self.transforms
            .iter()
            .fold(coords.to_vec(), |current, transform| {
                transform.transform_point(&current)
            })
    }

    pub fn transform_points(&self, coords: &[Vec<f64>]) -> Vec<Vec<f64>> {
        coords
            .iter()
            .map(|coord| self.transform_point(coord))
            .collect()
    }

    pub fn inverse(&self) -> Self {
        Self::new(
            self.transforms
                .iter()
                .rev()
                .map(|transform| transform.inverse())
                .collect(),
        )
    }

    pub fn set_slice(&self, axes: &[usize]) -> Self {
        Self::new(
            self.transforms
                .iter()
                .map(|transform| transform.set_slice(axes))
                .collect(),
        )
    }

    pub fn expand_dims(&self, axes: &[usize]) -> Self {
        Self::new(
            self.transforms
                .iter()
                .map(|transform| transform.expand_dims(axes))
                .collect(),
        )
    }

    pub fn simplified(&self) -> Result<Transform, TransformChainError> {
        if self.transforms.is_empty() {
            return Err(TransformChainError::EmptyChain);
        }
        let mut iter = self.transforms.iter();
        let mut result = iter.next().unwrap().to_affine();
        for transform in iter {
            result = transform.to_affine().compose(&result);
        }
        Ok(Transform::Affine(result))
    }

    pub fn is_diagonal(&self) -> bool {
        if self
            .transforms
            .iter()
            .all(|transform| transform.is_diagonal())
        {
            return true;
        }
        self.simplified()
            .is_ok_and(|transform| transform.is_diagonal())
    }

    pub fn compose(&self, transform: Transform) -> Self {
        let mut transforms = self.transforms.clone();
        transforms.push(transform);
        Self {
            transforms,
            name: self.name.clone(),
        }
    }

    pub fn push(&mut self, transform: Transform) {
        self.transforms.push(transform);
    }
}

fn diagonal_matrix(values: &[f64]) -> Matrix {
    let mut matrix = identity(values.len());
    for (index, value) in values.iter().enumerate() {
        matrix[index][index] = *value;
    }
    matrix
}

fn missing_indices(ndim: usize, axes: &[usize]) -> Vec<usize> {
    (0..ndim).filter(|axis| !axes.contains(axis)).collect()
}

fn select_matrix_rows_cols(matrix: &Matrix, axes: &[usize]) -> Matrix {
    axes.iter()
        .map(|&row| axes.iter().map(|&col| matrix[row][col]).collect())
        .collect()
}

fn set_matrix_rows_cols(target: &mut Matrix, axes: &[usize], source: &Matrix) {
    for (target_row_index, source_row) in axes.iter().copied().zip(source.iter()) {
        for (target_col_index, value) in axes.iter().copied().zip(source_row.iter()) {
            target[target_row_index][target_col_index] = *value;
        }
    }
}

fn select_vector(values: &[f64], axes: &[usize]) -> Vec<f64> {
    axes.iter().map(|&axis| values[axis]).collect()
}

fn rotate_shear_scale_matrix(rotate: &Matrix, shear: &Matrix, scale: &[f64]) -> Matrix {
    let rotated = mat_mul(rotate, shear);
    mat_mul(&rotated, &diagonal_matrix(scale))
}
