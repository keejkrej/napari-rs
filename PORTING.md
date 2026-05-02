# Porting Status

Source reference: sibling Python tree at `../napari`.

## Ported

- `napari._app_model.constants._menus` -> `src/app_model/constants.rs`
  - Covered by `tests/app_model_constants.rs`.
  - Ported: menu ID string values, contributable menu set, plugin-menu
    permissive contributability fallback, and menu group string constants.
- Pure helpers from `napari._app_model.utils` -> `src/app_model/utils.rs`
  - Covered by `tests/app_model_utils.rs`.
  - Ported: menu path ID-key extraction, dummy action ID construction, dummy
    menu item detection, empty-menu detection over explicit menu lists, and
    dummy action metadata generation.
  - Not ported: global app-model registry lookup or executable app-model
    `Action` callbacks.
- `napari._app_model.context._context.ContextMapping` ->
  `src/app_model/context.rs`
  - Covered by `tests/app_model_context.rs`.
  - Ported: mapping length/key membership/iteration over initial keys, lazy
    callable context value evaluation, per-mapping caching after first lookup,
    and missing-key error reporting.
  - Not ported: app-model parent context creation, settings-backed root context,
    or settings event updates.
- Selected pure helpers from `napari._app_model.context._layerlist_context` ->
  `src/app_model/layerlist_context.rs`
  - Covered by `tests/app_model_layerlist_context.rs`.
  - Ported: selected/all layer counts, active layer type/shape/ndim/dtype/RGB
    accessors, selected layer type counts and all-of-type predicates, same-shape
    and same-type checks, 3D image detection, empty-shapes-layer detection,
    active-feature support, and colorbar support checks.
  - Not ported: linked-layer graph lookups, weakref-backed callable context
    keys, real `LayerList`/event integration, or app-model `ContextKey`
    registration.
- `napari.utils.geometry` -> `src/utils/geometry.rs`
  - Covered by `tests/geometry.rs`.
- `napari.utils._indexing` -> `src/utils/indexing.rs`
  - Covered by `tests/indexing.rs`.
- `napari.utils.validators` -> `src/utils/validators.rs`
  - Covered by `tests/validators.rs`.
  - Rust API uses `Result` and explicit error variants instead of Python exceptions.
- Selected pure helpers from `napari.utils.misc` -> `src/utils/misc.rs`
  - Covered by `tests/misc.rs`.
  - Ported: `str_to_rgb`, iterable/sequence normalization equivalents,
    `camel_to_snake`, `camel_to_spaces`, `abspath_or_url`, `ensure_n_tuple`,
    and dimension-order ranking after dimensionality reduction.
  - Not ported: Python runtime inspection helpers, dynamic layer-data tuple
    validation, and array-library-specific equality operator selection.
- `napari.utils._dtype` -> `src/utils/dtype.rs`
  - Covered by `tests/dtype.rs`.
  - Rust API exposes a `DType` enum, `normalize_dtype`, `get_dtype_limits`, and
    `VISPY_TEXTURE_DTYPE`.
- `napari.utils.naming.inc_name_count` and the numbered-name matching behavior ->
  `src/utils/naming.rs`
  - Covered by `tests/naming.rs`.
  - `magic_name` is not ported; it depends on Python stack-frame inspection.
- Selected pure behavior from `napari.utils.notifications` ->
  `src/utils/notifications.rs`
  - Covered by `tests/notifications.rs`.
  - Ported: notification severity string values, icon strings, numeric ordering,
    basic notification formatting, warning notification formatting,
    console-notification threshold filtering, environment-derived manager flags,
    notification record dispatch, info dispatch, and repeated-warning
    de-duplication.
  - Not ported: Python exception hooks, warning hook installation, thread hook
    integration, traceback formatting, event emitters, or global manager side
    effects.
