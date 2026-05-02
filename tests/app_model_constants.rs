use napari_rs::app_model::constants::{MenuId, is_menu_contributable, menu_group};

#[test]
fn menu_id_strings_match_python_values() {
    assert_eq!(MenuId::MenubarFile.to_string(), "napari/file");
    assert_eq!(
        MenuId::FileOpenWithPlugin.to_string(),
        "napari/file/open_with_plugin"
    );
    assert_eq!(MenuId::FileSamples.to_string(), "napari/file/samples");
    assert_eq!(MenuId::FileNewLayer.to_string(), "napari/file/new_layer");
    assert_eq!(
        MenuId::FileIoUtilities.to_string(),
        "napari/file/io_utilities"
    );
    assert_eq!(MenuId::FileAcquire.to_string(), "napari/file/acquire");
    assert_eq!(MenuId::MenubarView.to_string(), "napari/view");
    assert_eq!(MenuId::ViewAxes.to_string(), "napari/view/axes");
    assert_eq!(MenuId::ViewScalebar.to_string(), "napari/view/scalebar");
    assert_eq!(MenuId::MenubarLayers.to_string(), "napari/layers");
    assert_eq!(
        MenuId::LayersVisualize.to_string(),
        "napari/layers/visualize"
    );
    assert_eq!(MenuId::LayersAnnotate.to_string(), "napari/layers/annotate");
    assert_eq!(MenuId::LayersData.to_string(), "napari/layers/data");
    assert_eq!(
        MenuId::LayersLayerType.to_string(),
        "napari/layers/layer_type"
    );
    assert_eq!(
        MenuId::LayersTransform.to_string(),
        "napari/layers/transform"
    );
    assert_eq!(MenuId::LayersMeasure.to_string(), "napari/layers/measure");
    assert_eq!(MenuId::LayersFilter.to_string(), "napari/layers/filter");
    assert_eq!(MenuId::LayersRegister.to_string(), "napari/layers/register");
    assert_eq!(MenuId::LayersProject.to_string(), "napari/layers/project");
    assert_eq!(MenuId::LayersSegment.to_string(), "napari/layers/segment");
    assert_eq!(MenuId::LayersTrack.to_string(), "napari/layers/track");
    assert_eq!(MenuId::LayersClassify.to_string(), "napari/layers/classify");
    assert_eq!(MenuId::MenubarWindow.to_string(), "napari/window");
    assert_eq!(MenuId::MenubarPlugins.to_string(), "napari/plugins");
    assert_eq!(MenuId::MenubarHelp.to_string(), "napari/help");
    assert_eq!(MenuId::MenubarDebug.to_string(), "napari/debug");
    assert_eq!(
        MenuId::DebugPerformance.to_string(),
        "napari/debug/performance_trace"
    );
    assert_eq!(
        MenuId::LayerlistContext.to_string(),
        "napari/layers/context"
    );
    assert_eq!(
        MenuId::LayersContextConvertDtype.to_string(),
        "napari/layers/context/convert_dtype"
    );
    assert_eq!(
        MenuId::LayersContextProject.to_string(),
        "napari/layers/contxt/project"
    );
    assert_eq!(
        MenuId::LayersContextCopySpatial.to_string(),
        "napari/layers/context/copy_spatial"
    );
    assert_eq!(
        MenuId::LayersContextVisualization.to_string(),
        "napari/layers/context/visualization"
    );

    assert_eq!(
        "napari/file".parse::<MenuId>().unwrap(),
        MenuId::MenubarFile
    );
    assert_eq!(
        "napari/layers/contxt/project".parse::<MenuId>().unwrap(),
        MenuId::LayersContextProject
    );
    assert!("napari/File".parse::<MenuId>().is_err());
}

#[test]
fn menu_contributability_matches_python_helper() {
    assert!(MenuId::FileIoUtilities.is_contributable());
    assert!(MenuId::LayersClassify.is_contributable());
    assert!(!MenuId::MenubarFile.is_contributable());
    assert!(!MenuId::LayerlistContext.is_contributable());

    assert!(is_menu_contributable("napari/file/io_utilities"));
    assert!(!is_menu_contributable("napari/file"));
    assert!(!is_menu_contributable("napari/not_registered"));
    assert!(is_menu_contributable("plugin/menu"));
    assert!(is_menu_contributable("other"));
}

#[test]
fn menu_group_strings_match_python_values() {
    assert_eq!(menu_group::NAVIGATION, "navigation");
    assert_eq!(menu_group::RENDER, "1_render");
    assert_eq!(menu_group::ZOOM, "zoom");
    assert_eq!(menu_group::PLUGINS, "1_plugins");
    assert_eq!(menu_group::PLUGIN_MULTI_SUBMENU, "2_plugin_multi_submenu");
    assert_eq!(
        menu_group::PLUGIN_SINGLE_CONTRIBUTIONS,
        "3_plugin_contributions"
    );
    assert_eq!(menu_group::OPEN, "1_open");
    assert_eq!(menu_group::UTIL, "2_util");
    assert_eq!(menu_group::PREFERENCES, "3_preferences");
    assert_eq!(menu_group::SAVE, "4_save");
    assert_eq!(menu_group::CLOSE, "5_close");
    assert_eq!(menu_group::layerlist_context::CONVERSION, "1_conversion");
    assert_eq!(
        menu_group::layerlist_context::COPY_SPATIAL,
        "4_copy_spatial"
    );
    assert_eq!(menu_group::layerlist_context::SPLIT_MERGE, "5_split_merge");
    assert_eq!(menu_group::layerlist_context::LINK, "9_link");
    assert_eq!(menu_group::layers::CONVERT, "1_convert");
    assert_eq!(menu_group::layers::GEOMETRY, "2_geometry");
    assert_eq!(menu_group::layers::GENERATE, "3_generate");
}
