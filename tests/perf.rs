use std::collections::BTreeMap;

use napari_rs::utils::perf::event::{PerfEvent, PerfEventOptions};
use napari_rs::utils::perf::stat::Stat;
use napari_rs::utils::perf::timers::{DummyTimer, PerfTimers};
use napari_rs::utils::perf::trace_file::{UnsupportedTracePhase, chrome_trace_event_data};

#[test]
fn stat_tracks_min_max_sum_count_and_integer_average_like_python() {
    let mut stat = Stat::new(10);

    assert_eq!(stat.min, 10);
    assert_eq!(stat.max, 10);
    assert_eq!(stat.sum, 10);
    assert_eq!(stat.count, 1);
    assert_eq!(stat.average(), 10);

    stat.add(5);
    stat.add(20);

    assert_eq!(stat.min, 5);
    assert_eq!(stat.max, 20);
    assert_eq!(stat.sum, 35);
    assert_eq!(stat.count, 3);
    assert_eq!(stat.average(), 11);
}

#[test]
fn stat_average_uses_python_floor_division_for_negative_sums() {
    let mut stat = Stat::new(-2);
    stat.add(1);

    assert_eq!(stat.average(), -1);
}

#[test]
fn perf_event_fields_and_time_conversions_match_python_properties() {
    let mut args = BTreeMap::new();
    args.insert("value".to_owned(), 42.0);
    let mut event = PerfEvent::with_options(
        "draw",
        1_500,
        4_500,
        PerfEventOptions {
            category: Some("render,update".to_owned()),
            process_id: Some(7),
            thread_id: Some("thread-1".to_owned()),
            args: args.clone(),
            ..PerfEventOptions::default()
        },
    );

    assert_eq!(event.name, "draw");
    assert_eq!(event.span.start_ns, 1_500);
    assert_eq!(event.span.end_ns, 4_500);
    assert_eq!(event.category.as_deref(), Some("render,update"));
    assert_eq!(event.origin.process_id, 7);
    assert_eq!(event.origin.thread_id, "thread-1");
    assert_eq!(event.phase, "X");
    assert_eq!(event.args, args);
    assert_eq!(event.start_us(), 1.5);
    assert_eq!(event.start_ms(), 0.0015);
    assert_eq!(event.duration_ns(), 3_000);
    assert_eq!(event.duration_us(), 3.0);
    assert_eq!(event.duration_ms(), 0.003);

    event.update_end_ns(6_500);

    assert_eq!(event.span.start_ns, 1_500);
    assert_eq!(event.span.end_ns, 6_500);
    assert_eq!(event.duration_ns(), 5_000);
}

#[test]
fn perf_event_default_constructor_matches_python_default_phase() {
    let event = PerfEvent::new("instant", 10, 10);

    assert_eq!(event.phase, "X");
    assert_eq!(event.duration_ns(), 0);
    assert!(event.category.is_none());
    assert!(event.args.is_empty());
}

#[test]
fn chrome_trace_event_data_matches_python_trace_file_mapping() {
    let mut args = BTreeMap::new();
    args.insert("count".to_owned(), 3.0);
    let complete = PerfEvent::with_options(
        "draw",
        1_000,
        3_500,
        PerfEventOptions {
            process_id: Some(9),
            thread_id: Some("thread-2".to_owned()),
            args: args.clone(),
            ..PerfEventOptions::default()
        },
    );

    let data = chrome_trace_event_data(&complete).unwrap();

    assert_eq!(data.pid, 9);
    assert_eq!(data.tid, "thread-2");
    assert_eq!(data.name, "draw");
    assert_eq!(data.cat, "none");
    assert_eq!(data.ph, "X");
    assert_eq!(data.ts, 1.0);
    assert_eq!(data.args, args);
    assert_eq!(data.dur, Some(2.5));
    assert_eq!(data.scope, None);

    let instant = PerfEvent::with_options(
        "marker",
        2_000,
        2_000,
        PerfEventOptions {
            phase: "I".to_owned(),
            ..PerfEventOptions::default()
        },
    );
    assert_eq!(
        chrome_trace_event_data(&instant).unwrap().scope.as_deref(),
        Some("p")
    );

    let counter = PerfEvent::with_options(
        "counter",
        2_000,
        2_000,
        PerfEventOptions {
            phase: "C".to_owned(),
            ..PerfEventOptions::default()
        },
    );
    let counter_data = chrome_trace_event_data(&counter).unwrap();
    assert_eq!(counter_data.dur, None);
    assert_eq!(counter_data.scope, None);
}

#[test]
fn chrome_trace_event_data_rejects_unsupported_phases_like_python_assertion() {
    let event = PerfEvent::with_options(
        "bad",
        0,
        0,
        PerfEventOptions {
            phase: "B".to_owned(),
            ..PerfEventOptions::default()
        },
    );

    assert_eq!(
        chrome_trace_event_data(&event),
        Err(UnsupportedTracePhase {
            phase: "B".to_owned()
        })
    );
}

#[test]
fn perf_timers_updates_stats_for_complete_events_only_and_can_trace_events() {
    let mut timers = PerfTimers::new();
    timers.start_trace();
    timers.add_event(PerfEvent::new("draw", 0, 2_600_000));
    timers.add_event(PerfEvent::new("draw", 0, 4_100_000));
    timers.add_event(PerfEvent::with_options(
        "marker",
        0,
        0,
        PerfEventOptions {
            phase: "I".to_owned(),
            ..PerfEventOptions::default()
        },
    ));

    let stat = timers.timers.get("draw").unwrap();
    assert_eq!(stat.min, 2);
    assert_eq!(stat.max, 4);
    assert_eq!(stat.count, 2);
    assert!(!timers.timers.contains_key("marker"));

    let traced = timers.stop_trace().unwrap();
    assert_eq!(traced.len(), 3);

    timers.clear();

    assert!(timers.timers.is_empty());
}

#[test]
fn dummy_timer_ignores_events_like_python_disabled_timer() {
    let mut timer = DummyTimer;

    timer.start_trace();
    timer.add_event(PerfEvent::new("draw", 0, 1_000));
    timer.clear();

    assert_eq!(timer.stop_trace(), None);
}
