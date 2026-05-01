#[derive(Debug, Clone, PartialEq)]
pub struct ScaleTranslate {
    pub scale: Vec<f64>,
    pub translate: Vec<f64>,
    pub name: Option<String>,
}

impl Default for ScaleTranslate {
    fn default() -> Self {
        Self::new([1.0], [0.0], None)
    }
}

impl ScaleTranslate {
    pub fn new(
        scale: impl Into<Vec<f64>>,
        translate: impl Into<Vec<f64>>,
        name: Option<String>,
    ) -> Self {
        let mut scale = scale.into();
        let mut translate = translate.into();
        if scale.len() > translate.len() {
            let mut padded = vec![0.0; scale.len() - translate.len()];
            padded.extend(translate);
            translate = padded;
        }
        if translate.len() > scale.len() {
            let mut padded = vec![1.0; translate.len() - scale.len()];
            padded.extend(scale);
            scale = padded;
        }
        Self {
            scale,
            translate,
            name,
        }
    }

    pub fn transform_point(&self, coords: &[f64]) -> Vec<f64> {
        let scale = leading_padded(&self.scale, coords.len(), 1.0);
        let translate = leading_padded(&self.translate, coords.len(), 0.0);
        coords
            .iter()
            .zip(scale)
            .zip(translate)
            .map(|((&coord, scale), translate)| scale * coord + translate)
            .collect()
    }

    pub fn transform_points(&self, coords: &[Vec<f64>]) -> Vec<Vec<f64>> {
        coords
            .iter()
            .map(|coord| self.transform_point(coord))
            .collect()
    }

    pub fn inverse(&self) -> Self {
        let inverse_scale: Vec<f64> = self.scale.iter().map(|scale| 1.0 / scale).collect();
        let inverse_translate: Vec<f64> = self
            .scale
            .iter()
            .zip(&self.translate)
            .map(|(scale, translate)| -translate / scale)
            .collect();
        Self::new(inverse_scale, inverse_translate, None)
    }

    pub fn compose(&self, transform: &Self) -> Self {
        let ndim = self.scale.len().max(transform.scale.len());
        let self_scale = leading_padded(&self.scale, ndim, 1.0);
        let self_translate = leading_padded(&self.translate, ndim, 0.0);
        let other_scale = leading_padded(&transform.scale, ndim, 1.0);
        let other_translate = leading_padded(&transform.translate, ndim, 0.0);

        let scale: Vec<f64> = self_scale
            .iter()
            .zip(&other_scale)
            .map(|(left, right)| left * right)
            .collect();
        let translate: Vec<f64> = self_translate
            .iter()
            .zip(&self_scale)
            .zip(&other_translate)
            .map(|((self_translate, self_scale), other_translate)| {
                self_translate + self_scale * other_translate
            })
            .collect();

        Self::new(scale, translate, None)
    }

    pub fn set_slice(&self, axes: &[usize]) -> Self {
        Self {
            scale: axes.iter().map(|&axis| self.scale[axis]).collect(),
            translate: axes.iter().map(|&axis| self.translate[axis]).collect(),
            name: self.name.clone(),
        }
    }

    pub fn expand_dims(&self, axes: &[usize]) -> Self {
        let ndim = axes.len() + self.scale.len();
        let not_axes: Vec<usize> = (0..ndim).filter(|axis| !axes.contains(axis)).collect();
        let mut scale = vec![1.0; ndim];
        let mut translate = vec![0.0; ndim];
        for (value_index, axis) in not_axes.into_iter().enumerate() {
            scale[axis] = self.scale[value_index];
            translate[axis] = self.translate[value_index];
        }
        Self {
            scale,
            translate,
            name: self.name.clone(),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        true
    }
}

fn leading_padded(values: &[f64], ndim: usize, fill: f64) -> Vec<f64> {
    if values.len() >= ndim {
        values[values.len() - ndim..].to_vec()
    } else {
        let mut padded = vec![fill; ndim - values.len()];
        padded.extend_from_slice(values);
        padded
    }
}