- `napari.utils.status_messages` -> `src/utils/status_messages.rs`
  - Covered by `tests/status_messages.rs`.
  - Ported: Python-style status value formatting, layer coordinate/value status
    string generation, multiscale value tuple handling, and NumPy-like half-even
    coordinate rounding.
- Core list behavior from `napari.utils.history` -> `src/utils/history.rs`
  - Covered by `tests/history.rs`.
  - Ported: recent-folder insertion from a filename, de-duplication by moving an
    existing folder to the front, ten-entry truncation, existing-directory
    filtering, and home-directory fallback.
  - Not ported: napari global settings object integration.
- `napari.utils.task_status` -> `src/utils/task_status.rs`
  - Covered by `tests/task_status.rs`.
  - Ported: task status enum strings, task registration/update, active-status
    filtering, status message generation, cancellation callbacks, and unknown-ID
    update behavior.
  - Rust uses monotonic `TaskStatusId` values and system-time strings instead of
    Python UUID and ISO timestamp objects.
- `napari.utils._units.PREFERRED_VALUES` -> `src/utils/units.rs`
  - Covered by `tests/units.rs`.
  - Ported: preferred scale-bar values.
  - Not ported: lazy Pint application registry accessor.
- Pure data helpers from `napari.utils.perf._stat` and
  `napari.utils.perf._event` -> `src/utils/perf/`
  - Covered by `tests/perf.rs`.
  - Ported: integer min/max/sum/count/average statistics, perf event span and
    origin storage, default phase, end-time updates, and ns/us/ms conversion
    properties, Chrome trace event field conversion, complete-event timer
    statistics, trace-event accumulation, clearing, and dummy disabled-timer
    behavior.
  - Not ported: trace-file JSON writing, config parsing, callable
    monkey-patching, context-manager timers, or Qt integration.
- `napari.utils.config._set` and selected `napari.utils._base` defaults ->
  `src/utils/{config,base}.rs`
  - Covered by `tests/config.rs`.
  - Ported: nonzero environment-value flag semantics, `NAPARI_MON` monitor
    variable name, settings filename, default locale, and config-path joining.
  - Not ported: appdirs user-config-directory discovery.
- `napari.utils.triangulation_backend.TriangulationBackend` ->
  `src/utils/triangulation_backend.rs`
  - Covered by `tests/backend_settings.rs`.
  - Ported: backend string values, enum-name reporting, and Python `_missing_`
    style lookup that normalizes spaces to underscores and lower-case names.
  - Not ported: global shape triangulation backend mutation or numba warmup
    side effects.
- `napari.utils.colormap_backend.ColormapBackend` ->
  `src/utils/colormap_backend.rs`
  - Covered by `tests/backend_settings.rs`.
  - Ported: backend string values, enum-name reporting, and Python `_missing_`
    style lookup that normalizes spaces to underscores and lower-case names.
  - Not ported: global accelerated-colormap backend mutation.
- `napari.utils._env_detection` -> `src/utils/env_detection.rs`
  - Covered by `tests/env_detection.rs`.
  - Ported: isolated-environment enum values, virtualenv and conda-prefix path
    accessors, uv detection from `pyvenv.cfg`, pixi detection from
    `conda-meta/pixi_env_prefix`, and Python's venv-before-conda environment
    detection order.
  - Rust exposes path-based helpers for deterministic tests in addition to the
    environment-variable based `detect_environment` entry point.
- `napari.settings._utils._coerce_extensions_to_globs` ->
  `src/settings/utils.rs`
  - Covered by `tests/settings_utils.rs`.
  - Ported: bare-extension to glob-pattern coercion without mutating the input
    settings map.
- `napari.settings._constants` -> `src/settings/constants.rs`
  - Covered by `tests/settings_constants.rs`.
  - Ported: label dtype values, dims animation loop modes with napari
    `StringEnum`-style case-insensitive value parsing, and brush-size mouse
    modifier setting strings.
