use napari_rs::components::dims::{Dims, DimsError, RangeTuple, ensure_axis_in_bounds, ensure_len};

fn range(start: f64, stop: f64, step: f64) -> RangeTuple {
    RangeTuple { start, stop, step }
}

#[test]
fn dims_defaults_and_displayed_axes_match_python_model() {
    let dims = Dims::default();
    assert_eq!(dims.ndim, 2);
    assert_eq!(dims.ndisplay, 2);
    assert_eq!(dims.order, vec![0, 1]);
    assert_eq!(dims.displayed(), &[0, 1]);
    assert_eq!(dims.displayed_order(), vec![0, 1]);
    assert_eq!(dims.not_displayed(), &[]);
    assert_eq!(dims.range, vec![RangeTuple::default(); 2]);
    assert_eq!(dims.point, vec![0.0, 0.0]);
    assert_eq!(dims.axis_labels, vec!["-2", "-1"]);
    assert_eq!(dims.rollable, vec![true, true]);

    let mut dims = Dims::new(4).unwrap();
    assert_eq!(dims.order, vec![0, 1, 2, 3]);
    assert_eq!(dims.displayed(), &[2, 3]);
    assert_eq!(dims.displayed_order(), vec![0, 1]);
    assert_eq!(dims.not_displayed(), &[0, 1]);

    dims.set_order(vec![2, 3, 1, 0]).unwrap();
    assert_eq!(dims.displayed(), &[1, 0]);
    assert_eq!(dims.displayed_order(), vec![1, 0]);
    assert_eq!(dims.not_displayed(), &[2, 3]);
}

