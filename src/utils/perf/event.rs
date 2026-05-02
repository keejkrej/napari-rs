use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start_ns: u64,
    pub end_ns: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    pub process_id: u32,
    pub thread_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerfEvent {
    pub name: String,
    pub span: Span,
    pub category: Option<String>,
    pub origin: Origin,
    pub args: BTreeMap<String, f64>,
    pub phase: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerfEventOptions {
    pub category: Option<String>,
    pub process_id: Option<u32>,
    pub thread_id: Option<String>,
    pub phase: String,
    pub args: BTreeMap<String, f64>,
}

impl Default for PerfEventOptions {
    fn default() -> Self {
        Self {
            category: None,
            process_id: None,
            thread_id: None,
            phase: "X".to_owned(),
            args: BTreeMap::new(),
        }
    }
}

impl PerfEvent {
    pub fn new(name: impl Into<String>, start_ns: u64, end_ns: u64) -> Self {
        Self::with_options(name, start_ns, end_ns, PerfEventOptions::default())
    }

    pub fn with_options(
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
        options: PerfEventOptions,
    ) -> Self {
        Self {
            name: name.into(),
            span: Span { start_ns, end_ns },
            category: options.category,
            origin: Origin {
                process_id: options.process_id.unwrap_or_else(std::process::id),
                thread_id: options
                    .thread_id
                    .unwrap_or_else(|| format!("{:?}", std::thread::current().id())),
            },
            args: options.args,
            phase: options.phase,
        }
    }

    pub fn update_end_ns(&mut self, end_ns: u64) {
        self.span.end_ns = end_ns;
    }

    pub fn start_us(&self) -> f64 {
        self.span.start_ns as f64 / 1e3
    }

    pub fn start_ms(&self) -> f64 {
        self.span.start_ns as f64 / 1e6
    }

    pub fn duration_ns(&self) -> u64 {
        self.span.end_ns - self.span.start_ns
    }

    pub fn duration_us(&self) -> f64 {
        self.duration_ns() as f64 / 1e3
    }

    pub fn duration_ms(&self) -> f64 {
        self.duration_ns() as f64 / 1e6
    }
}