- `napari.settings._fields.Version` -> `src/settings/fields.rs`
  - Covered by `tests/settings_fields.rs`.
  - Ported: SemVer core/prerelease/build parsing, tuple conversion,
    Python-style stringification, and numeric-part-only equality/ordering.
  - Not ported: Pydantic schema hooks or dynamic string fields for themes,
    logos, and languages.
- Pure defaults and validation from `napari.settings._application` ->
  `src/settings/application.rs`
  - Covered by `tests/settings_application.rs`.
  - Ported: application-settings scalar/default values, notification-level
    defaults, playback/grid defaults, orientation defaults, brush modifier
    default, label dtype default, Dask enabled default metadata, preferences
    exclusion list, grid spacing/default memory constants, and QByte window-state
    prefix validation.
  - Not ported: Pydantic field metadata, psutil-derived cache sizing, evented
    model behavior, startup-script caller inspection, or labels mouse-binding
    callback integration.
- Pure defaults and validation from `napari.settings._appearance` ->
  `src/settings/appearance.rs`
  - Covered by `tests/settings_appearance.rs`.
  - Ported: built-in theme/logo string validation with lower-case coercion,
    default theme/logo/font size, highlight defaults, layer tooltip/status
    defaults, preferences exclusion list, and the theme-change font-size update
    rule for built-in themes.
  - Not ported: dynamic plugin theme/logo discovery, Pydantic aliases/field
    constraints, evented model behavior, theme JSON schema refresh, or full
    theme color models.
- Pure defaults from `napari.settings._experimental` ->
  `src/settings/experimental.rs`
  - Covered by `tests/backend_settings.rs`.
  - Ported: async/autoswap/RDP/lasso/completion defaults, triangulation and
    colormap backend defaults, compiled-triangulation compatibility default,
    preferences exclusion list, and the 0.6-to-0.7 compiled-triangulation
    migration target selection.
  - Not ported: EventedSettings behavior, Pydantic aliases/field constraints,
    global backend update callbacks, or environment/settings integration.
- Pure defaults from `napari.settings._plugins` -> `src/settings/plugins.rs`
  - Covered by `tests/plugins_settings.rs`.
  - Ported: disabled plugin, extension-reader, and extension-writer defaults;
    plugin hook option metadata; call-order map shape; and preferences exclusion
    list.
  - Not ported: Pydantic settings config, plugin manager integration, or actual
    hook ordering application.
- `napari.utils.camera_orientations` -> `src/utils/camera_orientations.rs`
  - Covered by `tests/camera.rs`.
- `napari.components._viewer_constants` -> `src/components/viewer_constants.rs`
  - Covered by `tests/cursor.rs`.
- Pure, non-evented portions of `napari.components.Camera` ->
  `src/components/camera.rs`
  - Covered by `tests/camera.rs`.
  - Ported: defaults, orientation accessors, handedness, Vispy flip flags, view/up
    direction from Euler angles, view-direction-driven Euler decomposition, and
    nD direction embedding.
  - Not yet ported: evented model behavior.
- `napari.components.Cursor` -> `src/components/cursor.rs`
  - Covered by `tests/cursor.rs`.
- Pure, non-evented portions of `napari.components.Dims` ->
  `src/components/dims.rs`
  - Covered by `tests/dims.rs`.
  - Ported: dimensionality normalization, range/point clipping, step conversion,
    margins/thickness, displayed/not-displayed axis bookkeeping, order and axis
    label padding/cropping, axis bounds validation, transpose, rolling, focus
    movement, reset, and center-step movement.
  - Not yet ported: evented model behavior, Pint unit storage, play state,
    viewer/layer integration, or pydantic validation details.
- `napari.components.GridCanvas` -> `src/components/grid.rs`
  - Covered by `tests/grid.rs`.
  - Ported: default fields, shape calculation, layer position mapping,
    viewbox contents iteration, and canvas spacing calculation.
