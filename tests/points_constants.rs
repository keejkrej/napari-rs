use napari_rs::layers::points::constants::{
    ColorMode, Mode, PointsProjectionMode, Shading, Symbol,
};

#[test]
fn points_color_mode_strings_match_python_string_enum_values() {
    assert_eq!(ColorMode::Direct.to_string(), "direct");
    assert_eq!(ColorMode::Cycle.to_string(), "cycle");
    assert_eq!(ColorMode::Colormap.to_string(), "colormap");
    assert_eq!("COLORMAP".parse(), Ok(ColorMode::Colormap));
}

#[test]
fn points_mode_strings_match_python_string_enum_values() {
    assert_eq!(Mode::PanZoom.to_string(), "pan_zoom");
    assert_eq!(Mode::Transform.to_string(), "transform");
    assert_eq!(Mode::Add.to_string(), "add");
    assert_eq!(Mode::Select.to_string(), "select");
    assert_eq!("SELECT".parse(), Ok(Mode::Select));
}

#[test]
fn points_symbol_strings_match_python_string_enum_values() {
    assert_eq!(Symbol::Arrow.to_string(), "arrow");
    assert_eq!(Symbol::Clobber.to_string(), "clobber");
    assert_eq!(Symbol::Cross.to_string(), "cross");
    assert_eq!(Symbol::Diamond.to_string(), "diamond");
    assert_eq!(Symbol::Disc.to_string(), "disc");
    assert_eq!(Symbol::Hbar.to_string(), "hbar");
    assert_eq!(Symbol::Ring.to_string(), "ring");
    assert_eq!(Symbol::Square.to_string(), "square");
    assert_eq!(Symbol::Star.to_string(), "star");
    assert_eq!(Symbol::TailedArrow.to_string(), "tailed_arrow");
    assert_eq!(Symbol::TriangleDown.to_string(), "triangle_down");
    assert_eq!(Symbol::TriangleUp.to_string(), "triangle_up");
    assert_eq!(Symbol::Vbar.to_string(), "vbar");
    assert_eq!(Symbol::X.to_string(), "x");
    assert_eq!("TRIANGLE_UP".parse(), Ok(Symbol::TriangleUp));
    assert_eq!("tailed arrow".parse(), Ok(Symbol::TailedArrow));
}

#[test]
fn points_symbol_aliases_match_python_symbol_alias_map() {
    assert_eq!(">".parse(), Ok(Symbol::Arrow));
    assert_eq!("+".parse(), Ok(Symbol::Cross));
    assert_eq!("o".parse(), Ok(Symbol::Disc));
    assert_eq!("-".parse(), Ok(Symbol::Hbar));
    assert_eq!("s".parse(), Ok(Symbol::Square));
    assert_eq!("*".parse(), Ok(Symbol::Star));
    assert_eq!("->".parse(), Ok(Symbol::TailedArrow));
    assert_eq!("v".parse(), Ok(Symbol::TriangleDown));
    assert_eq!("^".parse(), Ok(Symbol::TriangleUp));
    assert_eq!("|".parse(), Ok(Symbol::Vbar));
}

#[test]
fn points_shading_strings_match_python_string_enum_values() {
    assert_eq!(Shading::None.to_string(), "none");
    assert_eq!(Shading::Spherical.to_string(), "spherical");
    assert_eq!("SPHERICAL".parse(), Ok(Shading::Spherical));
}

#[test]
fn points_projection_mode_strings_match_python_string_enum_values() {
    assert_eq!(PointsProjectionMode::None.to_string(), "none");
    assert_eq!(PointsProjectionMode::All.to_string(), "all");
    assert_eq!("ALL".parse(), Ok(PointsProjectionMode::All));
}
