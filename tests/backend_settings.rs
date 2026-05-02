use napari_rs::settings::experimental::{
    EXPERIMENTAL_PREFERENCES_EXCLUDE, ExperimentalSettings, migrate_compiled_triangulation_backend,
};
use napari_rs::utils::colormap_backend::ColormapBackend;
use napari_rs::utils::triangulation_backend::TriangulationBackend;

#[test]
fn triangulation_backend_strings_and_missing_lookup_match_python_enum() {
    assert_eq!(
        TriangulationBackend::FastestAvailable.to_string(),
        "Fastest available"
    );
    assert_eq!(TriangulationBackend::Bermuda.to_string(), "bermuda");
    assert_eq!(TriangulationBackend::PartSegCore.to_string(), "PartSegCore");
    assert_eq!(TriangulationBackend::Triangle.to_string(), "triangle");
    assert_eq!(TriangulationBackend::Numba.to_string(), "Numba");
    assert_eq!(TriangulationBackend::PurePython.to_string(), "Pure python");

    assert_eq!(
        TriangulationBackend::FastestAvailable.name(),
        "fastest_available"
    );
    assert_eq!(
        "Fastest available".parse::<TriangulationBackend>().unwrap(),
        TriangulationBackend::FastestAvailable
    );
    assert_eq!(
        "pure python".parse::<TriangulationBackend>().unwrap(),
        TriangulationBackend::PurePython
    );
    assert_eq!(
        "PartSegCore".parse::<TriangulationBackend>().unwrap(),
        TriangulationBackend::PartSegCore
    );
    assert!("missing".parse::<TriangulationBackend>().is_err());
}

#[test]
fn colormap_backend_strings_and_missing_lookup_match_python_enum() {
    assert_eq!(
        ColormapBackend::FastestAvailable.to_string(),
        "Fastest available"
    );
    assert_eq!(ColormapBackend::PurePython.to_string(), "Pure Python");
    assert_eq!(ColormapBackend::Numba.to_string(), "numba");
    assert_eq!(ColormapBackend::PartSegCore.to_string(), "PartSegCore");

    assert_eq!(ColormapBackend::PurePython.name(), "pure_python");
    assert_eq!(
        "Fastest available".parse::<ColormapBackend>().unwrap(),
        ColormapBackend::FastestAvailable
    );
    assert_eq!(
        "Pure Python".parse::<ColormapBackend>().unwrap(),
        ColormapBackend::PurePython
    );
    assert_eq!(
        "partsegcore".parse::<ColormapBackend>().unwrap(),
        ColormapBackend::PartSegCore
    );
    assert!("cython".parse::<ColormapBackend>().is_err());
}

#[test]
fn experimental_settings_defaults_match_python_model_defaults() {
    let settings = ExperimentalSettings::default();

    assert!(!settings.async_);
    assert!(!settings.autoswap_buffers);
    assert_eq!(settings.rdp_epsilon, 0.5);
    assert_eq!(settings.lasso_vertex_distance, 10);
    assert_eq!(settings.completion_radius, -1);
    assert_eq!(
        settings.triangulation_backend,
        TriangulationBackend::FastestAvailable
    );
    assert_eq!(settings.colormap_backend, ColormapBackend::FastestAvailable);
    assert!(!settings.compiled_triangulation);
    assert_eq!(
        EXPERIMENTAL_PREFERENCES_EXCLUDE,
        &["schema_version", "compiled_triangulation"]
    );
}

#[test]
fn compiled_triangulation_migration_matches_python_v060_070_logic() {
    assert_eq!(
        migrate_compiled_triangulation_backend(true),
        TriangulationBackend::PartSegCore
    );
    assert_eq!(
        migrate_compiled_triangulation_backend(false),
        TriangulationBackend::FastestAvailable
    );
}