- `napari.components.Tooltip` -> `src/components/tooltip.rs`
  - Covered by `tests/overlays.rs`.
- Selected overlay models from `napari.components.overlays` ->
  `src/components/overlays/`
  - Covered by `tests/overlays.rs`.
  - Ported: base canvas/scene overlay defaults, axes, brush circle, scale bar,
    text overlays, zoom overlay bounds validation, selection/transform box
    defaults, bounding box defaults, colorbar defaults, labels polygon state and
    reset behavior, welcome overlay static metadata, and blending enum defaults.
  - Not ported: overlay rendering, labels polygon mouse/layer paint integration,
    welcome shortcut resolution, or evented model behavior.
- `napari.layers.base._base_constants` -> `src/layers/base/constants.rs`
  - Covered by `tests/base_constants.rs`.
  - Ported: blending, base layer modes, interaction-box handles including
    opposite/corner helpers, action types, and base projection mode.
- `napari.layers.base._slice._next_request_id` -> `src/layers/base/slice.rs`
  - Covered indirectly by point/vector slice request tests.
  - Ported: monotonic request identifier generation.
- `napari.layers.utils.interaction_box` -> `src/layers/utils/interaction_box.rs`
  - Covered by `tests/interaction_box.rs`.
  - Ported: interaction-box vertices, contained-point bounds, dynamic 2D bounds
    validation, and nearby-handle lookup.
- Selected `napari.layers.utils.color_transformations` helpers ->
  `src/layers/utils/color_transformations.rs`
  - Covered by `tests/color_transformations.rs`.
  - Ported: normalized RGBA color broadcasting/reset behavior.
  - Not ported: arbitrary user color parsing and warning emission.
- Selected `napari.layers.utils.color_encoding` helpers ->
  `src/layers/utils/color_encoding.rs`
  - Covered by `tests/color_encoding.rs`.
  - Ported: Rust-native constant/manual/direct/nominal/quantitative color
    encoding specs, default cyan fallback color, and contrast-limit validation.
  - Not ported: pydantic validation, named-color parsing, categorical or
    quantitative colormap application, or feature-table evaluation.
- Selected `napari.layers.utils.color_manager_utils` helpers ->
  `src/layers/utils/color_manager_utils.rs`
  - Covered by `tests/color_manager_utils.rs`.
  - Ported: continuous/categorical feature guessing and color-mapped argument
    detection, quantitative contrast-limit calculation, and property-value
    normalization against contrast limits for colormap inputs.
  - Not ported: colormap object application or evented `ColorManager`
    validation.
- `napari.layers.utils._text_constants` and selected
  `napari.layers.utils._text_utils` helpers ->
  `src/layers/utils/text_constants.rs` and `src/layers/utils/text_utils.rs`
  - Covered by `tests/text_utils.rs`.
  - Ported: text anchor enum values and parsing, bounding-box center/extents
    calculation, 2D corner anchor calculation, and 3D fallback-to-center
    behavior.
- `napari.layers.utils._color_manager_constants` ->
  `src/layers/utils/color_manager_constants.rs`
  - Covered by `tests/color_manager_constants.rs`.
  - Ported: shared color manager mode string enum values and parsing.
- Selected structural helpers from `napari.layers.utils._slice_input` ->
  `src/layers/utils/slice_input.rs`
  - Covered by `tests/slice_input.rs`.
  - Ported: thick nD slice construction, ndim padding/cropping, row conversion,
    axis selection, per-dimension iteration, displayed/not-displayed axis
    selection, slice input dimensionality adjustment, and orthogonality checks
    from linear transform matrices, affine data-slice conversion, plus
    data-to-world unit scaling for supported transform units.
  - Not ported: dims model integration.
