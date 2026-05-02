use std::sync::atomic::{AtomicUsize, Ordering};

static REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

pub fn next_request_id() -> usize {
    REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}
