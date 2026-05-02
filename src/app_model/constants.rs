use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseMenuIdError {
    value: String,
}

impl fmt::Display for ParseMenuIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown menu id: {}", self.value)
    }
}

impl std::error::Error for ParseMenuIdError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuId {
    MenubarFile,
    FileOpenWithPlugin,
    FileSamples,
    FileNewLayer,
    FileIoUtilities,
    FileAcquire,
    MenubarView,
    ViewAxes,
    ViewScalebar,
    MenubarLayers,
    LayersVisualize,
    LayersAnnotate,
    LayersData,
    LayersLayerType,
    LayersTransform,
    LayersMeasure,
    LayersFilter,
    LayersRegister,
    LayersProject,
    LayersSegment,
    LayersTrack,
    LayersClassify,
    MenubarWindow,
    MenubarPlugins,
    MenubarHelp,
    MenubarDebug,
    DebugPerformance,
    LayerlistContext,
    LayersContextConvertDtype,
    LayersContextProject,
    LayersContextCopySpatial,
    LayersContextVisualization,
}

impl MenuId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenubarFile => "napari/file",
            Self::FileOpenWithPlugin => "napari/file/open_with_plugin",
            Self::FileSamples => "napari/file/samples",
            Self::FileNewLayer => "napari/file/new_layer",
            Self::FileIoUtilities => "napari/file/io_utilities",
            Self::FileAcquire => "napari/file/acquire",
            Self::MenubarView => "napari/view",
            Self::ViewAxes => "napari/view/axes",
            Self::ViewScalebar => "napari/view/scalebar",
            Self::MenubarLayers => "napari/layers",
            Self::LayersVisualize => "napari/layers/visualize",
            Self::LayersAnnotate => "napari/layers/annotate",
            Self::LayersData => "napari/layers/data",
            Self::LayersLayerType => "napari/layers/layer_type",
            Self::LayersTransform => "napari/layers/transform",
            Self::LayersMeasure => "napari/layers/measure",
            Self::LayersFilter => "napari/layers/filter",
            Self::LayersRegister => "napari/layers/register",
            Self::LayersProject => "napari/layers/project",
            Self::LayersSegment => "napari/layers/segment",
            Self::LayersTrack => "napari/layers/track",
            Self::LayersClassify => "napari/layers/classify",
            Self::MenubarWindow => "napari/window",
            Self::MenubarPlugins => "napari/plugins",
            Self::MenubarHelp => "napari/help",
            Self::MenubarDebug => "napari/debug",
            Self::DebugPerformance => "napari/debug/performance_trace",
            Self::LayerlistContext => "napari/layers/context",
            Self::LayersContextConvertDtype => "napari/layers/context/convert_dtype",
            Self::LayersContextProject => "napari/layers/contxt/project",
            Self::LayersContextCopySpatial => "napari/layers/context/copy_spatial",
            Self::LayersContextVisualization => "napari/layers/context/visualization",
        }
    }

    pub const fn contributables() -> &'static [MenuId] {
        &[
            Self::FileIoUtilities,
            Self::FileAcquire,
            Self::FileNewLayer,
            Self::LayersVisualize,
            Self::LayersAnnotate,
            Self::LayersData,
            Self::LayersLayerType,
            Self::LayersFilter,
            Self::LayersTransform,
            Self::LayersMeasure,
            Self::LayersRegister,
            Self::LayersProject,
            Self::LayersSegment,
            Self::LayersTrack,
            Self::LayersClassify,
        ]
    }

    pub fn is_contributable(self) -> bool {
        Self::contributables().contains(&self)
    }
}

impl fmt::Display for MenuId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MenuId {
    type Err = ParseMenuIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "napari/file" => Ok(Self::MenubarFile),
            "napari/file/open_with_plugin" => Ok(Self::FileOpenWithPlugin),
            "napari/file/samples" => Ok(Self::FileSamples),
            "napari/file/new_layer" => Ok(Self::FileNewLayer),
            "napari/file/io_utilities" => Ok(Self::FileIoUtilities),
            "napari/file/acquire" => Ok(Self::FileAcquire),
            "napari/view" => Ok(Self::MenubarView),
            "napari/view/axes" => Ok(Self::ViewAxes),
            "napari/view/scalebar" => Ok(Self::ViewScalebar),
            "napari/layers" => Ok(Self::MenubarLayers),
            "napari/layers/visualize" => Ok(Self::LayersVisualize),
            "napari/layers/annotate" => Ok(Self::LayersAnnotate),
            "napari/layers/data" => Ok(Self::LayersData),
            "napari/layers/layer_type" => Ok(Self::LayersLayerType),
            "napari/layers/transform" => Ok(Self::LayersTransform),
            "napari/layers/measure" => Ok(Self::LayersMeasure),
            "napari/layers/filter" => Ok(Self::LayersFilter),
            "napari/layers/register" => Ok(Self::LayersRegister),
            "napari/layers/project" => Ok(Self::LayersProject),
            "napari/layers/segment" => Ok(Self::LayersSegment),
            "napari/layers/track" => Ok(Self::LayersTrack),
            "napari/layers/classify" => Ok(Self::LayersClassify),
            "napari/window" => Ok(Self::MenubarWindow),
            "napari/plugins" => Ok(Self::MenubarPlugins),
            "napari/help" => Ok(Self::MenubarHelp),
            "napari/debug" => Ok(Self::MenubarDebug),
            "napari/debug/performance_trace" => Ok(Self::DebugPerformance),
            "napari/layers/context" => Ok(Self::LayerlistContext),
            "napari/layers/context/convert_dtype" => Ok(Self::LayersContextConvertDtype),
            "napari/layers/contxt/project" => Ok(Self::LayersContextProject),
            "napari/layers/context/copy_spatial" => Ok(Self::LayersContextCopySpatial),
            "napari/layers/context/visualization" => Ok(Self::LayersContextVisualization),
            _ => Err(ParseMenuIdError {
                value: value.to_string(),
            }),
        }
    }
}

pub fn is_menu_contributable(menu_id: &str) -> bool {
    if menu_id.starts_with("napari/") {
        menu_id
            .parse::<MenuId>()
            .is_ok_and(MenuId::is_contributable)
    } else {
        true
    }
}

pub mod menu_group {
    pub const NAVIGATION: &str = "navigation";
    pub const RENDER: &str = "1_render";
    pub const ZOOM: &str = "zoom";
    pub const PLUGINS: &str = "1_plugins";
    pub const PLUGIN_MULTI_SUBMENU: &str = "2_plugin_multi_submenu";
    pub const PLUGIN_SINGLE_CONTRIBUTIONS: &str = "3_plugin_contributions";
    pub const OPEN: &str = "1_open";
    pub const UTIL: &str = "2_util";
    pub const PREFERENCES: &str = "3_preferences";
    pub const SAVE: &str = "4_save";
    pub const CLOSE: &str = "5_close";

    pub mod layerlist_context {
        pub const CONVERSION: &str = "1_conversion";
        pub const COPY_SPATIAL: &str = "4_copy_spatial";
        pub const SPLIT_MERGE: &str = "5_split_merge";
        pub const LINK: &str = "9_link";
    }

    pub mod layers {
        pub const CONVERT: &str = "1_convert";
        pub const GEOMETRY: &str = "2_geometry";
        pub const GENERATE: &str = "3_generate";
    }
}