- Selected structural helpers from `napari.layers.utils.stack_utils` ->
  `src/layers/utils/stack_utils.rs`
  - Covered by `tests/stack_utils.rs`.
  - Ported: metadata-level single-axis slicing shape calculation, channel split
    shape calculation, multiscale channel split shape calculation, and default
    channel colormap/blending metadata selection.
  - Not ported: concrete array slicing, zarr/dask wrapping, layer object
    creation, RGB merge/split, or image stack merging.
- Selected `napari.layers.utils.string_encoding` helpers ->
  `src/layers/utils/string_encoding.rs`
  - Covered by `tests/string_encoding.rs`.
  - Ported: format-string detection, format field extraction, and Rust-native
    direct/format/manual string encoding selection.
  - Not ported: pandas-backed string encoding evaluation or pydantic schemas.
- Selected `napari.layers.utils.style_encoding` helpers ->
  `src/layers/utils/style_encoding.rs`
  - Covered by `tests/style_encoding.rs`.
  - Ported: generic constant/manual/derived style encoding cached-value behavior
    for apply, append, delete, and clear, including derived tail caching and
    fallback values for failed derivations, plus selected style-value indexing.
  - Not ported: evented model behavior, JSON schema handling, or dataframe
    derived encodings.
- Core data behavior from `napari.layers.utils.plane` ->
  `src/layers/utils/plane.rs`
  - Covered by `tests/plane.rs`.
  - Ported: normalized plane construction, plane creation from three points,
    shifting along the normal vector, line intersection, slicing/clipping plane
    array conversion, clipping-plane list array conversion, and bounding-box
    clipping plane generation.
  - Not ported: evented model/list behavior.
- Selected pure helpers from `napari.layers.utils.interactivity_utils` ->
  `src/layers/utils/interactivity_utils.rs`
  - Covered by `tests/interactivity_utils.rs`.
  - Ported: displayed plane extraction from an nD line segment, drag distance
    projection onto data vectors, and displayed data ray construction.
  - Not ported: viewer/layer mutation in `orient_plane_normal_around_cursor`.
- Selected pure helpers from `napari.layers.utils.layer_utils` ->
  `src/layers/utils/layer_utils.rs`
  - Covered by `tests/layer_utils.rs`.
  - Ported: finite min/max fallbacks, simple data range calculation, segment
    normals, current property extraction/coercion, property length validation,
    property choice normalization, displayed world-to-layer dimension mapping,
    multiscale level/corner selection, unique-element detection, and affine
    transform coercion, and world-extent calculation from transformed data
    extent corners.
  - Not ported: action-manager decorators, pandas/dask-specific behavior,
    feature tables, or layer/list extent model aggregation.
- `napari.layers._source` -> `src/layers/source.rs`
  - Covered by `tests/source.rs`.
  - Ported: source provenance fields and nested source context override/reset
    behavior. Parent source is represented by an ID until concrete layer objects
    exist.
- `napari.layers._multiscale_data` -> `src/layers/multiscale_data.rs`
  - Covered by `tests/multiscale_data.rs`.
  - Ported as metadata-level multiscale wrapper: level metadata, first-scale
  size/ndim/dtype/shape, shapes for all levels, indexing, length, and lowest
  resolution lookup.
- Runtime requirement reporting from `napari.layers._data_protocols` ->
  `src/layers/data_protocols.rs`
  - Covered by `tests/data_protocols.rs`.
  - Ported: required layer data protocol members, missing-method collection,
    custom protocol name support, and helpful error formatting.
  - Rust represents protocol conformance as explicit metadata because Python's
    runtime structural `Protocol` checks do not map directly to Rust values.
- Core contrast/gamma state from `napari.layers.intensity_mixin` ->
  `src/layers/intensity.rs`
  - Covered by `tests/intensity.rs`.
  - Ported: default gamma and contrast state, gamma setter, contrast-limit
    validation and formatted message generation, range expansion from contrast
    limits, range-boundary preservation for `None`, clipping/resetting contrast
    limits when the valid range changes, and integer dtype range reset behavior.
  - Not ported: event emission, thumbnail updates, colormap objects, colorbar
    overlay integration, or layer-specific data-range calculation.
