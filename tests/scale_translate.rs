use napari_rs::utils::transforms::scale_translate::ScaleTranslate;

const EPS: f64 = 1e-10;

#[test]
fn scale_translate_matches_python_basic_case() {
    let transform = ScaleTranslate::new([2.0, 3.0], [8.0, -5.0], Some("st".to_owned()));
    assert!(transform.is_diagonal());
    assert_eq!(transform.name.as_deref(), Some("st"));
    assert_vec_close(&transform.transform_point(&[10.0, 13.0]), &[28.0, 34.0]);
}

#[test]
fn scale_translate_broadcasts_scale_and_translate_like_python() {
    let transform = ScaleTranslate::new([4.0, 2.0, 3.0], [8.0, -5.0], Some("st".to_owned()));
    assert_vec_close(&transform.scale, &[4.0, 2.0, 3.0]);
    assert_vec_close(&transform.translate, &[0.0, 8.0, -5.0]);
    assert_vec_close(
        &transform.transform_point(&[1.0, 10.0, 13.0]),
        &[4.0, 28.0, 34.0],
    );

    let transform = ScaleTranslate::new([2.0, 3.0], [5.0, 8.0, -5.0], Some("st".to_owned()));
    assert_vec_close(&transform.scale, &[1.0, 2.0, 3.0]);
    assert_vec_close(&transform.translate, &[5.0, 8.0, -5.0]);
    assert_vec_close(
        &transform.transform_point(&[1.0, 10.0, 13.0]),
        &[6.0, 28.0, 34.0],
    );
}

#[test]
fn scale_translate_inverse_round_trips_coordinates() {
    let transform = ScaleTranslate::new([2.0, 3.0], [8.0, -5.0], None);
    let coord = vec![10.0, 13.0];
    let new_coord = transform.transform_point(&coord);
    assert_vec_close(&new_coord, &[28.0, 34.0]);
    assert_vec_close(&transform.inverse().transform_point(&new_coord), &coord);
}

#[test]
fn scale_translate_compose_matches_chained_application() {
    let transform_a = ScaleTranslate::new([2.0, 3.0], [8.0, -5.0], None);
    let transform_b = ScaleTranslate::new([0.3, 1.4], [-2.2, 3.0], None);
    let transform_c = transform_b.compose(&transform_a);
    let coord = vec![10.0, 13.0];

    assert_vec_close(
        &transform_c.transform_point(&coord),
        &transform_b.transform_point(&transform_a.transform_point(&coord)),
    );
}

#[test]
fn scale_translate_slice_and_expand_dims_match_python_cases() {
    let transform_a = ScaleTranslate::new([2.0, 3.0], [8.0, -5.0], Some("st".to_owned()));
    let transform_b = ScaleTranslate::new([2.0, 1.0, 3.0], [8.0, 3.0, -5.0], Some("st".to_owned()));
    let sliced = transform_b.set_slice(&[0, 2]);
    assert_vec_close(&sliced.scale, &transform_a.scale);
    assert_vec_close(&sliced.translate, &transform_a.translate);
    assert_eq!(sliced.name.as_deref(), Some("st"));

    let expanded = transform_a.expand_dims(&[1]);
    assert_vec_close(&expanded.scale, &[2.0, 1.0, 3.0]);
    assert_vec_close(&expanded.translate, &[8.0, 0.0, -5.0]);
    assert_eq!(expanded.name.as_deref(), Some("st"));
}

#[test]
fn scale_translate_identity_default_returns_input() {
    let transform = ScaleTranslate::default();
    assert_vec_close(&transform.transform_point(&[10.0, 13.0]), &[10.0, 13.0]);
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= EPS,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_close(actual, expected);
    }
}
