use napari_rs::layers::base::constants::{
    ActionType, BaseProjectionMode, Blending, InteractionBoxHandle, Mode,
};

#[test]
fn base_mode_strings_match_python_string_enum_values() {
    assert_eq!(Mode::PanZoom.to_string(), "pan_zoom");
    assert_eq!(Mode::Transform.to_string(), "transform");
    assert_eq!("PAN_ZOOM".parse(), Ok(Mode::PanZoom));
    assert_eq!("transform".parse(), Ok(Mode::Transform));
}

#[test]
fn action_type_strings_match_python_string_enum_values() {
    assert_eq!(ActionType::Adding.to_string(), "adding");
    assert_eq!(ActionType::Removing.to_string(), "removing");
    assert_eq!(ActionType::Changing.to_string(), "changing");
    assert_eq!(ActionType::Added.to_string(), "added");
    assert_eq!(ActionType::Removed.to_string(), "removed");
    assert_eq!(ActionType::Changed.to_string(), "changed");
    assert_eq!("CHANGED".parse(), Ok(ActionType::Changed));
}

#[test]
fn base_projection_mode_matches_python_string_enum_value() {
    assert_eq!(BaseProjectionMode::None.to_string(), "none");
    assert_eq!("NONE".parse(), Ok(BaseProjectionMode::None));
}

#[test]
fn interaction_box_handle_indices_match_python_int_enum_values() {
    assert_eq!(InteractionBoxHandle::TopLeft.index(), 0);
    assert_eq!(InteractionBoxHandle::TopCenter.index(), 4);
    assert_eq!(InteractionBoxHandle::TopRight.index(), 2);
    assert_eq!(InteractionBoxHandle::CenterLeft.index(), 5);
    assert_eq!(InteractionBoxHandle::CenterRight.index(), 6);
    assert_eq!(InteractionBoxHandle::BottomLeft.index(), 1);
    assert_eq!(InteractionBoxHandle::BottomCenter.index(), 7);
    assert_eq!(InteractionBoxHandle::BottomRight.index(), 3);
    assert_eq!(InteractionBoxHandle::Rotation.index(), 8);
    assert_eq!(InteractionBoxHandle::Inside.index(), 9);
}

#[test]
fn interaction_box_handle_opposites_and_corners_match_python_helpers() {
    assert_eq!(
        InteractionBoxHandle::TopLeft.opposite(),
        Ok(InteractionBoxHandle::BottomRight)
    );
    assert_eq!(
        InteractionBoxHandle::TopCenter.opposite(),
        Ok(InteractionBoxHandle::BottomCenter)
    );
    assert_eq!(
        InteractionBoxHandle::TopRight.opposite(),
        Ok(InteractionBoxHandle::BottomLeft)
    );
    assert_eq!(
        InteractionBoxHandle::CenterLeft.opposite(),
        Ok(InteractionBoxHandle::CenterRight)
    );
    assert!(InteractionBoxHandle::Rotation.opposite().is_err());
    assert_eq!(
        InteractionBoxHandle::corners(),
        [
            InteractionBoxHandle::TopLeft,
            InteractionBoxHandle::TopRight,
            InteractionBoxHandle::BottomLeft,
            InteractionBoxHandle::BottomRight,
        ]
    );
}

#[test]
fn existing_blending_enum_still_matches_python_values() {
    assert_eq!(Blending::Opaque.to_string(), "opaque");
    assert_eq!("MULTIPLICATIVE".parse(), Ok(Blending::Multiplicative));
}
