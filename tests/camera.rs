use napari_rs::components::camera::{Camera, DisplayDimensions};
use napari_rs::utils::camera_orientations::{
    DEFAULT_ORIENTATION, DEFAULT_ORIENTATION_STR, DepthAxisOrientation, Handedness,
    HorizontalAxisOrientation, VerticalAxisOrientation,
};

const EPS: f64 = 1e-10;

#[test]
fn camera_defaults_match_python_model_defaults() {
    let camera = Camera::default();

    assert_vec3_close(camera.center, [0.0, 0.0, 0.0]);
    assert_close(camera.zoom, 1.0);
    assert_vec3_close(camera.angles, [0.0, 0.0, 0.0]);
    assert_close(camera.perspective, 0.0);
    assert!(camera.mouse_pan);
    assert!(camera.mouse_zoom);
    assert_eq!(camera.orientation, DEFAULT_ORIENTATION);
}

#[test]
fn camera_orientation_strings_match_python_string_enum_values() {
    assert_eq!(VerticalAxisOrientation::Up.to_string(), "up");
    assert_eq!(VerticalAxisOrientation::Down.to_string(), "down");
    assert_eq!(HorizontalAxisOrientation::Left.to_string(), "left");
    assert_eq!(HorizontalAxisOrientation::Right.to_string(), "right");
    assert_eq!(DepthAxisOrientation::Away.to_string(), "away");
    assert_eq!(DepthAxisOrientation::Towards.to_string(), "towards");
    assert_eq!(Handedness::Right.to_string(), "right");
    assert_eq!(Handedness::Left.to_string(), "left");
    assert_eq!(DEFAULT_ORIENTATION_STR, ("towards", "down", "right"));
}

#[test]
fn camera_orientation_parsing_is_case_insensitive_like_string_enum() {
    assert_eq!("UP".parse(), Ok(VerticalAxisOrientation::Up));
    assert_eq!("DoWn".parse(), Ok(VerticalAxisOrientation::Down));
    assert_eq!("LEFT".parse(), Ok(HorizontalAxisOrientation::Left));
    assert_eq!("rIgHt".parse(), Ok(HorizontalAxisOrientation::Right));
    assert_eq!("AWAY".parse(), Ok(DepthAxisOrientation::Away));
    assert_eq!("toWaRds".parse(), Ok(DepthAxisOrientation::Towards));
    assert_eq!("LEFT".parse(), Ok(Handedness::Left));
}

#[test]
fn calculate_view_direction_3d_matches_python_cases() {
    let camera = Camera {
        angles: [90.0, 0.0, 0.0],
        ..Camera::default()
    };
    assert_vec3_close(camera.view_direction(), [-1.0, 0.0, 0.0]);

    let camera = Camera {
        center: [15.0, 15.0, 15.0],
        zoom: 10.0,
        angles: [90.0, 0.0, 0.0],
        ..Camera::default()
    };
    assert_vec3_close(camera.view_direction(), [-1.0, 0.0, 0.0]);
}

#[test]
fn calculate_up_direction_3d_matches_python_cases() {
    let camera = Camera {
        angles: [0.0, 0.0, 90.0],
        ..Camera::default()
    };
    assert_vec3_close(camera.up_direction(), [-1.0, 0.0, 0.0]);

    let camera = Camera {
        center: [15.0, 15.0, 15.0],
        zoom: 10.0,
        angles: [0.0, 0.0, 90.0],
        ..Camera::default()
    };
    assert_vec3_close(camera.up_direction(), [-1.0, 0.0, 0.0]);

    let camera = Camera {
        angles: [10.0, 20.0, 30.0],
        ..Camera::default()
    };
    assert_vec3_near(camera.up_direction(), [-0.47, -0.88, -0.02], 0.01);
}

