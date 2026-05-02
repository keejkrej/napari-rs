use std::collections::BTreeMap;
use std::fmt;

use crate::utils::perf::event::PerfEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum TraceValue {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChromeTraceEventData {
    pub pid: u32,
    pub tid: String,
    pub name: String,
    pub cat: String,
    pub ph: String,
    pub ts: f64,
    pub args: BTreeMap<String, f64>,
    pub dur: Option<f64>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedTracePhase {
    pub phase: String,
}

impl fmt::Display for UnsupportedTracePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unsupported trace event phase: {}", self.phase)
    }
}

impl std::error::Error for UnsupportedTracePhase {}

pub fn chrome_trace_event_data(
    event: &PerfEvent,
) -> Result<ChromeTraceEventData, UnsupportedTracePhase> {
    let mut data = ChromeTraceEventData {
        pid: event.origin.process_id,
        tid: event.origin.thread_id.clone(),
        name: event.name.clone(),
        cat: event.category.clone().unwrap_or_else(|| "none".to_owned()),
        ph: event.phase.clone(),
        ts: event.start_us(),
        args: event.args.clone(),
        dur: None,
        scope: None,
    };

    match event.phase.as_str() {
        "X" => data.dur = Some(event.duration_us()),
        "I" => data.scope = Some("p".to_owned()),
        "C" => {}
        _ => {
            return Err(UnsupportedTracePhase {
                phase: event.phase.clone(),
            });
        }
    }

    Ok(data)
}