#[test]
fn dims_initial_options_and_invalid_order_match_python_model() {
    let dims = Dims::with_options(
        3,
        2,
        vec![0, 2, 1],
        vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();

    assert_eq!(dims.order, vec![0, 2, 1]);
    assert_eq!(dims.axis_labels, vec!["x", "y", "z"]);

    let mut dims = Dims::new(3).unwrap();
    assert_eq!(
        dims.set_order(vec![0, 0, 1]),
        Err(DimsError::InvalidOrder {
            order: vec![0, 0, 1],
            ndim: 3
        })
    );
}

#[test]
fn ensure_len_and_axis_bounds_match_python_helpers() {
    assert_eq!(ensure_len(vec![1, 2], 4, 0), vec![0, 0, 1, 2]);
    assert_eq!(ensure_len(vec![1, 2, 3, 4], 2, 0), vec![3, 4]);

    assert_eq!(ensure_axis_in_bounds(1, 2), Ok(1));
    assert_eq!(ensure_axis_in_bounds(-1, 2), Ok(1));
    assert_eq!(ensure_axis_in_bounds(-3, 4), Ok(1));
    assert!(matches!(
        ensure_axis_in_bounds(2, 2),
        Err(DimsError::AxisOutOfBounds { axis: 2, ndim: 2 })
    ));
    assert!(matches!(
        ensure_axis_in_bounds(-3, 2),
        Err(DimsError::AxisOutOfBounds { axis: -3, ndim: 2 })
    ));
}

#[test]
fn range_and_point_setters_clip_and_validate_like_python_model() {
    let mut dims = Dims::new(4).unwrap();
    assert_eq!(dims.range, vec![RangeTuple::default(); 4]);

    dims.set_range(3, range(0.0, 4.0, 2.0)).unwrap();
    assert_eq!(
        dims.range,
        vec![
            RangeTuple::default(),
            RangeTuple::default(),
            RangeTuple::default(),
            range(0.0, 4.0, 2.0)
        ]
    );

    assert!(matches!(
        dims.set_range(0, range(1.0, 0.0, 1.0)),
        Err(DimsError::InvalidRangeOrder { .. })
    ));
    assert!(matches!(
        dims.set_range(0, range(0.0, 2.0, 0.0)),
        Err(DimsError::InvalidRangeStep { .. })
    ));

    dims.set_range(0, range(0.0, 5.0, 1.0)).unwrap();
    dims.set_point(3, 4.0).unwrap();
    dims.set_point(2, 1.0).unwrap();
    dims.set_points(&[0, 1, 2], &[2.1, 2.6, 0.0]).unwrap();
    assert_eq!(dims.point, vec![2.1, 2.0, 0.0, 4.0]);
}

#[test]
fn variable_step_current_steps_match_python_model() {
    let mut dims = Dims::new(3).unwrap();
    dims.set_ranges(
        &[0, 1, 2],
        &[
            range(0.0, 6.0, 0.5),
            range(0.0, 6.0, 1.0),
            range(0.0, 6.0, 2.0),
        ],
    )
    .unwrap();

    dims.set_points(&[0, 1, 2], &[2.9, 2.9, 2.9]).unwrap();
    assert_eq!(dims.current_step(), vec![6, 3, 1]);
    assert_eq!(dims.point, vec![2.9, 2.9, 2.9]);

    dims.set_current_steps(&[0, 1, 2], &[1, -3, 5]).unwrap();
    assert_eq!(dims.current_step(), vec![1, 0, 3]);
    assert_eq!(dims.point, vec![0.5, 0.0, 6.0]);

    dims.set_current_step(0, -1).unwrap();
    assert_eq!(dims.current_step(), vec![0, 0, 3]);
    assert_eq!(dims.point, vec![0.0, 0.0, 6.0]);

    assert_eq!(
        dims.set_points(&[0, 1], &[0.0, 0.0, 0.0]),
        Err(DimsError::MismatchedInputLength)
    );
}

#[test]
fn changing_ndim_pads_crops_order_points_and_axis_labels_like_python() {
    let mut dims = Dims::new(4).unwrap();
    dims.set_range(0, range(0.0, 4.0, 1.0)).unwrap();
    dims.set_point(0, 2.0).unwrap();

    dims.set_ndim(5).unwrap();
    assert_eq!(dims.point, vec![0.0, 2.0, 0.0, 0.0, 0.0]);
    assert_eq!(dims.order, vec![0, 1, 2, 3, 4]);
    assert_eq!(dims.axis_labels, vec!["-5", "-4", "-3", "-2", "-1"]);

    dims.set_range(2, range(0.0, 4.0, 1.0)).unwrap();
    dims.set_point(2, 3.0).unwrap();
    dims.set_ndim(3).unwrap();
    assert_eq!(dims.point, vec![3.0, 0.0, 0.0]);
    assert_eq!(dims.order, vec![0, 1, 2]);
    assert_eq!(dims.axis_labels, vec!["-3", "-2", "-1"]);
}

#[test]
fn axis_label_setters_match_python_model() {
    let mut dims = Dims::new(4).unwrap();
    assert_eq!(dims.axis_labels, vec!["-4", "-3", "-2", "-1"]);

    dims.set_axis_label(0, "t").unwrap();
    assert_eq!(dims.axis_labels, vec!["t", "-3", "-2", "-1"]);

    dims.set_axis_labels_for_axes(
        &[0, 1, 3],
        &["t".to_owned(), "c".to_owned(), "last".to_owned()],
    )
    .unwrap();
    assert_eq!(dims.axis_labels, vec!["t", "c", "-2", "last"]);

    dims.set_axis_labels(vec!["a".to_owned(), "b".to_owned()])
        .unwrap();
    assert_eq!(dims.axis_labels, vec!["-4", "-3", "a", "b"]);

    let mut dims = Dims::default();
    dims.set_axis_labels_from_str("TX").unwrap();
    assert_eq!(dims.axis_labels, vec!["T", "X"]);
}

#[test]
fn roll_skips_dummy_or_nonrollable_axes_like_python_model() {
    let mut dims = Dims::new(4).unwrap();
    for axis in 0..4 {
        dims.set_range(axis, range(0.0, 10.0, 1.0)).unwrap();
    }
    dims.roll().unwrap();
    assert_eq!(dims.order, vec![3, 0, 1, 2]);
    dims.roll().unwrap();
    assert_eq!(dims.order, vec![2, 3, 0, 1]);

    let mut dims = Dims::new(4).unwrap();
    dims.set_range(0, range(0.0, 0.0, 1.0)).unwrap();
    for axis in 1..4 {
        dims.set_range(axis, range(0.0, 10.0, 1.0)).unwrap();
    }
    dims.roll().unwrap();
    assert_eq!(dims.order, vec![0, 3, 1, 2]);
    dims.roll().unwrap();
    assert_eq!(dims.order, vec![0, 2, 3, 1]);
}

#[test]
fn focus_and_ndisplay_updates_match_python_model() {
    let mut dims = Dims::new(2).unwrap();
    assert_eq!(dims.last_used, 0);
    dims.focus_down();
    dims.focus_up();
    assert_eq!(dims.last_used, 0);

    dims.set_ndim(5).unwrap();
    assert_eq!(dims.last_used, 0);
    dims.focus_down();
    assert_eq!(dims.last_used, 2);
    dims.focus_down();
    assert_eq!(dims.last_used, 1);
    dims.focus_up();
    assert_eq!(dims.last_used, 2);
    dims.focus_up();
    assert_eq!(dims.last_used, 0);

    let mut dims = Dims::new(4).unwrap();
    dims.last_used = 1;
    dims.set_ndisplay(3).unwrap();
    assert_eq!(dims.last_used, 0);
}

#[test]
fn nsteps_and_thickness_accessors_match_python_model() {
    let mut dims = Dims::with_options(
        2,
        2,
        Vec::new(),
        Vec::new(),
        vec![range(0.0, 5.0, 1.0), range(0.0, 10.0, 0.5)],
        Vec::new(),
    )
    .unwrap();
    assert_eq!(dims.nsteps(), vec![6, 21]);
    dims.set_nsteps(&[11, 11]).unwrap();
    assert_eq!(
        dims.range,
        vec![range(0.0, 5.0, 0.5), range(0.0, 10.0, 1.0)]
    );

    dims.margin_left = vec![0.0, 0.5];
    dims.margin_right = vec![1.0, 1.0];
    assert_eq!(dims.thickness(), vec![1.0, 1.5]);
    assert_eq!(dims.margin_left_step(), vec![0, 1]);
    assert_eq!(dims.margin_right_step(), vec![2, 1]);
    assert_eq!(dims.thickness_step(), vec![2, 2]);
}
