use crate::utils::colormap_backend::ColormapBackend;
use crate::utils::triangulation_backend::TriangulationBackend;

pub const EXPERIMENTAL_PREFERENCES_EXCLUDE: &[&str] = &["schema_version", "compiled_triangulation"];

#[derive(Clone, Debug, PartialEq)]
pub struct ExperimentalSettings {
    pub async_: bool,
    pub autoswap_buffers: bool,
    pub rdp_epsilon: f64,
    pub lasso_vertex_distance: i32,
    pub completion_radius: i32,
    pub triangulation_backend: TriangulationBackend,
    pub colormap_backend: ColormapBackend,
    pub compiled_triangulation: bool,
}

impl Default for ExperimentalSettings {
    fn default() -> Self {
        Self {
            async_: false,
            autoswap_buffers: false,
            rdp_epsilon: 0.5,
            lasso_vertex_distance: 10,
            completion_radius: -1,
            triangulation_backend: TriangulationBackend::FastestAvailable,
            colormap_backend: ColormapBackend::FastestAvailable,
            compiled_triangulation: false,
        }
    }
}

pub fn migrate_compiled_triangulation_backend(
    compiled_triangulation: bool,
) -> TriangulationBackend {
    if compiled_triangulation {
        TriangulationBackend::PartSegCore
    } else {
        TriangulationBackend::FastestAvailable
    }
}
