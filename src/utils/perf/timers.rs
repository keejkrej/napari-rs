use std::collections::BTreeMap;

use crate::utils::perf::event::PerfEvent;
use crate::utils::perf::stat::Stat;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PerfTimers {
    pub timers: BTreeMap<String, Stat>,
    pub trace_events: Option<Vec<PerfEvent>>,
}

impl PerfTimers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, event: PerfEvent) {
        if let Some(trace_events) = &mut self.trace_events {
            trace_events.push(event.clone());
        }

        if event.phase == "X" {
            let duration_ms = event.duration_ms() as i64;
            self.timers
                .entry(event.name)
                .and_modify(|stat| stat.add(duration_ms))
                .or_insert_with(|| Stat::new(duration_ms));
        }
    }

    pub fn clear(&mut self) {
        self.timers.clear();
    }

    pub fn start_trace(&mut self) {
        self.trace_events = Some(Vec::new());
    }

    pub fn stop_trace(&mut self) -> Option<Vec<PerfEvent>> {
        self.trace_events.take()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DummyTimer;

impl DummyTimer {
    pub fn add_event(&mut self, _event: PerfEvent) {}

    pub fn clear(&mut self) {}

    pub fn start_trace(&mut self) {}

    pub fn stop_trace(&mut self) -> Option<Vec<PerfEvent>> {
        None
    }
}
