use napari_rs::utils::transforms::units::{Unit, get_unit_from_name, get_units_from_names};

#[test]
fn get_unit_from_name_matches_python_pixel_default_and_common_pint_names() {
    assert_eq!(get_unit_from_name(None), Ok(Unit::Pixel));
    assert_eq!(get_unit_from_name(Some("pixel")), Ok(Unit::Pixel));
    assert_eq!(get_unit_from_name(Some("mm")), Ok(Unit::Millimeter));
    assert_eq!(get_unit_from_name(Some("meter")), Ok(Unit::Meter));
    assert_eq!(get_unit_from_name(Some("cm")), Ok(Unit::Centimeter));
    assert_eq!(get_unit_from_name(Some("um")), Ok(Unit::Micrometer));
    assert_eq!(get_unit_from_name(Some("nm")), Ok(Unit::Nanometer));
}

#[test]
fn get_units_from_names_converts_sequences_and_rejects_unknown_units() {
    assert_eq!(get_units_from_names(None), Ok(vec![Unit::Pixel]));
    assert_eq!(
        get_units_from_names(Some(&["cm", "mm"])),
        Ok(vec![Unit::Centimeter, Unit::Millimeter])
    );
    assert!(get_units_from_names(Some(&["ugh", "ugh"])).is_err());
}
