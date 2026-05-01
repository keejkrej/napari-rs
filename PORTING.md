# Porting Status

Source reference: sibling Python tree at `../napari`.

## Ported

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
- `napari.utils.camera_orientations` -> `src/utils/camera_orientations.rs`
  - Covered by `tests/camera.rs`.
- `napari.components._viewer_constants` -> `src/components/viewer_constants.rs`
  - Covered by `tests/cursor.rs`.
- Pure, non-evented portions of `napari.components.Camera` ->
  `src/components/camera.rs`
  - Covered by `tests/camera.rs`.
  - Ported: defaults, orientation accessors, handedness, Vispy flip flags, view/up
    direction from Euler angles, and nD direction embedding.
  - Not yet ported: evented model behavior and `set_view_direction` Euler
    decomposition.
- `napari.components.Cursor` -> `src/components/cursor.rs`
  - Covered by `tests/cursor.rs`.
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
    defaults, and blending enum defaults.
- `napari.layers.base._base_constants` -> `src/layers/base/constants.rs`
  - Covered by `tests/base_constants.rs`.
  - Ported: blending, base layer modes, interaction-box handles including
    opposite/corner helpers, action types, and base projection mode.
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
    from linear transform matrices.
  - Not ported: affine data slicing, unit scaling, or dims model integration.
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
    multiscale level/corner selection, and unique-element detection.
  - Not ported: action-manager decorators, pandas/dask-specific behavior,
    feature tables, affine coercion, or full extent transformation.
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
- `napari.layers.surface._surface_utils` ->
  `src/layers/surface/surface_utils.rs`
  - Covered by `tests/surface_utils.rs`.
  - Ported: barycentric coordinate calculation for a point in a triangle and
    validation for invalid triangle inputs.
- Selected `napari.utils.transforms.transform_utils` helpers ->
  `src/utils/transforms/transform_utils.rs`
  - Covered by `tests/transform_utils.rs`.
  - Ported: vector padding for scale/translate, 2D/3D rotation matrices,
    shear expansion, matrix embedding, linear composition order, dimensionality
    inference, shear-from-angle, triangular checks, and diagonal checks.
- `napari.utils.transforms.ScaleTranslate` ->
  `src/utils/transforms/scale_translate.rs`
  - Covered by `tests/scale_translate.rs`.
  - Ported: scale/translate broadcasting, point/batch application, inverse,
    composition, slicing, dimension expansion, and identity default.
  - Not yet ported: full `Affine`, `CompositeAffine`, and evented
    `TransformChain` behavior.

## Current Verification

Run:

```sh
cargo fmt --all --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
