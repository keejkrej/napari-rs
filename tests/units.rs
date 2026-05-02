use napari_rs::utils::units::PREFERRED_VALUES;

#[test]
fn preferred_scale_bar_values_match_python_units_constant() {
    assert_eq!(
        PREFERRED_VALUES,
        [
            1, 2, 5, 10, 15, 20, 25, 50, 75, 100, 125, 150, 200, 500, 750
        ]
    );
}
