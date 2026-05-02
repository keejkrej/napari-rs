use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayerDataProtocolInfo {
    pub dtype: bool,
    pub shape: bool,
    pub get_item: bool,
    pub size: bool,
    pub ndim: bool,
}

impl LayerDataProtocolInfo {
    pub fn complete() -> Self {
        Self {
            dtype: true,
            shape: true,
            get_item: true,
            size: true,
            ndim: true,
        }
    }

    pub fn missing_methods(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.dtype {
            missing.push("dtype");
        }
        if !self.shape {
            missing.push("shape");
        }
        if !self.get_item {
            missing.push("__getitem__");
        }
        if !self.size {
            missing.push("size");
        }
        if !self.ndim {
            missing.push("ndim");
        }
        missing
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolError {
    pub type_name: String,
    pub protocol_name: String,
    pub missing_methods: Vec<&'static str>,
}

impl ProtocolError {
    pub fn new(
        type_name: impl Into<String>,
        protocol_name: impl Into<String>,
        missing_methods: Vec<&'static str>,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            protocol_name: protocol_name.into(),
            missing_methods,
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Object of type {:?} does not implement {:?} Protocol.\nMissing methods: {:?}",
            self.type_name, self.protocol_name, self.missing_methods
        )
    }
}

impl std::error::Error for ProtocolError {}

pub fn assert_layer_data_protocol(
    type_name: impl Into<String>,
    info: &LayerDataProtocolInfo,
) -> Result<(), ProtocolError> {
    assert_protocol("LayerDataProtocol", type_name, info)
}

pub fn assert_protocol(
    protocol_name: impl Into<String>,
    type_name: impl Into<String>,
    info: &LayerDataProtocolInfo,
) -> Result<(), ProtocolError> {
    let missing_methods = info.missing_methods();
    if missing_methods.is_empty() {
        Ok(())
    } else {
        Err(ProtocolError::new(
            type_name,
            protocol_name,
            missing_methods,
        ))
    }
}
