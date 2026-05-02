use napari_rs::layers::data_protocols::{
    LayerDataProtocolInfo, ProtocolError, assert_layer_data_protocol, assert_protocol,
};

#[test]
fn complete_layer_data_protocol_info_passes_validation() {
    assert_eq!(
        assert_layer_data_protocol("ArrayLike", &LayerDataProtocolInfo::complete()),
        Ok(())
    );
}

#[test]
fn missing_methods_match_python_layer_data_protocol_requirements() {
    let info = LayerDataProtocolInfo {
        dtype: true,
        shape: false,
        get_item: false,
        size: true,
        ndim: false,
    };

    let error = assert_layer_data_protocol("list", &info).unwrap_err();

    assert_eq!(
        error,
        ProtocolError::new(
            "list",
            "LayerDataProtocol",
            vec!["shape", "__getitem__", "ndim"]
        )
    );
    assert!(error.to_string().contains("Missing methods: "));
    assert!(error.to_string().contains("\"shape\""));
}

#[test]
fn protocol_name_can_be_overridden_like_python_assert_protocol_argument() {
    let info = LayerDataProtocolInfo::default();

    let error = assert_protocol("CustomProtocol", "object", &info).unwrap_err();

    assert_eq!(error.protocol_name, "CustomProtocol");
    assert_eq!(
        error.missing_methods,
        vec!["dtype", "shape", "__getitem__", "size", "ndim"]
    );
}
