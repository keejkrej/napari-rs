use napari_rs::layers::surface::normals::{
    BLUE, NormalMode, Normals, NormalsUpdate, ORANGE, SurfaceNormals, SurfaceNormalsUpdate,
};
use napari_rs::layers::surface::wireframe::{BLACK, SurfaceWireframe, SurfaceWireframeUpdate};

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

#[test]
fn surface_wireframe_defaults_and_partial_updates_match_python_model() {
    let mut wireframe = SurfaceWireframe::default();

    assert!(!wireframe.visible);
    assert_eq!(wireframe.color, BLACK);
    assert_eq!(wireframe.width, 1.0);

    wireframe.update(SurfaceWireframeUpdate {
        visible: Some(true),
        color: Some(RED),
        width: None,
    });

    assert!(wireframe.visible);
    assert_eq!(wireframe.color, RED);
    assert_eq!(wireframe.width, 1.0);

    wireframe.reset();

    assert_eq!(wireframe, SurfaceWireframe::default());
}

#[test]
fn normals_defaults_and_partial_updates_match_python_model() {
    let mut normals = Normals::new(NormalMode::Vertex, BLUE);

    assert_eq!(normals.mode(), NormalMode::Vertex);
    assert!(!normals.visible);
    assert_eq!(normals.color, BLUE);
    assert_eq!(normals.width, 1.0);
    assert_eq!(normals.length, 5.0);

    normals.update(NormalsUpdate {
        visible: Some(true),
        color: Some(RED),
        width: Some(2.0),
        length: None,
    });

    assert_eq!(normals.mode(), NormalMode::Vertex);
    assert!(normals.visible);
    assert_eq!(normals.color, RED);
    assert_eq!(normals.width, 2.0);
    assert_eq!(normals.length, 5.0);

    normals.reset();

    assert_eq!(normals.mode(), NormalMode::Vertex);
    assert_eq!(normals.color, [0.0, 0.0, 0.0, 1.0]);
    assert!(!normals.visible);
}

#[test]
fn surface_normals_defaults_and_nested_updates_match_python_model() {
    let mut surface_normals = SurfaceNormals::default();

    assert_eq!(surface_normals.face.mode(), NormalMode::Face);
    assert_eq!(surface_normals.face.color, ORANGE);
    assert_eq!(surface_normals.vertex.mode(), NormalMode::Face);
    assert_eq!(surface_normals.vertex.color, BLUE);

    surface_normals.update(SurfaceNormalsUpdate {
        face: Some(NormalsUpdate {
            visible: Some(true),
            color: Some(RED),
            width: None,
            length: None,
        }),
        vertex: None,
    });

    assert!(surface_normals.face.visible);
    assert_eq!(surface_normals.face.color, RED);
    assert!(!surface_normals.vertex.visible);
    assert_eq!(surface_normals.vertex.color, BLUE);

    surface_normals.reset();

    assert_eq!(surface_normals, SurfaceNormals::default());
}