- Selected pure helpers from `napari.layers._scalar_field._slice` ->
  `src/layers/scalar_field/slice.rs`
  - Covered by `tests/scalar_field_slice.rs`.
  - Ported: point-to-index/slice conversion, thick data-slice range conversion,
    displayed-axis transpose ordering including RGB final-axis handling, and
    scalar-field out-of-bounds checks for point and thick projection modes.
  - Not ported: actual array slicing/projection, dask indexing, multiscale tile
    selection, async request execution, or image/labels layer integration.
- `napari.layers.image._image_constants` -> `src/layers/image/constants.rs`
  - Covered by `tests/image_constants.rs`.
  - Ported: interpolation modes including the viewer subset helper, image
    rendering modes, volume depiction modes, and image projection modes.
- Selected `napari.layers.image._image_utils` helpers ->
  `src/layers/image/image_utils.rs`
  - Covered by `tests/image_utils.rs`.
  - Ported at metadata level: RGB/RGBA shape guessing, multiscale sequence
    guessing by strictly decreasing total size, and integer dtype label/image
    guessing.
- Layer-specific constants from labels, points, vectors, surface, and shapes ->
  `src/layers/{labels,points,vectors,surface,shapes}/constants.rs`
  - Covered by `tests/labels_constants.rs`, `tests/points_constants.rs`,
    `tests/vectors_constants.rs`, `tests/surface_constants.rs`, and
    `tests/shapes_constants.rs`.
  - Ported: string enum values and parsing, labels/shapes backspace platform
    constants, points symbol aliases, and shapes interaction-box index
    constants.
- Selected pure helpers from `napari.layers.labels._labels_utils` ->
  `src/layers/labels/labels_utils.rs`
  - Covered by `tests/labels_utils.rs`.
  - Ported: brush coordinate interpolation, scaled sphere/ellipsoid index
    generation, coordinate filtering by shape, dense first-nonzero ray lookup,
    and slice expansion.
  - Not ported: SciPy contour extraction, dtype introspection against live
    layer data, or mouse-event/layer conversion.
- Selected pure helpers from `napari.layers.points._points_utils` ->
  `src/layers/points/points_utils.rs`
  - Covered by `tests/points_utils.rs`.
  - Ported: 2D/3D selection-box construction, point square expansion,
    2D/3D point-in-box selection, point data normalization, and symbol
    conversion/coercion.
- Pure slicing behavior from `napari.layers.points._slice` ->
  `src/layers/points/slice.rs`
  - Covered by `tests/point_vector_slices.rs`.
  - Ported: empty slice responses, all-displayed fast path, exact and thick
    not-displayed-axis slice inclusion, half-pixel handling for zero-thickness
    slices, and out-of-slice point scaling.
  - Not ported: threaded slicing/request orchestration or full `Points` layer
    integration.
- Selected pure geometry helpers from `napari.layers.shapes._shapes_utils` ->
  `src/layers/shapes/shapes_utils.rs`
  - Covered by `tests/shapes_utils.rs`.
  - Ported: planar-axis detection, fan triangulation indices, origin-in-box
    checks, triangle/box intersection checks, line intersection predicates,
    collinearity, nearest-line lookup, interaction-box creation, rectangle box
    conversion, corner finding, center/radii corner expansion, default shape
    type selection, shape dimensionality detection, shape counting,
    embedded shape-type extraction, vertex-count validation,
    point-in-polygon ray casting, grid mask generation from polygon vertices,
    path-to-mask rasterization, ellipse triangulation, triangle centroid
    culling against polygons, planar-axis vertex restoration, perpendicular
    distance, and RDP simplification.
  - Not ported: VisPy/skimage/triangle polygon triangulation paths, skimage
    polygon-to-mask behavior, edge mesh reconstruction, tube mesh generation,
    or serialization of failed triangulations.
