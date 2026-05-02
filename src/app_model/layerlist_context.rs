#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayerInfo {
    pub layer_type: String,
    pub ndim: Option<usize>,
    pub shape: Option<Vec<usize>>,
    pub dtype: Option<String>,
    pub rgb: bool,
    pub has_features: bool,
    pub has_colorbar: bool,
    pub data_len: usize,
}

impl LayerInfo {
    pub fn new(layer_type: impl Into<String>) -> Self {
        Self {
            layer_type: layer_type.into(),
            ndim: None,
            shape: None,
            dtype: None,
            rgb: false,
            has_features: false,
            has_colorbar: false,
            data_len: 0,
        }
    }

    pub fn with_ndim(mut self, ndim: usize) -> Self {
        self.ndim = Some(ndim);
        self
    }

    pub fn with_shape(mut self, shape: impl Into<Vec<usize>>) -> Self {
        self.shape = Some(shape.into());
        self
    }

    pub fn with_dtype(mut self, dtype: impl Into<String>) -> Self {
        self.dtype = Some(dtype.into());
        self
    }

    pub fn with_rgb(mut self, rgb: bool) -> Self {
        self.rgb = rgb;
        self
    }

    pub fn with_features(mut self, has_features: bool) -> Self {
        self.has_features = has_features;
        self
    }

    pub fn with_colorbar(mut self, has_colorbar: bool) -> Self {
        self.has_colorbar = has_colorbar;
        self
    }

    pub fn with_data_len(mut self, data_len: usize) -> Self {
        self.data_len = data_len;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayerSelection {
    pub layers: Vec<LayerInfo>,
    pub active: Option<usize>,
}

impl LayerSelection {
    pub fn new(layers: Vec<LayerInfo>, active: Option<usize>) -> Self {
        let active = active.filter(|&index| index < layers.len());
        Self { layers, active }
    }

    pub fn empty() -> Self {
        Self {
            layers: Vec::new(),
            active: None,
        }
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    pub fn active(&self) -> Option<&LayerInfo> {
        self.active.and_then(|index| self.layers.get(index))
    }
}

pub fn num_layers(layers: &[LayerInfo]) -> usize {
    layers.len()
}

pub fn num_selected_layers(selection: &LayerSelection) -> usize {
    selection.len()
}

pub fn active_layer_is_rgb(selection: &LayerSelection) -> bool {
    selection.active().is_some_and(|layer| layer.rgb)
}

pub fn active_layer_type(selection: &LayerSelection) -> Option<&str> {
    selection.active().map(|layer| layer.layer_type.as_str())
}

pub fn active_layer_ndim(selection: &LayerSelection) -> Option<usize> {
    selection.active().and_then(|layer| layer.ndim)
}

pub fn active_layer_shape(selection: &LayerSelection) -> Option<&[usize]> {
    selection.active().and_then(|layer| layer.shape.as_deref())
}

pub fn active_layer_dtype(selection: &LayerSelection) -> Option<&str> {
    selection.active().and_then(|layer| layer.dtype.as_deref())
}

pub fn count_selected_layers_of_type(selection: &LayerSelection, layer_type: &str) -> usize {
    selection
        .layers
        .iter()
        .filter(|layer| layer.layer_type == layer_type)
        .count()
}

pub fn all_selected_layers_of_type(selection: &LayerSelection, layer_type: &str) -> bool {
    !selection.is_empty()
        && selection
            .layers
            .iter()
            .all(|layer| layer.layer_type == layer_type)
}

pub fn all_selected_layers_same_type(selection: &LayerSelection) -> bool {
    let Some(first) = selection.layers.first() else {
        return true;
    };
    selection
        .layers
        .iter()
        .all(|layer| layer.layer_type == first.layer_type)
}

pub fn all_selected_layers_same_shape(selection: &LayerSelection) -> bool {
    let Some(first) = selection.layers.first() else {
        return true;
    };
    selection
        .layers
        .iter()
        .all(|layer| layer.shape == first.shape)
}

pub fn active_layer_is_image_3d(selection: &LayerSelection) -> bool {
    active_layer_type(selection) == Some("image")
        && active_layer_ndim(selection)
            .is_some_and(|ndim| ndim > 3 || (ndim > 2 && !active_layer_is_rgb(selection)))
}

pub fn selected_empty_shapes_layer(selection: &LayerSelection) -> bool {
    selection
        .layers
        .iter()
        .any(|layer| layer.layer_type == "shapes" && layer.data_len == 0)
}

pub fn active_layer_supports_features(selection: &LayerSelection) -> bool {
    selection.active().is_some_and(|layer| layer.has_features)
}

pub fn all_selected_layers_support_colorbar(selection: &LayerSelection) -> bool {
    !selection.is_empty() && selection.layers.iter().all(|layer| layer.has_colorbar)
}
