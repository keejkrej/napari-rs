use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Source {
    pub path: Option<String>,
    pub reader_plugin: Option<String>,
    pub sample: Option<(String, String)>,
    pub widget: Option<String>,
    pub parent: Option<usize>,
}

impl Source {
    pub fn merge_overrides(&self, overrides: SourceOverrides) -> Self {
        Self {
            path: overrides.path.or_else(|| self.path.clone()),
            reader_plugin: overrides
                .reader_plugin
                .or_else(|| self.reader_plugin.clone()),
            sample: overrides.sample.or_else(|| self.sample.clone()),
            widget: overrides.widget.or_else(|| self.widget.clone()),
            parent: overrides.parent.or(self.parent),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceOverrides {
    pub path: Option<String>,
    pub reader_plugin: Option<String>,
    pub sample: Option<(String, String)>,
    pub widget: Option<String>,
    pub parent: Option<usize>,
}

impl SourceOverrides {
    pub fn path(path: impl Into<String>) -> Self {
        Self {
            path: Some(path.into()),
            ..Self::default()
        }
    }

    pub fn reader_plugin(reader_plugin: impl Into<String>) -> Self {
        Self {
            reader_plugin: Some(reader_plugin.into()),
            ..Self::default()
        }
    }

    pub fn sample(plugin: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            sample: Some((plugin.into(), name.into())),
            ..Self::default()
        }
    }

    pub fn widget(widget: impl Into<String>) -> Self {
        Self {
            widget: Some(widget.into()),
            ..Self::default()
        }
    }

    pub fn parent(parent: usize) -> Self {
        Self {
            parent: Some(parent),
            ..Self::default()
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_reader_plugin(mut self, reader_plugin: impl Into<String>) -> Self {
        self.reader_plugin = Some(reader_plugin.into());
        self
    }

    pub fn with_sample(mut self, plugin: impl Into<String>, name: impl Into<String>) -> Self {
        self.sample = Some((plugin.into(), name.into()));
        self
    }

    pub fn with_widget(mut self, widget: impl Into<String>) -> Self {
        self.widget = Some(widget.into());
        self
    }

    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }
}

thread_local! {
    static LAYER_SOURCE: RefCell<Vec<Source>> = const { RefCell::new(Vec::new()) };
}

pub fn current_source() -> Source {
    LAYER_SOURCE.with(|sources| sources.borrow().last().cloned().unwrap_or_default())
}

pub fn with_layer_source<R>(overrides: SourceOverrides, body: impl FnOnce() -> R) -> R {
    let merged = current_source().merge_overrides(overrides);
    let guard = LayerSourceGuard::push(merged);
    let result = body();
    drop(guard);
    result
}

pub struct LayerSourceGuard;

impl LayerSourceGuard {
    pub fn push(source: Source) -> Self {
        LAYER_SOURCE.with(|sources| sources.borrow_mut().push(source));
        Self
    }
}

impl Drop for LayerSourceGuard {
    fn drop(&mut self) {
        LAYER_SOURCE.with(|sources| {
            sources.borrow_mut().pop();
        });
    }
}