- `napari.layers.vectors._vector_utils` ->
  `src/layers/vectors/vector_utils.rs`
  - Covered by `tests/vector_utils.rs`.
  - Ported: image-like vector data coordinate-grid conversion, empty/default
    dimensionality behavior, single-vector normalization, coordinate-like data
    validation, and ndim mismatch errors.
- Pure slicing behavior from `napari.layers.vectors._slice` ->
  `src/layers/vectors/slice.rs`
  - Covered by `tests/point_vector_slices.rs`.
  - Ported: empty slice responses, all-displayed fast path, exact and thick
    not-displayed-axis slice inclusion by vector start point, half-pixel
    handling for zero-thickness slices, and out-of-slice vector alpha scaling.
  - Not ported: threaded slicing/request orchestration or full `Vectors` layer
    integration.
- `napari.layers.surface._surface_utils` ->
  `src/layers/surface/surface_utils.rs`
  - Covered by `tests/surface_utils.rs`.
  - Ported: barycentric coordinate calculation for a point in a triangle and
    validation for invalid triangle inputs.
- Pure state from `napari.layers.surface.normals` and
  `napari.layers.surface.wireframe` ->
  `src/layers/surface/{normals,wireframe}.rs`
  - Covered by `tests/surface_display_models.rs`.
  - Ported: face/vertex normals defaults, immutable normal mode storage, partial
    update/reset behavior, and surface wireframe defaults/update/reset behavior.
  - Not ported: evented model behavior, Pydantic validation, named-color parsing,
    or rendering integration with the `Surface` layer.
- Selected pure metadata helpers from `napari.layers.tracks._track_utils` ->
  `src/layers/tracks/track_utils.rs`
  - Covered by `tests/track_utils.rs`.
  - Ported: track data validation and sorting, time-to-slice lookup,
    track-id/unique-id accessors, track vertex connection arrays, graph
    normalization and graph edge arrays, track time/end-time calculations,
    completed-track masking, and current-time track labels.
  - Not ported: pandas feature table integration, SciPy sparse/KDTree lookup,
    the evented `Tracks` layer model, or rendering/coloring behavior.
- Selected `napari.utils.transforms.transform_utils` helpers ->
  `src/utils/transforms/transform_utils.rs`
  - Covered by `tests/transform_utils.rs`.
  - Ported: vector padding for scale/translate, 2D/3D rotation matrices,
    shear expansion, matrix embedding, linear composition/decomposition order,
    matrix inversion, dimensionality inference, shear-from-angle, triangular
    checks, and diagonal checks.
- Selected `napari.utils.transforms._units` helpers ->
  `src/utils/transforms/units.rs`
  - Covered by `tests/transform_units.rs`.
  - Ported: pixel default, common Pint-like distance unit name parsing, sequence
    conversion, and unknown-unit errors.
  - Not ported: arbitrary Pint registry parsing or unit algebra.
- `napari.utils.transforms.ScaleTranslate` ->
  `src/utils/transforms/scale_translate.rs`
  - Covered by `tests/scale_translate.rs`.
  - Ported: scale/translate broadcasting, point/batch application, inverse,
    composition, slicing, dimension expansion, and identity default.
- `napari.utils.transforms.Affine`, `CompositeAffine`, and `TransformChain`
  - `src/utils/transforms/affine.rs`
  - Covered by `tests/transforms_affine.rs`.
  - Partially ported: construction, application, inverse, compose (for affine-like
    members), slicing, expansion, simplified chain composition, and decompose-driven
    scale/rotate/shear recomposition, plus axis-label/unit metadata and physical
    scale reporting.
  - Not yet ported: evented behavior and Python event-model parity (`changed`,
    metadata events, and validation detail).

## Current Verification

Run:

```sh
cargo fmt --all --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
