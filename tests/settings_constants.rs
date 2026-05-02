use napari_rs::settings::constants::{BrushSizeOnMouseModifiers, LabelDType, LoopMode};

#[test]
fn label_dtype_strings_match_python_str_enum_values() {
    assert_eq!(LabelDType::Uint8.to_string(), "uint8");
    assert_eq!(LabelDType::Int8.to_string(), "int8");
    assert_eq!(LabelDType::Uint16.to_string(), "uint16");
    assert_eq!(LabelDType::Int16.to_string(), "int16");
    assert_eq!(LabelDType::Uint32.to_string(), "uint32");
    assert_eq!(LabelDType::Int32.to_string(), "int32");
    assert_eq!(LabelDType::Uint64.to_string(), "uint64");
    assert_eq!(LabelDType::Int64.to_string(), "int64");
    assert_eq!(LabelDType::Uint.to_string(), "uint");
    assert_eq!(LabelDType::Int.to_string(), "int");

    assert_eq!("uint8".parse::<LabelDType>().unwrap(), LabelDType::Uint8);
    assert_eq!("int".parse::<LabelDType>().unwrap(), LabelDType::Int);
    assert!("Uint8".parse::<LabelDType>().is_err());
}

#[test]
fn loop_mode_strings_match_python_string_enum_values() {
    assert_eq!(LoopMode::Once.to_string(), "once");
    assert_eq!(LoopMode::Loop.to_string(), "loop");
    assert_eq!(LoopMode::BackAndForth.to_string(), "back_and_forth");

    assert_eq!("once".parse::<LoopMode>().unwrap(), LoopMode::Once);
    assert_eq!("ONCE".parse::<LoopMode>().unwrap(), LoopMode::Once);
    assert_eq!("Loop".parse::<LoopMode>().unwrap(), LoopMode::Loop);
    assert_eq!(
        "BACK_AND_FORTH".parse::<LoopMode>().unwrap(),
        LoopMode::BackAndForth
    );
    assert!("back-and-forth".parse::<LoopMode>().is_err());
}

#[test]
fn brush_size_modifier_strings_match_python_str_enum_values() {
    assert_eq!(BrushSizeOnMouseModifiers::Alt.to_string(), "Alt");
    assert_eq!(BrushSizeOnMouseModifiers::Ctrl.to_string(), "Control");
    assert_eq!(
        BrushSizeOnMouseModifiers::CtrlAlt.to_string(),
        "Control+Alt"
    );
    assert_eq!(
        BrushSizeOnMouseModifiers::CtrlShift.to_string(),
        "Control+Shift"
    );
    assert_eq!(BrushSizeOnMouseModifiers::Disabled.to_string(), "Disabled");

    assert_eq!(
        "Alt".parse::<BrushSizeOnMouseModifiers>().unwrap(),
        BrushSizeOnMouseModifiers::Alt
    );
    assert_eq!(
        "Control+Shift"
            .parse::<BrushSizeOnMouseModifiers>()
            .unwrap(),
        BrushSizeOnMouseModifiers::CtrlShift
    );
    assert!("control".parse::<BrushSizeOnMouseModifiers>().is_err());
}