#[test]
fn calculate_nd_view_direction_matches_python_cases() {
    let camera = Camera {
        angles: [90.0, 0.0, 0.0],
        ..Camera::default()
    };

    assert_eq!(camera.calculate_nd_view_direction(2, &[0, 1]), None);
    assert_vec_close(
        &camera.calculate_nd_view_direction(3, &[0, 1, 2]).unwrap(),
        &[-1.0, 0.0, 0.0],
    );

    let view_direction = camera.calculate_nd_view_direction(5, &[0, 2, 4]).unwrap();
    assert_eq!(view_direction.len(), 5);
    assert_close(view_direction[0], -1.0);
    assert_close(view_direction[2], 0.0);
    assert_close(view_direction[4], 0.0);
}

#[test]
fn orientation2d_setter_preserves_depth_orientation() {
    let mut camera = Camera::default();
    camera.orientation.0 = DepthAxisOrientation::Away;
    camera.set_orientation2d(VerticalAxisOrientation::Up, HorizontalAxisOrientation::Left);

    assert_eq!(
        camera.orientation,
        (
            DepthAxisOrientation::Away,
            VerticalAxisOrientation::Up,
            HorizontalAxisOrientation::Left
        )
    );
    assert_eq!(
        camera.orientation2d(),
        (VerticalAxisOrientation::Up, HorizontalAxisOrientation::Left)
    );
}

#[test]
fn handedness_matches_python_product_cases() {
    let cases = [
        (
            (
                DepthAxisOrientation::Towards,
                VerticalAxisOrientation::Down,
                HorizontalAxisOrientation::Right,
            ),
            Handedness::Right,
        ),
        (
            (
                DepthAxisOrientation::Towards,
                VerticalAxisOrientation::Down,
                HorizontalAxisOrientation::Left,
            ),
            Handedness::Left,
        ),
        (
            (
                DepthAxisOrientation::Towards,
                VerticalAxisOrientation::Up,
                HorizontalAxisOrientation::Right,
            ),
            Handedness::Left,
        ),
        (
            (
                DepthAxisOrientation::Towards,
                VerticalAxisOrientation::Up,
                HorizontalAxisOrientation::Left,
            ),
            Handedness::Right,
        ),
        (
            (
                DepthAxisOrientation::Away,
                VerticalAxisOrientation::Down,
                HorizontalAxisOrientation::Right,
            ),
            Handedness::Left,
        ),
        (
            (
                DepthAxisOrientation::Away,
                VerticalAxisOrientation::Down,
                HorizontalAxisOrientation::Left,
            ),
            Handedness::Right,
        ),
        (
            (
                DepthAxisOrientation::Away,
                VerticalAxisOrientation::Up,
                HorizontalAxisOrientation::Right,
            ),
            Handedness::Right,
        ),
        (
            (
                DepthAxisOrientation::Away,
                VerticalAxisOrientation::Up,
                HorizontalAxisOrientation::Left,
            ),
            Handedness::Left,
        ),
    ];

    for (orientation, expected_handedness) in cases {
        let camera = Camera {
            orientation,
            ..Camera::default()
        };
        assert_eq!(camera.handedness(), expected_handedness);
    }
}

#[test]
fn vispy_flipped_axes_matches_python_orientation_logic() {
    let camera = Camera::default();
    assert_eq!(camera.vispy_flipped_axes(DisplayDimensions::Two), [0, 1, 0]);
    assert_eq!(
        camera.vispy_flipped_axes(DisplayDimensions::Three),
        [0, 0, 1]
    );

    let camera = Camera {
        orientation: (
            DepthAxisOrientation::Away,
            VerticalAxisOrientation::Up,
            HorizontalAxisOrientation::Left,
        ),
        ..Camera::default()
    };
    assert_eq!(camera.vispy_flipped_axes(DisplayDimensions::Two), [1, 0, 1]);
    assert_eq!(
        camera.vispy_flipped_axes(DisplayDimensions::Three),
        [1, 1, 0]
    );
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

fn assert_vec3_close(actual: [f64; 3], expected: [f64; 3]) {
    assert_vec_close(&actual, &expected);
}

fn assert_vec3_near(actual: [f64; 3], expected: [f64; 3], tolerance: f64) {
    for (&actual, &expected) in actual.iter().zip(&expected) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }
}
